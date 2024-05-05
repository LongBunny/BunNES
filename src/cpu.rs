use std::cell::RefCell;
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

    bus: Rc<RefCell<Bus>>
}


impl Cpu {
    pub fn new(bus: Rc<RefCell<Bus>>) -> Cpu {
        Cpu {
            pc: 0,
            sp: 0,
            acc: 0,
            x: 0,
            y: 0,
            ps: ProcessorStatus::new(),

            bus,
        }
    }

    pub fn set_pc(&mut self, value: u16) {
        self.pc = value;
    }



    fn set_zero(&mut self, value: u8) {
        self.ps.set_zero(value == 0);
    }

    fn set_negative(&mut self, value: u8) {
        self.ps.set_negative(value.bit(7) == true);
    }




    pub fn step(&mut self) {
        let op_code = self.bus.borrow().read_8(self.pc);
        let inst = OP_CODES[op_code as usize];
        println!("pc: {:#04X} op_code: {:#04X?} op: {:?}", self.pc, op_code, inst);
        let step = match inst {
            Some(OpCode::Sei) => {
                self.sei()
            }
            Some(OpCode::Cld) => {
                self.cld()
            }
            Some(OpCode::Txs) => {
                self.txs()
            }
            Some(OpCode::Ldx(addr_mode)) => {
                self.ldx(addr_mode)
            }
            Some(OpCode::Lda(addr_mode)) => {
                self.lda(addr_mode)
            }
            Some(OpCode::Sta(addr_mode)) => {
                self.sta(addr_mode)
            }
            Some(OpCode::Bpl(AddrMode::Relative)) => {
                self.bpl()
            }
            _ => panic!("unknown instruction: {op_code:#04X} {inst:?}")
        };
        self.pc += step.pc_inc as u16;
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

    fn bpl(&mut self) -> Step {
        let offset: i8 = unsafe {
            std::mem::transmute(self.bus.borrow().read_8(self.pc + 1))
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
                let value = self.bus.borrow().read_8(self.pc + 1);
                self.x = value;
                self.set_zero(value);
                self.set_negative(value);
                Step::next(2, 2)
            }
            _ => panic!("unimplemented: lda {addr_mode:?}")
        }
    }

    fn lda(&mut self, addr_mode: AddrMode) -> Step {
        let result = match addr_mode {
            AddrMode::Absolute => {
                let addr = self.bus.borrow().read_16(self.pc + 1);
                let value = self.bus.borrow().read_8(addr);
                (value, Step::next(3, 4))
            },
            AddrMode::Immediate => {
                let value = self.bus.borrow().read_8(self.pc + 1);
                (value, Step::next(2, 2))
            },
            _ => panic!("unimplemented: lda {addr_mode:?}")
        };

        self.acc = result.0;
        self.set_zero(result.0);
        self.set_negative(result.0);
        result.1
    }

    fn sta(&mut self, addr_mode: AddrMode) -> Step {
        let result = match addr_mode {
            AddrMode::Absolute => {
                let addr = self.bus.borrow().read_16(self.pc + 1);
                (addr, Step::next(3, 4))
            },
            _ => panic!("unimplemented: lda {addr_mode:?}")
        };

        self.bus.borrow_mut().write(result.0, self.acc);
        result.1
    }
}