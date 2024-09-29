use crate::nes::bus::Bus;
use crate::nes::opcodes::{AddrMode, Instruction, OpCode, OP_CODES};
use crate::nes::rom::Cartridge;
use bit::BitIndex;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::{Duration, Instant};

pub const WIDTH: u32 = 256 + 100;
pub const HEIGHT: u32 = 240;

pub type RenderImage = Vec<u8>;

#[derive(Eq, PartialEq)]
pub struct ProcessorStatus {
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
    pub fn new() -> ProcessorStatus {
        ProcessorStatus {
            reg: 0
        }
    }

    pub fn carry(&self) -> bool {
        self.reg.bit(0)
    }

    pub fn set_carry(&mut self, value: bool) {
        self.reg.set_bit(0, value);
    }

    pub fn zero(&self) -> bool {
        self.reg.bit(1)
    }

    pub fn set_zero(&mut self, value: bool) {
        self.reg.set_bit(1, value);
    }

    pub fn irqb(&self) -> bool {
        self.reg.bit(2)
    }

    pub fn set_irqb(&mut self, value: bool) {
        self.reg.set_bit(2, value);
    }

    pub fn decimal(&self) -> bool {
        self.reg.bit(3)
    }

    pub fn set_decimal(&mut self, value: bool) {
        self.reg.set_bit(3, value);
    }

    pub fn brk(&self) -> bool {
        self.reg.bit(4)
    }

    pub fn set_brk(&mut self, value: bool) {
        self.reg.set_bit(4, value);
    }

    pub fn overflow(&self) -> bool {
        self.reg.bit(6)
    }

    pub fn set_overflow(&mut self, value: bool) {
        self.reg.set_bit(6, value);
    }

    pub fn negative(&self) -> bool {
        self.reg.bit(7)
    }

    pub fn set_negative(&mut self, value: bool) {
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
            pc_inc: bytes,
        }
    }
}

#[allow(unused_variables, dead_code)]
pub struct Cpu {
    /// program counter
    pub pc: u16,
    /// stack pointer
    pub sp: u8,
    /// accumulator
    pub acc: u8,
    /// index register x
    pub x: u8,
    /// index register y
    pub y: u8,
    /// processor status
    pub ps: ProcessorStatus,

    pub bus: Bus,

    pub cycles_to_finish: u8,
}


impl Cpu {
    pub fn new(cartridge: Cartridge) -> Cpu {
        Cpu {
            pc: 0,
            sp: 0,
            acc: 0,
            x: 0,
            y: 0,
            ps: ProcessorStatus::new(),

            bus: Bus::new(cartridge),

            cycles_to_finish: 0,
        }
    }
    
    pub fn soft_reset(&mut self) {
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

            {
                // println!("ppu render to image");

            }


            frame_count += 1;

            // if frame_count > 100 {
            //     println!("100 frames");
            //     break;
            // }

            let time_elapsed = now.elapsed();
            // println!("time elapsed: {:.3}ms", time_elapsed.as_secs_f32() * 1000f32);

            // let image = self.bus.ppu_image();

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

    pub fn step(&mut self) -> bool {
        if self.cycles_to_finish > 0 {
            self.cycles_to_finish -= 1;
            return false;
        }

        let (instruction, byte_code) = self.get_instruction(self.pc);

        let step: Step = if let Some(instruction) = instruction {
            let addr_mode = instruction.addr_mode;
            match instruction.op_code {
                OpCode::Sei => {
                    self.sei()
                }
                OpCode::Cld => {
                    self.cld()
                }
                OpCode::Cpx => {
                    self.cpx(addr_mode)
                }
                OpCode::Bne => {
                    self.bne()
                }
                OpCode::Bpl => {
                    self.bpl()
                }
                OpCode::Txs => {
                    self.txs()
                }
                OpCode::Inx => {
                    self.inx()
                }
                OpCode::Dex => {
                    self.dex()
                }
                OpCode::Dey => {
                    self.dey()
                }
                OpCode::Ldx => {
                    self.ldx(addr_mode)
                }
                OpCode::Ldy => {
                    self.ldy(addr_mode)
                }
                OpCode::Lda => {
                    self.lda(addr_mode)
                }
                OpCode::Sta => {
                    self.sta(addr_mode)
                }
                OpCode::Stx => {
                    self.stx(addr_mode)
                }
                _ => {
                    unimplemented!("opcode is not implemented yet: {} {}", instruction.op_code, instruction.addr_mode)
                }
            }
        } else {
            panic!("unknown instruction: {:#04X}: {:#04X}", self.pc, byte_code)
        };

        self.pc += step.pc_inc as u16;
        self.cycles_to_finish = step.cycles;

        true
    }

    pub fn get_instruction(&mut self, pc: u16) -> (Option<Instruction>, u8) {
        let byte_code = self.bus.read_8(pc);
        let instruction = OP_CODES[byte_code as usize];
        (instruction, byte_code)
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
                (value, Step::next(2, 2))
            }
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
            }
            AddrMode::AbsoluteX => {
                let addr = self.bus.read_16(self.pc + 1);
                let (addr, extra_step) = self.add_absolute_x(addr);
                let value = self.bus.read_8(addr);
                (value, Step::next(3, 4 + extra_step))
            }
            AddrMode::Immediate => {
                let value = self.bus.read_8(self.pc + 1);
                (value, Step::next(2, 2))
            }
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
            }
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
            }
            _ => panic!("unimplemented: lda {addr_mode:?}")
        };

        self.bus.write(value, self.x);
        step
    }


    /// returns address with x offset and if it needed an extra cycle
    fn add_absolute_x(&self, addr: u16) -> (u16, u8) {
        let page_index = (addr & 0xFF00) / 256;
        let addr = addr.wrapping_add(self.x as u16);
        let extra_step = if page_index != ((addr & 0xFF00) / 256) { 1 } else { 0 };
        (addr, extra_step)
    }
}