use bunNES::nes::opcodes::{AddrMode, OpCode};
use crate::opcodes::helpers::{get_cpu, instruction};
use bunNES::nes::cpu::Cpu;

// 8/8
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
        test_results(&mut cpu, 0, 0);
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
        cpu.bus.ram[1] = 35;
        cpu.bus.ram[2] = 0;
        cpu.bus.ram[3] = 255;

        test_results(&mut cpu, 0, 0);
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
        cpu.x = 5;
        cpu.bus.ram[6] = 35;
        cpu.bus.ram[7] = 0;
        cpu.bus.ram[8] = 255;

        test_results(&mut cpu, 0, 0);
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
        cpu.bus.ram[0x100] = 35;
        cpu.bus.ram[0x101] = 0;
        cpu.bus.ram[0x102] = 255;

        test_results(&mut cpu, 0, 0);
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
        cpu.x = 5;
        cpu.bus.ram[0x105] = 35;
        cpu.bus.ram[0x106] = 0;
        cpu.bus.ram[0x107] = 255;

        test_results(&mut cpu, 0, 0);
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
        cpu.y = 5;
        cpu.bus.ram[0x105] = 35;
        cpu.bus.ram[0x106] = 0;
        cpu.bus.ram[0x107] = 255;

        test_results(&mut cpu, 0, 0);
    }

    #[test]
    fn and_indirect_x() {
        let code: Vec<u8> = vec![
            instruction(OpCode::And, AddrMode::IndirectX),
            0x20,
            instruction(OpCode::And, AddrMode::IndirectX),
            0x20,
            instruction(OpCode::And, AddrMode::IndirectX),
            0x20,
        ];
        let mut cpu = get_cpu(code);
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

        test_results(&mut cpu, 2, 0);
    }

    #[test]
    fn and_indirect_y() {
        let code: Vec<u8> = vec![
            instruction(OpCode::And, AddrMode::IndirectY),
            0x20,
            instruction(OpCode::And, AddrMode::IndirectY),
            0x20,
            instruction(OpCode::And, AddrMode::IndirectY),
            0x20,
        ];
        
        let mut cpu = get_cpu(code);
        cpu.y = 4;
        cpu.bus.ram[0x20] = 0x00;
        cpu.bus.ram[0x21] = 0x05;
        cpu.bus.ram[0x504] = 35;
        cpu.bus.ram[0x505] = 0;
        cpu.bus.ram[0x506] = 255;

        test_results(&mut cpu, 0, 1);
    }

    fn test_results(cpu: &mut Cpu, x_inc: u8, y_inc: u8) {
        cpu.acc = 34;
        while !cpu.step() {};
        assert_eq!(cpu.ps.zero(), false);
        assert_eq!(cpu.ps.negative(), false);
        assert_eq!(cpu.acc, 34);
        cpu.x += x_inc;
        cpu.y += y_inc;

        cpu.acc = 0;
        while !cpu.step() {};
        assert_eq!(cpu.ps.zero(), true);
        assert_eq!(cpu.ps.negative(), false);
        assert_eq!(cpu.acc, 0);
        cpu.x += x_inc;
        cpu.y += y_inc;

        cpu.acc = 255;
        while !cpu.step() {};
        assert_eq!(cpu.ps.zero(), false);
        assert_eq!(cpu.ps.negative(), true);
        assert_eq!(cpu.acc, 255);
    }
}

// 2/2
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

// 8/8
#[cfg(test)]
mod cmp {
    use super::*;

