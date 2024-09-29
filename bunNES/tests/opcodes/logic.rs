use bunNES::nes::opcodes::{AddrMode, OpCode};
use crate::opcodes::helpers::{get_cpu, instruction};

#[cfg(test)]
mod and {
    use super::*;

    #[test]
    fn and_immediate() {
        let code: Vec<u8> = vec![
            instruction(OpCode::And, AddrMode::Immediate),
            35,
            instruction(OpCode::And, AddrMode::Immediate),
            0,
            instruction(OpCode::And, AddrMode::Immediate),
            255,
        ];
        let mut cpu = get_cpu(code);
        cpu.acc = 35;
        while !cpu.step() {};
        assert_eq!(cpu.ps.zero(), false);
        assert_eq!(cpu.ps.negative(), false);

        cpu.acc = 0;
        while !cpu.step() {};
        assert_eq!(cpu.ps.zero(), true);

        cpu.acc = 255;
        while !cpu.step() {};
        assert_eq!(cpu.ps.negative(), true);
    }

    #[test]
    fn and_zero_page() {
        let code: Vec<u8> = vec![
            instruction(OpCode::And, AddrMode::Zp),
            1,
            instruction(OpCode::And, AddrMode::Zp),
            2,
            instruction(OpCode::And, AddrMode::Zp),
            3,
        ];
        let mut cpu = get_cpu(code);
        cpu.acc = 35;
        cpu.bus.ram[1] = 35;
        cpu.bus.ram[2] = 0;
        cpu.bus.ram[3] = 255;

        while !cpu.step() {};
        assert_eq!(cpu.ps.zero(), false);
        assert_eq!(cpu.ps.negative(), false);

        cpu.acc = 0;
        while !cpu.step() {};
        assert_eq!(cpu.ps.zero(), true);

        cpu.acc = 255;
        while !cpu.step() {};
        assert_eq!(cpu.ps.negative(), true);
    }

    #[test]
    fn and_zero_page_x() {
        let code: Vec<u8> = vec![
            instruction(OpCode::And, AddrMode::ZpX),
            1,
            instruction(OpCode::And, AddrMode::ZpX),
            2,
            instruction(OpCode::And, AddrMode::ZpX),
            3,
        ];
        let mut cpu = get_cpu(code);
        cpu.acc = 35;
        cpu.x = 5;
        cpu.bus.ram[6] = 35;
        cpu.bus.ram[7] = 0;
        cpu.bus.ram[8] = 255;

        while !cpu.step() {};
        assert_eq!(cpu.ps.zero(), false);
        assert_eq!(cpu.ps.negative(), false);

        cpu.acc = 0;
        while !cpu.step() {};
        assert_eq!(cpu.ps.zero(), true);

        cpu.acc = 255;
        while !cpu.step() {};
        assert_eq!(cpu.ps.negative(), true);
    }

    #[test]
    fn and_absolute() {
        let code: Vec<u8> = vec![
            instruction(OpCode::And, AddrMode::Absolute),
            0x00, 0x01,
            instruction(OpCode::And, AddrMode::Absolute),
            0x01, 0x01,
            instruction(OpCode::And, AddrMode::Absolute),
            0x02, 0x01,
        ];
        let mut cpu = get_cpu(code);
        cpu.acc = 35;
        cpu.bus.ram[0x100] = 35;
        cpu.bus.ram[0x101] = 0;
        cpu.bus.ram[0x102] = 255;

        while !cpu.step() {};
        assert_eq!(cpu.ps.zero(), false);
        assert_eq!(cpu.ps.negative(), false);

        cpu.acc = 0;
        while !cpu.step() {};
        assert_eq!(cpu.ps.zero(), true);

        cpu.acc = 255;
        while !cpu.step() {};
        assert_eq!(cpu.ps.negative(), true);
    }

    #[test]
    fn and_absolute_x() {
        let code: Vec<u8> = vec![
            instruction(OpCode::And, AddrMode::AbsoluteX),
            0x00, 0x01,
            instruction(OpCode::And, AddrMode::AbsoluteX),
            0x01, 0x01,
            instruction(OpCode::And, AddrMode::AbsoluteX),
            0x02, 0x01,
        ];
        let mut cpu = get_cpu(code);
        cpu.acc = 35;
        cpu.x = 5;
        cpu.bus.ram[0x105] = 35;
        cpu.bus.ram[0x106] = 0;
        cpu.bus.ram[0x107] = 255;

        while !cpu.step() {};
        assert_eq!(cpu.ps.zero(), false);
        assert_eq!(cpu.ps.negative(), false);

        cpu.acc = 0;
        while !cpu.step() {};
        assert_eq!(cpu.ps.zero(), true);

        cpu.acc = 255;
        while !cpu.step() {};
        assert_eq!(cpu.ps.negative(), true);
    }

