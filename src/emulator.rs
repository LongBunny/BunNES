use std::cell::RefCell;
use std::rc::Rc;
use crate::bus::Bus;
use crate::cpu::Cpu;
use crate::ppu::Ppu;
use crate::rom::Rom;


const RAM_CAP: usize = 2 * 1024;
pub(crate) type Ram = [u8; RAM_CAP];

pub struct Emulator {
    cpu: Cpu,
    rom: Rc<Rom>,
    ram: Rc<Ram>,
    ppu: Rc<RefCell<Ppu>>,
    bus: Rc<RefCell<Bus>>
}

impl Emulator {
    pub fn new(rom: Rom) -> Emulator {
        let ram = Rc::new([0u8; RAM_CAP]);
        let rom = Rc::new(rom);
        let ppu = Rc::new(RefCell::new(Ppu::new()));
        let bus = Rc::new(RefCell::new(Bus::new(ram.clone(), rom.clone(), ppu.clone())));


        let cpu = Cpu::new(bus.clone());

        Emulator {
            rom,
            ram,
            cpu,
            ppu,
            bus
        }
    }

    pub fn run(mut self) {
        println!("prg size: {}", self.rom.prg().len());

        let reset: u16 = self.bus.borrow().read_8(0xFFFC) as u16 | (self.bus.borrow().read_8(0xFFFD) as u16) << 8;
        println!("reset vector: {:#04X}", reset);
        self.cpu.set_pc(reset);

        let mut count = 0;
        loop {
            self.cpu.step();
            count += 1;
            if count >= 100 {
                println!("100 iterations pag");
                break;
            }
        }
    }

    fn parse_rom(&mut self) {

    }
}







