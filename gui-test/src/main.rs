use bunNES::emulator::*;
use bunNES::nes::rom::Cartridge;
use raylib::prelude::*;
use std::fs::File;
use std::io::Read;
use std::ops::Add;
use bunNES::nes::opcodes::{AddrMode, OpCode};

const NES_WIDTH: i32 = 256;
const NES_HEIGHT: i32 = 240;
const NES_SCALE: i32 = 4;

const DEBUG_WIDTH: i32 = 600;
const FONT_SIZE: i32 = 20;
const PADDING: i32 = 5;
const DEBUG_DISASSEMBLY_WIDTH: i32 = 370;

type Mem = [u8; 2048];


struct Window {
    font: Font,

    emulator: Emulator,
    running: bool,
}

impl Window {
    fn new(font: Font, emulator: Emulator) -> Self {
        Self {
            font,
            emulator,
            running: false,
        }
    }


    fn run(&mut self, rl: &mut RaylibHandle, thread: RaylibThread) {
        self.emulator.reset();

        let mut debug_draw_pos = Vector2::new(0.0, 0.0);

        while !rl.window_should_close() {
            let mut d = rl.begin_drawing(&thread);

            d.clear_background(Color::PURPLE);

            if self.running {
                // fuck cycle accuracy
                self.emulator.step();
            }

            self.draw_emulator(&mut d);

            if d.is_key_pressed(KeyboardKey::KEY_S) {
                self.emulator.step();
            }
            if d.is_key_pressed(KeyboardKey::KEY_SPACE) {
                self.running = !self.running;
            }

            if d.is_key_pressed(KeyboardKey::KEY_R) {
                self.emulator.reset();
            }

            if d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                let mouse_pos = d.get_mouse_position();
                debug_draw_pos = mouse_pos;
                println!("Start:  ({}, {})", mouse_pos.x, mouse_pos.y);
            }
            if d.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
                let mouse_pos = d.get_mouse_position();
                println!("End:    ({}, {})", mouse_pos.x, mouse_pos.y);
                let diff = mouse_pos - debug_draw_pos;
                println!("Length: ({}, {})", diff.x.abs(), diff.y.abs());
            }

        }
    }

    fn draw_emulator(&mut self, d: &mut RaylibDrawHandle) {
        self.draw_nes(d);
        self.draw_debug(d);
    }

    fn draw_nes(&self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle(0, 0,
                         NES_WIDTH * NES_SCALE, NES_HEIGHT * NES_SCALE,
                         Color::BLACK);
    }

    fn draw_debug(&mut self, d: &mut RaylibDrawHandle) {
        // background
        d.draw_rectangle(NES_WIDTH * NES_SCALE, 0,
                         DEBUG_WIDTH, NES_HEIGHT * NES_SCALE,
                         Color::LIGHTPINK);


        let x = NES_WIDTH * NES_SCALE + PADDING;
        let mut y = PADDING;

        y = self.draw_disassembly(x, y, d);
        y = self.draw_registers(x + DEBUG_DISASSEMBLY_WIDTH + 50, PADDING, d);
        // y = self.draw_memory(x, y, d);
    }

    fn draw_registers(&mut self, x: i32, y: i32, d: &mut RaylibDrawHandle) -> i32 {
        let cpu = &self.emulator.cpu;

        let regs = [
            ("pc", cpu.pc),
            ("sp", cpu.sp as u16),
            ("acc", cpu.acc as u16),
            ("x", cpu.x as u16),
            ("y", cpu.y as u16)
        ];

        let carry = if cpu.ps.carry() { 1 } else { 0 };
        let zero = if cpu.ps.zero() { 1 } else { 0 };
        let irqb = if cpu.ps.irqb() { 1 } else { 0 };
        let decimal = if cpu.ps.decimal() { 1 } else { 0 };
        let brk = if cpu.ps.brk() { 1 } else { 0 };
        let overflow = if cpu.ps.overflow() { 1 } else { 0 };
        let negative = if cpu.ps.negative() { 1 } else { 0 };

        let mut y = y;
        for (reg_name, reg_value) in regs {
            self.draw_text(d, reg_name, x, y, FONT_SIZE, Color::BLACK);
            if reg_name == "pc" {
                self.draw_text(d, format!("{:04X}", reg_value).as_str(), x + 50, y, FONT_SIZE, Color::BLACK);
            } else {
                self.draw_text(d, format!("{:02X}", reg_value).as_str(), x + 50, y, FONT_SIZE, Color::BLACK);
            }
            y += FONT_SIZE + PADDING;
        }

        y += PADDING * 2;

        let status_regs = [
            ("c", carry),
            ("z", zero),
            ("i", irqb),
            ("d", decimal),
            ("b", brk),
            ("o", overflow),
            ("n", negative),
        ];
        for (reg_name, reg_value) in status_regs {
            self.draw_text(d, reg_name, x, y, FONT_SIZE, Color::BLACK);
            self.draw_text(d, format!("{}", reg_value).as_str(), x + 50, y, FONT_SIZE, Color::BLACK);
            y += FONT_SIZE + PADDING;
        }

        y
    }

    fn draw_memory(&mut self, x: i32, y: i32, d: &mut RaylibDrawHandle) -> i32 {
        let mut y = y;

        let cpu = &mut self.emulator.cpu;
        const PEEK_MEMORY_SIZE: usize = 0xFF;
        const BYTES_PER_LINE: i32 = 16;
        const LINES: i32 = PEEK_MEMORY_SIZE as i32 / BYTES_PER_LINE;

        let memory = cpu.bus.memory_chunk(cpu.pc, PEEK_MEMORY_SIZE);


        for i in 0..LINES {
            let mut str = String::new();

            for j in 0..BYTES_PER_LINE {
                str.push_str(&format!("{:02X} ", memory[(j + i * BYTES_PER_LINE) as usize]));
            }

            self.draw_text(d, &str, x, y, FONT_SIZE, Color::BLACK);
            y += FONT_SIZE + PADDING;
        }

        y
    }

    fn draw_text(&mut self, d: &mut RaylibDrawHandle, text: &str, x: i32, y: i32, font_size: i32, color: Color) {
        d.draw_text_ex(&self.font, text, Vector2 { x: x as f32, y: y as f32 }, font_size as f32, 0f32, color);
    }

    fn draw_disassembly(&mut self, x: i32, y: i32, d: &mut RaylibDrawHandle) -> i32 {
        let mut y = y;

        let cpu = &mut self.emulator.cpu;

        let mut location = cpu.pc;

        let amount_of_instructions = (((NES_HEIGHT * NES_SCALE) as f32 / (FONT_SIZE + PADDING) as f32) as i32) - 1;

        for _ in 0..amount_of_instructions {
            let (y_next, size) = self.draw_instruction(location, x, y, d);
            y = y_next;
            location += size as u16;
        }
        y
    }

    fn draw_instruction(&mut self, location: u16, x: i32, y: i32, d: &mut RaylibDrawHandle) -> (i32, u8) {
        let mut y = y;
        let cpu = &mut self.emulator.cpu;

        if location == cpu.pc {
            d.draw_rectangle(x - PADDING, y, DEBUG_DISASSEMBLY_WIDTH, FONT_SIZE, Color::new(150, 255, 0, 255));
        }

        let (instruction, byte_code) = cpu.get_instruction(location);
        
        let size: u8 = if let Some(instruction) = instruction {
            let op_code = instruction.op_code;
            let size = instruction.size;

            // machine code
            let mut operand = String::new();
            assert!(size > 0);
            for i in 1..size {
                let arg = cpu.bus.read_8(location + i as u16);
                operand.push_str(format!("{:02X} ", arg).as_str());
            }

            // disassembly
            let operand_normal = match size {
                1 => 0,
                2 => cpu.bus.read_8(location + 1) as u16,
                3 => cpu.bus.read_16(location + 1),
                _ => panic!("operand size too big: {size}")
            };

            let operand_normal = match instruction.addr_mode {
                AddrMode::Implicit => String::new(),
                AddrMode::Accumulator => String::from("A"),
                AddrMode::Immediate => format!("#{:02X}", operand_normal),
                AddrMode::Zp => format!("${:02X}", operand_normal),
                AddrMode::ZpX => format!("${:02X}, X", operand_normal),
                AddrMode::ZpY => format!("${:02X}, Y", operand_normal),
                AddrMode::Relative => {
                    let op = operand_normal as i8;
                    let jump_loc = if op >= 0 {
                        (location + 2).wrapping_add(op as u16)
                    } else {
                        (location + 2).wrapping_sub(op.wrapping_abs() as u16)
                    };
                    format!(
                        "*{}{:02} (${:04X})",
                        if op > 0 { "+" } else { "-" }, (op).abs(),
                        jump_loc
                    )
                },
                AddrMode::Absolute => format!("${:04X}", operand_normal),
                AddrMode::AbsoluteX => format!("${:04X}, X", operand_normal),
                AddrMode::AbsoluteY => format!("${:04X}, Y", operand_normal),
                AddrMode::Indirect => format!("({:04X})", operand_normal),
                AddrMode::IndirectX => format!("({:02X}, X)", operand_normal),
                AddrMode::IndirectY => format!("({:02X}), Y", operand_normal),
            };

            self.draw_text(d, &format!("{:04X}: {:02X} {}", location, byte_code, operand), x, y, FONT_SIZE, Color::BLACK);
            self.draw_text(d, &format!("{} {}", op_code, operand_normal), x + 200, y, FONT_SIZE, Color::BLACK);
            size
        } else {
            self.draw_text(d, format!("unknown: {:02X}", byte_code).as_str(), x, y, FONT_SIZE, Color::BLACK);
            1
        };

        y += FONT_SIZE + PADDING;
        (y, size)
    }
}


fn main() {
    let file_path = "roms/nestest.nes";
    let mut f = File::open(file_path).unwrap_or_else(|e| panic!("Couldn't open file: {e}"));
    let mut rom_bytes = vec!();
    f.read_to_end(&mut rom_bytes).unwrap_or_else(|e| panic!("Couldn't read file: {e}"));


    let cartridge = Cartridge::new(rom_bytes);
    println!("{}", cartridge);

    let emulator = Emulator::new(cartridge);


    let (mut rl, thread) = raylib::init()
        .size(NES_WIDTH * NES_SCALE + DEBUG_WIDTH, NES_HEIGHT * NES_SCALE)
        .title("raylib-rs example")
        .build();

    let font = rl.load_font(&thread, "assets/FiraCode-Regular.ttf").unwrap_or_else(|e| panic!("couldn't load font: {}", e));
    let mut test_memory: Mem = [0; 2048];

    for i in 0..2048 {
        test_memory[i] = (i % 256) as u8;
    }
    let mut window = Window::new(font, emulator);

    window.run(&mut rl, thread);
}

