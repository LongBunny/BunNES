
#[cfg(test)]
mod lda_tests {
    use crate::nes::opcodes::{AddrMode, OpCode};
    use crate::tests::opcode_test::{get_cpu, instruction};

    #[test]
    fn test_lda_immediate() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Lda, AddrMode::Immediate),
            69,
        ];
        let mut cpu = get_cpu(code);
        cpu.step();
        assert_eq!(cpu.acc, 69);
    }

    #[test]
    fn test_lda_zero_page() {
        let code: Vec<u8> = vec![
            instruction(OpCode::Lda, AddrMode::Zp),
            69,
        ];
        let mut cpu = get_cpu(code);
        cpu.bus.ram[1] = 69;
        cpu.step();
        assert_eq!(cpu.acc, 69);
    }
}