mod emulator;
mod nes;
mod gui;

use std::fs::File;
use std::io::Read;
use crate::emulator::Emulator;
use crate::nes::rom::Cartridge;


fn main() {
    // let file_path = "roms/tetris.nes";
    let file_path = "roms/nestest.nes";
    let mut f = File::open(file_path).unwrap();
    let mut rom_bytes = vec!();
    f.read_to_end(&mut rom_bytes).unwrap();

    let cartridge = Cartridge::new(rom_bytes);
    println!("{}", cartridge);

    let mut emulator = Emulator::new(cartridge);
    emulator.run();
}