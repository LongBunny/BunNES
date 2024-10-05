use bunNES::nes::opcodes::{AddrMode, OpCode};
use crate::opcodes::helpers::{get_cpu, instruction};
use bunNES::nes::cpu::Cpu;

// 8/8
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

// 5/5
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

// 4/4
#[cfg(test)]
mod dec {
    use super::*;

    #[test]
    fn dec_zero_page() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Dec, AddrMode::Zp),
            0x01,
            instruction(OpCode::Dec, AddrMode::Zp),
            0x02,
            instruction(OpCode::Dec, AddrMode::Zp),
            0x03,
        ];
        let mut cpu = get_cpu(code);
        cpu.bus.ram[0x01] = 2;
        cpu.bus.ram[0x02] = 1;
        cpu.bus.ram[0x03] = 0;

        test_flags(&mut cpu);
    }

    #[test]
    fn dec_zero_page_x() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Dec, AddrMode::ZpX),
            0x00,
            instruction(OpCode::Dec, AddrMode::ZpX),
            0x01,
            instruction(OpCode::Dec, AddrMode::ZpX),
            0x02,
        ];
        let mut cpu = get_cpu(code);
        cpu.x = 0x01;
        cpu.bus.ram[0x01] = 2;
        cpu.bus.ram[0x02] = 1;
        cpu.bus.ram[0x03] = 0;

        test_flags(&mut cpu);
    }

    #[test]
    fn dec_zero_absolute() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Dec, AddrMode::Absolute),
            0x01, 0x00,
            instruction(OpCode::Dec, AddrMode::Absolute),
            0x02, 0x00,
            instruction(OpCode::Dec, AddrMode::Absolute),
            0x03, 0x00,
        ];
        let mut cpu = get_cpu(code);
        cpu.bus.ram[0x01] = 2;
        cpu.bus.ram[0x02] = 1;
        cpu.bus.ram[0x03] = 0;

        test_flags(&mut cpu);
    }

    #[test]
    fn dec_zero_absolute_x() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Dec, AddrMode::AbsoluteX),
            0x00, 0x00,
            instruction(OpCode::Dec, AddrMode::AbsoluteX),
            0x01, 0x00,
            instruction(OpCode::Dec, AddrMode::AbsoluteX),
            0x02, 0x00,
        ];
        let mut cpu = get_cpu(code);
        cpu.x = 0x01;
        cpu.bus.ram[0x01] = 2;
        cpu.bus.ram[0x02] = 1;
        cpu.bus.ram[0x03] = 0;

        test_flags(&mut cpu);
    }
    
    fn test_flags(cpu: &mut Cpu) {
        while !cpu.step() {};
        assert_eq!(cpu.bus.ram[0x01], 1);
        assert_eq!(cpu.ps.zero(), false);
        assert_eq!(cpu.ps.negative(), false);

        while !cpu.step() {};
        assert_eq!(cpu.bus.ram[0x02], 0);
        assert_eq!(cpu.ps.zero(), true);
        assert_eq!(cpu.ps.negative(), false);

        while !cpu.step() {};
        assert_eq!(cpu.bus.ram[0x03], 255);
        assert_eq!(cpu.ps.zero(), false);
        assert_eq!(cpu.ps.negative(), true);
    }
}

// 1/1
#[cfg(test)]
mod dex {
    use super::*;

    #[test]
    fn dex() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Dex, AddrMode::Implicit),
            instruction(OpCode::Dex, AddrMode::Implicit),
            instruction(OpCode::Dex, AddrMode::Implicit),
        ];
        let mut cpu = get_cpu(code);

        cpu.x = 2;
        while !cpu.step() {};
        assert_eq!(cpu.x, 1);
        assert_eq!(cpu.ps.zero(), false);
        assert_eq!(cpu.ps.negative(), false);
        
        while !cpu.step() {};
        assert_eq!(cpu.x, 0);
        assert_eq!(cpu.ps.zero(), true);
        assert_eq!(cpu.ps.negative(), false);

        while !cpu.step() {};
        assert_eq!(cpu.x, 255);
        assert_eq!(cpu.ps.zero(), false);
        assert_eq!(cpu.ps.negative(), true);
    }
}

// 1/1
#[cfg(test)]
mod dey {
    use super::*;

