use std::{
    fs::File,
    io::{BufReader, ErrorKind, Read},
};

use crate::cpu::CPU;

pub mod cpu;

pub fn run() {
    let mut rom: [u8; 64 * 1024] = [0; 64 * 1024];

    let file = File::open("vasm/a.out").unwrap();
    let mut reader = BufReader::new(file);

    loop {
        if let Err(e) = reader.read_exact(&mut rom) {
            if e.kind() == ErrorKind::UnexpectedEof {
                break;
            }
        }
    }

    // for i in 0..32 {
    //     // println!("{:#04X}", rom[i]);
    // }

    let mut cpu = CPU::new(rom);
    cpu.run();

    println!("{}", cpu);
}