    #[test]
    fn cmp_immediate() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Cmp, AddrMode::Immediate),
            35,
            instruction(OpCode::Cmp, AddrMode::Immediate),
            0,
            instruction(OpCode::Cmp, AddrMode::Immediate),
            255,
        ];
        let mut cpu = get_cpu(code);
        test_flags(&mut cpu);
    }

    #[test]
    fn cmp_zero_page() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Cmp, AddrMode::Zp),
            1,
            instruction(OpCode::Cmp, AddrMode::Zp),
            2,
            instruction(OpCode::Cmp, AddrMode::Zp),
            3,
        ];
        let mut cpu = get_cpu(code);
        cpu.bus.ram[1] = 35;
        cpu.bus.ram[2] = 0;
        cpu.bus.ram[3] = 255;

        test_flags(&mut cpu);
    }

    #[test]
    fn cmp_zero_page_x() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Cmp, AddrMode::ZpX),
            1,
            instruction(OpCode::Cmp, AddrMode::ZpX),
            2,
            instruction(OpCode::Cmp, AddrMode::ZpX),
            3,
        ];
        let mut cpu = get_cpu(code);
        cpu.x = 5;
        cpu.bus.ram[6] = 35;
        cpu.bus.ram[7] = 0;
        cpu.bus.ram[8] = 255;

        test_flags(&mut cpu);
    }

    #[test]
    fn cmp_absolute() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Cmp, AddrMode::Absolute),
            0x00, 0x01,
            instruction(OpCode::Cmp, AddrMode::Absolute),
            0x01, 0x01,
            instruction(OpCode::Cmp, AddrMode::Absolute),
            0x02, 0x01,
        ];
        let mut cpu = get_cpu(code);
        cpu.bus.ram[0x100] = 35;
        cpu.bus.ram[0x101] = 0;
        cpu.bus.ram[0x102] = 255;

        test_flags(&mut cpu);
    }

    #[test]
    fn cmp_absolute_x() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Cmp, AddrMode::AbsoluteX),
            0x00, 0x01,
            instruction(OpCode::Cmp, AddrMode::AbsoluteX),
            0x01, 0x01,
            instruction(OpCode::Cmp, AddrMode::AbsoluteX),
            0x02, 0x01,
        ];
        let mut cpu = get_cpu(code);
        cpu.x = 5;
        cpu.bus.ram[0x105] = 35;
        cpu.bus.ram[0x106] = 0;
        cpu.bus.ram[0x107] = 255;

        test_flags(&mut cpu);
    }

    #[test]
    fn cmp_absolute_y() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Cmp, AddrMode::AbsoluteY),
            0x00, 0x01,
            instruction(OpCode::Cmp, AddrMode::AbsoluteY),
            0x01, 0x01,
            instruction(OpCode::Cmp, AddrMode::AbsoluteY),
            0x02, 0x01,
        ];
        let mut cpu = get_cpu(code);
        cpu.acc = 35;
        cpu.y = 5;
        cpu.bus.ram[0x105] = 35;
        cpu.bus.ram[0x106] = 0;
        cpu.bus.ram[0x107] = 255;

        test_flags(&mut cpu);
    }

    #[test]
    fn cmp_indirect_x() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Cmp, AddrMode::IndirectX),
            0x20,
            instruction(OpCode::Cmp, AddrMode::IndirectX),
            0x20,
            instruction(OpCode::Cmp, AddrMode::IndirectX),
            0x20,
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

        test_flags(&mut cpu);
    }

    #[test]
    fn cmp_indirect_y() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Cmp, AddrMode::IndirectY),
            0x20,
            instruction(OpCode::Cmp, AddrMode::IndirectY),
            0x20,
            instruction(OpCode::Cmp, AddrMode::IndirectY),
            0x20,
        ];
        let mut cpu = get_cpu(code);
        cpu.acc = 35;
        cpu.y = 4;
        cpu.bus.ram[0x20] = 0x00;
        cpu.bus.ram[0x21] = 0x05;
        cpu.bus.ram[0x504] = 34;
        cpu.bus.ram[0x505] = 0;
        cpu.bus.ram[0x506] = 255;

        test_flags(&mut cpu);
    }


    fn test_flags(cpu: &mut Cpu) {
        cpu.acc = 0;
        while !cpu.step() {};
        assert_eq!(cpu.ps.carry(), false);
        assert_eq!(cpu.ps.zero(), false);
        assert_eq!(cpu.ps.negative(), false);

        cpu.acc = 35;
        while !cpu.step() {};
        assert_eq!(cpu.ps.carry(), true);
        assert_eq!(cpu.ps.zero(), true);
        assert_eq!(cpu.ps.negative(), false);

        cpu.acc = 255;
        while !cpu.step() {};
        assert_eq!(cpu.ps.carry(), false);
        assert_eq!(cpu.ps.zero(), true);
        assert_eq!(cpu.ps.negative(), true);
    }

}

// 3/3
#[cfg(test)]
mod cpx {
    use super::*;

