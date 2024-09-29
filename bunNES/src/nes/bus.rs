use crate::nes::cpu::RenderImage;
use crate::nes::ppu::Ppu;
use crate::nes::rom::Cartridge;
use std::sync::{Arc, Mutex};

const RAM_CAP: usize = 2 * 1024;
pub(crate) type Ram = [u8; RAM_CAP];

pub struct Bus {
    pub ppu: Ppu,
    rom: Arc<Cartridge>,
    pub ram: Ram,
}

impl Bus {
    pub fn new(cartridge: Cartridge) -> Bus {
        let ram = [0u8; RAM_CAP];
        let rom = Arc::new(cartridge);

        let ppu = Ppu::new(rom.clone());

        Bus {
            ram,
            rom,
            ppu,
        }
    }

    pub fn step_ppu(&mut self, scanline: u64) {
        self.ppu.step(scanline);
    }

    pub fn rom_len(&self) -> usize {
        self.rom.prg().len()
    }

    pub fn read_8(&mut self, addr: u16) -> u8 {
        self.map_addr(addr)
    }

    pub fn read_16(&mut self, addr: u16) -> u16 {
        let lsb = self.map_addr(addr);
        let msb = self.map_addr(addr + 1);

        ((msb as u16) << 8) | lsb as u16
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x2000..=0x3FFF => {
                self.ppu.set_register((addr % 8) as u8, value);
            }
            _ => unimplemented!("write for addr [{:#04X}]", addr)
        }
    }
    
    pub fn memory_chunk(&mut self, start_addr: u16, size: usize) -> Vec<u8> {
        let mut result = vec![0; size];
        for i in 0..size {
            let addr = start_addr + i as u16;
                result[i] = self.map_addr(addr);
        }
        result
    }

    fn map_addr(&mut self, addr: u16) -> u8 {
        // TODO: replace with mapper
        // mapper 000 hardcoded
        match addr {
            // ram
            0x0000..=0x1FFF => self.ram[(addr % 0x07FF) as usize],
            // ppu
            0x2000..=0x3FFF => {
                self.ppu.register((addr % 8) as u8)
            },
            // apu
            0x4000..=0x4017 => unimplemented!("APU"),
            // apu and io
            0x4018..=0x401F => panic!("APU and IO. Should be disabled"),
            // rom (cartridge)
            0x4020..=0xFFFF => {
                match addr {
                    0x6000..=0x7FFF => panic!("Cartridge: Family Basic only"),
                    0x8000..=0xFFFF => {

                        match self.rom.rom_len() {
                            0x4000 => {
                                // 16k rom
                                // subtract rom location and mirror upper 0x4000 bytes
                                let addr = (addr as usize -0x8000) % 0x4000;
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