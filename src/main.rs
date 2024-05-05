mod emulator;
mod nes;

use std::fs::File;
use std::io::Read;
use crate::emulator::Emulator;
use crate::nes::rom::Rom;


fn main() {
    // let file_path = "roms/tetris.nes";
    let file_path = "roms/nestest.nes";
    let mut f = File::open(file_path).unwrap();
    let mut rom_bytes = vec!();
    f.read_to_end(&mut rom_bytes).unwrap();

    let rom = Rom::new(rom_bytes);
    println!("{}", rom);

    let mut emulator = Emulator::new(rom);
    emulator.run();
}