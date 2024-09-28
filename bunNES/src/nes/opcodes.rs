use std::fmt::{Display, Formatter};
use crate::nes::opcodes::AddrMode::*;
use crate::nes::opcodes::OpCode::*;

#[derive(Debug, Copy, Clone)]
pub enum AddrMode {
    Abs,
    Implicit,
    Immediate,
    Absolute,
    AbsoluteX,
    Relative,
}

#[derive(Debug, Copy, Clone)]
pub enum OpCode {
    Sei,
    Cld,
    Txs,
    Cpx(AddrMode),
    Bne,
    Bpl(AddrMode),
    Inx,
    Dex,
    Dey,
    Ldx(AddrMode),
    Ldy(AddrMode),
    Lda(AddrMode),
    Sta(AddrMode),
    Stx(AddrMode),
}

impl OpCode {
    fn to_string(&self) -> &str {
        match self {
            Sei => "SEI",
            Cld => "CLD",
            Txs => "TXS",
            Cpx(_) => "CPX",
            Bne => "BNE",
            Bpl(_) => "BPL",
            Inx => "INX",
            Dex => "DEX",
            Dey => "DEY",
            Ldx(_) => "LDX",
            Ldy(_) => "LDY",
            Lda(_) => "LDA",
            Sta(_) => "STA",
            Stx(_) => "STX",
        }
    }
}

impl Display for OpCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

