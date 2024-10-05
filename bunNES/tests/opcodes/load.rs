use bunNES::nes::opcodes::{AddrMode, OpCode};
use crate::opcodes::helpers::{get_cpu, instruction};

// 8/8
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
            0x20
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
            0x20
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

// 5/5
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

// 5/5
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

#[cfg(test)]
mod sta {
    use super::*;
    
    #[test]
    fn sta_zero_page() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Sta, AddrMode::Zp),
            1,
        ];
        let mut cpu = get_cpu(code);
        cpu.acc = 69;
        cpu.step();
        assert_eq!(cpu.bus.ram[1], 69);
    }
    
    #[test]
    fn sta_zero_page_x() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Sta, AddrMode::ZpX),
            1,
        ];
        let mut cpu = get_cpu(code);
        cpu.x = 5;
        cpu.acc = 69;
        cpu.step();
        assert_eq!(cpu.bus.ram[6], 69);
    }
    
    #[test]
    fn sta_absolute() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Sta, AddrMode::Absolute),
            0x00, 0x01,
        ];
        let mut cpu = get_cpu(code);
        cpu.acc = 69;
        cpu.step();
        assert_eq!(cpu.bus.ram[0x0100], 69);
    }
    
    #[test]
    fn sta_absolute_x() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Sta, AddrMode::AbsoluteX),
            0x00, 0x01,
        ];
        let mut cpu = get_cpu(code);
        cpu.acc = 69;
        cpu.x = 5;
        cpu.step();
        assert_eq!(cpu.bus.ram[0x105], 69);
    }
    
    #[test]
    fn sta_absolute_y() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Sta, AddrMode::AbsoluteY),
            0x00, 0x01,
        ];
        let mut cpu = get_cpu(code);
        cpu.acc = 69;
        cpu.y = 5;
        cpu.step();
        assert_eq!(cpu.bus.ram[0x105], 69);
    }
    
    #[test]
    fn sta_indirect_x() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Sta, AddrMode::IndirectX),
            0x20
        ];
        let mut cpu = get_cpu(code);
        cpu.acc = 69;
        cpu.x = 4;
        cpu.bus.ram[0x24] = 0x00;
        cpu.bus.ram[0x25] = 0x05;
        cpu.step();
        assert_eq!(cpu.bus.ram[0x500], 69);
    }
    
    #[test]
    fn sta_indirect_y() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Sta, AddrMode::IndirectY),
            0x20
        ];
        let mut cpu = get_cpu(code);
        cpu.acc = 69;
        cpu.y = 4;
        cpu.bus.ram[0x20] = 0x00;
        cpu.bus.ram[0x21] = 0x05;
        cpu.step();
        assert_eq!(cpu.bus.ram[0x504], 69);
    }
}

// 3/3
#[cfg(test)]
mod stx {
    use super::*;
    
    #[test]
    fn stx_zero_page() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Stx, AddrMode::Zp),
            1,
        ];
        let mut cpu = get_cpu(code);
        cpu.x = 69;
        cpu.step();
        assert_eq!(cpu.bus.ram[1], 69);
    }
    
    #[test]
    fn stx_zero_page_y() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Stx, AddrMode::ZpY),
            1,
        ];
        let mut cpu = get_cpu(code);
        cpu.y = 5;
        cpu.x = 69;
        cpu.step();
        assert_eq!(cpu.bus.ram[6], 69);
    }
    
    #[test]
    fn stx_absolute() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Stx, AddrMode::Absolute),
            0x00, 0x01,
        ];
        let mut cpu = get_cpu(code);
        cpu.x = 69;
        cpu.step();
        assert_eq!(cpu.bus.ram[0x0100], 69);
    }
}

// 3/3
#[cfg(test)]
mod sty {
    use super::*;
    
    #[test]
    fn sty_zero_page() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Sty, AddrMode::Zp),
            1,
        ];
        let mut cpu = get_cpu(code);
        cpu.y = 69;
        cpu.step();
        assert_eq!(cpu.bus.ram[1], 69);
    }
    
    #[test]
    fn sty_zero_page_x() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Sty, AddrMode::ZpX),
            1,
        ];
        let mut cpu = get_cpu(code);
        cpu.x = 5;
        cpu.y = 69;
        cpu.step();
        assert_eq!(cpu.bus.ram[6], 69);
    }
    
    #[test]
    fn sty_absolute() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Sty, AddrMode::Absolute),
            0x00, 0x01,
        ];
        let mut cpu = get_cpu(code);
        cpu.y = 69;
        cpu.step();
        assert_eq!(cpu.bus.ram[0x0100], 69);
    }
}

// 1/1
#[cfg(test)]
mod tax {
    use super::*;
    
