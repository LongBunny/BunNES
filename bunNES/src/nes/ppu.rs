use std::process::exit;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use bit::BitIndex;
use bitflags::bitflags;
use rand::random;
use crate::nes::cpu::{HEIGHT, RenderImage, WIDTH};
use crate::nes::rom::Cartridge;

const PPU_INIT_TIME: u64 = 29658;
const MAX_DOT_COUNT: u32 = 283 * 242;

#[allow(unused_variables, dead_code)]
#[derive(Debug)]
pub struct Ppu {
    ppu_ctrl: PpuCtrl,
    ppu_mask: PpuMask,
    ppu_status: PpuStatus,
    oam_addr: u8,
    oam_data: u8,
    ppu_scroll: u8,
    ppu_addr: PpuAddr,
    ppu_data: PpuData,
    oam_dma: u8,

    cartridge: Arc<Cartridge>,


    cpu_cycle_count: u64,
    ppu_cycle_count: u64,
    dot_count: u64,
}

#[derive(Debug)]
struct PpuCtrl(u8);

bitflags! {
    impl PpuCtrl: u8 {
        const base_nametable   = 0b0000_0011;
        const vram_addr_inc    = 0b0000_0100;
        const spr_ptrn_addr    = 0b0000_1000;
        const bgr_ptrn_addr    = 0b0001_0000;
        const spr_size         = 0b0010_0000;
        const ppu_master_slave = 0b0100_0000;
        const gen_nmi_vblank   = 0b1000_0000;
    }
}

#[derive(Debug)]
struct PpuMask(u8);

bitflags! {
    impl PpuMask: u8 {
        const grayscale         = 0b0000_0001;
        const show_bgr_leftmost = 0b0000_0010;
        const show_spr_leftmost = 0b0000_0100;
        const show_bgr          = 0b0000_1000;
        const show_spr          = 0b0001_0000;
        const emphasize_red     = 0b0010_0000;
        const emphasize_grn     = 0b0100_0000;
        const emphasize_blu     = 0b1000_0000;
    }
}

#[derive(Debug)]
struct PpuStatus(u8);

bitflags! {
    impl PpuStatus: u8 {
        const stale_bus    = 0b0001_1111;
        const spr_oflo     = 0b0010_0000;
        const spr_0_hit    = 0b0100_0000;
        const vblank_start = 0b1000_0000;
    }
}

type PpuAddr = u8;
type PpuData = u8;

struct PatternTable {
    tbl: [u8; 256 * 16 * 2],
}

struct NameTable {
    tbl: [u8; 1024],
}

struct AttributeTable {
    tbl: [u8; 64],
}

struct OAM {
    tbl: [u8; 64 * 4],
}

struct Palette {

}


impl Ppu {
    pub fn new(cartridge: Arc<Cartridge>) -> Ppu {

        Ppu {
            ppu_ctrl: PpuCtrl(0),
            ppu_mask: PpuMask(0),
            ppu_status: PpuStatus(255),
            oam_addr: 0,
            oam_data: 0,
            ppu_scroll: 0,
            ppu_addr: 0,
            ppu_data: 0,
            oam_dma: 0,

            cartridge,


            cpu_cycle_count: 0,
            ppu_cycle_count: 0,
            dot_count: 0,
        }
    }

    pub fn register(&mut self, register: u8) -> u8 {
        let value = match register {
            0 => self.ppu_ctrl.0,
            1 => self.ppu_mask.0,
            2 => {
                
                // set vblank to false after read
                // https://www.nesdev.org/wiki/PPU_registers#PPUSTATUS
                self.ppu_status.bits().set_bit(7, false);
                self.ppu_status.0
            },
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
            0 => self.ppu_ctrl.0 = value,
            1 => self.ppu_mask.0 = value,
            2 => self.ppu_status.0 = value,
            3 => self.oam_addr = value,
            4 => self.oam_data = value,
            5 => self.ppu_scroll = value,
            6 => self.ppu_addr = value,
            7 => self.ppu_data = value,
            _ => panic!("unknown register: {register:#04X}")
        };

        // println!("PPU: set register: {register}, value: {value:#04X}");
    }

    pub fn step(&mut self, scanline: u64) {
        let scanline = scanline % 262;
        match scanline {
            0..=239 => {
                // visible scanlines
                match self.ppu_cycle_count % 340 {
                    0 => {
                        // idle cycle
                    },
                    1..=256 => {
                        // fetching data for tiles
                    },
                    257..=320 => {
                        // next scanline tile data fetch
                    },
                    321..=336 => {
                        // first two tiles for next scanline
                    },
                    337..=340 => {
                        // unknown fetches
                    },
                    _ => panic!("ppu cycle not matched: {} [{}]", self.ppu_cycle_count, self.ppu_cycle_count % 340)
                }


            },
            240 => {
            },
            241..=260 => {
                if self.ppu_cycle_count % 340 == 1 {
                    self.ppu_status.bits().set_bit(7, true);
                }
            },
            261 => {
                // or -1
                // dummy
                if self.ppu_cycle_count % 340 == 1 {
                    self.ppu_status.bits().set_bit(7, false);
                }
            }
            _ => panic!("scanline not matched: {scanline}")
        }

        self.ppu_cycle_count += 1;
    }

    fn fetch(&self) {

    }
}