use std::sync::{Arc, Mutex};
use std::thread;
use crate::nes::cpu::{Cpu, HEIGHT, RenderImage, WIDTH};
use crate::nes::rom::Cartridge;


pub struct Emulator {
    pub cpu: Cpu,
}

impl Emulator {
    pub fn new(cartridge: Cartridge) -> Emulator {
        Emulator {
            cpu: Cpu::new(cartridge),
        }
    }
    
    pub fn reset(&mut self) {
        // probably a good idea
        self.cpu.soft_reset();
    }

    pub fn run(mut self) {

        self.cpu.soft_reset();

        let threads = thread::spawn(move || {
            self.cpu.run();
        });
        
        threads.join().unwrap();
        
    }
    
    pub fn step(&mut self) {
        while !self.cpu.step() {}
    }
}