    #[test]
    fn dey() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Dey, AddrMode::Implicit),
            instruction(OpCode::Dey, AddrMode::Implicit),
            instruction(OpCode::Dey, AddrMode::Implicit),
        ];
        let mut cpu = get_cpu(code);

        cpu.y = 2;
        while !cpu.step() {};
        assert_eq!(cpu.y, 1);
        assert_eq!(cpu.ps.zero(), false);
        assert_eq!(cpu.ps.negative(), false);

        while !cpu.step() {};
        assert_eq!(cpu.y, 0);
        assert_eq!(cpu.ps.zero(), true);
        assert_eq!(cpu.ps.negative(), false);

        while !cpu.step() {};
        assert_eq!(cpu.y, 255);
        assert_eq!(cpu.ps.zero(), false);
        assert_eq!(cpu.ps.negative(), true);
    }
}

// 4/4
#[cfg(test)]
mod inc {
    use super::*;

    #[test]
    fn inc_zero_page() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Inc, AddrMode::Zp),
            0x01,
            instruction(OpCode::Inc, AddrMode::Zp),
            0x02,
            instruction(OpCode::Inc, AddrMode::Zp),
            0x03,
        ];
        let mut cpu = get_cpu(code);
        cpu.bus.ram[0x01] = 254;
        cpu.bus.ram[0x02] = 255;
        cpu.bus.ram[0x03] = 1;

        test_flags(&mut cpu);
    }

    #[test]
    fn inc_zero_page_x() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Inc, AddrMode::ZpX),
            0x00,
            instruction(OpCode::Inc, AddrMode::ZpX),
            0x01,
            instruction(OpCode::Inc, AddrMode::ZpX),
            0x02,
        ];
        let mut cpu = get_cpu(code);
        cpu.x = 0x01;
        cpu.bus.ram[0x01] = 254;
        cpu.bus.ram[0x02] = 255;
        cpu.bus.ram[0x03] = 0;

        test_flags(&mut cpu);
    }

    #[test]
    fn inc_zero_absolute() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Inc, AddrMode::Absolute),
            0x01, 0x00,
            instruction(OpCode::Inc, AddrMode::Absolute),
            0x02, 0x00,
            instruction(OpCode::Inc, AddrMode::Absolute),
            0x03, 0x00,
        ];
        let mut cpu = get_cpu(code);
        cpu.bus.ram[0x01] = 254;
        cpu.bus.ram[0x02] = 255;
        cpu.bus.ram[0x03] = 0;

        test_flags(&mut cpu);
    }

    #[test]
    fn inc_zero_absolute_x() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Inc, AddrMode::AbsoluteX),
            0x00, 0x00,
            instruction(OpCode::Inc, AddrMode::AbsoluteX),
            0x01, 0x00,
            instruction(OpCode::Inc, AddrMode::AbsoluteX),
            0x02, 0x00,
        ];
        let mut cpu = get_cpu(code);
        cpu.x = 0x01;
        cpu.bus.ram[0x01] = 254;
        cpu.bus.ram[0x02] = 255;
        cpu.bus.ram[0x03] = 0;

        test_flags(&mut cpu);
    }

    fn test_flags(cpu: &mut Cpu) {
        while !cpu.step() {};
        assert_eq!(cpu.bus.ram[0x01], 255);
        assert_eq!(cpu.ps.zero(), false);
        assert_eq!(cpu.ps.negative(), true);

        while !cpu.step() {};
        assert_eq!(cpu.bus.ram[0x02], 0);
        assert_eq!(cpu.ps.zero(), true);
        assert_eq!(cpu.ps.negative(), false);

        while !cpu.step() {};
        assert_eq!(cpu.bus.ram[0x03], 1);
        assert_eq!(cpu.ps.zero(), false);
        assert_eq!(cpu.ps.negative(), false);
    }
}

// 1/1
#[cfg(test)]
mod inx {
    use super::*;

    #[test]
    fn inx() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Inx, AddrMode::Implicit),
            instruction(OpCode::Inx, AddrMode::Implicit),
            instruction(OpCode::Inx, AddrMode::Implicit),
        ];
        let mut cpu = get_cpu(code);

        cpu.x = 254;
        while !cpu.step() {};
        assert_eq!(cpu.x, 255);
        assert_eq!(cpu.ps.zero(), false);
        assert_eq!(cpu.ps.negative(), true);

        while !cpu.step() {};
        assert_eq!(cpu.x, 0);
        assert_eq!(cpu.ps.zero(), true);
        assert_eq!(cpu.ps.negative(), false);

        while !cpu.step() {};
        assert_eq!(cpu.x, 1);
        assert_eq!(cpu.ps.zero(), false);
        assert_eq!(cpu.ps.negative(), false);
    }
}

