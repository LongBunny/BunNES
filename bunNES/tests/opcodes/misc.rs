use bunNES::nes::opcodes::{AddrMode, OpCode};
use crate::opcodes::helpers::{get_cpu, instruction};

// 1/1
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
        todo!("stack not implemented");
        // assert_eq!(cpu.bus.read_16(0x0100 + cpu.sp as u16), pc);
        // assert_eq!(cpu.bus.read_16(0x0100 + cpu.sp as u16 + 0x02), ps.reg);
    }
}

// 1/1
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

// 1/1
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

// 1/1
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

// 1/1
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

// 1/1
#[cfg(test)]
mod nop {
    use super::*;

    #[test]
    fn nop() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Nop, AddrMode::Implicit),
        ];
        let mut cpu = get_cpu(code);
        let old_reg = cpu.ps.get_reg();
        while !cpu.step() {};
        assert_eq!(cpu.ps.get_reg(), old_reg);
    }
}

// 1/1
#[cfg(test)]
mod pha {
    use super::*;

    #[test]
    fn pha() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Pha, AddrMode::Implicit),
        ];
        let mut cpu = get_cpu(code);
        cpu.acc = 69;
        let old_sp = cpu.sp;
        while !cpu.step() {};
        assert_eq!(cpu.sp, old_sp - 0x01);
        assert_eq!(cpu.bus.ram[0x01FF], 69);
    }
}

// 1/1
#[cfg(test)]
mod php {
    use super::*;

    #[test]
    fn php() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Php, AddrMode::Implicit),
        ];
        let mut cpu = get_cpu(code);
        let old_sp = cpu.sp;
        cpu.ps.set_carry(true);
        cpu.ps.set_negative(true);
        cpu.ps.set_decimal(true);
        let old_ps = cpu.ps.get_reg();
        while !cpu.step() {};
        assert_eq!(cpu.sp, old_sp - 0x01);
        assert_eq!(cpu.bus.ram[0x01FF], old_ps);
    }
}

// 1/1
#[cfg(test)]
mod pla {
    use super::*;

    #[test]
    fn pla() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Pla, AddrMode::Implicit),
        ];
        let mut cpu = get_cpu(code);
        cpu.bus.ram[0x01FF] = 69;
        let old_sp = cpu.sp;
        while !cpu.step() {};
        assert_eq!(cpu.sp, old_sp - 0x01);
        assert_eq!(cpu.acc, 69);
    }
}

#[cfg(test)]
mod plp {
    use super::*;

    #[test]
    fn plp() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Plp, AddrMode::Implicit),
        ];
        let mut cpu = get_cpu(code);
        cpu.bus.ram[0x01FF] = 0b1111_1111;
        let old_sp = cpu.sp;
        while !cpu.step() {};
        assert_eq!(cpu.sp, old_sp - 0x01);
        assert_eq!(cpu.ps.get_reg(), 0b1111_1111);
    }
}

// 1/1
#[cfg(test)]
mod sec {
    use super::*;
    
    #[test]
    fn sec() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Sec, AddrMode::Implicit),
        ];
        let mut cpu = get_cpu(code);
        cpu.ps.set_carry(false);
        while !cpu.step() {};
        assert_eq!(cpu.ps.carry(), true);
    }
}

// 1/1
#[cfg(test)]
mod sed {
    use super::*;
    
    #[test]
    fn sed() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Sed, AddrMode::Implicit),
        ];
        let mut cpu = get_cpu(code);
        cpu.ps.set_decimal(false);
        while !cpu.step() {};
        assert_eq!(cpu.ps.decimal(), true);
    }
}

// 1/1
#[cfg(test)]
mod sei {
    use super::*;
    
    #[test]
    fn sei() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Sei, AddrMode::Implicit),
        ];
        let mut cpu = get_cpu(code);
        cpu.ps.set_irqb(false);
        while !cpu.step() {};
        assert_eq!(cpu.ps.irqb(), true);
    }
}


