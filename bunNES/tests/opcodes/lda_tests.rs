
#[cfg(test)]
mod lda {
    use bunNES::nes::opcodes::{AddrMode, OpCode};
    use crate::opcodes::helpers::{get_cpu, instruction};

    #[test]
    fn test_lda_immediate() {
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
        assert_eq!(cpu.acc, 69);
        assert!(!cpu.ps.zero());
        while !cpu.step() {};
        assert_eq!(cpu.acc, 0);
        assert!(cpu.ps.zero());
        assert!(!cpu.ps.negative());
        while !cpu.step() {};
        assert_eq!(cpu.acc, 255);
        assert!(cpu.ps.negative());
    }

    #[test]
    fn test_lda_zero_page() {
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
    fn test_lda_zero_page_x() {
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
    fn test_lda_absolute() {
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
    fn test_lda_absolute_x() {
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
    fn test_lda_absolute_y() {
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
}