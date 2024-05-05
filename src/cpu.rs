use std::rc::Rc;
use bit::BitIndex;
use crate::bus::Bus;
use crate::opcodes::{AddrMode, OP_CODES, OpCode};

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
    fn next(cycles: u8, pc_inc: u8) -> Step {
        Step {
            cycles,
            pc_inc
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
}


impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            pc: 0,
            sp: 0,
            acc: 0,
            x: 0,
            y: 0,
            ps: ProcessorStatus::new(),
        }
    }

    pub fn step(&mut self, bus: Rc<Bus>) {
        let op_code = bus.read(self.pc);
        let inst = OP_CODES[op_code as usize];
        println!("pc: {:#04X} op_code: {:#04X?} op: {:?}", self.pc, op_code, inst);
        let step = match inst {
            Some(OpCode::Sei(addrMode)) => {
                self.sei(addrMode)
            }
            _ => panic!("unknown instruction: {op_code:#04X} {inst:?}")
        };
        self.pc += step.pc_inc as u16;
    }

    pub fn set_pc(&mut self, value: u16) {
        self.pc = value;
    }

    /// set interrupt disable
    fn sei(&mut self, addr_mode: AddrMode) -> Step {
        match addr_mode {
            AddrMode::Implicit => {
                self.ps.set_irqb(false);
                Step::next(2, 1)
            }
            _ => panic!("unimplemented: SEI {addr_mode:?}")
        }
    }
}