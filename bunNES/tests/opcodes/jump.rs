use bunNES::nes::opcodes::{AddrMode, OpCode};
use crate::opcodes::helpers::{get_cpu, instruction};


// 1/1
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

// 1/1
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

// 1/1
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

// 1/1
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

// 1/1
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

// 1/1
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

// 1/1
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

// 1/1
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

// 2/2
#[cfg(test)]
mod jmp {
    use super::*;

    #[test]
    fn jmp_absolute() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Bvs, AddrMode::Absolute),
            0x01, 0x00,
        ];
        let mut cpu = get_cpu(code);
        let initial_pc = cpu.pc;
        while !cpu.step() {};
        assert_eq!(cpu.pc, initial_pc + 0x0100 + 0x01);
    }
    
    #[test]
    fn jmp_indirect() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Bvs, AddrMode::Indirect),
            0x10,
        ];
        let mut cpu = get_cpu(code);
        let initial_pc = cpu.pc;
        while !cpu.step() {};
        assert_eq!(cpu.pc, initial_pc + 0x10 + 0x01);
        
        // note: 
        // An original 6502 has does not correctly fetch the target address
        // if the indirect vector falls on a page boundary
        // (e.g. $xxFF where xx is any value from $00 to $FF).
        // In this case fetches the LSB from $xxFF as expected
        // but takes the MSB from $xx00.
        // This is fixed in some later chips like the 65SC02
        // so for compatibility always ensure the indirect vector is not at the end of the page.
    }
}

// 1/1
#[cfg(test)]
mod jsr {
    use super::*;

    #[test]
    fn jsr() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Jsr, AddrMode::Absolute),
            0x00, 0x01,
        ];
        let mut cpu = get_cpu(code);
        while !cpu.step() {};
        assert_eq!(cpu.pc, 0x0100);
        todo!("jsr: check stack")
    }
}

// 1/1
#[cfg(test)]
mod rti {
    use super::*;

    #[test]
    fn rti() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Rti, AddrMode::Implicit),
        ];
        let mut cpu = get_cpu(code);
        cpu.bus.ram[0x01FF] = 0b1111_1111;
        cpu.bus.ram[0x01FF - 1] = 0x01;
        cpu.bus.ram[0x01FF - 2] = 0x02;
        while !cpu.step() {};
        assert_eq!(cpu.ps.get_reg(), 0b1111_1111);
        assert_eq!(cpu.pc, 0x0201);
    }
}

// 1/1
#[cfg(test)]
mod rts {
    use super::*;

    #[test]
    fn rts() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Rts, AddrMode::Implicit),
        ];
        let mut cpu = get_cpu(code);
        cpu.bus.ram[0x01FF - 0] = 0x01;
        cpu.bus.ram[0x01FF - 1] = 0x02;
        while !cpu.step() {};
        assert_eq!(cpu.pc, 0x0201);
    }
}


