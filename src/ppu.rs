use std::process::exit;

const PPU_INIT_TIME: u64 = 29658;
const MAX_DOT_COUNT: u32 = 283 * 242;

#[allow(unused_variables, dead_code)]
#[derive(Debug)]
pub struct Ppu {
    ppu_ctrl: u8,
    ppu_mask: u8,
    ppu_status: u8,
    oam_addr: u8,
    oam_data: u8,
    ppu_scroll: u8,
    ppu_addr: u8,
    ppu_data: u8,
    oam_dma: u8,

    cpu_cycle_count: u64,
    ppu_cycle_count: u64,
    dot_count: u64,
}


impl Ppu {
    pub fn new() -> Ppu {
        Ppu {
            ppu_ctrl: 0,
            ppu_mask: 0,
            ppu_status: 0,
            oam_addr: 0,
            oam_data: 0,
            ppu_scroll: 0,
            ppu_addr: 0,
            ppu_data: 0,
            oam_dma: 0,

            cpu_cycle_count: 0,
            ppu_cycle_count: 0,
            dot_count: 0,
        }
    }

    pub fn register(&self, register: u8) -> u8 {
        let value = match register {
            0 => self.ppu_ctrl,
            1 => self.ppu_mask,
            2 => self.ppu_status,
            3 => self.oam_addr,
            4 => self.oam_data,
            5 => self.ppu_scroll,
            6 => self.ppu_addr,
            7 => self.ppu_data,
            _ => panic!("unknown register: {register:#04X}")
        };

        // println!("PPU: get register: {register}, value: {value:#04X}");
        value
    }

    pub fn set_register(&mut self, register: u8, value: u8) {
        // delay until ppu boots i guess?
        let mut could_write = true;
        match register {
            0 => { if self.cpu_cycle_count >= PPU_INIT_TIME { self.ppu_ctrl = value } else { could_write = false } }
            1 => { if self.cpu_cycle_count >= PPU_INIT_TIME { self.ppu_mask = value } else { could_write = false } }
            2 => { self.ppu_status = value }
            3 => { self.oam_addr = value }
            4 => { self.oam_data = value }
            5 => { if self.cpu_cycle_count >= PPU_INIT_TIME { self.ppu_scroll = value } else { could_write = false } }
            6 => { if self.cpu_cycle_count >= PPU_INIT_TIME { self.ppu_addr = value } else { could_write = false } }
            7 => { self.ppu_data = value }
            _ => panic!("unknown register: {register:#04X}")
        };

        if could_write {
            println!("PPU: set register: {register}, value: {value:#04X}");
            println!("CPU cycle: {}", self.cpu_cycle_count);
            if register == 0 || register == 1 || register == 5 || register == 6 {
                exit(0);
            }
        }
    }

    pub fn step(&mut self, cycle_count: u64) {
        self.cpu_cycle_count = cycle_count;


        match self.ppu_cycle_count % 340 {
            0 => {},
            1..=256 => {

            }
            256..=320 => {

            },
            321..=336 => {

            },
            337..=340 => {

            }

            _ => {} //panic!("cycle count not handled: {}", self.ppu_cycle_count)
        }

        self.ppu_cycle_count += 1;
    }

    fn fetch(&self) {

    }
}