// 1/1
#[cfg(test)]
mod iny {
    use super::*;

    #[test]
    fn iny() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Iny, AddrMode::Implicit),
            instruction(OpCode::Iny, AddrMode::Implicit),
            instruction(OpCode::Iny, AddrMode::Implicit),
        ];
        let mut cpu = get_cpu(code);

        cpu.y = 254;
        while !cpu.step() {};
        assert_eq!(cpu.y, 255);
        assert_eq!(cpu.ps.zero(), false);
        assert_eq!(cpu.ps.negative(), true);

        while !cpu.step() {};
        assert_eq!(cpu.y, 0);
        assert_eq!(cpu.ps.zero(), true);
        assert_eq!(cpu.ps.negative(), false);

        while !cpu.step() {};
        assert_eq!(cpu.y, 1);
        assert_eq!(cpu.ps.zero(), false);
        assert_eq!(cpu.ps.negative(), false);
    }
}

// 5/5
#[cfg(test)]
mod lsr {
    use super::*;

    #[test]
    fn lsr_flags() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Lsr, AddrMode::Accumulator),
            instruction(OpCode::Lsr, AddrMode::Accumulator),
            instruction(OpCode::Lsr, AddrMode::Accumulator),
        ];
        let mut cpu = get_cpu(code);
        cpu.acc = 0b1;
        while !cpu.step() {};
        assert_eq!(cpu.ps.carry(), true);
        assert_eq!(cpu.ps.zero(), true);
        assert_eq!(cpu.ps.negative(), false);

        cpu.acc = 0b10;
        while !cpu.step() {};
        assert_eq!(cpu.ps.carry(), false);
        assert_eq!(cpu.ps.zero(), false);
        assert_eq!(cpu.ps.negative(), false);

        cpu.acc = 0b1111_1111;
        while !cpu.step() {};
        assert_eq!(cpu.ps.carry(), true);
        assert_eq!(cpu.ps.zero(), false);
        assert_eq!(cpu.ps.negative(), false);
    }

    #[test]
    fn lsr_accumulator() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Lsr, AddrMode::Accumulator),
            instruction(OpCode::Lsr, AddrMode::Accumulator),
        ];
        let mut cpu = get_cpu(code);
        cpu.acc = 8;
        while !cpu.step() {};
        assert_eq!(cpu.acc, 4);
        while !cpu.step() {};
        assert_eq!(cpu.acc, 2);
    }

    #[test]
    fn lsr_zero_page() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Lsr, AddrMode::Zp),
            1,
        ];
        let mut cpu = get_cpu(code);
        cpu.bus.ram[1] = 8;
        while !cpu.step() {};
        assert_eq!(cpu.bus.ram[1], 4);
    }

    #[test]
    fn lsr_zero_page_x() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Lsr, AddrMode::ZpX),
            1,
        ];
        let mut cpu = get_cpu(code);
        cpu.x = 2;
        cpu.bus.ram[3] = 8;
        while !cpu.step() {};
        assert_eq!(cpu.bus.ram[3], 4);
    }

    #[test]
    fn lsr_absolute() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Lsr, AddrMode::Absolute),
            0x00, 0x01
        ];
        let mut cpu = get_cpu(code);
        cpu.bus.ram[0x100] = 8;
        while !cpu.step() {};
        assert_eq!(cpu.bus.ram[0x100], 4);
    }

    #[test]
    fn lsr_absolute_x() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Lsr, AddrMode::AbsoluteX),
            0x00, 0x01
        ];
        let mut cpu = get_cpu(code);
        cpu.x = 5;
        cpu.bus.ram[0x105] = 8;
        while !cpu.step() {};
        assert_eq!(cpu.bus.ram[0x105], 4);
    }
}

// 1/1
#[cfg(test)]
mod rol {
    use super::*;

