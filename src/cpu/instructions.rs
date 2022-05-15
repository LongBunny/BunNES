#[allow(unused_variables)]
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Instruction {
    ADC,
    AND,
    ASL,
    BCC,
    BCS,
    BEQ,
    BIT,
    BMI,
    BNE,
    BPL,
    BRK,
    BVC,
    BVS,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP,
    CPX,
    CPY,
    DEC,
    DEX,
    DEY,
    EOR,
    INC,
    INX,
    INY,
    JMP,
    JSR,
    LDA,
    LDX,
    LDY,
    LSR,
    NOP,
    ORA,
    PHA,
    PHP,
    PLA,
    PLP,
    ROL,
    ROR,
    RTI,
    RTS,
    SBC,
    SEC,
    SED,
    SEI,
    STA,
    STX,
    STY,
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum AddressingMode {
    Implied,
    Accumulator,
    Immediate,
    Zeropage,
    ZeropageX,
    ZeropageY,
    Relative,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndexedIndirect,
    IndirectIndexed,
}

impl Instruction {
    pub fn mode(&self, am: AddressingMode) -> u8 {
        for (i, op_code) in OP_CODES.iter().enumerate() {
            let v = match op_code {
                Some(val) => val,
                _ => continue,
            };
            if v.0 == *self && v.1 == am {
                return i as u8;
            }
        }
        panic!("unknown instruction: {:?} {:?}", self, am);
    }

    pub fn test_mode(ins: &Instruction, am: AddressingMode) -> u8 {
        for (i, op_code) in OP_CODES.iter().enumerate() {
            let v = match op_code {
                Some(val) => val,
                _ => continue,
            };
            if v.0 == *ins && v.1 == am {
                return i as u8;
            }
        }
        panic!("unknown instruction: {:?} {:?}", ins, am);
    }

    pub fn get(op_code: u8) -> Option<(Instruction, AddressingMode)> {
        OP_CODES[op_code as usize]
    }
}

pub static OP_CODES: [Option<(Instruction, AddressingMode)>; 256] = [
    Some((Instruction::BRK, AddressingMode::Implied)), // 0x00
    Some((Instruction::ORA, AddressingMode::ZeropageX)), // 0x01
    None,                                              // 0x02
    None,                                              // 0x03
    None,                                              // 0x04
    Some((Instruction::ORA, AddressingMode::Zeropage)), // 0x05
    Some((Instruction::ASL, AddressingMode::Zeropage)), // 0x06
    None,                                              // 0x07
    Some((Instruction::PHP, AddressingMode::Implied)), // 0x08
    Some((Instruction::ORA, AddressingMode::Immediate)), // 0x09
    Some((Instruction::ASL, AddressingMode::Accumulator)), // 0x0A
    None,                                              // 0x0B
    None,                                              // 0x0C
    Some((Instruction::ORA, AddressingMode::Absolute)), // 0x0D
    Some((Instruction::ASL, AddressingMode::Absolute)), // 0x0E
    None,                                              // 0x0F
    Some((Instruction::BPL, AddressingMode::Relative)), // 0x10
    Some((Instruction::ORA, AddressingMode::ZeropageY)), // 0x11
    Some((Instruction::ORA, AddressingMode::Zeropage)), // 0x12
    None,                                              // 0x13
    None,                                              // 0x14
    Some((Instruction::ORA, AddressingMode::ZeropageX)), // 0x15
    Some((Instruction::ASL, AddressingMode::ZeropageX)), // 0x16
    None,                                              // 0x17
    Some((Instruction::CLC, AddressingMode::Implied)), // 0x18
    Some((Instruction::ORA, AddressingMode::AbsoluteY)), // 0x19
    Some((Instruction::INC, AddressingMode::Accumulator)), // 0x1A
    None,                                              // 0x1B
    None,                                              // 0x1C
    Some((Instruction::ORA, AddressingMode::AbsoluteX)), // 0x1D
    Some((Instruction::ASL, AddressingMode::AbsoluteX)), // 0x1E
    None,                                              // 0x1F
    Some((Instruction::JSR, AddressingMode::Absolute)), // 0x20
    Some((Instruction::AND, AddressingMode::ZeropageX)), // 0x21
    None,                                              // 0x22
    None,                                              // 0x23
    Some((Instruction::BIT, AddressingMode::Zeropage)), // 0x24
    Some((Instruction::AND, AddressingMode::Zeropage)), // 0x25
    Some((Instruction::ROL, AddressingMode::Zeropage)), // 0x26
    None,                                              // 0x27
    Some((Instruction::PLP, AddressingMode::Implied)), // 0x28
    Some((Instruction::AND, AddressingMode::Immediate)), // 0x29
    Some((Instruction::ROL, AddressingMode::Accumulator)), // 0x2A
    None,                                              // 0x2B
    Some((Instruction::BIT, AddressingMode::Absolute)), // 0x2C
    Some((Instruction::AND, AddressingMode::Absolute)), // 0x2D
    Some((Instruction::ROL, AddressingMode::Absolute)), // 0x2E
    None,                                              // 0x2F
    Some((Instruction::BMI, AddressingMode::Relative)), // 0x30
    Some((Instruction::AND, AddressingMode::ZeropageY)), // 0x31
    Some((Instruction::AND, AddressingMode::Zeropage)), // 0x32
    None,                                              // 0x33
    Some((Instruction::BIT, AddressingMode::ZeropageX)), // 0x34
    Some((Instruction::AND, AddressingMode::ZeropageX)), // 0x35
    Some((Instruction::ROL, AddressingMode::ZeropageX)), // 0x36
    None,                                              // 0x37
    Some((Instruction::SEC, AddressingMode::Implied)), // 0x38
    Some((Instruction::AND, AddressingMode::AbsoluteY)), // 0x39
    Some((Instruction::DEC, AddressingMode::Accumulator)), // 0x3A
    None,                                              // 0x3B
    Some((Instruction::BIT, AddressingMode::AbsoluteX)), // 0x3C
    Some((Instruction::AND, AddressingMode::AbsoluteX)), // 0x3D
    Some((Instruction::ROL, AddressingMode::AbsoluteX)), // 0x3E
    None,                                              // 0x3F
    Some((Instruction::RTI, AddressingMode::Relative)), // 0x40
    Some((Instruction::EOR, AddressingMode::ZeropageX)), // 0x41
    None,                                              // 0x42
    None,                                              // 0x43
    None,                                              // 0x44
    Some((Instruction::EOR, AddressingMode::Zeropage)), // 0x45
    Some((Instruction::LSR, AddressingMode::Zeropage)), // 0x46
    None,                                              // 0x47
    Some((Instruction::PHA, AddressingMode::Implied)), // 0x48
    Some((Instruction::EOR, AddressingMode::Immediate)), // 0x49
    Some((Instruction::LSR, AddressingMode::Accumulator)), // 0x4A
    None,                                              // 0x4B
    Some((Instruction::JMP, AddressingMode::Absolute)), // 0x4C
    Some((Instruction::EOR, AddressingMode::Absolute)), // 0x4D
    Some((Instruction::LSR, AddressingMode::Absolute)), // 0x4E
    None,                                              // 0x4F
    Some((Instruction::BVC, AddressingMode::Relative)), // 0x50
    Some((Instruction::EOR, AddressingMode::ZeropageY)), // 0x51
    Some((Instruction::EOR, AddressingMode::Zeropage)), // 0x52
    None,                                              // 0x53
    None,                                              // 0x54
    Some((Instruction::EOR, AddressingMode::ZeropageX)), // 0x55
    Some((Instruction::LSR, AddressingMode::ZeropageX)), // 0x56
    None,                                              // 0x57
    Some((Instruction::CLI, AddressingMode::Implied)), // 0x58
    Some((Instruction::EOR, AddressingMode::AbsoluteY)), // 0x59
    None,                                              // 0x5A
    None,                                              // 0x5B
    None,                                              // 0x5C
    Some((Instruction::EOR, AddressingMode::AbsoluteX)), // 0x5D
    Some((Instruction::LSR, AddressingMode::AbsoluteX)), // 0x5E
    None,                                              // 0x5F
    Some((Instruction::RTS, AddressingMode::Implied)), // 0x60
    Some((Instruction::ADC, AddressingMode::ZeropageX)), // 0x61
    None,                                              // 0x62
    None,                                              // 0x63
    None,                                              // 0x64
    Some((Instruction::ADC, AddressingMode::Zeropage)), // 0x65
    Some((Instruction::ROR, AddressingMode::Zeropage)), // 0x66
    None,                                              // 0x67
    Some((Instruction::PLA, AddressingMode::Implied)), // 0x68
    Some((Instruction::ADC, AddressingMode::Immediate)), // 0x69
    Some((Instruction::ROR, AddressingMode::Accumulator)), // 0x6A
    None,                                              // 0x6B
    Some((Instruction::JMP, AddressingMode::Absolute)), // 0x6C
    Some((Instruction::ADC, AddressingMode::Absolute)), // 0x6D
    Some((Instruction::ROR, AddressingMode::Absolute)), // 0x6E
    None,                                              // 0x6F
    Some((Instruction::BVS, AddressingMode::Relative)), // 0x70
    Some((Instruction::ADC, AddressingMode::ZeropageY)), // 0x71
    Some((Instruction::ADC, AddressingMode::Zeropage)), // 0x72
    None,                                              // 0x73
    None,                                              // 0x74
    Some((Instruction::ADC, AddressingMode::ZeropageX)), // 0x75
    Some((Instruction::ROR, AddressingMode::ZeropageX)), // 0x76
    None,                                              // 0x77
    Some((Instruction::SEI, AddressingMode::Implied)), // 0x78
    Some((Instruction::ADC, AddressingMode::AbsoluteY)), // 0x79
    None,                                              // 0x7A
    None,                                              // 0x7B
    None,                                              // 0x7C
    Some((Instruction::ADC, AddressingMode::AbsoluteX)), // 0x7D
    Some((Instruction::ROR, AddressingMode::AbsoluteX)), // 0x7E
    None,                                              // 0x7F
    None,                                              // 0x80
    Some((Instruction::STA, AddressingMode::ZeropageX)), // 0x81
    None,                                              // 0x82
    None,                                              // 0x83
    Some((Instruction::STY, AddressingMode::Zeropage)), // 0x84
    Some((Instruction::STA, AddressingMode::Zeropage)), // 0x85
    Some((Instruction::STX, AddressingMode::Zeropage)), // 0x86
    None,                                              // 0x87
    Some((Instruction::DEY, AddressingMode::Implied)), // 0x88
    Some((Instruction::BIT, AddressingMode::Immediate)), // 0x89
    Some((Instruction::TXA, AddressingMode::Implied)), // 0x8A
    None,                                              // 0x8B
    Some((Instruction::STY, AddressingMode::Absolute)), // 0x8C
    Some((Instruction::STA, AddressingMode::Absolute)), // 0x8D
    Some((Instruction::STX, AddressingMode::Absolute)), // 0x8E
    None,                                              // 0x8F
    Some((Instruction::BCC, AddressingMode::Relative)), // 0x90
    Some((Instruction::STA, AddressingMode::ZeropageY)), // 0x91
    Some((Instruction::STA, AddressingMode::Zeropage)), // 0x92
    None,                                              // 0x93
    Some((Instruction::STY, AddressingMode::ZeropageX)), // 0x94
    Some((Instruction::STA, AddressingMode::ZeropageX)), // 0x95
    Some((Instruction::STX, AddressingMode::ZeropageY)), // 0x96
    None,                                              // 0x97
    Some((Instruction::TYA, AddressingMode::Implied)), // 0x98
    Some((Instruction::STA, AddressingMode::AbsoluteY)), // 0x99
    Some((Instruction::TXS, AddressingMode::Implied)), // 0x9A
    None,                                              // 0x9B
    None,                                              // 0x9C
    Some((Instruction::STA, AddressingMode::AbsoluteX)), // 0x9D
    None,                                              // 0x9E
    None,                                              // 0x9F
    Some((Instruction::LDY, AddressingMode::Immediate)), // 0xA0
    Some((Instruction::LDA, AddressingMode::ZeropageX)), // 0xA1
    Some((Instruction::LDX, AddressingMode::Immediate)), // 0xA2
    None,                                              // 0xA3
    Some((Instruction::LDY, AddressingMode::Zeropage)), // 0xA4
    Some((Instruction::LDA, AddressingMode::Zeropage)), // 0xA5
    Some((Instruction::LDX, AddressingMode::Zeropage)), // 0xA6
    None,                                              // 0xA7
    Some((Instruction::TAY, AddressingMode::Implied)), // 0xA8
    Some((Instruction::LDA, AddressingMode::Immediate)), // 0xA9
    Some((Instruction::TAX, AddressingMode::Implied)), // 0xAA
    None,                                              // 0xAB
    Some((Instruction::LDY, AddressingMode::Accumulator)), // 0xAC
    Some((Instruction::LDA, AddressingMode::Absolute)), // 0xAD
    Some((Instruction::LDY, AddressingMode::Absolute)), // 0xAE
    None,                                              // 0xAF
    Some((Instruction::BCS, AddressingMode::Relative)), // 0xB0
    Some((Instruction::LDA, AddressingMode::ZeropageY)), // 0xB1
    Some((Instruction::LDA, AddressingMode::Zeropage)), // 0xB2
    None,                                              // 0xB3
    Some((Instruction::LDY, AddressingMode::ZeropageX)), // 0xB4
    Some((Instruction::LDA, AddressingMode::ZeropageX)), // 0xB5
    Some((Instruction::LDX, AddressingMode::ZeropageY)), // 0xB6
    None,                                              // 0xB7
    Some((Instruction::CLV, AddressingMode::Implied)), // 0xB8
    Some((Instruction::LDA, AddressingMode::AbsoluteY)), // 0xB9
    Some((Instruction::TSX, AddressingMode::Implied)), // 0xBA
    None,                                              // 0xBB
    Some((Instruction::LDY, AddressingMode::AbsoluteX)), // 0xBC
    Some((Instruction::LDA, AddressingMode::AbsoluteX)), // 0xBD
    Some((Instruction::LDX, AddressingMode::AbsoluteY)), // 0xBE
    None,                                              // 0xBF
    Some((Instruction::CPY, AddressingMode::Immediate)), // 0xC0
    Some((Instruction::CMP, AddressingMode::ZeropageX)), // 0xC1
    None,                                              // 0xC2
    None,                                              // 0xC3
    Some((Instruction::CPY, AddressingMode::Zeropage)), // 0xC4
    Some((Instruction::CMP, AddressingMode::Zeropage)), // 0xC5
    Some((Instruction::DEC, AddressingMode::Zeropage)), // 0xC6
    None,                                              // 0xC7
    Some((Instruction::INY, AddressingMode::Implied)), // 0xC8
    Some((Instruction::CMP, AddressingMode::Immediate)), // 0xC9
    Some((Instruction::DEX, AddressingMode::Implied)), // 0xCA
    None,                                              // 0xCB
    Some((Instruction::CPY, AddressingMode::Absolute)), // 0xCC
    Some((Instruction::CMP, AddressingMode::Absolute)), // 0xCD
    Some((Instruction::DEC, AddressingMode::Absolute)), // 0xCE
    None,                                              // 0xCF
    Some((Instruction::BNE, AddressingMode::Relative)), // 0xD0
    Some((Instruction::CMP, AddressingMode::ZeropageY)), // 0xD1
    Some((Instruction::CMP, AddressingMode::Zeropage)), // 0xD2
    None,                                              // 0xD3
    None,                                              // 0xD4
    Some((Instruction::CMP, AddressingMode::ZeropageX)), // 0xD5
    Some((Instruction::DEC, AddressingMode::ZeropageX)), // 0xD6
    None,                                              // 0xD7
    Some((Instruction::CLD, AddressingMode::Implied)), // 0xD8
    Some((Instruction::CMP, AddressingMode::AbsoluteY)), // 0xD9
    None,                                              // 0xDA
    None,                                              // 0xDB
    None,                                              // 0xDC
    Some((Instruction::CMP, AddressingMode::AbsoluteX)), // 0xDD
    Some((Instruction::DEC, AddressingMode::AbsoluteX)), // 0xDE
    None,                                              // 0xDF
    Some((Instruction::CPX, AddressingMode::Immediate)), // 0xE0
    Some((Instruction::SBC, AddressingMode::ZeropageX)), // 0xE1
    None,                                              // 0xE2
    None,                                              // 0xE3
    Some((Instruction::CPX, AddressingMode::Zeropage)), // 0xE4
    Some((Instruction::SBC, AddressingMode::Zeropage)), // 0xE5
    Some((Instruction::INC, AddressingMode::Zeropage)), // 0xE6
    None,                                              // 0xE7
    Some((Instruction::INX, AddressingMode::Implied)), // 0xE8
    Some((Instruction::SBC, AddressingMode::Immediate)), // 0xE9
    Some((Instruction::NOP, AddressingMode::Implied)), // 0xEA
    None,                                              // 0xEB
    Some((Instruction::CPX, AddressingMode::Absolute)), // 0xEC
    Some((Instruction::SBC, AddressingMode::Absolute)), // 0xED
    Some((Instruction::INC, AddressingMode::Absolute)), // 0xEE
    None,                                              // 0xEF
    Some((Instruction::BEQ, AddressingMode::Relative)), // 0xF0
    Some((Instruction::SBC, AddressingMode::ZeropageY)), // 0xF1
    Some((Instruction::SBC, AddressingMode::Zeropage)), // 0xF2
    None,                                              // 0xF3
    None,                                              // 0xF4
    Some((Instruction::SBC, AddressingMode::ZeropageX)), // 0xF5
    Some((Instruction::INC, AddressingMode::ZeropageX)), // 0xF6
    None,                                              // 0xF7
    Some((Instruction::SED, AddressingMode::Implied)), // 0xF8
    Some((Instruction::SBC, AddressingMode::AbsoluteY)), // 0xF9
    None,                                              // 0xFA
    None,                                              // 0xFB
    None,                                              // 0xFC
    Some((Instruction::SBC, AddressingMode::AbsoluteX)), // 0xFD
    Some((Instruction::INC, AddressingMode::AbsoluteX)), // 0xFE
    None,                                              // 0xFF
];


