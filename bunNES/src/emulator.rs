use std::sync::{Arc, Mutex};
use std::thread;
use crate::nes::cpu::{Cpu, HEIGHT, RenderImage, WIDTH};
use crate::nes::rom::Cartridge;


pub struct Emulator {
    pub cpu: Cpu,
    image: Arc<Mutex<RenderImage>>,
}

impl Emulator {
    pub fn new(cartridge: Cartridge) -> Emulator {
        let image: Arc<Mutex<RenderImage>> = Arc::new(Mutex::new([0; (WIDTH * HEIGHT * 4) as usize].to_vec()));

        Emulator {
            cpu: Cpu::new(cartridge, image.clone()),
            image
        }
    }
    
    pub fn reset(&mut self) {
        // probably a good idea
        self.cpu.reset();
    }

    pub fn run(mut self) {

        self.cpu.reset();

        let threads = thread::spawn(move || {
            self.cpu.run();
        });
        
        threads.join().unwrap();
        
    }
    
    pub fn step(&mut self) {
        self.cpu.step();
    }
}
