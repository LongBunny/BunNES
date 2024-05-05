use crate::nes::cpu::Cpu;
use crate::nes::rom::Rom;

pub struct Emulator {
    cpu: Cpu
}

impl Emulator {
    pub fn new(rom: Rom) -> Emulator {
        Emulator {
            cpu: Cpu::new(rom)
        }
    }

    pub fn run(mut self) {
        self.cpu.reset();
        self.cpu.run();
    }
}
