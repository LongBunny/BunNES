use bunNES::emulator::*;
use bunNES::nes::rom::Cartridge;
use raylib::prelude::*;
use std::fs::File;
use std::io::Read;
use std::ops::Add;
use bunNES::nes::opcodes::OpCode;

const NES_WIDTH: i32 = 256;
const NES_HEIGHT: i32 = 240;
const NES_SCALE: i32 = 4;

const DEBUG_WIDTH: i32 = 600;
const FONT_SIZE: i32 = 20;
const PADDING: i32 = 5;

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

        y = self.draw_registers(x, y, d);
        y = self.draw_disassembly(x, y, d);
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
        let width_reg = DEBUG_WIDTH / regs.len() as i32;
        for (i, reg) in regs.iter().enumerate() {
            let reg_name = reg.0;
            let reg_value = reg.1;
            self.draw_text(d, reg_name, x + (i as i32 * width_reg), y, FONT_SIZE, Color::BLACK);
            self.draw_text(d, format!("{:X}", reg_value).as_str(), x + (i as i32 * width_reg), y + FONT_SIZE + PADDING, FONT_SIZE, Color::BLACK);
        }

        y += (FONT_SIZE + PADDING) * 2;
        d.draw_line(x - PADDING, y, x + DEBUG_WIDTH, y, Color::BLACK);
        y += PADDING;


        let status_regs = [
            ("c", carry),
            ("z", zero),
            ("i", irqb),
            ("d", decimal),
            ("b", brk),
            ("o", overflow),
            ("n", negative),
        ];
        let width_status_regs = DEBUG_WIDTH / status_regs.len() as i32;
        for (i, reg) in status_regs.iter().enumerate() {
            let reg_name = reg.0;
            let reg_value = reg.1;
            self.draw_text(d, reg_name, x + (i as i32 * width_status_regs), y, FONT_SIZE, Color::BLACK);
            self.draw_text(d, format!("{:X}", reg_value).as_str(), x + (i as i32 * width_status_regs), y + FONT_SIZE + PADDING, FONT_SIZE, Color::BLACK);
        }
        
        y += (FONT_SIZE + PADDING) * 2;
        d.draw_line(x - PADDING, y, x + DEBUG_WIDTH, y, Color::BLACK);
        y += PADDING;

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

        for _ in 0..10 {
            let (y_next, size) = self.draw_instruction(location, x, y, d);
            y = y_next;
            location += size as u16;
        }
        y
    }
    
    fn draw_instruction(&mut self, location: u16, x: i32, y: i32, d: &mut RaylibDrawHandle) -> (i32, u8) {
        let mut y = y;
        let cpu = &mut self.emulator.cpu;
        
        let instruction = cpu.get_instruction(location);
        let op_code = instruction.op_code;
        let size = instruction.size;

        let mut args = String::new();
        for _ in 0..size - 1 {
            let arg = cpu.bus.read_8(location + 1);
            args.push_str(format!("{:02X} ", arg).as_str());
        }

        self.draw_text(d, &format!("{:04X}: {:02X} {}", location, instruction.byte_code, args), x, y, FONT_SIZE, Color::BLACK);
        if let Some(op_code) = op_code {
            self.draw_text(d, &format!("{}", op_code), x + 150, y, FONT_SIZE, Color::BLACK);
        } else {
            self.draw_text(d, "unknown", x, y, FONT_SIZE, Color::BLACK);
        }

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

