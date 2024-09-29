use bunNES::nes::opcodes::{AddrMode, OpCode};
use crate::opcodes::helpers::{get_cpu, instruction};

#[cfg(test)]
mod lda {
    use super::*;

    #[test]
    fn lda_flags() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Lda, AddrMode::Immediate),
            69,
            instruction(OpCode::Lda, AddrMode::Immediate),
            0,
            instruction(OpCode::Lda, AddrMode::Immediate),
            255,
        ];
        let mut cpu = get_cpu(code);

        while !cpu.step() {};
        assert_eq!(!cpu.ps.zero(), true);

        while !cpu.step() {};
        assert_eq!(cpu.ps.zero(), true);
        assert_eq!(cpu.ps.negative(), false);

        while !cpu.step() {};
        assert_eq!(cpu.ps.negative(), true);
    }

    #[test]
    fn lda_immediate() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Lda, AddrMode::Immediate),
            69
        ];
        let mut cpu = get_cpu(code);
        while !cpu.step() {};
        assert_eq!(cpu.acc, 69);
    }

    #[test]
    fn lda_zero_page() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Lda, AddrMode::Zp),
            1,
        ];
        let mut cpu = get_cpu(code);
        cpu.bus.ram[1] = 69;
        cpu.step();
        assert_eq!(cpu.acc, 69);
    }

    #[test]
    fn lda_zero_page_x() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Lda, AddrMode::ZpX),
            1,
        ];
        let mut cpu = get_cpu(code);
        cpu.x = 5;
        cpu.bus.ram[6] = 69;
        cpu.step();
        assert_eq!(cpu.acc, 69);
    }

    #[test]
    fn lda_absolute() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Lda, AddrMode::Absolute),
            0x00, 0x01,
        ];
        let mut cpu = get_cpu(code);
        cpu.bus.ram[0x100] = 69;
        cpu.step();
        assert_eq!(cpu.acc, 69);
    }

    #[test]
    fn lda_absolute_x() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Lda, AddrMode::AbsoluteX),
            0x00, 0x01,
        ];
        let mut cpu = get_cpu(code);
        cpu.x = 5;
        cpu.bus.ram[0x105] = 69;
        cpu.step();
        assert_eq!(cpu.acc, 69);
    }

    #[test]
    fn lda_absolute_y() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Lda, AddrMode::AbsoluteY),
            0x00, 0x01,
        ];
        let mut cpu = get_cpu(code);
        cpu.y = 5;
        cpu.bus.ram[0x105] = 69;
        cpu.step();
        assert_eq!(cpu.acc, 69);
    }

    #[test]
    fn lda_indirect_x() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Lda, AddrMode::IndirectX),
            20
        ];
        let mut cpu = get_cpu(code);
        cpu.x = 4;
        cpu.bus.ram[0x24] = 0x00;
        cpu.bus.ram[0x25] = 0x05;
        cpu.bus.ram[0x500] = 69;
        cpu.step();
        assert_eq!(cpu.acc, 69);
    }

    #[test]
    fn lda_indirect_y() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Lda, AddrMode::IndirectY),
            20
        ];
        let mut cpu = get_cpu(code);
        cpu.y = 4;
        cpu.bus.ram[0x20] = 0x00;
        cpu.bus.ram[0x21] = 0x05;
        cpu.bus.ram[0x504] = 69;
        cpu.step();
        assert_eq!(cpu.acc, 69);
    }
}

#[cfg(test)]
mod ldx {
    use super::*;

    #[test]
    fn ldx_flags() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Ldx, AddrMode::Immediate),
            69,
            instruction(OpCode::Ldx, AddrMode::Immediate),
            0,
            instruction(OpCode::Ldx, AddrMode::Immediate),
            255,
        ];
        let mut cpu = get_cpu(code);

        while !cpu.step() {};
        assert_eq!(!cpu.ps.zero(), true);

        while !cpu.step() {};
        assert_eq!(cpu.ps.zero(), true);
        assert_eq!(cpu.ps.negative(), false);

        while !cpu.step() {};
        assert_eq!(cpu.ps.negative(), true);
    }

    #[test]
    fn ldx_immediate() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Ldx, AddrMode::Immediate),
            69
        ];
        let mut cpu = get_cpu(code);
        while !cpu.step() {};
        assert_eq!(cpu.x, 69);
    }

    #[test]
    fn ldx_zero_page() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Ldx, AddrMode::Zp),
            1,
        ];
        let mut cpu = get_cpu(code);
        cpu.bus.ram[1] = 69;
        cpu.step();
        assert_eq!(cpu.x, 69);
    }

    #[test]
    fn ldx_zero_page_y() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Ldx, AddrMode::ZpY),
            1,
        ];
        let mut cpu = get_cpu(code);
        cpu.y = 5;
        cpu.bus.ram[6] = 69;
        cpu.step();
        assert_eq!(cpu.x, 69);
    }

    #[test]
    fn ldx_absolute() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Ldx, AddrMode::Absolute),
            0x00, 0x01,
        ];
        let mut cpu = get_cpu(code);
        cpu.bus.ram[0x100] = 69;
        cpu.step();
        assert_eq!(cpu.x, 69);
    }

    #[test]
    fn ldx_absolute_y() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Ldx, AddrMode::AbsoluteY),
            0x00, 0x01,
        ];
        let mut cpu = get_cpu(code);
        cpu.y = 5;
        cpu.bus.ram[0x105] = 69;
        cpu.step();
        assert_eq!(cpu.x, 69);
    }
}

#[cfg(test)]
mod ldy {
    use super::*;

    #[test]
    fn ldy_flags() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Ldy, AddrMode::Immediate),
            69,
            instruction(OpCode::Ldy, AddrMode::Immediate),
            0,
            instruction(OpCode::Ldy, AddrMode::Immediate),
            255,
        ];
        let mut cpu = get_cpu(code);

        while !cpu.step() {};
        assert_eq!(!cpu.ps.zero(), true);

        while !cpu.step() {};
        assert_eq!(cpu.ps.zero(), true);
        assert_eq!(cpu.ps.negative(), false);

        while !cpu.step() {};
        assert_eq!(cpu.ps.negative(), true);
    }

    #[test]
    fn ldy_immediate() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Ldy, AddrMode::Immediate),
            69
        ];
        let mut cpu = get_cpu(code);
        while !cpu.step() {};
        assert_eq!(cpu.y, 69);
    }

    #[test]
    fn ldy_zero_page() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Ldy, AddrMode::Zp),
            1,
        ];
        let mut cpu = get_cpu(code);
        cpu.bus.ram[1] = 69;
        cpu.step();
        assert_eq!(cpu.y, 69);
    }

    #[test]
    fn ldy_zero_page_x() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Ldy, AddrMode::ZpX),
            1,
        ];
        let mut cpu = get_cpu(code);
        cpu.x = 5;
        cpu.bus.ram[6] = 69;
        cpu.step();
        assert_eq!(cpu.y, 69);
    }

    #[test]
    fn ldy_absolute() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Ldy, AddrMode::Absolute),
            0x00, 0x01,
        ];
        let mut cpu = get_cpu(code);
        cpu.bus.ram[0x100] = 69;
        cpu.step();
        assert_eq!(cpu.y, 69);
    }

    #[test]
    fn ldy_absolute_x() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Ldy, AddrMode::AbsoluteX),
            0x00, 0x01,
        ];
        let mut cpu = get_cpu(code);
        cpu.x = 5;
        cpu.bus.ram[0x105] = 69;
        cpu.step();
        assert_eq!(cpu.y, 69);
    }
}