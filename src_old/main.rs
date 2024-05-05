use std::{
    fs::File,
    io::{BufReader, ErrorKind, Read},
};

pub mod cpu;

fn main() {
    let mut rom: [u8; 64 * 1024] = [0; 64 * 1024];

    let file = File::open("../a.out").unwrap();
    let mut reader = BufReader::new(file);

    loop {
        if let Err(e) = reader.read_exact(&mut rom) {
            if e.kind() == ErrorKind::UnexpectedEof {
                break;
            }
        }
    }

    let mut cpu = CPU::new(rom);
    cpu.run();

    println!("{}", cpu);
}