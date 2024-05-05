use std::thread::sleep;
use std::time::{Duration, Instant};
use bit::BitIndex;
use crate::nes::bus::Bus;
use crate::nes::opcodes::{AddrMode, OP_CODES, OpCode};
use crate::nes::rom::Rom;

struct ProcessorStatus {
    /// [0] carry
    /// [1] zero
    /// [2] irqb disable
    /// [3] decimal
    /// [4] break
    /// [5] unavailable
    /// [6] overflow
    /// [7] negative
    reg: u8,
}

impl ProcessorStatus {
    fn new() -> ProcessorStatus {
        ProcessorStatus {
            reg: 0
        }
    }

    fn carry(&self) -> bool {
        self.reg.bit(0)
    }

    fn set_carry(&mut self, value: bool) {
        self.reg.set_bit(0, value);
    }

    fn zero(&self) -> bool {
        self.reg.bit(1)
    }

    fn set_zero(&mut self, value: bool) {
        self.reg.set_bit(1, value);
    }

    fn irqb(&self) -> bool {
        self.reg.bit(2)
    }

    fn set_irqb(&mut self, value: bool) {
        self.reg.set_bit(2, value);
    }

    fn decimal(&self) -> bool {
        self.reg.bit(3)
    }

    fn set_decimal(&mut self, value: bool) {
        self.reg.set_bit(3, value);
    }

    fn brk(&self) -> bool {
        self.reg.bit(4)
    }

    fn set_brk(&mut self, value: bool) {
        self.reg.set_bit(4, value);
    }

    fn overflow(&self) -> bool {
        self.reg.bit(6)
    }

    fn set_overflow(&mut self, value: bool) {
        self.reg.set_bit(6, value);
    }

    fn negative(&self) -> bool {
        self.reg.bit(7)
    }

    fn set_negative(&mut self, value: bool) {
        self.reg.set_bit(7, value);
    }

}

struct Step {
    cycles: u8,
    pc_inc: u8,
}

impl Step {
    fn next(bytes: u8, cycles: u8) -> Step {
        Step {
            cycles,
            pc_inc: bytes
        }
    }
}

#[allow(unused_variables, dead_code)]
pub struct Cpu {
    /// program counter
    pc: u16,
    /// stack pointer
    sp: u8,
    /// accumulator
    acc: u8,
    /// index register x
    x: u8,
    /// index register y
    y: u8,
    /// processor status
    ps: ProcessorStatus,

    bus: Bus,

    cycles_to_finish: u8,
}


impl Cpu {
    pub fn new(rom: Rom) -> Cpu {
        Cpu {
            pc: 0,
            sp: 0,
            acc: 0,
            x: 0,
            y: 0,
            ps: ProcessorStatus::new(),

            bus: Bus::new(rom),

            cycles_to_finish: 0,
        }
    }

    pub fn reset(&mut self) {
        println!("reset!");
        println!("rom size: {}", self.bus.rom_len());
        let reset: u16 = self.bus.read_8(0xFFFC) as u16 | (self.bus.read_8(0xFFFD) as u16) << 8;
        println!("reset vector: {:#04X}", reset);
        self.set_pc(reset);
    }

    pub fn run(&mut self) {
        let time_per_frame: Duration = Duration::from_secs_f32(1f32 / 60f32);
        let mut now = Instant::now();
        let mut frame_count: u64 = 0;
        // frame every 16ms (60 fps)
        loop {

            // EMULATOR OWNS CPU, CPU OWNS BUS, BUS OWNS REST EZ
            // NO MORE CIRCULAR SHITTYNESS


            // 262 scanlines per frame
            for scanline in 0..262 {
                // 341 ppu cycles per scanline
                for ppu_cycle in 0..341 {
                    if ppu_cycle % 3 == 0 {
                        self.step();
                    }
                    self.bus.step_ppu(scanline);
                }
            }


            frame_count += 1;

            // if frame_count > 100 {
            //     println!("100 frames");
            //     break;
            // }

            let time_elapsed = now.elapsed();
            // println!("time elapsed: {:.3}ms", time_elapsed.as_secs_f32() * 1000f32);
            if time_elapsed < time_per_frame {
                sleep(time_per_frame - time_elapsed);
            }
            now = Instant::now();
        }

    }