    #[test]
    fn cpx_immediate() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Cpx, AddrMode::Immediate),
            35,
            instruction(OpCode::Cpx, AddrMode::Immediate),
            0,
            instruction(OpCode::Cpx, AddrMode::Immediate),
            255,
        ];
        let mut cpu = get_cpu(code);
        test_flags(&mut cpu);
    }

    #[test]
    fn cpx_zero_page() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Cpx, AddrMode::Zp),
            1,
            instruction(OpCode::Cpx, AddrMode::Zp),
            2,
            instruction(OpCode::Cpx, AddrMode::Zp),
            3,
        ];
        let mut cpu = get_cpu(code);
        cpu.bus.ram[1] = 35;
        cpu.bus.ram[2] = 0;
        cpu.bus.ram[3] = 255;

        test_flags(&mut cpu);
    }

    #[test]
    fn cpx_absolute() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Cpx, AddrMode::Absolute),
            0x00, 0x01,
            instruction(OpCode::Cpx, AddrMode::Absolute),
            0x01, 0x01,
            instruction(OpCode::Cpx, AddrMode::Absolute),
            0x02, 0x01,
        ];
        let mut cpu = get_cpu(code);
        cpu.bus.ram[0x100] = 35;
        cpu.bus.ram[0x101] = 0;
        cpu.bus.ram[0x102] = 255;

       test_flags(&mut cpu);
    }

    fn test_flags(cpu: &mut Cpu) {
        cpu.acc = 0;
        while !cpu.step() {};
        assert_eq!(cpu.ps.carry(), false);
        assert_eq!(cpu.ps.zero(), false);
        assert_eq!(cpu.ps.negative(), false);

        cpu.acc = 35;
        while !cpu.step() {};
        assert_eq!(cpu.ps.carry(), true);
        assert_eq!(cpu.ps.zero(), true);
        assert_eq!(cpu.ps.negative(), false);

        cpu.acc = 255;
        while !cpu.step() {};
        assert_eq!(cpu.ps.carry(), false);
        assert_eq!(cpu.ps.zero(), true);
        assert_eq!(cpu.ps.negative(), true);
    }

}

// 3/3
#[cfg(test)]
mod cpy {
    use super::*;

    #[test]
    fn cpy_immediate() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Cpy, AddrMode::Immediate),
            35,
            instruction(OpCode::Cpy, AddrMode::Immediate),
            0,
            instruction(OpCode::Cpy, AddrMode::Immediate),
            255,
        ];
        let mut cpu = get_cpu(code);
        test_flags(&mut cpu);
    }

    #[test]
    fn cpy_zero_page() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Cpy, AddrMode::Zp),
            1,
            instruction(OpCode::Cpy, AddrMode::Zp),
            2,
            instruction(OpCode::Cpy, AddrMode::Zp),
            3,
        ];
        let mut cpu = get_cpu(code);
        cpu.bus.ram[1] = 35;
        cpu.bus.ram[2] = 0;
        cpu.bus.ram[3] = 255;

        test_flags(&mut cpu);
    }

    #[test]
    fn cpy_absolute() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Cpy, AddrMode::Absolute),
            0x00, 0x01,
            instruction(OpCode::Cpy, AddrMode::Absolute),
            0x01, 0x01,
            instruction(OpCode::Cpy, AddrMode::Absolute),
            0x02, 0x01,
        ];
        let mut cpu = get_cpu(code);
        cpu.bus.ram[0x100] = 35;
        cpu.bus.ram[0x101] = 0;
        cpu.bus.ram[0x102] = 255;

        test_flags(&mut cpu);
    }

    fn test_flags(cpu: &mut Cpu) {
        cpu.acc = 0;
        while !cpu.step() {};
        assert_eq!(cpu.ps.carry(), false);
        assert_eq!(cpu.ps.zero(), false);
        assert_eq!(cpu.ps.negative(), false);

        cpu.acc = 35;
        while !cpu.step() {};
        assert_eq!(cpu.ps.carry(), true);
        assert_eq!(cpu.ps.zero(), true);
        assert_eq!(cpu.ps.negative(), false);

        cpu.acc = 255;
        while !cpu.step() {};
        assert_eq!(cpu.ps.carry(), false);
        assert_eq!(cpu.ps.zero(), true);
        assert_eq!(cpu.ps.negative(), true);
    }

}

// 8/8
#[cfg(test)]
mod eor {
    use super::*;