    #[test]
    fn rol_flags() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Rol, AddrMode::Accumulator),
            instruction(OpCode::Rol, AddrMode::Accumulator),
            instruction(OpCode::Rol, AddrMode::Accumulator),
        ];
        let mut cpu = get_cpu(code);
        cpu.acc = 0b0000_0001;
        while !cpu.step() {};
        assert_eq!(cpu.ps.carry(), false);
        assert_eq!(cpu.ps.zero(), false);
        assert_eq!(cpu.ps.negative(), false);

        cpu.acc = 0b1000_0000;
        while !cpu.step() {};
        assert_eq!(cpu.ps.carry(), true);
        assert_eq!(cpu.ps.zero(), true);
        assert_eq!(cpu.ps.negative(), false);

        cpu.acc = 0b0100_0000;
        while !cpu.step() {};
        assert_eq!(cpu.ps.carry(), false);
        assert_eq!(cpu.ps.zero(), false);
        assert_eq!(cpu.ps.negative(), true);
    }

    #[test]
    fn rol_accumulator() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Rol, AddrMode::Accumulator),
            instruction(OpCode::Rol, AddrMode::Accumulator),
        ];
        let mut cpu = get_cpu(code);
        cpu.acc = 0b0000_0001;
        cpu.ps.set_carry(false);
        while !cpu.step() {};
        assert_eq!(cpu.acc, 2);

        cpu.acc = 0b0000_0001;
        cpu.ps.set_carry(true);
        while !cpu.step() {};
        assert_eq!(cpu.acc, 3);
    }

    #[test]
    fn rol_zero_page() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Rol, AddrMode::Zp),
            1,
        ];
        let mut cpu = get_cpu(code);
        cpu.bus.ram[1] = 0b0000_0001;
        cpu.ps.set_carry(false);
        while !cpu.step() {};
        assert_eq!(cpu.bus.ram[1], 2);

        cpu.bus.ram[1] = 0b0000_0001;
        cpu.ps.set_carry(true);
        while !cpu.step() {};
        assert_eq!(cpu.bus.ram[1], 3);
    }

    #[test]
    fn rol_zero_page_x() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Rol, AddrMode::ZpX),
            1,
        ];
        let mut cpu = get_cpu(code);
        cpu.x = 2;
        cpu.bus.ram[3] = 0b0000_0001;
        cpu.ps.set_carry(false);
        while !cpu.step() {};
        assert_eq!(cpu.bus.ram[3], 2);

        cpu.bus.ram[3] = 0b0000_0001;
        cpu.ps.set_carry(true);
        while !cpu.step() {};
        assert_eq!(cpu.bus.ram[3], 3);
    }

    #[test]
    fn rol_absolute() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Rol, AddrMode::Absolute),
            0x00, 0x01
        ];
        let mut cpu = get_cpu(code);
        cpu.bus.ram[0x0100] = 0b0000_0001;
        cpu.ps.set_carry(false);
        while !cpu.step() {};
        assert_eq!(cpu.bus.ram[0x0100], 2);

        cpu.bus.ram[0x0100] = 0b0000_0001;
        cpu.ps.set_carry(true);
        while !cpu.step() {};
        assert_eq!(cpu.bus.ram[0x0100], 3);
    }

    #[test]
    fn rol_absolute_x() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Rol, AddrMode::AbsoluteX),
            0x00, 0x01
        ];
        let mut cpu = get_cpu(code);
        cpu.x = 5;
        cpu.bus.ram[0x0105] = 0b0000_0001;
        cpu.ps.set_carry(false);
        while !cpu.step() {};
        assert_eq!(cpu.bus.ram[0x0105], 2);

        cpu.bus.ram[0x0105] = 0b0000_0001;
        cpu.ps.set_carry(true);
        while !cpu.step() {};
        assert_eq!(cpu.bus.ram[0x0105], 3);
    }
}

// 5/5
#[cfg(test)]
mod ror {
    use super::*;

