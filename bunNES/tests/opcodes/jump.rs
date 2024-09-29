use bunNES::nes::opcodes::{AddrMode, OpCode};
use crate::opcodes::helpers::{get_cpu, instruction};


#[cfg(test)]
mod bcc {
    use super::*;

    #[test]
    fn bcc() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Bcc, AddrMode::Relative),
            0x10,
            instruction(OpCode::Bcc, AddrMode::Relative),
            0x10,
        ];
        let mut cpu = get_cpu(code);
        cpu.ps.set_carry(true);
        let initial_pc = cpu.pc;
        while !cpu.step() {};
        assert_eq!(cpu.pc, initial_pc + 0x02);
        cpu.ps.set_carry(false);
        let initial_pc = cpu.pc;
        while !cpu.step() {};
        assert_eq!(cpu.pc, initial_pc + 0x10 + 0x02);
    }
}

#[cfg(test)]
mod bcs {
    use super::*;

    #[test]
    fn bcs() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Bcs, AddrMode::Relative),
            0x10,
            instruction(OpCode::Bcs, AddrMode::Relative),
            0x10,
        ];
        let mut cpu = get_cpu(code);
        cpu.ps.set_carry(false);
        let initial_pc = cpu.pc;
        while !cpu.step() {};
        assert_eq!(cpu.pc, initial_pc + 0x02);
        cpu.ps.set_carry(true);
        let initial_pc = cpu.pc;
        while !cpu.step() {};
        assert_eq!(cpu.pc, initial_pc + 0x10 + 0x02);
    }
}

#[cfg(test)]
mod beq {
    use super::*;

    #[test]
    fn beq() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Beq, AddrMode::Relative),
            0x10,
            instruction(OpCode::Beq, AddrMode::Relative),
            0x10,
        ];
        let mut cpu = get_cpu(code);
        cpu.ps.set_zero(false);
        let initial_pc = cpu.pc;
        while !cpu.step() {};
        assert_eq!(cpu.pc, initial_pc + 0x02);
        cpu.ps.set_zero(true);
        let initial_pc = cpu.pc;
        while !cpu.step() {};
        assert_eq!(cpu.pc, initial_pc + 0x10 + 0x02);
    }
}

#[cfg(test)]
mod bmi {
    use super::*;

    #[test]
    fn bmi() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Bmi, AddrMode::Relative),
            0x10,
            instruction(OpCode::Bmi, AddrMode::Relative),
            0x10,
        ];
        let mut cpu = get_cpu(code);
        cpu.ps.set_negative(false);
        let initial_pc = cpu.pc;
        while !cpu.step() {};
        assert_eq!(cpu.pc, initial_pc + 0x02);
        cpu.ps.set_negative(true);
        let initial_pc = cpu.pc;
        while !cpu.step() {};
        assert_eq!(cpu.pc, initial_pc + 0x10 + 0x02);
    }
}

#[cfg(test)]
mod bne {
    use super::*;

    #[test]
    fn bne() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Bne, AddrMode::Relative),
            0x10,
            instruction(OpCode::Bne, AddrMode::Relative),
            0x10,
        ];
        let mut cpu = get_cpu(code);
        cpu.ps.set_zero(true);
        let initial_pc = cpu.pc;
        while !cpu.step() {};
        assert_eq!(cpu.pc, initial_pc + 0x02);
        cpu.ps.set_zero(false);
        let initial_pc = cpu.pc;
        while !cpu.step() {};
        assert_eq!(cpu.pc, initial_pc + 0x10 + 0x02);
    }
}

#[cfg(test)]
mod bpl {
    use super::*;

    #[test]
    fn bpl() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Bpl, AddrMode::Relative),
            0x10,
            instruction(OpCode::Bpl, AddrMode::Relative),
            0x10,
        ];
        let mut cpu = get_cpu(code);
        cpu.ps.set_negative(true);
        let initial_pc = cpu.pc;
        while !cpu.step() {};
        assert_eq!(cpu.pc, initial_pc + 0x02);
        cpu.ps.set_negative(false);
        let initial_pc = cpu.pc;
        while !cpu.step() {};
        assert_eq!(cpu.pc, initial_pc + 0x10 + 0x02);
    }
}

#[cfg(test)]
mod bvc {
    use super::*;

    #[test]
    fn bvc() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Bvc, AddrMode::Relative),
            0x10,
            instruction(OpCode::Bvc, AddrMode::Relative),
            0x10,
        ];
        let mut cpu = get_cpu(code);
        cpu.ps.set_overflow(true);
        let initial_pc = cpu.pc;
        while !cpu.step() {};
        assert_eq!(cpu.pc, initial_pc + 0x02);
        cpu.ps.set_overflow(false);
        let initial_pc = cpu.pc;
        while !cpu.step() {};
        assert_eq!(cpu.pc, initial_pc + 0x10 + 0x02);
    }
}

#[cfg(test)]
mod bvs {
    use super::*;

    #[test]
    fn bvc() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Bvs, AddrMode::Relative),
            0x10,
            instruction(OpCode::Bvs, AddrMode::Relative),
            0x10,
        ];
        let mut cpu = get_cpu(code);
        cpu.ps.set_overflow(false);
        let initial_pc = cpu.pc;
        while !cpu.step() {};
        assert_eq!(cpu.pc, initial_pc + 0x02);
        cpu.ps.set_overflow(true);
        let initial_pc = cpu.pc;
        while !cpu.step() {};
        assert_eq!(cpu.pc, initial_pc + 0x10 + 0x02);
    }
}