    #[test]
    fn eor_immediate() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Eor, AddrMode::Immediate),
            35,
            instruction(OpCode::Eor, AddrMode::Immediate),
            0,
            instruction(OpCode::Eor, AddrMode::Immediate),
            255,
            instruction(OpCode::Eor, AddrMode::Immediate),
            5,
        ];
        let mut cpu = get_cpu(code);
        test_results(&mut cpu, 0, 0);
    }

    #[test]
    fn eor_zero_page() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Eor, AddrMode::Zp),
            1,
            instruction(OpCode::Eor, AddrMode::Zp),
            2,
            instruction(OpCode::Eor, AddrMode::Zp),
            3,
            instruction(OpCode::Eor, AddrMode::Zp),
            4,
        ];
        let mut cpu = get_cpu(code);
        cpu.acc = 35;
        cpu.bus.ram[1] = 35;
        cpu.bus.ram[2] = 0;
        cpu.bus.ram[3] = 255;
        cpu.bus.ram[4] = 5;

        test_results(&mut cpu, 0, 0);
    }

    #[test]
    fn eor_zero_page_x() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Eor, AddrMode::ZpX),
            1,
            instruction(OpCode::Eor, AddrMode::ZpX),
            2,
            instruction(OpCode::Eor, AddrMode::ZpX),
            3,
            instruction(OpCode::Eor, AddrMode::ZpX),
            4,
        ];
        let mut cpu = get_cpu(code);
        cpu.acc = 35;
        cpu.x = 5;
        cpu.bus.ram[6] = 35;
        cpu.bus.ram[7] = 0;
        cpu.bus.ram[8] = 255;
        cpu.bus.ram[4] = 5;

        test_results(&mut cpu, 0, 0);
    }

    #[test]
    fn eor_absolute() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Eor, AddrMode::Absolute),
            0x00, 0x01,
            instruction(OpCode::Eor, AddrMode::Absolute),
            0x01, 0x01,
            instruction(OpCode::Eor, AddrMode::Absolute),
            0x02, 0x01,
            instruction(OpCode::Eor, AddrMode::Absolute),
            0x03, 0x01,
        ];
        let mut cpu = get_cpu(code);
        cpu.acc = 35;
        cpu.bus.ram[0x100] = 35;
        cpu.bus.ram[0x101] = 0;
        cpu.bus.ram[0x102] = 255;
        cpu.bus.ram[0x103] = 5;

        test_results(&mut cpu, 0, 0);
    }

    #[test]
    fn eor_absolute_x() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Eor, AddrMode::AbsoluteX),
            0x00, 0x01,
            instruction(OpCode::Eor, AddrMode::AbsoluteX),
            0x01, 0x01,
            instruction(OpCode::Eor, AddrMode::AbsoluteX),
            0x02, 0x01,
            instruction(OpCode::Eor, AddrMode::AbsoluteX),
            0x03, 0x01,
        ];
        let mut cpu = get_cpu(code);
        cpu.acc = 35;
        cpu.x = 5;
        cpu.bus.ram[0x105] = 35;
        cpu.bus.ram[0x106] = 0;
        cpu.bus.ram[0x107] = 255;
        cpu.bus.ram[0x108] = 5;

        test_results(&mut cpu, 0, 0);
    }

    #[test]
    fn eor_absolute_y() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Eor, AddrMode::AbsoluteY),
            0x00, 0x01,
            instruction(OpCode::Eor, AddrMode::AbsoluteY),
            0x01, 0x01,
            instruction(OpCode::Eor, AddrMode::AbsoluteY),
            0x02, 0x01,
            instruction(OpCode::Eor, AddrMode::AbsoluteY),
            0x03, 0x01,
        ];
        let mut cpu = get_cpu(code);
        cpu.acc = 35;
        cpu.y = 5;
        cpu.bus.ram[0x105] = 35;
        cpu.bus.ram[0x106] = 0;
        cpu.bus.ram[0x107] = 255;
        cpu.bus.ram[0x108] = 5;

        test_results(&mut cpu, 0, 0);
    }

    #[test]
    fn eor_indirect_x() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Eor, AddrMode::IndirectX),
            0x20,
            instruction(OpCode::Eor, AddrMode::IndirectX),
            0x20,
            instruction(OpCode::Eor, AddrMode::IndirectX),
            0x20,
            instruction(OpCode::Eor, AddrMode::IndirectX),
            0x20,
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
        cpu.bus.ram[0x503] = 5;

        test_results(&mut cpu, 2, 0);
    }

    #[test]
    fn eor_indirect_y() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Eor, AddrMode::IndirectY),
            0x20,
            instruction(OpCode::Eor, AddrMode::IndirectY),
            0x20,
            instruction(OpCode::Eor, AddrMode::IndirectY),
            0x20,
            instruction(OpCode::Eor, AddrMode::IndirectY),
            0x20,
        ];
        let mut cpu = get_cpu(code);
        cpu.acc = 35;
        cpu.y = 4;
        cpu.bus.ram[0x20] = 0x00;
        cpu.bus.ram[0x21] = 0x05;
        cpu.bus.ram[0x504] = 34;
        cpu.bus.ram[0x505] = 0;
        cpu.bus.ram[0x506] = 255;
        cpu.bus.ram[0x507] = 5;

        test_results(&mut cpu, 0, 1);
    }

    fn test_results(cpu: &mut Cpu, x_inc: u8, y_inc: u8) {
        cpu.acc = 34;
        while !cpu.step() {};
        assert_eq!(cpu.ps.zero(), false);
        assert_eq!(cpu.ps.negative(), false);
        assert_eq!(cpu.acc, 1);
        cpu.x += x_inc;
        cpu.y += y_inc;

        cpu.acc = 0;
        while !cpu.step() {};
        assert_eq!(cpu.ps.zero(), true);
        assert_eq!(cpu.ps.negative(), false);
        assert_eq!(cpu.acc, 0);
        cpu.x += x_inc;
        cpu.y += y_inc;

        cpu.acc = 255;
        while !cpu.step() {};
        assert_eq!(cpu.ps.zero(), true);
        assert_eq!(cpu.ps.negative(), true);
        assert_eq!(cpu.acc, 0);
        cpu.x += x_inc;
        cpu.y += y_inc;

        cpu.acc = 3;
        while !cpu.step() {};
        assert_eq!(cpu.ps.zero(), false);
        assert_eq!(cpu.ps.negative(), true);
        assert_eq!(cpu.acc, 6);
    }
}