    pub fn set_pc(&mut self, value: u16) {
        self.pc = value;
    }


    fn set_carry(&mut self, reg: u8, value: u8) {
        self.ps.set_carry(reg >= value);
    }

    fn set_zero(&mut self, value: u8) {
        self.ps.set_zero(value == 0);
    }

    fn set_negative(&mut self, value: u8) {
        self.ps.set_negative(value.bit(7) == true);
    }

    pub fn step(&mut self) {
        if self.cycles_to_finish > 0 {
            self.cycles_to_finish -= 1;
            return;
        }

        let op_code = self.bus.read_8(self.pc);
        let inst = OP_CODES[op_code as usize];
        // println!("pc: {:#04X} op_code: {:#04X?} op: {:?}", self.pc, op_code, inst);
        let step = match inst {
            Some(OpCode::Sei) => {
                self.sei()
            }
            Some(OpCode::Cld) => {
                self.cld()
            }
            Some(OpCode::Cpx(addr_mode)) => {
                self.cpx(addr_mode)
            },
            Some(OpCode::Bne) => {
                self.bne()
            },
            Some(OpCode::Bpl(AddrMode::Relative)) => {
                self.bpl()
            }
            Some(OpCode::Txs) => {
                self.txs()
            }
            Some(OpCode::Inx) => {
                self.inx()
            }
            Some(OpCode::Dex) => {
                self.dex()
            }
            Some(OpCode::Dey) => {
                self.dey()
            }
            Some(OpCode::Ldx(addr_mode)) => {
                self.ldx(addr_mode)
            }
            Some(OpCode::Ldy(addr_mode)) => {
                self.ldy(addr_mode)
            }
            Some(OpCode::Lda(addr_mode)) => {
                self.lda(addr_mode)
            }
            Some(OpCode::Sta(addr_mode)) => {
                self.sta(addr_mode)
            }
            Some(OpCode::Stx(addr_mode)) => {
                self.stx(addr_mode)
            }
            _ => panic!("unknown instruction: {:#04X}: {op_code:#04X} {inst:?}", self.pc)
        };
        self.pc += step.pc_inc as u16;
        self.cycles_to_finish = step.cycles;
    }

    /// set interrupt disable
    fn sei(&mut self) -> Step {
        self.ps.set_irqb(false);
        Step::next(1, 2)
    }

    fn cld(&mut self) -> Step {
        self.ps.set_decimal(false);
        Step::next(1, 2)
    }

    fn txs(&mut self) -> Step {
            self.sp = self.x;
            Step::next(1, 2)
    }

    fn inx(&mut self) -> Step {
        self.x = self.x.wrapping_add(1);
        self.set_zero(self.x);
        self.set_negative(self.x);
        Step::next(1, 2)
    }

    fn dex(&mut self) -> Step {
        self.x = self.x.wrapping_sub(1);
        self.set_zero(self.x);
        self.set_negative(self.x);
        Step::next(1, 2)
    }

    fn dey(&mut self) -> Step {
        self.y = self.y.wrapping_sub(1);
        self.set_zero(self.y);
        self.set_negative(self.y);
        Step::next(1, 2)
    }

    fn cpx(&mut self, addr_mode: AddrMode) -> Step {
        let (value, step) = match addr_mode {
            AddrMode::Immediate => {
                let value = self.bus.read_8(self.pc + 1);
                (value ,Step::next(2, 2))
            },
            _ => panic!("unimplemented: cpx {addr_mode:?}")
        };
        self.set_carry(self.x, value);
        self.set_zero(value);
        self.set_negative(value);
        step
    }

