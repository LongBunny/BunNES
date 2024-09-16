use bunNES::emulator::*;
use bunNES::nes::rom::Cartridge;
use raylib::prelude::*;
use std::fs::File;
use std::io::Read;

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

        self.draw_text(d, "Test", x, y, FONT_SIZE, Color::BLACK);
        y += FONT_SIZE + PADDING;
        self.draw_text(d, "Test 2", x, y, FONT_SIZE, Color::BLACK);
        y += FONT_SIZE + PADDING;

        self.draw_registers(x, &mut y, d);
        self.draw_memory(x, &mut y, d);
    }

    fn draw_registers(&mut self, x: i32, y: &mut i32, d: &mut RaylibDrawHandle) {
        let cpu = &self.emulator.cpu;
        let regs = format!(
            "pc: {} sp: {} acc: {} x: {} y: {}",
            cpu.pc, cpu.sp, cpu.acc, cpu.x as u16, cpu.y as u16
        );


        let carry = if cpu.ps.carry() { 1 } else { 0 };
        let zero = if cpu.ps.zero() { 1 } else { 0 };
        let irqb = if cpu.ps.irqb() { 1 } else { 0 };
        let decimal = if cpu.ps.decimal() { 1 } else { 0 };
        let brk = if cpu.ps.brk() { 1 } else { 0 };
        let overflow = if cpu.ps.overflow() { 1 } else { 0 };
        let negative = if cpu.ps.negative() { 1 } else { 0 };

        let ps = format!(
            "c: {} z: {} i: {} d: {} b: {} o: {} n: {}",
            carry, zero, irqb, decimal, brk, overflow, negative
        );

        self.draw_text(d, &regs, x, *y, FONT_SIZE, Color::BLACK);
        *y += FONT_SIZE + PADDING;
        self.draw_text(d, &ps, x, *y, FONT_SIZE, Color::BLACK);
        *y += FONT_SIZE + PADDING;
    }

    fn draw_memory(&mut self, x: i32, y: &mut i32, d: &mut RaylibDrawHandle) {
        for i in 0..16 {

            // let mem_slice = &self.mem[i * 16..(i + 1) * 16];
            // let formatted_values: Vec<String> = mem_slice.iter().map(|val| format!("{:02X}", val)).collect();
            // let s = formatted_values.join(" ");
            //
            // self.draw_text(d, &s, x, *y, FONT_SIZE, Color::BLACK);
            //
            // *y += FONT_SIZE;

        }
    }

    fn draw_text(&mut self, d: &mut RaylibDrawHandle, text: &str, x: i32, y: i32, font_size: i32, color: Color) {
        d.draw_text_ex(&self.font, text, Vector2 { x: x as f32, y: y as f32 }, font_size as f32, 0f32, color);
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

