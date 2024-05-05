use std::cell::RefCell;
use std::rc::Rc;
use std::thread::sleep;
use std::time::{Duration, Instant};
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


        let time_per_frame: Duration = Duration::from_secs_f32(1f32 / 60f32);
        let mut now = Instant::now();
        let mut frame_count: u64 = 0;
        let mut cpu_cycles: u64 = 0;

        // frame every 16ms (60 fps)
        loop {


            // 262 scanlines per frame
            for scanline in 0..262 {
                // 341 ppu cycles per scanline
                for ppu_cycle in 0..341 {
                    if ppu_cycle % 3 == 0 {
                        self.cpu.step();
                        cpu_cycles += 1;
                    }
                    self.ppu.borrow_mut().step(cpu_cycles);
                }
            }


            frame_count += 1;

            if frame_count > 100 {
                println!("100 frames");
                break;
            }

            let time_elapsed = now.elapsed();
            println!("time elapsed: {:.3}ms", time_elapsed.as_secs_f32() * 1000f32);
            if time_elapsed < time_per_frame {
                sleep(time_per_frame - time_elapsed);
            }
            now = Instant::now();
        }
    }
}
