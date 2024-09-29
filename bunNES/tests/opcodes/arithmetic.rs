use bunNES::nes::opcodes::{AddrMode, OpCode};
use crate::opcodes::helpers::{get_cpu, instruction};

#[cfg(test)]
mod asl {
    use super::*;

    #[test]
    fn asl_flags() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Asl, AddrMode::Accumulator),
            instruction(OpCode::Asl, AddrMode::Accumulator),
            instruction(OpCode::Asl, AddrMode::Accumulator),
        ];
        let mut cpu = get_cpu(code);
        cpu.acc = 0;
        while !cpu.step() {};
        assert_eq!(cpu.ps.carry(), false);
        assert_eq!(cpu.ps.zero(), true);
        assert_eq!(cpu.ps.negative(), false);

        cpu.acc = 69;
        while !cpu.step() {};
        assert_eq!(cpu.ps.carry(), false);
        assert_eq!(cpu.ps.zero(), false);
        assert_eq!(cpu.ps.negative(), true);

        cpu.acc = 138;
        while !cpu.step() {};
        assert_eq!(cpu.ps.carry(), true);
        assert_eq!(cpu.ps.zero(), false);
        assert_eq!(cpu.ps.negative(), false);
    }

    #[test]
    fn asl_accumulator() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Asl, AddrMode::Accumulator),
            instruction(OpCode::Asl, AddrMode::Accumulator),
        ];
        let mut cpu = get_cpu(code);
        cpu.acc = 1;
        while !cpu.step() {};
        assert_eq!(cpu.acc, 2);
        while !cpu.step() {};
        assert_eq!(cpu.acc, 4);
    }

    #[test]
    fn asl_zero_page() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Asl, AddrMode::Zp),
            1,
        ];
        let mut cpu = get_cpu(code);
        cpu.bus.ram[1] = 2;
        while !cpu.step() {};
        assert_eq!(cpu.bus.ram[1], 4);
    }

    #[test]
    fn asl_zero_page_x() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Asl, AddrMode::ZpX),
            1,
        ];
        let mut cpu = get_cpu(code);
        cpu.x = 2;
        cpu.bus.ram[3] = 2;
        while !cpu.step() {};
        assert_eq!(cpu.bus.ram[3], 4);
    }

    #[test]
    fn asl_absolute() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Asl, AddrMode::Absolute),
            0x00, 0x01
        ];
        let mut cpu = get_cpu(code);
        cpu.bus.ram[0x100] = 2;
        while !cpu.step() {};
        assert_eq!(cpu.bus.ram[0x100], 4);
    }

    #[test]
    fn asl_absolute_x() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Asl, AddrMode::AbsoluteX),
            0x00, 0x01
        ];
        let mut cpu = get_cpu(code);
        cpu.x = 5;
        cpu.bus.ram[0x105] = 2;
        while !cpu.step() {};
        assert_eq!(cpu.bus.ram[0x105], 4);
    }
}

#[cfg(test)]
mod adc {
    use super::*;

    #[test]
    fn adc_flags() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Adc, AddrMode::Immediate),
            34,
            instruction(OpCode::Adc, AddrMode::Immediate),
            1,
            instruction(OpCode::Adc, AddrMode::Immediate),
            10,
        ];
        let mut cpu = get_cpu(code);
        cpu.acc = 35;
        while !cpu.step() {};
        assert_eq!(cpu.ps.carry(), false);
        assert_eq!(cpu.ps.zero(), false);
        assert_eq!(cpu.ps.overflow(), false);
        assert_eq!(cpu.ps.negative(), false);

        cpu.acc = 255;
        while !cpu.step() {};
        assert_eq!(cpu.ps.carry(), true);
        assert_eq!(cpu.ps.zero(), true);
        assert_eq!(cpu.ps.overflow(), true);

        cpu.acc = 128;
        while !cpu.step() {};
        assert_eq!(cpu.ps.negative(), true);
    }

    #[test]
    fn adc_immediate() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Adc, AddrMode::Immediate),
            34,
            instruction(OpCode::Adc, AddrMode::Immediate),
            33,
        ];
        let mut cpu = get_cpu(code);
        cpu.acc = 35;
        while !cpu.step() {};
        assert_eq!(cpu.acc, 69);
        cpu.acc = 35;
        cpu.ps.set_carry(true);
        while !cpu.step() {};
        assert_eq!(cpu.acc, 69);
    }

    #[test]
    fn adc_zero_page() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Adc, AddrMode::Zp),
            1,
        ];
        let mut cpu = get_cpu(code);
        cpu.bus.ram[1] = 34;
        cpu.acc = 35;
        cpu.step();
        assert_eq!(cpu.acc, 69);
    }

    #[test]
    fn adc_zero_page_x() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Adc, AddrMode::ZpX),
            1,
        ];
        let mut cpu = get_cpu(code);
        cpu.acc = 35;
        cpu.x = 5;
        cpu.bus.ram[6] = 34;
        cpu.step();
        assert_eq!(cpu.acc, 69);
    }

    #[test]
    fn adc_absolute() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Adc, AddrMode::Absolute),
            0x00, 0x01,
        ];
        let mut cpu = get_cpu(code);
        cpu.acc = 35;
        cpu.bus.ram[0x100] = 34;
        cpu.step();
        assert_eq!(cpu.acc, 69);
    }

    #[test]
    fn adc_absolute_x() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Adc, AddrMode::AbsoluteX),
            0x00, 0x01,
        ];
        let mut cpu = get_cpu(code);
        cpu.acc = 35;
        cpu.x = 5;
        cpu.bus.ram[0x105] = 34;
        cpu.step();
        assert_eq!(cpu.acc, 69);
    }

    #[test]
    fn adc_absolute_y() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Adc, AddrMode::AbsoluteY),
            0x00, 0x01,
        ];
        let mut cpu = get_cpu(code);
        cpu.acc = 35;
        cpu.y = 5;
        cpu.bus.ram[0x105] = 34;
        cpu.step();
        assert_eq!(cpu.acc, 69);
    }

    #[test]
    fn adc_indirect_x() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Adc, AddrMode::IndirectX),
            20
        ];
        let mut cpu = get_cpu(code);
        cpu.acc = 35;
        cpu.x = 4;
        cpu.bus.ram[0x24] = 0x00;
        cpu.bus.ram[0x25] = 0x05;
        cpu.bus.ram[0x500] = 34;
        cpu.step();
        assert_eq!(cpu.acc, 69);
    }

    #[test]
    fn adc_indirect_y() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Adc, AddrMode::IndirectY),
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
    }
}