    #[test]
    fn ror_flags() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Ror, AddrMode::Accumulator),
            instruction(OpCode::Ror, AddrMode::Accumulator),
            instruction(OpCode::Ror, AddrMode::Accumulator),
        ];
        let mut cpu = get_cpu(code);
        cpu.acc = 0b0000_0010;
        while !cpu.step() {};
        assert_eq!(cpu.ps.carry(), false);
        assert_eq!(cpu.ps.zero(), false);
        assert_eq!(cpu.ps.negative(), false);

        cpu.acc = 0b0000_0001;
        while !cpu.step() {};
        assert_eq!(cpu.ps.carry(), true);
        assert_eq!(cpu.ps.zero(), true);
        assert_eq!(cpu.ps.negative(), false);

        cpu.acc = 0b0000_0000;
        cpu.ps.set_carry(true);
        while !cpu.step() {};
        assert_eq!(cpu.ps.carry(), false);
        assert_eq!(cpu.ps.zero(), false);
        assert_eq!(cpu.ps.negative(), true);
    }

    #[test]
    fn ror_accumulator() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Ror, AddrMode::Accumulator),
            instruction(OpCode::Ror, AddrMode::Accumulator),
        ];
        let mut cpu = get_cpu(code);
        cpu.acc = 0b0000_0010;
        cpu.ps.set_carry(false);
        while !cpu.step() {};
        assert_eq!(cpu.acc, 1);

        cpu.acc = 0b0000_0001;
        cpu.ps.set_carry(true);
        while !cpu.step() {};
        assert_eq!(cpu.acc, 0b1000_0000);
    }

    #[test]
    fn ror_zero_page() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Ror, AddrMode::Zp),
            1,
        ];
        let mut cpu = get_cpu(code);
        cpu.bus.ram[1] = 0b0000_0010;
        cpu.ps.set_carry(false);
        while !cpu.step() {};
        assert_eq!(cpu.bus.ram[1], 1);

        cpu.bus.ram[1] = 0b0000_0001;
        cpu.ps.set_carry(true);
        while !cpu.step() {};
        assert_eq!(cpu.bus.ram[1], 0b1000_0000);
    }

    #[test]
    fn ror_zero_page_x() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Ror, AddrMode::ZpX),
            1,
        ];
        let mut cpu = get_cpu(code);
        cpu.x = 2;
        cpu.bus.ram[3] = 0b0000_0010;
        cpu.ps.set_carry(false);
        while !cpu.step() {};
        assert_eq!(cpu.bus.ram[3], 1);

        cpu.bus.ram[3] = 0b0000_0001;
        cpu.ps.set_carry(true);
        while !cpu.step() {};
        assert_eq!(cpu.bus.ram[3], 0b1000_0000);
    }

    #[test]
    fn ror_absolute() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Ror, AddrMode::Absolute),
            0x00, 0x01
        ];
        let mut cpu = get_cpu(code);
        cpu.bus.ram[0x0100] = 0b0000_0010;
        cpu.ps.set_carry(false);
        while !cpu.step() {};
        assert_eq!(cpu.bus.ram[0x0100], 1);

        cpu.bus.ram[0x0100] = 0b0000_0001;
        cpu.ps.set_carry(true);
        while !cpu.step() {};
        assert_eq!(cpu.bus.ram[0x0100], 0b1000_0000);
    }

    #[test]
    fn ror_absolute_x() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Ror, AddrMode::AbsoluteX),
            0x00, 0x01
        ];
        let mut cpu = get_cpu(code);
        cpu.x = 5;
        cpu.bus.ram[0x0105] = 0b0000_0010;
        cpu.ps.set_carry(false);
        while !cpu.step() {};
        assert_eq!(cpu.bus.ram[0x0105], 1);

        cpu.bus.ram[0x0105] = 0b0000_0001;
        cpu.ps.set_carry(true);
        while !cpu.step() {};
        assert_eq!(cpu.bus.ram[0x0105], 0b1000_0000);
    }
}

// 8/8
#[cfg(test)]
mod sbc {
    use super::*;

