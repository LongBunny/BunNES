use bunNES::nes::opcodes::{AddrMode, OpCode};
use crate::opcodes::helpers::{get_cpu, instruction};


#[cfg(test)]
mod bcc {
    use super::*;

    #[test]
    fn bcc() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Bcc, AddrMode::Relative),
            0x00, 0x01,
            instruction(OpCode::Bcc, AddrMode::Relative),
            0x00, 0x01,
        ];
        let mut cpu = get_cpu(code);
        cpu.ps.set_carry(true);
        while !cpu.step() {};
        assert_eq!(cpu.pc, 0x3);
        cpu.ps.set_carry(false);
        while !cpu.step() {};
        assert_eq!(cpu.pc, 0x0100);
    }
}

#[cfg(test)]
mod bcs {
    use super::*;

    #[test]
    fn bcs() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Bcs, AddrMode::Relative),
            0x00, 0x01,
            instruction(OpCode::Bcs, AddrMode::Relative),
            0x00, 0x01,
        ];
        let mut cpu = get_cpu(code);
        cpu.ps.set_carry(false);
        while !cpu.step() {};
        assert_eq!(cpu.pc, 0x3);
        cpu.ps.set_carry(true);
        while !cpu.step() {};
        assert_eq!(cpu.pc, 0x0100);
    }
}

#[cfg(test)]
mod beq {
    use super::*;

    #[test]
    fn beq() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Beq, AddrMode::Relative),
            0x00, 0x01,
            instruction(OpCode::Beq, AddrMode::Relative),
            0x00, 0x01,
        ];
        let mut cpu = get_cpu(code);
        cpu.ps.set_zero(false);
        while !cpu.step() {};
        assert_eq!(cpu.pc, 0x3);
        cpu.ps.set_zero(true);
        while !cpu.step() {};
        assert_eq!(cpu.pc, 0x0100);
    }
}
