use std::fmt::{Display, Formatter};
use std::path::absolute;
use crate::nes::opcodes::AddrMode::*;
use crate::nes::opcodes::OpCode::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AddrMode {
    Implicit,
    Accumulator,
    Immediate,
    Zp,
    ZpX,
    ZpY,
    Relative,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY,
}

impl AddrMode {
    fn to_string(&self) -> &str {
        match self {
            Implicit => "Implicit",
            Accumulator => "Accumulator",
            Immediate => "Immediate",
            Zp => "Zero page",
            ZpX => "Zero page X",
            ZpY => "Zero page Y",
            Relative => "Relative",
            Absolute => "Absolute",
            AbsoluteX => "Absolute X",
            AbsoluteY => "Absolute Y",
            Indirect => "Indirect",
            IndirectX => "Indirect X",
            IndirectY => "Indirect Y",
        }
    }
}

impl Display for AddrMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Instruction {
    pub op_code: OpCode,
    pub addr_mode: AddrMode,
    // TODO: remove
    pub size: u8,
}


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum OpCode {
    Adc,
    And,
    Asl,
    Bcc,
    Bcs,
    Beq,
    Bew,
    Bit,
    Bmi,
    Bne,
    Bpl,
    Brk,
    Bvc,
    Bvs,
    Clc,
    Cld,
    Cli,
    Clv,
    Cmp,
    Cpx,
    Cpy,
    Dec,
    Dex,
    Dey,
    Eor,
    Inc,
    Inx,
    Iny,
    Jmp,
    Jsr,
    Lda,
    Ldx,
    Ldy,
    Lsr,
    Nop,
    Ora,
    Pha,
    Php,
    Pla,
    Plp,
    Rol,
    Ror,
    Rti,
    Rts,
    Sbc,
    Sec,
    Sed,
    Sei,
    Sta,
    Stx,
    Sty,
    Tax,
    Tay,
    Tsx,
    Txa,
    Txs,
    Tya,
}

impl OpCode {
    fn to_string(&self) -> &str {
        match self {
            Adc => "ADC",
            And => "AND",
            Asl => "ASL",
            Bcc => "BCC",
            Bcs => "BCS",
            Beq => "BEQ",
            Bew => "BEW",
            Bit => "BIT",
            Bmi => "BMI",
            Bne => "BNE",
            Bpl => "BPL",
            Brk => "BRK",
            Bvc => "BVC",
            Bvs => "BVS",
            Clc => "CLC",
            Cld => "CLD",
            Cli => "CLI",
            Clv => "CLV",
            Cmp => "CMP",
            Cpx => "CPX",
            Cpy => "CPY",
            Dec => "DEC",
            Dex => "DEX",
            Dey => "DEY",
            Eor => "EOR",
            Inc => "INC",
            Inx => "INX",
            Iny => "INY",
            Jmp => "JMP",
            Jsr => "JSR",
            Lda => "LDA",
            Ldx => "LDX",
            Ldy => "LDY",
            Lsr => "LSR",
            Nop => "NOP",
            Ora => "ORA",
            Pha => "PHA",
            Php => "PHP",
            Pla => "PLA",
            Plp => "PLP",
            Rol => "ROL",
            Ror => "ROR",
            Rti => "RTI",
            Rts => "RTS",
            Sbc => "SBC",
            Sec => "SEC",
            Sed => "SED",
            Sei => "SEI",
            Sta => "STA",
            Stx => "STX",
            Sty => "STY",
            Tax => "TAX",
            Tay => "TAY",
            Tsx => "TSX",
            Txa => "TXA",
            Txs => "TXS",
            Tya => "TYA",
        }
    }
}

impl Display for OpCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

pub fn op_code_from_instruction(to_find: Instruction) -> Option<usize> {
    OP_CODES.iter().position(|instruction| {
        if let Some(instruction) = instruction {
            instruction.op_code == to_find.op_code && instruction.addr_mode == to_find.addr_mode
        } else {
            false
        }
    })
}