    #[test]
    fn tax_implicit() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Tax, AddrMode::Implicit),
            instruction(OpCode::Tax, AddrMode::Implicit),
            instruction(OpCode::Tax, AddrMode::Implicit),
        ];
        let mut cpu = get_cpu(code);
        cpu.acc = 69;
        cpu.step();
        assert_eq!(cpu.x, 69);
        assert_eq!(cpu.ps.zero(), false);
        assert_eq!(cpu.ps.negative(), false);
        
        cpu.acc = 0;
        cpu.step();
        assert_eq!(cpu.x, 0);
        assert_eq!(cpu.ps.zero(), true);
        assert_eq!(cpu.ps.negative(), false);
        
        cpu.acc = 255;
        cpu.step();
        assert_eq!(cpu.x, 255);
        assert_eq!(cpu.ps.zero(), false);
        assert_eq!(cpu.ps.negative(), true);
    }
}

// 1/1
#[cfg(test)]
mod tay {
    use super::*;
    
    #[test]
    fn tay_implicit() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Tay, AddrMode::Implicit),
            instruction(OpCode::Tay, AddrMode::Implicit),
            instruction(OpCode::Tay, AddrMode::Implicit),
        ];
        let mut cpu = get_cpu(code);
        cpu.acc = 69;
        cpu.step();
        assert_eq!(cpu.y, 69);
        assert_eq!(cpu.ps.zero(), false);
        assert_eq!(cpu.ps.negative(), false);
        
        cpu.acc = 0;
        cpu.step();
        assert_eq!(cpu.y, 0);
        assert_eq!(cpu.ps.zero(), true);
        assert_eq!(cpu.ps.negative(), false);
        
        cpu.acc = 255;
        cpu.step();
        assert_eq!(cpu.y, 255);
        assert_eq!(cpu.ps.zero(), false);
        assert_eq!(cpu.ps.negative(), true);
    }
}

// 1/1
#[cfg(test)]
mod tsx {
    use super::*;
    
    #[test]
    fn tsx_implicit() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Tsx, AddrMode::Implicit),
            instruction(OpCode::Tsx, AddrMode::Implicit),
            instruction(OpCode::Tsx, AddrMode::Implicit),
        ];
        let mut cpu = get_cpu(code);
        cpu.sp = 69;
        cpu.step();
        assert_eq!(cpu.x, 69);
        assert_eq!(cpu.ps.zero(), false);
        assert_eq!(cpu.ps.negative(), false);
        
        cpu.sp = 0;
        cpu.step();
        assert_eq!(cpu.x, 0);
        assert_eq!(cpu.ps.zero(), true);
        assert_eq!(cpu.ps.negative(), false);
        
        cpu.sp = 255;
        cpu.step();
        assert_eq!(cpu.x, 255);
        assert_eq!(cpu.ps.zero(), false);
        assert_eq!(cpu.ps.negative(), true);
    }
}

// 1/1
#[cfg(test)]
mod txa {
    use super::*;
    
    #[test]
    fn txa_implicit() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Txa, AddrMode::Implicit),
            instruction(OpCode::Txa, AddrMode::Implicit),
            instruction(OpCode::Txa, AddrMode::Implicit),
        ];
        let mut cpu = get_cpu(code);
        cpu.sp = 69;
        cpu.step();
        assert_eq!(cpu.acc, 69);
        assert_eq!(cpu.ps.zero(), false);
        assert_eq!(cpu.ps.negative(), false);
        
        cpu.sp = 0;
        cpu.step();
        assert_eq!(cpu.acc, 0);
        assert_eq!(cpu.ps.zero(), true);
        assert_eq!(cpu.ps.negative(), false);
        
        cpu.sp = 255;
        cpu.step();
        assert_eq!(cpu.acc, 255);
        assert_eq!(cpu.ps.zero(), false);
        assert_eq!(cpu.ps.negative(), true);
    }
}

// 1/1
#[cfg(test)]
mod txs {
    use super::*;
    
    #[test]
    fn txs_implicit() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Txs, AddrMode::Implicit),
        ];
        let mut cpu = get_cpu(code);
        cpu.x = 69;
        cpu.step();
        assert_eq!(cpu.sp, 69);
    }
}

// 1/1
#[cfg(test)]
mod tya {
    use super::*;
    
    #[test]
    fn tya_implicit() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Tya, AddrMode::Implicit),
            instruction(OpCode::Tya, AddrMode::Implicit),
            instruction(OpCode::Tya, AddrMode::Implicit),
        ];
        let mut cpu = get_cpu(code);
        cpu.y = 69;
        cpu.step();
        assert_eq!(cpu.acc, 69);
        assert_eq!(cpu.ps.zero(), false);
        assert_eq!(cpu.ps.negative(), false);
        
        cpu.y = 0;
        cpu.step();
        assert_eq!(cpu.acc, 0);
        assert_eq!(cpu.ps.zero(), true);
        assert_eq!(cpu.ps.negative(), false);
        
        cpu.y = 255;
        cpu.step();
        assert_eq!(cpu.acc, 255);
        assert_eq!(cpu.ps.zero(), false);
        assert_eq!(cpu.ps.negative(), true);
    }
}

