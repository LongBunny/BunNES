
#[cfg(test)]
mod lda {
    use bunNES::nes::opcodes::{AddrMode, OpCode};
    use crate::opcodes::helpers::{get_cpu, instruction};

    #[test]
    fn test_ldx_immediate() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Ldx, AddrMode::Immediate),
            69,
        ];
        let mut cpu = get_cpu(code);
        cpu.step();
        assert_eq!(cpu.x, 69);
    }

    #[test]
    fn test_ldx_zero_page() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Ldx, AddrMode::Zp),
            1,
        ];
        let mut cpu = get_cpu(code);
        cpu.bus.ram[1] = 69;
        cpu.step();
        assert_eq!(cpu.x, 69);
    }
}