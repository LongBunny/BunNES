use crate::nes::ppu::Ppu;
use crate::nes::rom::Rom;

const RAM_CAP: usize = 2 * 1024;
pub(crate) type Ram = [u8; RAM_CAP];

pub struct Bus {
    ppu: Ppu,
    rom: Rom,
    ram: Ram,
}

impl Bus {
    pub fn new(rom: Rom) -> Bus {
        let ram = [0u8; RAM_CAP];
        let rom = rom;

        let ppu = Ppu::new();

        Bus {
            ram,
            rom,
            ppu,
        }
    }

    pub fn step_ppu(&mut self) {
        self.ppu.step();
    }

    pub fn rom_len(&self) -> usize {
        self.rom.prg().len()
    }

    pub fn read_8(&self, addr: u16) -> u8 {
        self.map_addr(addr)
    }

    pub fn read_16(&self, addr: u16) -> u16 {
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

    fn map_addr(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x1FFF => self.ram[(addr % 0x07FF) as usize],
            0x2000..=0x3FFF => {
                self.ppu.register((addr % 8) as u8)
            },
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
                                let mut addr = (addr as usize -0x8000) % 0x4000;
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