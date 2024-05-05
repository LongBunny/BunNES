#[allow(unused_variables, dead_code)]
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
        }
    }

    pub fn register(&self, register: u8) -> u8 {
        match register {
            0 => self.ppu_ctrl,
            1 => self.ppu_mask,
            2 => self.ppu_status,
            3 => self.oam_addr,
            4 => self.oam_data,
            5 => self.ppu_scroll,
            6 => self.ppu_addr,
            7 => self.ppu_data,
            _ => panic!("unknown register: {register:#04X}")
        }
    }
}