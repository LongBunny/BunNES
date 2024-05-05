use std::rc::Rc;
use crate::bus::Bus;
use crate::cpu::Cpu;
use crate::rom::Rom;


const RAM_CAP: usize = 2 * 1024;
pub(crate) type Ram = [u8; RAM_CAP];

pub struct Emulator {
    cpu: Cpu,
    rom: Rc<Rom>,
    ram: Rc<Ram>,
    bus: Rc<Bus>
}

impl Emulator {
    pub fn new(rom: Rom) -> Emulator {
        let ram = Rc::new([0u8; RAM_CAP]);
        let rom = Rc::new(rom);
        let cpu = Cpu::new();
        let bus = Rc::new(Bus::new(ram.clone(), rom.clone()));

        Emulator {
            rom,
            ram,
            cpu,
            bus
        }
    }

    pub fn run(mut self) {
        println!("prg size: {}", self.rom.prg().len());

        let reset: u16 = self.bus.read(0xFFFC) as u16 | (self.bus.read(0xFFFD) as u16) << 8;
        println!("reset vector: {:#04X}", reset);
        self.cpu.set_pc(reset);

        loop {
            self.cpu.step(self.bus.clone());
        }
    }

    fn parse_rom(&mut self) {

    }
}







