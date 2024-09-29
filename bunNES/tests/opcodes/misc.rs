use bunNES::nes::opcodes::{AddrMode, OpCode};
use crate::opcodes::helpers::{get_cpu, instruction};

#[cfg(test)]
mod brk {
    use super::*;

    #[test]
    fn brk() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Brk, AddrMode::Implicit),
        ];
        let mut cpu = get_cpu(code);
        while !cpu.step() {};
        assert_eq!(cpu.ps.brk(), true);
        assert_eq!(cpu.pc, 0x0100);
        // TODO: figure out how to test this correctly
        // assert_eq!(cpu.bus.read_16(0x0100 + cpu.sp as u16), pc);
        // assert_eq!(cpu.bus.read_16(0x0100 + cpu.sp as u16 + 0x02), ps.reg);
    }
}

#[cfg(test)]
mod clc {
    use super::*;

    #[test]
    fn clc() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Clc, AddrMode::Implicit),
        ];
        let mut cpu = get_cpu(code);
        cpu.ps.set_carry(true);
        while !cpu.step() {};
        assert_eq!(cpu.ps.carry(), false);
    }
}

#[cfg(test)]
mod cld {
    use super::*;

    #[test]
    fn cld() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Cld, AddrMode::Implicit),
        ];
        let mut cpu = get_cpu(code);
        cpu.ps.set_decimal(true);
        while !cpu.step() {};
        assert_eq!(cpu.ps.decimal(), false);
    }
}

#[cfg(test)]
mod cli {
    use super::*;

    #[test]
    fn cli() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Cli, AddrMode::Implicit),
        ];
        let mut cpu = get_cpu(code);
        cpu.ps.set_irqb(true);
        while !cpu.step() {};
        assert_eq!(cpu.ps.irqb(), false);
    }
}

#[cfg(test)]
mod clv {
    use super::*;

    #[test]
    fn clv() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Clv, AddrMode::Implicit),
        ];
        let mut cpu = get_cpu(code);
        cpu.ps.set_overflow(true);
        while !cpu.step() {};
        assert_eq!(cpu.ps.overflow(), false);
    }
}