    #[test]
    fn and_absolute_y() {
        let code: Vec<u8> = vec![
            instruction(OpCode::And, AddrMode::AbsoluteY),
            0x00, 0x01,
            instruction(OpCode::And, AddrMode::AbsoluteY),
            0x01, 0x01,
            instruction(OpCode::And, AddrMode::AbsoluteY),
            0x02, 0x01,
        ];
        let mut cpu = get_cpu(code);
        cpu.acc = 35;
        cpu.y = 5;
        cpu.bus.ram[0x105] = 35;
        cpu.bus.ram[0x106] = 0;
        cpu.bus.ram[0x107] = 255;

        while !cpu.step() {};
        assert_eq!(cpu.ps.zero(), false);
        assert_eq!(cpu.ps.negative(), false);

        cpu.acc = 0;
        while !cpu.step() {};
        assert_eq!(cpu.ps.zero(), true);

        cpu.acc = 255;
        while !cpu.step() {};
        assert_eq!(cpu.ps.negative(), true);
    }

    #[test]
    fn and_indirect_x() {
        let code: Vec<u8> = vec![
            instruction(OpCode::And, AddrMode::IndirectX),
            20,
            instruction(OpCode::And, AddrMode::IndirectX),
            20,
            instruction(OpCode::And, AddrMode::IndirectX),
            20,
        ];
        let mut cpu = get_cpu(code);
        cpu.acc = 35;
        cpu.x = 4;
        cpu.bus.ram[0x24] = 0x00;
        cpu.bus.ram[0x25] = 0x05;
        cpu.bus.ram[0x26] = 0x01;
        cpu.bus.ram[0x27] = 0x05;
        cpu.bus.ram[0x28] = 0x02;
        cpu.bus.ram[0x29] = 0x05;
        cpu.bus.ram[0x500] = 35;
        cpu.bus.ram[0x501] = 0;
        cpu.bus.ram[0x502] = 255;

        while !cpu.step() {};
        assert_eq!(cpu.ps.zero(), false);
        assert_eq!(cpu.ps.negative(), false);

        cpu.acc = 0;
        cpu.x = 6;
        while !cpu.step() {};
        assert_eq!(cpu.ps.zero(), true);

        cpu.acc = 255;
        cpu.x = 8;
        while !cpu.step() {};
        assert_eq!(cpu.ps.negative(), true);
    }

    #[test]
    fn and_indirect_y() {
        let code: Vec<u8> = vec![
            instruction(OpCode::And, AddrMode::IndirectY),
            20
        ];
        let mut cpu = get_cpu(code);
        cpu.acc = 35;
        cpu.y = 4;
        cpu.bus.ram[0x20] = 0x00;
        cpu.bus.ram[0x21] = 0x05;
        cpu.bus.ram[0x504] = 34;
        cpu.step();
        assert_eq!(cpu.acc, 69);

        let code: Vec<u8> = vec![
            instruction(OpCode::And, AddrMode::IndirectY),
            20,
            instruction(OpCode::And, AddrMode::IndirectY),
            20,
            instruction(OpCode::And, AddrMode::IndirectY),
            20,
        ];
        let mut cpu = get_cpu(code);
        cpu.acc = 35;
        cpu.y = 4;
        cpu.bus.ram[0x20] = 0x00;
        cpu.bus.ram[0x21] = 0x05;
        cpu.bus.ram[0x504] = 34;
        cpu.bus.ram[0x505] = 0;
        cpu.bus.ram[0x506] = 255;

        while !cpu.step() {};
        assert_eq!(cpu.ps.zero(), false);
        assert_eq!(cpu.ps.negative(), false);

        cpu.acc = 0;
        cpu.y = 5;
        while !cpu.step() {};
        assert_eq!(cpu.ps.zero(), true);

        cpu.acc = 255;
        cpu.y = 6;
        while !cpu.step() {};
        assert_eq!(cpu.ps.negative(), true);
    }
}

#[cfg(test)]
mod bit {
    use super::*;

    #[test]
    fn bit_zp() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Bit, AddrMode::Zp),
            0x01,
            instruction(OpCode::Bit, AddrMode::Zp),
            0x02,
        ];
        let mut cpu = get_cpu(code);
        cpu.acc = 0b0000_0001;
        cpu.bus.ram[0x01] = 0b0000_0000;
        cpu.bus.ram[0x02] = 0b1100_0000;
        while !cpu.step() {};
        assert_eq!(cpu.ps.zero(), true);
        assert_eq!(cpu.ps.overflow(), false);
        assert_eq!(cpu.ps.negative(), false);

        cpu.acc = 0b1111_1111;
        while !cpu.step() {};
        assert_eq!(cpu.ps.zero(), false);
        assert_eq!(cpu.ps.overflow(), true);
        assert_eq!(cpu.ps.negative(), true);
    }

    fn bit_absolute() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Bit, AddrMode::Absolute),
            0x00, 0x01,
            instruction(OpCode::Bit, AddrMode::Absolute),
            0x01, 0x01,
        ];
        let mut cpu = get_cpu(code);
        cpu.acc = 0b0000_0001;
        cpu.bus.ram[0x0100] = 0b0000_0000;
        cpu.bus.ram[0x0101] = 0b1100_0000;
        while !cpu.step() {};
        assert_eq!(cpu.ps.zero(), true);
        assert_eq!(cpu.ps.overflow(), false);
        assert_eq!(cpu.ps.negative(), false);

        cpu.acc = 0b1111_1111;
        while !cpu.step() {};
        assert_eq!(cpu.ps.zero(), false);
        assert_eq!(cpu.ps.overflow(), true);
        assert_eq!(cpu.ps.negative(), true);
    }
}
