use std::rc::Rc;
use crate::rom::Rom;

pub struct Bus {
    ram: Rc<crate::emulator::Ram>,
    rom: Rc<Rom>,
}

impl Bus {
    pub fn new(ram: Rc<crate::emulator::Ram>, rom: Rc<Rom>) -> Bus {
        Bus {
            ram,
            rom
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.map_addr(addr)
    }

    pub fn write(&self, addr: u16) {
        unimplemented!("bus write is not implemented")
    }

    fn map_addr(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x07FF => self.ram[addr as usize],
            0x0800..=0x1FFF => unimplemented!("RAM mirrors"),
            0x2000..=0x2007 => unimplemented!("PPU"),
            0x2008..=0x3FFF => unimplemented!("PPU mirrors"),
            0x4000..=0x4017 => unimplemented!("APU"),
            0x4018..=0x401F => panic!("APU and IO. Should be disabled"),
            0x4020..=0xFFFF => {
                match addr {
                    0x6000..=0x7FFF => panic!("Cartridge: Family Basic only"),
                    0x8000..=0xFFFF => {

                        match self.rom.rom_len() {
                            0x4000 => {
                                // 16k rom
                                // subtract rom location and mirror upper 0x4000 bytes
                                let mut addr = addr as usize -0x8000;
                                if addr > 0x4000 {
                                    addr -= 0x4000;
                                }
                                self.rom.prg()[addr]
                            },
                            0x8000 => {
                                // 32k rom
                                // subtract rom location
                                let addr = addr as usize - 0x8000;
                                self.rom.prg()[addr]
                            },
                            _ => panic!("Cartridge: Rom: no matching address")
                        }
                    }
                    _ => panic!("Cartridge: no matching address")
                }
            },
            _ => panic!("no matching address")
        }
    }
}