pub static OP_CODES: [(Option<Instruction>); 256] = [
    Some(Instruction { op_code: Brk, addr_mode: Implicit, size: 1 }), // 0x00
    Some(Instruction { op_code: Ora, addr_mode: IndirectY, size: 2 }), // 0x01
    None, // 0x02
    None, // 0x03
    None, // 0x04
    Some(Instruction { op_code: Ora, addr_mode: Zp, size: 2 }), // 0x05
    Some(Instruction { op_code: Asl, addr_mode: Zp, size: 2 }), // 0x06
    None, // 0x07
    Some(Instruction { op_code: Php, addr_mode: Implicit, size: 1 }), // 0x08
    Some(Instruction { op_code: Ora, addr_mode: Immediate, size: 2 }), // 0x09
    Some(Instruction { op_code: Asl, addr_mode: Accumulator, size: 1 }), // 0x0a
    None, // 0x0b
    None, // 0x0c
    Some(Instruction { op_code: Ora, addr_mode: Absolute, size: 3 }), // 0x0d
    Some(Instruction { op_code: Asl, addr_mode: Absolute, size: 3 }), // 0x0e
    None, // 0x0f
    Some(Instruction { op_code: Bpl, addr_mode: Relative, size: 2 }), // 0x10
    Some(Instruction { op_code: Ora, addr_mode: IndirectX, size: 2 }), // 0x11
    Some(Instruction { op_code: Ora, addr_mode: Zp, size: 2 }), // 0x12
    None, // 0x13
    None, // 0x14
    Some(Instruction { op_code: Ora, addr_mode: ZpX, size: 2 }), // 0x15
    Some(Instruction { op_code: Asl, addr_mode: ZpX, size: 2 }), // 0x16
    None, // 0x17
    Some(Instruction { op_code: Clc, addr_mode: Implicit, size: 1 }), // 0x18
    Some(Instruction { op_code: Ora, addr_mode: AbsoluteY, size: 3 }), // 0x19
    Some(Instruction { op_code: Inc, addr_mode: Accumulator, size: 1 }), // 0x1a
    None, // 0x1b
    None, // 0x1c
    Some(Instruction { op_code: Ora, addr_mode: AbsoluteX, size: 3 }), // 0x1d
    Some(Instruction { op_code: Asl, addr_mode: AbsoluteX, size: 3 }), // 0x1e
    None, // 0x1f
    Some(Instruction { op_code: Jsr, addr_mode: Absolute, size: 3 }), // 0x20
    Some(Instruction { op_code: And, addr_mode: IndirectY, size: 2 }), // 0x21
    None, // 0x22
    None, // 0x23
    Some(Instruction { op_code: Bit, addr_mode: Zp, size: 2 }), // 0x24
    Some(Instruction { op_code: And, addr_mode: Zp, size: 2 }), // 0x25
    Some(Instruction { op_code: Rol, addr_mode: Zp, size: 2 }), // 0x26
    None, // 0x27
    Some(Instruction { op_code: Plp, addr_mode: Implicit, size: 1 }), // 0x28
    Some(Instruction { op_code: And, addr_mode: Immediate, size: 2 }), // 0x29
    Some(Instruction { op_code: Rol, addr_mode: Accumulator, size: 1 }), // 0x2a
    None, // 0x2b
    Some(Instruction { op_code: Bit, addr_mode: Absolute, size: 3 }), // 0x2c
    Some(Instruction { op_code: And, addr_mode: Absolute, size: 3 }), // 0x2d
    Some(Instruction { op_code: Rol, addr_mode: Absolute, size: 3 }), // 0x2e
    None, // 0x2f
    Some(Instruction { op_code: Bmi, addr_mode: Relative, size: 2 }), // 0x30
    Some(Instruction { op_code: And, addr_mode: IndirectX, size: 2 }), // 0x31
    Some(Instruction { op_code: And, addr_mode: Zp, size: 2 }), // 0x32
    None, // 0x33
    Some(Instruction { op_code: Bit, addr_mode: ZpX, size: 2 }), // 0x34
    Some(Instruction { op_code: And, addr_mode: ZpX, size: 2 }), // 0x35
    Some(Instruction { op_code: Rol, addr_mode: ZpX, size: 2 }), // 0x36
    None, // 0x37
    Some(Instruction { op_code: Sec, addr_mode: Implicit, size: 1 }), // 0x38
    Some(Instruction { op_code: And, addr_mode: AbsoluteY, size: 3 }), // 0x39
    Some(Instruction { op_code: Dec, addr_mode: Accumulator, size: 1 }), // 0x3a
    None, // 0x3b
    Some(Instruction { op_code: Bit, addr_mode: AbsoluteX, size: 3 }), // 0x3c
    Some(Instruction { op_code: And, addr_mode: AbsoluteX, size: 3 }), // 0x3d
    Some(Instruction { op_code: Rol, addr_mode: AbsoluteX, size: 3 }), // 0x3e
    None, // 0x3f
    Some(Instruction { op_code: Rti, addr_mode: Implicit, size: 1 }), // 0x40
    Some(Instruction { op_code: Eor, addr_mode: IndirectY, size: 2 }), // 0x41
    None, // 0x42
    None, // 0x43
    None, // 0x44
    Some(Instruction { op_code: Eor, addr_mode: Zp, size: 2 }), // 0x45
    Some(Instruction { op_code: Lsr, addr_mode: Zp, size: 2 }), // 0x46
    None, // 0x47
    Some(Instruction { op_code: Pha, addr_mode: Implicit, size: 1 }), // 0x48
    Some(Instruction { op_code: Eor, addr_mode: Immediate, size: 2 }), // 0x49
    Some(Instruction { op_code: Lsr, addr_mode: Accumulator, size: 1 }), // 0x4a
    None, // 0x4b
    Some(Instruction { op_code: Jmp, addr_mode: Absolute, size: 3 }), // 0x4c
    Some(Instruction { op_code: Eor, addr_mode: Absolute, size: 3 }), // 0x4d
    Some(Instruction { op_code: Lsr, addr_mode: Absolute, size: 3 }), // 0x4e
    None, // 0x4f
    Some(Instruction { op_code: Bvc, addr_mode: Relative, size: 2 }), // 0x50
    Some(Instruction { op_code: Eor, addr_mode: IndirectX, size: 2 }), // 0x51
    Some(Instruction { op_code: Eor, addr_mode: Zp, size: 2 }), // 0x52
    None, // 0x53
    None, // 0x54
    Some(Instruction { op_code: Eor, addr_mode: ZpX, size: 2 }), // 0x55
    Some(Instruction { op_code: Lsr, addr_mode: ZpX, size: 2 }), // 0x56
    None, // 0x57
    Some(Instruction { op_code: Cli, addr_mode: Implicit, size: 1 }), // 0x58
    Some(Instruction { op_code: Eor, addr_mode: AbsoluteY, size: 3 }), // 0x59
    None, // 0x5a
    None, // 0x5b
    None, // 0x5c
    Some(Instruction { op_code: Eor, addr_mode: AbsoluteX, size: 3 }), // 0x5d
    Some(Instruction { op_code: Lsr, addr_mode: AbsoluteX, size: 3 }), // 0x5e
    None, // 0x5f
    Some(Instruction { op_code: Rts, addr_mode: Implicit, size: 1 }), // 0x60
    Some(Instruction { op_code: Adc, addr_mode: IndirectY, size: 2 }), // 0x61
    None, // 0x62
    None, // 0x63
    None, // 0x64
    Some(Instruction { op_code: Adc, addr_mode: Zp, size: 2 }), // 0x65
    Some(Instruction { op_code: Ror, addr_mode: Zp, size: 2 }), // 0x66
    None, // 0x67
    Some(Instruction { op_code: Pla, addr_mode: Implicit, size: 1 }), // 0x68
    Some(Instruction { op_code: Adc, addr_mode: Immediate, size: 2 }), // 0x69
    Some(Instruction { op_code: Ror, addr_mode: Accumulator, size: 1 }), // 0x6a
    None, // 0x6b
    Some(Instruction { op_code: Jmp, addr_mode: Indirect, size: 3 }), // 0x6c
    Some(Instruction { op_code: Adc, addr_mode: Absolute, size: 3 }), // 0x6d
    Some(Instruction { op_code: Ror, addr_mode: Absolute, size: 3 }), // 0x6e
    None, // 0x6f
    Some(Instruction { op_code: Bvs, addr_mode: Relative, size: 2 }), // 0x70
    Some(Instruction { op_code: Adc, addr_mode: IndirectX, size: 2 }), // 0x71
    Some(Instruction { op_code: Adc, addr_mode: Zp, size: 2 }), // 0x72
    None, // 0x73
    None, // 0x74
    Some(Instruction { op_code: Adc, addr_mode: ZpX, size: 2 }), // 0x75
    Some(Instruction { op_code: Ror, addr_mode: ZpX, size: 2 }), // 0x76
    None, // 0x77
    Some(Instruction { op_code: Sei, addr_mode: Implicit, size: 1 }), // 0x78
    Some(Instruction { op_code: Adc, addr_mode: AbsoluteY, size: 3 }), // 0x79
    None, // 0x7a
    None, // 0x7b
    Some(Instruction { op_code: Jmp, addr_mode: AbsoluteX, size: 3 }), // 0x7c
    Some(Instruction { op_code: Adc, addr_mode: AbsoluteX, size: 3 }), // 0x7d
    Some(Instruction { op_code: Ror, addr_mode: AbsoluteX, size: 3 }), // 0x7e
    None, // 0x7f
    None, // 0x80
    Some(Instruction { op_code: Sta, addr_mode: IndirectY, size: 2 }), // 0x81
    None, // 0x82
    None, // 0x83
    Some(Instruction { op_code: Sty, addr_mode: Zp, size: 2 }), // 0x84
    Some(Instruction { op_code: Sta, addr_mode: Zp, size: 2 }), // 0x85
    Some(Instruction { op_code: Stx, addr_mode: Zp, size: 2 }), // 0x86
    None, // 0x87
    Some(Instruction { op_code: Dey, addr_mode: Implicit, size: 1 }), // 0x88
    Some(Instruction { op_code: Bit, addr_mode: Immediate, size: 2 }), // 0x89
    Some(Instruction { op_code: Txa, addr_mode: Implicit, size: 1 }), // 0x8a
    None, // 0x8b
    Some(Instruction { op_code: Sty, addr_mode: Absolute, size: 3 }), // 0x8c
    Some(Instruction { op_code: Sta, addr_mode: Absolute, size: 3 }), // 0x8d
    Some(Instruction { op_code: Stx, addr_mode: Absolute, size: 3 }), // 0x8e
    None, // 0x8f
    Some(Instruction { op_code: Bcc, addr_mode: Relative, size: 2 }), // 0x90
    Some(Instruction { op_code: Sta, addr_mode: IndirectX, size: 2 }), // 0x91
    Some(Instruction { op_code: Sta, addr_mode: Zp, size: 2 }), // 0x92
    None, // 0x93
    Some(Instruction { op_code: Sty, addr_mode: ZpX, size: 2 }), // 0x94
    Some(Instruction { op_code: Sta, addr_mode: ZpX, size: 2 }), // 0x95
    Some(Instruction { op_code: Stx, addr_mode: ZpY, size: 2 }), // 0x96
    None, // 0x97
    Some(Instruction { op_code: Tya, addr_mode: Implicit, size: 1 }), // 0x98
    Some(Instruction { op_code: Sta, addr_mode: AbsoluteY, size: 3 }), // 0x99
    Some(Instruction { op_code: Txs, addr_mode: Implicit, size: 1 }), // 0x9a
    None, // 0x9b
    None, // 0x9c
    Some(Instruction { op_code: Sta, addr_mode: AbsoluteX, size: 3 }), // 0x9d
    None, // 0x9e
    None, // 0x9f
    Some(Instruction { op_code: Ldy, addr_mode: Immediate, size: 2 }), // 0xa0
    Some(Instruction { op_code: Lda, addr_mode: IndirectY, size: 2 }), // 0xa1
    Some(Instruction { op_code: Ldx, addr_mode: Immediate, size: 2 }), // 0xa2
    None, // 0xa3
    Some(Instruction { op_code: Ldy, addr_mode: Zp, size: 2 }), // 0xa4
    Some(Instruction { op_code: Lda, addr_mode: Zp, size: 2 }), // 0xa5
    Some(Instruction { op_code: Ldx, addr_mode: Zp, size: 2 }), // 0xa6
    None, // 0xa7
    Some(Instruction { op_code: Tay, addr_mode: Implicit, size: 1 }), // 0xa8
    Some(Instruction { op_code: Lda, addr_mode: Immediate, size: 2 }), // 0xa9
    Some(Instruction { op_code: Tax, addr_mode: Implicit, size: 1 }), // 0xaa
    None, // 0xab
    Some(Instruction { op_code: Ldy, addr_mode: Absolute, size: 3 }), // 0xac
    Some(Instruction { op_code: Lda, addr_mode: Absolute, size: 3 }), // 0xad
    Some(Instruction { op_code: Ldx, addr_mode: Absolute, size: 3 }), // 0xae
    None, // 0xaf
    Some(Instruction { op_code: Bcs, addr_mode: Relative, size: 2 }), // 0xb0
    Some(Instruction { op_code: Lda, addr_mode: IndirectX, size: 2 }), // 0xb1
    Some(Instruction { op_code: Lda, addr_mode: Zp, size: 2 }), // 0xb2
    None, // 0xb3
    Some(Instruction { op_code: Ldy, addr_mode: ZpX, size: 2 }), // 0xb4
    Some(Instruction { op_code: Lda, addr_mode: ZpX, size: 2 }), // 0xb5
    Some(Instruction { op_code: Ldx, addr_mode: ZpY, size: 2 }), // 0xb6
    None, // 0xb7
    Some(Instruction { op_code: Clv, addr_mode: Implicit, size: 1 }), // 0xb8
    Some(Instruction { op_code: Lda, addr_mode: AbsoluteY, size: 3 }), // 0xb9
    Some(Instruction { op_code: Tsx, addr_mode: Implicit, size: 1 }), // 0xba
    None, // 0xbb
    Some(Instruction { op_code: Ldy, addr_mode: AbsoluteX, size: 3 }), // 0xbc
    Some(Instruction { op_code: Lda, addr_mode: AbsoluteX, size: 3 }), // 0xbd
    Some(Instruction { op_code: Ldx, addr_mode: AbsoluteY, size: 3 }), // 0xbe
    None, // 0xbf
    Some(Instruction { op_code: Cpy, addr_mode: Immediate, size: 2 }), // 0xc0
    Some(Instruction { op_code: Cmp, addr_mode: IndirectY, size: 2 }), // 0xc1
    None, // 0xc2
    None, // 0xc3
    Some(Instruction { op_code: Cpy, addr_mode: Zp, size: 2 }), // 0xc4
    Some(Instruction { op_code: Cmp, addr_mode: Zp, size: 2 }), // 0xc5
    Some(Instruction { op_code: Dec, addr_mode: Zp, size: 2 }), // 0xc6
    None, // 0xc7
    Some(Instruction { op_code: Iny, addr_mode: Implicit, size: 1 }), // 0xc8
    Some(Instruction { op_code: Cmp, addr_mode: Immediate, size: 2 }), // 0xc9
    Some(Instruction { op_code: Dex, addr_mode: Implicit, size: 1 }), // 0xca
    None, // 0xcb
    Some(Instruction { op_code: Cpy, addr_mode: Absolute, size: 3 }), // 0xcc
    Some(Instruction { op_code: Cmp, addr_mode: Absolute, size: 3 }), // 0xcd
    Some(Instruction { op_code: Dec, addr_mode: Absolute, size: 3 }), // 0xce
    None, // 0xcf
    Some(Instruction { op_code: Bne, addr_mode: Relative, size: 2 }), // 0xd0
    Some(Instruction { op_code: Cmp, addr_mode: IndirectX, size: 2 }), // 0xd1
    Some(Instruction { op_code: Cmp, addr_mode: Zp, size: 2 }), // 0xd2
    None, // 0xd3
    None, // 0xd4
    Some(Instruction { op_code: Cmp, addr_mode: ZpX, size: 2 }), // 0xd5
    Some(Instruction { op_code: Dec, addr_mode: ZpX, size: 2 }), // 0xd6
    None, // 0xd7
    Some(Instruction { op_code: Cld, addr_mode: Implicit, size: 1 }), // 0xd8
    Some(Instruction { op_code: Cmp, addr_mode: AbsoluteY, size: 3 }), // 0xd9
    None, // 0xda
    None, // 0xdb
    None, // 0xdc
    Some(Instruction { op_code: Cmp, addr_mode: AbsoluteX, size: 3 }), // 0xdd
    Some(Instruction { op_code: Dec, addr_mode: AbsoluteX, size: 3 }), // 0xde
    None, // 0xdf
    Some(Instruction { op_code: Cpx, addr_mode: Immediate, size: 2 }), // 0xe0
    Some(Instruction { op_code: Sbc, addr_mode: IndirectY, size: 2 }), // 0xe1
    None, // 0xe2
    None, // 0xe3
    Some(Instruction { op_code: Cpx, addr_mode: Zp, size: 2 }), // 0xe4
    Some(Instruction { op_code: Sbc, addr_mode: Zp, size: 2 }), // 0xe5
    Some(Instruction { op_code: Inc, addr_mode: Zp, size: 2 }), // 0xe6
    None, // 0xe7
    Some(Instruction { op_code: Inx, addr_mode: Implicit, size: 1 }), // 0xe8
    Some(Instruction { op_code: Sbc, addr_mode: Immediate, size: 2 }), // 0xe9
    Some(Instruction { op_code: Nop, addr_mode: Implicit, size: 1 }), // 0xea
    None, // 0xeb
    Some(Instruction { op_code: Cpx, addr_mode: Absolute, size: 3 }), // 0xec
    Some(Instruction { op_code: Sbc, addr_mode: Absolute, size: 3 }), // 0xed
    Some(Instruction { op_code: Inc, addr_mode: Absolute, size: 3 }), // 0xee
    None, // 0xef
    Some(Instruction { op_code: Beq, addr_mode: Relative, size: 2 }), // 0xf0
    Some(Instruction { op_code: Sbc, addr_mode: IndirectX, size: 2 }), // 0xf1
    Some(Instruction { op_code: Sbc, addr_mode: Zp, size: 2 }), // 0xf2
    None, // 0xf3
    None, // 0xf4
    Some(Instruction { op_code: Sbc, addr_mode: ZpX, size: 2 }), // 0xf5
    Some(Instruction { op_code: Inc, addr_mode: ZpX, size: 2 }), // 0xf6
    None, // 0xf7
    Some(Instruction { op_code: Sed, addr_mode: Implicit, size: 1 }), // 0xf8
    Some(Instruction { op_code: Sbc, addr_mode: AbsoluteY, size: 3 }), // 0xf9
    None, // 0xfa
    None, // 0xfb
    None, // 0xfc
    Some(Instruction { op_code: Sbc, addr_mode: AbsoluteX, size: 3 }), // 0xfd
    Some(Instruction { op_code: Inc, addr_mode: AbsoluteX, size: 3 }), // 0xfe
    None, // 0xff
];