    #[test]
    fn sbc_flags() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Sbc, AddrMode::Immediate),
            34,
            instruction(OpCode::Sbc, AddrMode::Immediate),
            34,
            instruction(OpCode::Sbc, AddrMode::Immediate),
            1,
            instruction(OpCode::Sbc, AddrMode::Immediate),
            10,
        ];
        
        
        let mut cpu = get_cpu(code);
        cpu.ps.set_carry(true);
        cpu.acc = 35;
        while !cpu.step() {};
        assert_eq!(cpu.ps.carry(), false);
        assert_eq!(cpu.ps.zero(), false);
        assert_eq!(cpu.ps.overflow(), false);
        assert_eq!(cpu.ps.negative(), false);
        
        cpu.ps.set_carry(false);
        cpu.acc = 35;
        while !cpu.step() {};
        assert_eq!(cpu.ps.carry(), false);
        assert_eq!(cpu.ps.zero(), true);
        assert_eq!(cpu.ps.overflow(), false);
        assert_eq!(cpu.ps.negative(), false);
        
        cpu.ps.set_carry(true);
        cpu.acc = 255;
        while !cpu.step() {};
        assert_eq!(cpu.ps.carry(), true);
        assert_eq!(cpu.ps.zero(), false);
        assert_eq!(cpu.ps.overflow(), false);
        assert_eq!(cpu.ps.negative(), true);
        
        cpu.ps.set_carry(true);
        cpu.acc = 129;
        while !cpu.step() {};
        assert_eq!(cpu.ps.negative(), false);
        assert_eq!(cpu.ps.overflow(), false);
    }

    #[test]
    fn sbc_immediate() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Sbc, AddrMode::Immediate),
            34,
            instruction(OpCode::Sbc, AddrMode::Immediate),
            34,
        ];
        let mut cpu = get_cpu(code);
        cpu.ps.set_carry(true);
        cpu.acc = 35;
        while !cpu.step() {};
        assert_eq!(cpu.acc, 1);
        cpu.acc = 35;
        cpu.ps.set_carry(false);
        while !cpu.step() {};
        assert_eq!(cpu.acc, 0);
    }

    #[test]
    fn sbc_zero_page() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Sbc, AddrMode::Zp),
            1,
        ];
        let mut cpu = get_cpu(code);
        cpu.ps.set_carry(true);
        cpu.bus.ram[1] = 34;
        cpu.acc = 35;
        cpu.step();
        assert_eq!(cpu.acc, 1);
    }

    #[test]
    fn sbc_zero_page_x() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Sbc, AddrMode::ZpX),
            1,
        ];
        let mut cpu = get_cpu(code);
        cpu.ps.set_carry(true);
        cpu.acc = 35;
        cpu.x = 5;
        cpu.bus.ram[6] = 34;
        cpu.step();
        assert_eq!(cpu.acc, 1);
    }

    #[test]
    fn sbc_absolute() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Sbc, AddrMode::Absolute),
            0x00, 0x01,
        ];
        let mut cpu = get_cpu(code);
        cpu.ps.set_carry(true);
        cpu.acc = 35;
        cpu.bus.ram[0x100] = 34;
        cpu.step();
        assert_eq!(cpu.acc, 1);
    }

    #[test]
    fn sbc_absolute_x() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Sbc, AddrMode::AbsoluteX),
            0x00, 0x01,
        ];
        let mut cpu = get_cpu(code);
        cpu.ps.set_carry(true);
        cpu.acc = 35;
        cpu.x = 5;
        cpu.bus.ram[0x105] = 34;
        cpu.step();
        assert_eq!(cpu.acc, 1);
    }

    #[test]
    fn sbc_absolute_y() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Sbc, AddrMode::AbsoluteY),
            0x00, 0x01,
        ];
        let mut cpu = get_cpu(code);
        cpu.ps.set_carry(true);
        cpu.acc = 35;
        cpu.y = 5;
        cpu.bus.ram[0x105] = 34;
        cpu.step();
        assert_eq!(cpu.acc, 1);
    }

    #[test]
    fn sbc_indirect_x() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Sbc, AddrMode::IndirectX),
            20
        ];
        let mut cpu = get_cpu(code);
        cpu.ps.set_carry(true);
        cpu.acc = 35;
        cpu.x = 4;
        cpu.bus.ram[0x24] = 0x00;
        cpu.bus.ram[0x25] = 0x05;
        cpu.bus.ram[0x500] = 34;
        cpu.step();
        assert_eq!(cpu.acc, 1);
    }

    #[test]
    fn sbc_indirect_y() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Sbc, AddrMode::IndirectY),
            20
        ];
        let mut cpu = get_cpu(code);
        cpu.ps.set_carry(true);
        cpu.acc = 35;
        cpu.y = 4;
        cpu.bus.ram[0x20] = 0x00;
        cpu.bus.ram[0x21] = 0x05;
        cpu.bus.ram[0x504] = 34;
        cpu.step();
        assert_eq!(cpu.acc, 1);
    }
}

