use crate::nes::bus::Bus;
use crate::nes::cpu::{Cpu, ProcessorStatus};
use crate::nes::opcodes::{op_code_from_instruction, AddrMode, Instruction, OpCode, OP_CODES};
use crate::nes::rom::Cartridge;

#[cfg(test)]
mod opcodes_tests {
    
    
}

pub fn get_cpu(mut code: Vec<u8>) -> Cpu {
    code.resize(0x4000, 0);
    let cartridge = Cartridge::test_cartride(code);

    Cpu {
        pc: 0x8000,
        sp: 0,
        acc: 0,
        x: 0,
        y: 0,
        ps: ProcessorStatus::new(),
        bus: Bus::new(cartridge),
        cycles_to_finish: 0,
    }
}

pub fn instruction(op_code: OpCode, addr_mode: AddrMode) -> u8 {
    let instruction = Instruction {
        op_code,
        addr_mode,
        size: 1
    };
    if let Some(byte_code) =  op_code_from_instruction(instruction) {
        byte_code as u8
    } else {
        panic!("Invalid instruction")
    }
}
