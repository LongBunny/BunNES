use crate::nes::bus::Bus;
use crate::nes::opcodes::{AddrMode, Instruction, OpCode, OP_CODES};
use crate::nes::rom::Cartridge;
use bit::BitIndex;
use std::thread::sleep;
use std::time::{Duration, Instant};

pub const WIDTH: u32 = 256 + 100;
pub const HEIGHT: u32 = 240;

pub type RenderImage = Vec<u8>;

type ExtraStep = u8;

#[derive(Eq, PartialEq, Debug)]
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

impl Default for ProcessorStatus {
    fn default() -> Self {
        Self::new()
    }
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

    pub fn get_reg(&self) -> u8 { self.reg }
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
            sp: 0xFF,
            acc: 0,
            x: 0,
            y: 0,
            ps: ProcessorStatus::default(),

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
        self.ps.set_negative(value.bit(7));
    }
    
    fn set_overflow(&mut self, value: u8, old_value: u8) {
        let overflow = value.bit(7) != old_value.bit(7);
        self.ps.set_overflow(overflow);
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
                OpCode::Adc => self.adc(addr_mode),
                OpCode::And => self.and(addr_mode),
                OpCode::Asl => self.asl(addr_mode),
                OpCode::Bcc => self.bcc(),
                OpCode::Bcs => self.bcs(),
                OpCode::Beq => self.beq(),
                OpCode::Bit => self.bit(addr_mode),
                OpCode::Bmi => self.bmi(),
                OpCode::Bne => self.bne(),
                OpCode::Bpl => self.bpl(),
                OpCode::Brk => self.brk(),
                OpCode::Bvc => self.bvc(),
                OpCode::Bvs => self.bvs(),
                OpCode::Clc => self.clc(),
                OpCode::Cld => self.cld(),
                OpCode::Cli => self.cli(),
                OpCode::Clv => self.clv(),
                OpCode::Cmp => self.cmp(addr_mode),
                OpCode::Cpx => self.cpx(addr_mode),
                OpCode::Cpy => self.cpy(addr_mode),
                OpCode::Dec => self.dec(addr_mode),
                OpCode::Dex => self.dex(),
                OpCode::Dey => self.dey(),
                OpCode::Eor => self.eor(addr_mode),
                OpCode::Inc => self.inc(addr_mode),
                OpCode::Inx => self.inx(),
                OpCode::Iny => self.iny(),
                OpCode::Jmp => self.jmp(addr_mode),
                OpCode::Jsr => self.jsr(),
                OpCode::Lda => self.lda(addr_mode),
                OpCode::Ldx => self.ldx(addr_mode),
                OpCode::Ldy => self.ldy(addr_mode),
                OpCode::Lsr => self.lsr(addr_mode),
                OpCode::Nop => self.nop(),
                OpCode::Ora => self.ora(addr_mode),
                OpCode::Pha => self.pha(),
                OpCode::Php => self.php(),
                OpCode::Pla => self.pla(),
                OpCode::Plp => self.plp(),
                OpCode::Rol => self.rol(addr_mode),
                OpCode::Ror => self.ror(addr_mode),
                OpCode::Rti => self.rti(),
                OpCode::Rts => self.rts(),
                OpCode::Sbc => self.sbc(addr_mode),
                OpCode::Sec => self.sec(),
                OpCode::Sed => self.sed(),
                OpCode::Sei => self.sei(),
                OpCode::Sta => self.sta(addr_mode),
                OpCode::Stx => self.stx(addr_mode),
                OpCode::Sty => self.sty(addr_mode),
                OpCode::Tax => self.tax(),
                OpCode::Tay => self.tay(),
                OpCode::Tsx => self.tsx(),
                OpCode::Txs => self.txs(),
                OpCode::Txa => self.txa(),
                OpCode::Tya => self.tya(),
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
    
    // instructions
    
    fn adc(&mut self, addr_mode: AddrMode) -> Step {
        let (value, step) = match addr_mode {
            AddrMode::Immediate => {
                let value = self.bus.read_8(self.pc + 1);
                (value, Step::next(2, 2))
            }
            AddrMode::Zp => {
                let arg = self.bus.read_8(self.pc + 1);
                let value = self.value_zp(arg);
                (value, Step::next(2, 3))
            }
            AddrMode::ZpX => {
                let arg = self.bus.read_8(self.pc + 1);
                let value = self.value_zp_offset(arg, self.x);
                (value, Step::next(2, 4))
            }
            AddrMode::Absolute => {
                let arg = self.bus.read_16(self.pc + 1);
                let value = self.bus.read_8(arg);
                (value, Step::next(2, 4))
            }
            AddrMode::AbsoluteX => {
                let arg = self.bus.read_16(self.pc + 1);
                let (addr, extra_step) = self.addr_absolute_with_offset(arg, self.x as u16);
                let value = self.bus.read_8(addr);
                (value, Step::next(2, 4 + extra_step))
            }
            AddrMode::AbsoluteY => {
                let arg = self.bus.read_16(self.pc + 1);
                let (addr, extra_step) = self.addr_absolute_with_offset(arg, self.y as u16);
                let value = self.bus.read_8(addr);
                (value, Step::next(2, 4 + extra_step))
            }
            AddrMode::IndirectX => {
                let addr = self.addr_indirect_x();
                let value = self.bus.read_8(addr);
                (value, Step::next(2, 6))
            }
            AddrMode::IndirectY => {
                let (addr, extra_step) = self.addr_indirect_y();
                let value = self.bus.read_8(addr);
                (value, Step::next(2, 5 + extra_step))
            }
            _ => panic!("unknown addr_mode: adc {addr_mode:?}")
        };
        
        let prev_acc = self.acc;
        self.acc = self.acc.wrapping_add(value);
        if self.ps.carry() {
            self.acc = self.acc.wrapping_add(1);
        }
        self.set_zero(self.acc);
        self.set_carry(prev_acc, self.acc);
        self.set_overflow(self.acc, prev_acc);
        self.set_negative(self.acc);
        step
    }
    
    fn and(&mut self, addr_mode: AddrMode) -> Step {
        let (value, step) = match addr_mode {
            AddrMode::Immediate => {
                let value = self.bus.read_8(self.pc + 1);
                (value, Step::next(2, 2))
            }
            AddrMode::Zp => {
                let arg = self.bus.read_8(self.pc + 1);
                let value = self.value_zp(arg);
                (value, Step::next(2, 3))
            }
            AddrMode::ZpX => {
                let arg = self.bus.read_8(self.pc + 1);
                let value = self.value_zp_offset(arg, self.x);
                (value, Step::next(2, 4))
            }
            AddrMode::Absolute => {
                let arg = self.bus.read_16(self.pc + 1);
                let value = self.bus.read_8(arg);
                (value, Step::next(3, 4))
            }
            AddrMode::AbsoluteX => {
                let arg = self.bus.read_16(self.pc + 1);
                let (addr, extra_step) = self.addr_absolute_with_offset(arg, self.x as u16);
                let value = self.bus.read_8(addr);
                (value, Step::next(3, 4 + extra_step))
            }
            AddrMode::AbsoluteY => {
                let arg = self.bus.read_16(self.pc + 1);
                let (addr, extra_step) = self.addr_absolute_with_offset(arg, self.y as u16);
                let value = self.bus.read_8(addr);
                (value, Step::next(3, 4 + extra_step))
            }
            AddrMode::IndirectX => {
                let addr = self.addr_indirect_x();
                let value = self.bus.read_8(addr);
                (value, Step::next(2, 6))
            }
            AddrMode::IndirectY => {
                let (addr, extra_step) = self.addr_indirect_y();
                let value = self.bus.read_8(addr);
                (value, Step::next(2, 5 + extra_step))
            }
            _ => panic!("unknown addr_mode: adc {addr_mode:?}")
        };
        
        self.acc &= value;
        self.set_zero(self.acc);
        self.set_negative(self.acc);
        step
    }
    
    fn asl(&mut self, addr_mode: AddrMode) -> Step {
        if addr_mode == AddrMode::Accumulator {
            self.ps.set_carry(self.acc.bit(7));
            self.acc <<= 1;
            
            self.set_zero(self.acc);
            self.set_negative(self.acc);
            Step::next(1, 2)
        } else {
            let (addr, step) = match addr_mode {
                AddrMode::Zp => {
                    let addr = self.bus.read_8(self.pc + 1) as u16;
                    (addr, Step::next(2, 5))
                }
                AddrMode::ZpX => {
                    let addr = self.bus.read_8(self.pc + 1);
                    let addr = addr.wrapping_add(self.x) as u16;
                    (addr, Step::next(2, 6))
                }
                AddrMode::Absolute => {
                    let addr = self.bus.read_16(self.pc + 1);
                    (addr, Step::next(2, 6))
                }
                AddrMode::AbsoluteX => {
                    let arg = self.bus.read_16(self.pc + 1);
                    let (addr, _) = self.addr_absolute_with_offset(arg, self.x as u16);
                    (addr, Step::next(2, 7))
                }
                _ => panic!("unknown addr_mode: asl {addr_mode:?}")
            };
            
            let mut value = self.bus.read_8(addr);
            self.ps.set_carry(value.bit(7));
            value <<= 1;
            self.set_negative(value);
            self.bus.write(addr, value);
            
            step
        }
    }
    
    fn bcc(&mut self, ) -> Step {
        unimplemented!()
    }
    
    fn bcs(&mut self, ) -> Step {
        unimplemented!()
    }
    
    fn beq(&mut self, ) -> Step {
        unimplemented!()
    }
    
    fn bit(&mut self, addr_mode: AddrMode) -> Step {
        unimplemented!()
    }
    
    fn bmi(&mut self, ) -> Step {
        unimplemented!()
    }
    
    fn bne(&mut self) -> Step {
        if self.ps.zero() == false {
            let offset: i8 = unsafe {
                std::mem::transmute(self.bus.read_8(self.pc + 1))
            };
            
            let mut addr = self.pc as i32;
            addr += offset as i32;
            self.pc = addr as u16;
        }
        // todo: cycles can be 2, 3 or 4
        // https://www.nesdev.org/obelisk-6502-guide/reference.html#BNE
        Step::next(2, 2)
    }
    
    fn bpl(&mut self) -> Step {
        if self.ps.negative() == false {
            let offset: i8 = unsafe {
                std::mem::transmute(self.bus.read_8(self.pc + 1))
            };
            
            let mut addr = self.pc as i32;
            addr += offset as i32;
            self.pc = addr as u16;
        }
        // todo: cycles can be 2, 3 or 4
        // https://www.nesdev.org/obelisk-6502-guide/reference.html#BPL
        Step::next(2, 2)
    }
    
    fn brk(&mut self) -> Step {
        unimplemented!()
    }
    
    fn bvc(&mut self) -> Step {
        unimplemented!()
    }
    
    fn bvs(&mut self) -> Step {
        unimplemented!()
    }
    
    fn clc(&mut self) -> Step {
        self.ps.set_carry(false);
        Step::next(1, 2)
    }
    
    fn cld(&mut self) -> Step {
        self.ps.set_decimal(false);
        Step::next(1, 2)
    }
    
    fn cli(&mut self) -> Step {
        self.ps.set_irqb(false);
        Step::next(1, 2)
    }
    
    fn clv(&mut self) -> Step {
        self.ps.set_overflow(false);
        Step::next(1, 2)
    }
    
    fn cmp(&mut self, addr_mode: AddrMode) -> Step {
        unimplemented!()
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
    
    fn cpy(&mut self, addr_mode: AddrMode) -> Step {
        unimplemented!()
    }
    
    fn dec(&mut self, addr_mode: AddrMode) -> Step {
        let (addr, step) = match addr_mode {
            AddrMode::Zp => {
                let addr = self.bus.read_8(self.pc + 1) as u16;
                (addr, Step::next(2, 5))
            }
            AddrMode::ZpX => {
                let addr = self.bus.read_8(self.pc + 1);
                let addr = addr.wrapping_add(self.x) as u16;
                (addr, Step::next(2, 6))
            }
            AddrMode::Absolute => {
                let addr = self.bus.read_16(self.pc + 1);
                (addr, Step::next(3, 6))
            }
            AddrMode::AbsoluteX => {
                let addr = self.bus.read_16(self.pc + 1);
                let addr = addr.wrapping_add(self.x as u16);
                (addr, Step::next(3, 7))
            }
            _ => panic!("unimplemented: cpx {addr_mode:?}")
        };
        
        let mut value = self.bus.read_8(addr);
        
        value = value.wrapping_sub(1);
        
        self.set_zero(value);
        self.set_negative(value);
        
        self.bus.write(addr, value);
        
        step
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
    
    fn eor(&mut self, addr_mode: AddrMode) -> Step {
        unimplemented!()
    }
    
    fn inc(&mut self, addr_mode: AddrMode) -> Step {
        let (addr, step) = match addr_mode {
            AddrMode::Zp => {
                let addr = self.bus.read_8(self.pc + 1) as u16;
                (addr, Step::next(2, 5))
            }
            AddrMode::ZpX => {
                let addr = self.bus.read_8(self.pc + 1);
                let addr = addr.wrapping_add(self.x) as u16;
                (addr, Step::next(2, 6))
            }
            AddrMode::Absolute => {
                let addr = self.bus.read_16(self.pc + 1);
                (addr, Step::next(3, 6))
            }
            AddrMode::AbsoluteX => {
                let addr = self.bus.read_16(self.pc + 1);
                let addr = addr.wrapping_add(self.x as u16);
                (addr, Step::next(3, 7))
            }
            _ => panic!("unimplemented: cpx {addr_mode:?}")
        };
        
        let mut value = self.bus.read_8(addr);
        
        value = value.wrapping_add(1);
        
        self.set_zero(value);
        self.set_negative(value);
        
        self.bus.write(addr, value);
        
        step
    }
    
    fn inx(&mut self) -> Step {
        self.x = self.x.wrapping_add(1);
        self.set_zero(self.x);
        self.set_negative(self.x);
        Step::next(1, 2)
    }
    
    fn iny(&mut self) -> Step {
        self.y = self.y.wrapping_add(1);
        self.set_zero(self.y);
        self.set_negative(self.y);
        Step::next(1, 2)
    }
    
    fn jmp(&mut self, addr_mode: AddrMode) -> Step {
        unimplemented!()
    }
    
    fn jsr(&mut self) -> Step {
        unimplemented!()
    }
    
    fn lda(&mut self, addr_mode: AddrMode) -> Step {
        let (value, step) = match addr_mode {
            AddrMode::Immediate => {
                let value = self.bus.read_8(self.pc + 1);
                (value, Step::next(2, 2))
            }
            AddrMode::Zp => {
                let addr = self.bus.read_8(self.pc + 1);
                let value = self.value_zp(addr);
                (value, Step::next(2, 3))
            }
            AddrMode::ZpX => {
                let addr = self.bus.read_8(self.pc + 1);
                let value = self.value_zp_offset(addr, self.x);
                (value, Step::next(2, 4))
            }
            AddrMode::Absolute => {
                let addr = self.bus.read_16(self.pc + 1);
                let value = self.bus.read_8(addr);
                (value, Step::next(3, 4))
            }
            AddrMode::AbsoluteX => {
                let addr = self.bus.read_16(self.pc + 1);
                let (addr, extra_step) = self.addr_absolute_with_offset(addr, self.x as u16);
                let value = self.bus.read_8(addr);
                (value, Step::next(3, 4 + extra_step))
            }
            AddrMode::AbsoluteY => {
                let addr = self.bus.read_16(self.pc + 1);
                let (addr, extra_step) = self.addr_absolute_with_offset(addr, self.y as u16);
                let value = self.bus.read_8(addr);
                (value, Step::next(3, 4 + extra_step))
            }
            AddrMode::IndirectX => {
                let addr = self.addr_indirect_x();
                let value = self.bus.read_8(addr);
                (value, Step::next(2, 6))
            }
            AddrMode::IndirectY => {
                let (addr, extra_step) = self.addr_indirect_y();
                let value = self.bus.read_8(addr);
                (value, Step::next(2, 5 + extra_step))
            }
            _ => panic!("unimplemented: lda {addr_mode:?}")
        };
        
        self.acc = value;
        self.set_zero(value);
        self.set_negative(value);
        step
    }
    
    fn ldx(&mut self, addr_mode: AddrMode) -> Step {
        let (value, step) = match addr_mode {
            AddrMode::Immediate => {
                let value = self.bus.read_8(self.pc + 1);
                (value, Step::next(2, 2))
            }
            AddrMode::Zp => {
                let addr = self.bus.read_8(self.pc + 1);
                let value = self.value_zp(addr);
                (value, Step::next(2, 3))
            }
            AddrMode::ZpY => {
                let addr = self.bus.read_8(self.pc + 1);
                let value = self.value_zp_offset(addr, self.y);
                (value, Step::next(2, 4))
            }
            AddrMode::Absolute => {
                let addr = self.bus.read_16(self.pc + 1);
                let value = self.bus.read_8(addr);
                (value, Step::next(3, 4))
            }
            AddrMode::AbsoluteY => {
                let addr = self.bus.read_16(self.pc + 1);
                let (addr, extra_step) = self.addr_absolute_with_offset(addr, self.y as u16);
                let value = self.bus.read_8(addr);
                (value, Step::next(3, 4 + extra_step))
            }
            _ => panic!("unimplemented: lda {addr_mode:?}")
        };
        
        self.x = value;
        self.set_zero(value);
        self.set_negative(value);
        step
    }
    
    fn ldy(&mut self, addr_mode: AddrMode) -> Step {
        let (value, step) = match addr_mode {
            AddrMode::Immediate => {
                let value = self.bus.read_8(self.pc + 1);
                (value, Step::next(2, 2))
            }
            AddrMode::Zp => {
                let addr = self.bus.read_8(self.pc + 1);
                let value = self.value_zp(addr);
                (value, Step::next(2, 3))
            }
            AddrMode::ZpX => {
                let addr = self.bus.read_8(self.pc + 1);
                let value = self.value_zp_offset(addr, self.x);
                (value, Step::next(2, 4))
            }
            AddrMode::Absolute => {
                let addr = self.bus.read_16(self.pc + 1);
                let value = self.bus.read_8(addr);
                (value, Step::next(3, 4))
            }
            AddrMode::AbsoluteX => {
                let addr = self.bus.read_16(self.pc + 1);
                let (addr, extra_step) = self.addr_absolute_with_offset(addr, self.x as u16);
                let value = self.bus.read_8(addr);
                (value, Step::next(3, 4 + extra_step))
            }
            _ => panic!("unimplemented: lda {addr_mode:?}")
        };
        
        self.y = value;
        self.set_zero(value);
        self.set_negative(value);
        step
    }
    
    fn lsr(&mut self, addr_mode: AddrMode) -> Step {
        if addr_mode == AddrMode::Accumulator {
            self.ps.set_carry(self.acc.bit(0));
            self.acc >>= 1;
            
            self.set_zero(self.acc);
            self.set_negative(self.acc);
            Step::next(1, 2)
        } else {
            let (addr, step) = match addr_mode {
                AddrMode::Zp => {
                    let addr = self.bus.read_8(self.pc + 1) as u16;
                    (addr, Step::next(2, 5))
                }
                AddrMode::ZpX => {
                    let addr = self.bus.read_8(self.pc + 1);
                    let addr = addr.wrapping_add(self.x) as u16;
                    (addr, Step::next(2, 6))
                }
                AddrMode::Absolute => {
                    let addr = self.bus.read_16(self.pc + 1);
                    (addr, Step::next(2, 6))
                }
                AddrMode::AbsoluteX => {
                    let arg = self.bus.read_16(self.pc + 1);
                    let (addr, _) = self.addr_absolute_with_offset(arg, self.x as u16);
                    (addr, Step::next(2, 7))
                }
                _ => panic!("unknown addr_mode: lsr {addr_mode:?}")
            };
            
            let mut value = self.bus.read_8(addr);
            self.ps.set_carry(value.bit(0));
            value >>= 1;
            self.set_negative(value);
            self.bus.write(addr, value);
            
            step
        }
    }
    
    fn nop(&self) -> Step {
        Step::next(1, 2)
    }
    
    fn ora(&mut self, addr_mode: AddrMode) -> Step {
        unimplemented!()
    }
    
    fn pha(&mut self) -> Step {
        unimplemented!();
        Step::next(1, 3)
    }
    
    fn php(&mut self) -> Step {
        unimplemented!();
        Step::next(1, 3)
    }
    
    fn pla(&mut self) -> Step {
        unimplemented!();
        // self.set_zero(value);
        // self.set_negative(value);
        Step::next(1, 4)
    }
    
    fn plp(&mut self) -> Step {
        unimplemented!();
        // self.set_zero(value);
        // self.set_negative(value);
        Step::next(1, 4)
    }
    
    fn rol(&mut self, addr_mode: AddrMode) -> Step {
        if addr_mode == AddrMode::Accumulator {
            let carry = self.ps.carry();
            self.ps.set_carry(self.acc.bit(7));
            self.acc <<= 1;
            self.acc.set_bit(0, carry);
            self.set_zero(self.acc);
            self.set_negative(self.acc);
            Step::next(1, 2)
        } else {
            let (addr, step) = match addr_mode {
                AddrMode::Zp => {
                    let addr = self.bus.read_8(self.pc + 1) as u16;
                    (addr, Step::next(2, 5))
                }
                AddrMode::ZpX => {
                    let addr = self.bus.read_8(self.pc + 1);
                    let addr = addr.wrapping_add(self.x) as u16;
                    (addr, Step::next(2, 6))
                }
                AddrMode::Absolute => {
                    let addr = self.bus.read_16(self.pc + 1);
                    (addr, Step::next(3, 6))
                }
                AddrMode::AbsoluteX => {
                    let arg = self.bus.read_16(self.pc + 1);
                    let (addr, _) = self.addr_absolute_with_offset(arg, self.x as u16);
                    (addr, Step::next(3, 7))
                }
                _ => panic!("unknown addr_mode: rol {addr_mode:?}")
            };
            
            let mut value = self.bus.read_8(addr);
            let carry = self.ps.carry();
            self.ps.set_carry(value.bit(7));
            value <<= 1;
            value.set_bit(0, carry);
            self.set_zero(value);
            self.set_negative(value);
            self.bus.write(addr, value);
            
            step
        }
    }
    
    fn ror(&mut self, addr_mode: AddrMode) -> Step {
        if addr_mode == AddrMode::Accumulator {
            let carry = self.ps.carry();
            self.ps.set_carry(self.acc.bit(0));
            self.acc >>= 1;
            self.acc.set_bit(7, carry);
            self.set_zero(self.acc);
            self.set_negative(self.acc);
            Step::next(1, 2)
        } else {
            let (addr, step) = match addr_mode {
                AddrMode::Zp => {
                    let addr = self.bus.read_8(self.pc + 1) as u16;
                    (addr, Step::next(2, 5))
                }
                AddrMode::ZpX => {
                    let addr = self.bus.read_8(self.pc + 1);
                    let addr = addr.wrapping_add(self.x) as u16;
                    (addr, Step::next(2, 6))
                }
                AddrMode::Absolute => {
                    let addr = self.bus.read_16(self.pc + 1);
                    (addr, Step::next(3, 6))
                }
                AddrMode::AbsoluteX => {
                    let arg = self.bus.read_16(self.pc + 1);
                    let (addr, _) = self.addr_absolute_with_offset(arg, self.x as u16);
                    (addr, Step::next(3, 7))
                }
                _ => panic!("unknown addr_mode: rol {addr_mode:?}")
            };
            
            let mut value = self.bus.read_8(addr);
            let carry = self.ps.carry();
            self.ps.set_carry(value.bit(0));
            value >>= 1;
            value.set_bit(7, carry);
            self.set_zero(value);
            self.set_negative(value);
            self.bus.write(addr, value);
            
            step
        }
    }
    
    fn rti(&mut self, ) -> Step {
        unimplemented!()
    }
    
    fn rts(&mut self, ) -> Step {
        unimplemented!()
    }
    
    fn sbc(&mut self, addr_mode: AddrMode) -> Step {
        let (value, step) = match addr_mode {
            AddrMode::Immediate => {
                let value = self.bus.read_8(self.pc + 1);
                (value, Step::next(2, 2))
            }
            AddrMode::Zp => {
                let arg = self.bus.read_8(self.pc + 1);
                let value = self.value_zp(arg);
                (value, Step::next(2, 3))
            }
            AddrMode::ZpX => {
                let arg = self.bus.read_8(self.pc + 1);
                let value = self.value_zp_offset(arg, self.x);
                (value, Step::next(2, 4))
            }
            AddrMode::Absolute => {
                let arg = self.bus.read_16(self.pc + 1);
                let value = self.bus.read_8(arg);
                (value, Step::next(2, 4))
            }
            AddrMode::AbsoluteX => {
                let arg = self.bus.read_16(self.pc + 1);
                let (addr, extra_step) = self.addr_absolute_with_offset(arg, self.x as u16);
                let value = self.bus.read_8(addr);
                (value, Step::next(2, 4 + extra_step))
            }
            AddrMode::AbsoluteY => {
                let arg = self.bus.read_16(self.pc + 1);
                let (addr, extra_step) = self.addr_absolute_with_offset(arg, self.y as u16);
                let value = self.bus.read_8(addr);
                (value, Step::next(2, 4 + extra_step))
            }
            AddrMode::IndirectX => {
                let addr = self.addr_indirect_x();
                let value = self.bus.read_8(addr);
                (value, Step::next(2, 6))
            }
            AddrMode::IndirectY => {
                let (addr, extra_step) = self.addr_indirect_y();
                let value = self.bus.read_8(addr);
                (value, Step::next(2, 5 + extra_step))
            }
            _ => panic!("unknown addr_mode: adc {addr_mode:?}")
        };
        // TODO: yek
        let prev_acc = self.acc;
        self.acc = self.acc.wrapping_sub(value);
        if !self.ps.carry() {
            self.acc = self.acc.wrapping_sub(1);
        }
        self.set_zero(self.acc);
        self.set_carry(prev_acc, self.acc);
        self.set_overflow(self.acc, prev_acc);
        self.set_negative(self.acc);
        step
    }
    
    
    fn sec(&mut self) -> Step {
        self.ps.set_carry(true);
        Step::next(1, 2)
    }
    
    fn sed(&mut self) -> Step {
        self.ps.set_decimal(true);
        Step::next(1, 2)
    }
    
    fn sei(&mut self) -> Step {
        self.ps.set_irqb(true);
        Step::next(1, 2)
    }
    
    fn sta(&mut self, addr_mode: AddrMode) -> Step {
        let (value, step) = match addr_mode {
            AddrMode::Zp => {
                let addr = self.bus.read_8(self.pc + 1) as u16;
                (addr, Step::next(2, 3))
            }
            AddrMode::ZpX => {
                let addr = self.bus.read_8(self.pc + 1);
                let addr = addr.wrapping_add(self.x) as u16;
                (addr, Step::next(2, 3))
            }
            AddrMode::Absolute => {
                let addr = self.bus.read_16(self.pc + 1);
                (addr, Step::next(3, 4))
            }
            AddrMode::AbsoluteX => {
                let addr = self.bus.read_16(self.pc + 1);
                let addr = addr.wrapping_add(self.x as u16);
                (addr, Step::next(3, 5))
            }
            AddrMode::AbsoluteY => {
                let addr = self.bus.read_16(self.pc + 1);
                let addr = addr.wrapping_add(self.y as u16);
                (addr, Step::next(3, 5))
            }
            AddrMode::IndirectX => {
                let addr = self.addr_indirect_x();
                (addr, Step::next(2, 6))
            }
            AddrMode::IndirectY => {
                let (addr, _) = self.addr_indirect_y();
                (addr, Step::next(2, 6))
            }
            _ => panic!("unimplemented: lda {addr_mode:?}")
        };
        
        self.bus.write(value, self.acc);
        step
    }
    
    fn stx(&mut self, addr_mode: AddrMode) -> Step {
        let (value, step) = match addr_mode {
            AddrMode::Zp => {
                let addr = self.bus.read_8(self.pc + 1) as u16;
                (addr, Step::next(2, 4))
            }
            AddrMode::ZpY => {
                let addr = self.bus.read_8(self.pc + 1);
                let addr = addr.wrapping_add(self.y) as u16;
                (addr, Step::next(2, 4))
            }
            AddrMode::Absolute => {
                let addr = self.bus.read_16(self.pc + 1);
                (addr, Step::next(3, 4))
            }
            _ => panic!("unimplemented: lda {addr_mode:?}")
        };
        
        self.bus.write(value, self.x);
        step
    }
    
    fn sty(&mut self, addr_mode: AddrMode) -> Step {
        let (value, step) = match addr_mode {
            AddrMode::Zp => {
                let addr = self.bus.read_8(self.pc + 1) as u16;
                (addr, Step::next(2, 4))
            }
            AddrMode::ZpX => {
                let addr = self.bus.read_8(self.pc + 1);
                let addr = addr.wrapping_add(self.x) as u16;
                (addr, Step::next(2, 4))
            }
            AddrMode::Absolute => {
                let addr = self.bus.read_16(self.pc + 1);
                (addr, Step::next(3, 4))
            }
            _ => panic!("unimplemented: lda {addr_mode:?}")
        };
        
        self.bus.write(value, self.y);
        step
    }
    
    fn tax(&mut self) -> Step {
        self.x = self.acc;
        self.set_zero(self.x);
        self.set_negative(self.x);
        Step::next(1, 2)
    }
    
    fn tay(&mut self) -> Step {
        self.y = self.acc;
        self.set_zero(self.y);
        self.set_negative(self.y);
        Step::next(1, 2)
    }
    
    fn tsx(&mut self) -> Step {
        self.x = self.sp;
        self.set_zero(self.x);
        self.set_negative(self.x);
        Step::next(1, 2)
    }
    
    fn txa(&mut self) -> Step {
        self.acc = self.x;
        self.set_zero(self.acc);
        self.set_negative(self.acc);
        Step::next(1, 2)
    }
    
    fn txs(&mut self) -> Step {
        self.sp = self.x;
        Step::next(1, 2)
    }
    
    fn tya(&mut self) -> Step {
        self.acc = self.y;
        self.set_zero(self.acc);
        self.set_negative(self.acc);
        
        Step::next(1, 2)
    }
    
    
    // helper
    fn value_zp(&self, addr: u8) -> u8 {
        self.value_zp_offset(addr, 0)
    }
    
    fn value_zp_offset(&self, addr: u8, offset: u8) -> u8 {
        let addr = addr.wrapping_add(offset);
        self.bus.ram[addr as usize]
    }
    
    /// returns address with offset and if it needed an extra cycle on page boundary cross
    fn addr_absolute_with_offset(&self, addr: u16, offset: u16) -> (u16, ExtraStep) {
        let page_index = (addr & 0xFF00) / 256;
        let addr = addr.wrapping_add(offset);
        let extra_step = if page_index != ((addr & 0xFF00) / 256) { 1 } else { 0 };
        (addr, extra_step)
    }
    
    fn addr_indirect_x(&mut self) -> u16 {
        let addr = self.bus.read_8(self.pc + 1);
        let addr = addr.wrapping_add(self.x);
        self.bus.read_16(addr as u16)
        
    }
    
    fn addr_indirect_y(&mut self) -> (u16, ExtraStep) {
        let param = self.bus.read_16(self.pc + 1);
        let fetched_addr = self.bus.read_16(param);
        let addr = fetched_addr + self.y as u16;
        let page_index = (addr & 0xFF00) / 256;
        let extra_step = if page_index != ((addr & 0xFF00) / 256) { 1 } else { 0 };
        (addr, extra_step)
    }
}