    fn bne(&mut self) -> Step {
        let offset: i8 = unsafe {
            std::mem::transmute(self.bus.read_8(self.pc + 1))
        };

        if self.ps.zero() == false {
            // this is cheating :)
            let mut addr = self.pc as i32;
            addr += offset as i32;
            self.pc = addr as u16;
        }
        // todo: cycles can be 2, 3 or 4
        // https://www.nesdev.org/obelisk-6502-guide/reference.html#BNE
        Step::next(2, 2)
    }

    fn bpl(&mut self) -> Step {
        let offset: i8 = unsafe {
            std::mem::transmute(self.bus.read_8(self.pc + 1))
        };

        if self.ps.negative() == false {
            // this is cheating :)
            let mut addr = self.pc as i32;
             addr += offset as i32;
            self.pc = addr as u16;
        }
        // todo: cycles can be 2, 3 or 4
        // https://www.nesdev.org/obelisk-6502-guide/reference.html#BPL
        Step::next(2, 2)
    }

    fn ldx(&mut self, addr_mode: AddrMode) -> Step {
        match addr_mode {
            AddrMode::Immediate => {
                let value = self.bus.read_8(self.pc + 1);
                self.x = value;
                self.set_zero(value);
                self.set_negative(value);
                Step::next(2, 2)
            }
            _ => panic!("unimplemented: lda {addr_mode:?}")
        }
    }

    fn ldy(&mut self, addr_mode: AddrMode) -> Step {
        match addr_mode {
            AddrMode::Immediate => {
                let value = self.bus.read_8(self.pc + 1);
                self.y = value;
                self.set_zero(value);
                self.set_negative(value);
                Step::next(2, 2)
            }
            _ => panic!("unimplemented: lda {addr_mode:?}")
        }
    }

    fn lda(&mut self, addr_mode: AddrMode) -> Step {
        let (value, step) = match addr_mode {
            AddrMode::Absolute => {
                let addr = self.bus.read_16(self.pc + 1);
                let value = self.bus.read_8(addr);
                (value, Step::next(3, 4))
            },
            AddrMode::AbsoluteX => {
                let addr = self.bus.read_16(self.pc + 1);
                let (addr, extra_step) = self.add_absolute_x(addr);
                let value = self.bus.read_8(addr);
                (value, Step::next(3, 4 + extra_step))
            },
            AddrMode::Immediate => {
                let value = self.bus.read_8(self.pc + 1);
                (value, Step::next(2, 2))
            },
            _ => panic!("unimplemented: lda {addr_mode:?}")
        };

        self.acc = value;
        self.set_zero(value);
        self.set_negative(value);
        step
    }

    fn sta(&mut self, addr_mode: AddrMode) -> Step {
        let (value, step) = match addr_mode {
            AddrMode::Absolute => {
                let addr = self.bus.read_16(self.pc + 1);
                (addr, Step::next(3, 4))
            },
            _ => panic!("unimplemented: lda {addr_mode:?}")
        };

        self.bus.write(value, self.acc);
        step
    }

    fn stx(&mut self, addr_mode: AddrMode) -> Step {
        let (value, step) = match addr_mode {
            AddrMode::Absolute => {
                let addr = self.bus.read_16(self.pc + 1);
                (addr, Step::next(3, 4))
            },
            _ => panic!("unimplemented: lda {addr_mode:?}")
        };

        self.bus.write(value, self.x);
        step
    }


    /// returns address with x offset and if it needed an extra cycle
    fn add_absolute_x(&self, addr: u16) -> (u16, u8) {
        let page_index = addr % 0xFF;
        let addr = addr.wrapping_add(self.x as u16);
        let extra_step = if page_index != (addr % 0xFF) { 1 } else { 0 };
        (addr, extra_step)
    }
}