// 8/8
#[cfg(test)]
mod ora {
    use super::*;

    #[test]
    fn ora_immediate() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Ora, AddrMode::Immediate),
            35,
            instruction(OpCode::Ora, AddrMode::Immediate),
            0,
            instruction(OpCode::Ora, AddrMode::Immediate),
            255,
            instruction(OpCode::Ora, AddrMode::Immediate),
            5,
        ];
        let mut cpu = get_cpu(code);
        test_results(&mut cpu, 0, 0);
    }



    #[test]
    fn ora_zero_page() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Ora, AddrMode::Zp),
            1,
            instruction(OpCode::Ora, AddrMode::Zp),
            2,
            instruction(OpCode::Ora, AddrMode::Zp),
            3,
            instruction(OpCode::Ora, AddrMode::Zp),
            4,
        ];
        let mut cpu = get_cpu(code);
        cpu.acc = 35;
        cpu.bus.ram[1] = 35;
        cpu.bus.ram[2] = 0;
        cpu.bus.ram[3] = 255;
        cpu.bus.ram[4] = 5;

        test_results(&mut cpu, 0, 0);
    }

    #[test]
    fn ora_zero_page_x() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Ora, AddrMode::ZpX),
            1,
            instruction(OpCode::Ora, AddrMode::ZpX),
            2,
            instruction(OpCode::Ora, AddrMode::ZpX),
            3,
            instruction(OpCode::Ora, AddrMode::ZpX),
            4,
        ];
        let mut cpu = get_cpu(code);
        cpu.acc = 35;
        cpu.x = 5;
        cpu.bus.ram[6] = 35;
        cpu.bus.ram[7] = 0;
        cpu.bus.ram[8] = 255;
        cpu.bus.ram[9] = 5;

        test_results(&mut cpu, 0, 0);
    }

    #[test]
    fn ora_absolute() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Ora, AddrMode::Absolute),
            0x00, 0x01,
            instruction(OpCode::Ora, AddrMode::Absolute),
            0x01, 0x01,
            instruction(OpCode::Ora, AddrMode::Absolute),
            0x02, 0x01,
            instruction(OpCode::Ora, AddrMode::Absolute),
            0x03, 0x01,
        ];
        let mut cpu = get_cpu(code);
        cpu.acc = 35;
        cpu.bus.ram[0x100] = 35;
        cpu.bus.ram[0x101] = 0;
        cpu.bus.ram[0x102] = 255;
        cpu.bus.ram[0x103] = 5;

        test_results(&mut cpu, 0, 0);
    }

    #[test]
    fn ora_absolute_x() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Ora, AddrMode::AbsoluteX),
            0x00, 0x01,
            instruction(OpCode::Ora, AddrMode::AbsoluteX),
            0x01, 0x01,
            instruction(OpCode::Ora, AddrMode::AbsoluteX),
            0x02, 0x01,
            instruction(OpCode::Ora, AddrMode::AbsoluteX),
            0x03, 0x01,
        ];
        let mut cpu = get_cpu(code);
        cpu.acc = 35;
        cpu.x = 5;
        cpu.bus.ram[0x105] = 35;
        cpu.bus.ram[0x106] = 0;
        cpu.bus.ram[0x107] = 255;
        cpu.bus.ram[0x108] = 5;

        test_results(&mut cpu, 0, 0);
    }

    #[test]
    fn ora_absolute_y() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Ora, AddrMode::AbsoluteY),
            0x00, 0x01,
            instruction(OpCode::Ora, AddrMode::AbsoluteY),
            0x01, 0x01,
            instruction(OpCode::Ora, AddrMode::AbsoluteY),
            0x02, 0x01,
            instruction(OpCode::Ora, AddrMode::AbsoluteY),
            0x03, 0x01,
        ];
        let mut cpu = get_cpu(code);
        cpu.acc = 35;
        cpu.y = 5;
        cpu.bus.ram[0x105] = 35;
        cpu.bus.ram[0x106] = 0;
        cpu.bus.ram[0x107] = 255;
        cpu.bus.ram[0x108] = 5;

        test_results(&mut cpu, 0, 0);
    }

    #[test]
    fn ora_indirect_x() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Ora, AddrMode::IndirectX),
            0x20,
            instruction(OpCode::Ora, AddrMode::IndirectX),
            0x20,
            instruction(OpCode::Ora, AddrMode::IndirectX),
            0x20,
            instruction(OpCode::Ora, AddrMode::IndirectX),
            0x20,
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
        cpu.bus.ram[0x503] = 5;

        test_results(&mut cpu, 2, 0);
    }

    #[test]
    fn ora_indirect_y() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Ora, AddrMode::IndirectY),
            0x20,
            instruction(OpCode::Ora, AddrMode::IndirectY),
            0x20,
            instruction(OpCode::Ora, AddrMode::IndirectY),
            0x20,
            instruction(OpCode::Ora, AddrMode::IndirectY),
            0x20,
        ];
        let mut cpu = get_cpu(code);
        cpu.acc = 35;
        cpu.y = 4;
        cpu.bus.ram[0x20] = 0x00;
        cpu.bus.ram[0x21] = 0x05;
        cpu.bus.ram[0x504] = 34;
        cpu.bus.ram[0x505] = 0;
        cpu.bus.ram[0x506] = 255;
        cpu.bus.ram[0x507] = 5;

        test_results(&mut cpu, 0, 1);
    }

    fn test_results(cpu: &mut Cpu, x_inc: u8, y_inc: u8) {
        cpu.acc = 34;
        while !cpu.step() {};
        assert_eq!(cpu.ps.zero(), false);
        assert_eq!(cpu.ps.negative(), false);
        assert_eq!(cpu.acc, 35);
        cpu.x += x_inc;
        cpu.y += y_inc;

        cpu.acc = 0;
        while !cpu.step() {};
        assert_eq!(cpu.ps.zero(), true);
        assert_eq!(cpu.ps.negative(), false);
        assert_eq!(cpu.acc, 0);
        cpu.x += x_inc;
        cpu.y += y_inc;

        cpu.acc = 255;
        while !cpu.step() {};
        assert_eq!(cpu.ps.zero(), false);
        assert_eq!(cpu.ps.negative(), true);
        assert_eq!(cpu.acc, 255);
        cpu.x += x_inc;
        cpu.y += y_inc;

        cpu.acc = 3;
        while !cpu.step() {};
        assert_eq!(cpu.ps.zero(), false);
        assert_eq!(cpu.ps.negative(), true);
        assert_eq!(cpu.acc, 7);
    }
}

