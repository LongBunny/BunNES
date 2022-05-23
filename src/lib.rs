use std::{
    fs::File,
    io::{BufReader, ErrorKind, Read},
};

use crate::cpu::CPU;

pub mod cpu;

pub fn run(filename: String) {
    let mut rom: [u8; 64 * 1024] = [0; 64 * 1024];

    let file = File::open(filename).unwrap();
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

#[no_mangle]
pub extern "C" fn ext_run() {
    run(String::from("a.out"));
}