pub static OP_CODES: [(Option<OpCode>, u8); 256] = [
    (None, 1), // 0x00
    (None, 1), // 0x01
    (None, 1), // 0x02
    (None, 1), // 0x03
    (None, 1), // 0x04
    (None, 1), // 0x05
    (None, 1), // 0x06
    (None, 1), // 0x07
    (None, 1), // 0x08
    (None, 1), // 0x09
    (None, 1), // 0x0a
    (None, 1), // 0x0b
    (None, 1), // 0x0c
    (None, 1), // 0x0d
    (None, 1), // 0x0e
    (None, 1), // 0x0f
    (Some(Bpl(Relative)), 2), // 0x10
    (None, 1), // 0x11
    (None, 1), // 0x12
    (None, 1), // 0x13
    (None, 1), // 0x14
    (None, 1), // 0x15
    (None, 1), // 0x16
    (None, 1), // 0x17
    (None, 1), // 0x18
    (None, 1), // 0x19
    (None, 1), // 0x1a
    (None, 1), // 0x1b
    (None, 1), // 0x1c
    (None, 1), // 0x1d
    (None, 1), // 0x1e
    (None, 1), // 0x1f
    (None, 1), // 0x20
    (None, 1), // 0x21
    (None, 1), // 0x22
    (None, 1), // 0x23
    (None, 1), // 0x24
    (None, 1), // 0x25
    (None, 1), // 0x26
    (None, 1), // 0x27
    (None, 1), // 0x28
    (None, 1), // 0x29
    (None, 1), // 0x2a
    (None, 1), // 0x2b
    (None, 1), // 0x2c
    (None, 1), // 0x2d
    (None, 1), // 0x2e
    (None, 1), // 0x2f
    (None, 1), // 0x30
    (None, 1), // 0x31
    (None, 1), // 0x32
    (None, 1), // 0x33
    (None, 1), // 0x34
    (None, 1), // 0x35
    (None, 1), // 0x36
    (None, 1), // 0x37
    (None, 1), // 0x38
    (None, 1), // 0x39
    (None, 1), // 0x3a
    (None, 1), // 0x3b
    (None, 1), // 0x3c
    (None, 1), // 0x3d
    (None, 1), // 0x3e
    (None, 1), // 0x3f
    (None, 1), // 0x40
    (None, 1), // 0x41
    (None, 1), // 0x42
    (None, 1), // 0x43
    (None, 1), // 0x44
    (None, 1), // 0x45
    (None, 1), // 0x46
    (None, 1), // 0x47
    (None, 1), // 0x48
    (None, 1), // 0x49
    (None, 1), // 0x4a
    (None, 1), // 0x4b
    (None, 1), // 0x4c
    (None, 1), // 0x4d
    (None, 1), // 0x4e
    (None, 1), // 0x4f
    (None, 1), // 0x50
    (None, 1), // 0x51
    (None, 1), // 0x52
    (None, 1), // 0x53
    (None, 1), // 0x54
    (None, 1), // 0x55
    (None, 1), // 0x56
    (None, 1), // 0x57
    (None, 1), // 0x58
    (None, 1), // 0x59
    (None, 1), // 0x5a
    (None, 1), // 0x5b
    (None, 1), // 0x5c
    (None, 1), // 0x5d
    (None, 1), // 0x5e
    (None, 1), // 0x5f
    (None, 1), // 0x60
    (None, 1), // 0x61
    (None, 1), // 0x62
    (None, 1), // 0x63
    (None, 1), // 0x64
    (None, 1), // 0x65
    (None, 1), // 0x66
    (None, 1), // 0x67
    (None, 1), // 0x68
    (None, 1), // 0x69
    (None, 1), // 0x6a
    (None, 1), // 0x6b
    (None, 1), // 0x6c
    (None, 1), // 0x6d
    (None, 1), // 0x6e
    (None, 1), // 0x6f
    (None, 1), // 0x70
    (None, 1), // 0x71
    (None, 1), // 0x72
    (None, 1), // 0x73
    (None, 1), // 0x74
    (None, 1), // 0x75
    (None, 1), // 0x76
    (None, 1), // 0x77
    (Some(Sei), 1), // 0x78
    (None, 1), // 0x79
    (None, 1), // 0x7a
    (None, 1), // 0x7b
    (None, 1), // 0x7c
    (None, 1), // 0x7d
    (None, 1), // 0x7e
    (None, 1), // 0x7f
    (None, 1), // 0x80
    (None, 1), // 0x81
    (None, 1), // 0x82
    (None, 1), // 0x83
    (None, 1), // 0x84
    (None, 1), // 0x85
    (None, 1), // 0x86
    (None, 1), // 0x87
    (Some(Dey), 1), // 0x88
    (None, 1), // 0x89
    (None, 1), // 0x8a
    (None, 1), // 0x8b
    (None, 1), // 0x8c
    (Some(Sta(Absolute)), 3), // 0x8d
    (Some(Stx(Absolute)), 3), // 0x8e
    (None, 1), // 0x8f
    (None, 1), // 0x90
    (None, 1), // 0x91
    (None, 1), // 0x92
    (None, 1), // 0x93
    (None, 1), // 0x94
    (None, 1), // 0x95
    (None, 1), // 0x96
    (None, 1), // 0x97
    (None, 1), // 0x98
    (None, 1), // 0x99
    (Some(Txs), 1), // 0x9a
    (None, 1), // 0x9b
    (None, 1), // 0x9c
    (None, 1), // 0x9d
    (None, 1), // 0x9e
    (None, 1), // 0x9f
    (Some(Ldy(Immediate)), 2), // 0xa0
    (None, 1), // 0xa1
    (Some(Ldx(Immediate)), 2), // 0xa2
    (None, 1), // 0xa3
    (None, 1), // 0xa4
    (None, 1), // 0xa5
    (None, 1), // 0xa6
    (None, 1), // 0xa7
    (None, 1), // 0xa8
    (Some(Lda(Immediate)), 2), // 0xa9
    (None, 1), // 0xaa
    (None, 1), // 0xab
    (None, 1), // 0xac
    (Some(Lda(Absolute)), 3), // 0xad
    (None, 1), // 0xae
    (None, 1), // 0xaf
    (None, 1), // 0xb0
    (None, 1), // 0xb1
    (None, 1), // 0xb2
    (None, 1), // 0xb3
    (None, 1), // 0xb4
    (None, 1), // 0xb5
    (None, 1), // 0xb6
    (None, 1), // 0xb7
    (None, 1), // 0xb8
    (None, 1), // 0xb9
    (None, 1), // 0xba
    (None, 1), // 0xbb
    (None, 1), // 0xbc
    (Some(Lda(AbsoluteX)), 3), // 0xbd
    (None, 1), // 0xbe
    (None, 1), // 0xbf
    (None, 1), // 0xc0
    (None, 1), // 0xc1
    (None, 1), // 0xc2
    (None, 1), // 0xc3
    (None, 1), // 0xc4
    (None, 1), // 0xc5
    (None, 1), // 0xc6
    (None, 1), // 0xc7
    (None, 1), // 0xc8
    (None, 1), // 0xc9
    (Some(Dex), 1), // 0xca
    (None, 1), // 0xcb
    (None, 1), // 0xcc
    (None, 1), // 0xcd
    (None, 1), // 0xce
    (None, 1), // 0xcf
    (Some(Bne), 1), // 0xd0
    (None, 1), // 0xd1
    (None, 1), // 0xd2
    (None, 1), // 0xd3
    (None, 1), // 0xd4
    (None, 1), // 0xd5
    (None, 1), // 0xd6
    (None, 1), // 0xd7
    (Some(Cld), 1), // 0xd8
    (None, 1), // 0xd9
    (None, 1), // 0xda
    (None, 1), // 0xdb
    (None, 1), // 0xdc
    (None, 1), // 0xdd
    (None, 1), // 0xde
    (None, 1), // 0xdf
    (Some(Cpx(Immediate)), 2), // 0xe0
    (None, 1), // 0xe1
    (None, 1), // 0xe2
    (None, 1), // 0xe3
    (None, 1), // 0xe4
    (None, 1), // 0xe5
    (None, 1), // 0xe6
    (None, 1), // 0xe7
    (Some(Inx), 1), // 0xe8
    (None, 1), // 0xe9
    (None, 1), // 0xea
    (None, 1), // 0xeb
    (None, 1), // 0xec
    (None, 1), // 0xed
    (None, 1), // 0xee
    (None, 1), // 0xef
    (None, 1), // 0xf0
    (None, 1), // 0xf1
    (None, 1), // 0xf2
    (None, 1), // 0xf3
    (None, 1), // 0xf4
    (None, 1), // 0xf5
    (None, 1), // 0xf6
    (None, 1), // 0xf7
    (None, 1), // 0xf8
    (None, 1), // 0xf9
    (None, 1), // 0xfa
    (None, 1), // 0xfb
    (None, 1), // 0xfc
    (None, 1), // 0xfd
    (None, 1), // 0xfe
    (None, 1), // 0xff
];