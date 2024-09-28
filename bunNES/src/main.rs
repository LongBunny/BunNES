mod emulator;
mod nes;

use std::fs::File;
use std::io::Read;
use crate::emulator::Emulator;
use crate::nes::rom::Cartridge;


fn main() {
    let file_path = "roms/nestest.nes";
    let mut f = File::open(file_path).unwrap_or_else(|e| panic!("Couldn't open file: {e}"));
    let mut rom_bytes = vec!();
    f.read_to_end(&mut rom_bytes).unwrap_or_else(|e| panic!("Couldn't read file: {e}"));

    let cartridge = Cartridge::new(rom_bytes);
    println!("{}", cartridge);

    let mut emulator = Emulator::new(cartridge);
    emulator.run();
    
}