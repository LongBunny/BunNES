use std::fmt::{Display, Formatter};
use std::str;

// https://www.nesdev.org/wiki/INES

#[allow(unused_variables)]
#[derive(Debug)]
pub struct Cartridge {
    header: RomHeader,
    trainer: Vec<u8>,
    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>,

    // play choice
    inst_rom: Vec<u8>,
    p_rom: Vec<u8>,
}

impl Cartridge {
    pub(crate) fn new(bytes: Vec<u8>) -> Cartridge {
        let mut offset: usize = 0;

        let header = RomHeader::parse(&bytes[0..16]);
        offset += 16;

        let trainer = if header.trainer() {
            offset += 512;
            bytes[offset..512 + offset].to_vec()
        } else { vec!() };

        println!("prg_rom start: {}", offset);
        let prg_rom = bytes[offset..offset + header.prg_len()].to_vec();
        offset += header.prg_len();

        let chr_rom = bytes[offset..offset + header.chr_len()].to_vec();

        Cartridge {
            header,
            trainer,
            prg_rom,
            chr_rom,

            // play choice
            inst_rom: vec!(),
            p_rom: vec!()
        }
    }

    pub fn prg(&self) -> &Vec<u8> {
        &self.prg_rom
    }

    pub fn rom_len(&self) -> usize {
        self.prg_rom.len()
    }
}

impl Display for Cartridge {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.header)?;
        write!(f, "Mapper: {}", (self.header.flags6 & 0b1111_0000) >> 4 | self.header.flags7 & 0b1111_0000)?;
        Ok(())
    }
}

#[derive(Debug)]
struct RomHeader {
    magic: [u8; 4],
    prg_rom: u8,
    chr_rom: u8,
    flags6: u8,
    flags7: u8,
    flags8: u8,
    flags9: u8,
    flags10: u8,
}

impl RomHeader {
    fn parse(values: &[u8]) -> RomHeader {
        RomHeader {
            magic: values[0..4].try_into().unwrap(),
            prg_rom: values[4],
            chr_rom: values[5],
            flags6: values[6],
            flags7: values[7],
            flags8: values[8],
            flags9: values[9],
            flags10: values[10],
        }
    }

    fn trainer(&self) -> bool {
        self.flags6 & 0b0000_0100 >> 2 == 1
    }

    fn prg_len(&self) -> usize {
        self.prg_rom as usize * 16384
    }

    fn chr_len(&self) -> usize {
        self.chr_rom as usize * 8192
    }
}

impl Display for RomHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Header:\n")?;
        write!(f, "  Magic: {}\n", str::from_utf8(&self.magic).unwrap())?;
        write!(f, "  PRG ROM size: {}(KB)\n", self.prg_rom as u32 * 16)?;
        write!(f, "  CHR ROM size: {}(KB)\n", self.chr_rom as u32 * 8)?;

        write!(f, "  Flags 6\n")?;
        write!(f, "    Mirroring: {}\n", if (self.flags6 & 0b0000_0001) >> 0 == 0 { "horizontal" } else { "vertical" })?;
        write!(f, "    Battery backed: {}\n", if (self.flags6 & 0b0000_0010) >> 1 == 1 { "true" } else { "false" })?;
        write!(f, "    Trainer: {}\n", if (self.flags6 & 0b0000_0100) >> 2 == 1 { "true" } else { "false" })?;
        write!(f, "    Ignore mirror: {}\n", if (self.flags6 & 0b0000_1000) >> 3 == 1 { "true" } else { "false" })?;
        write!(f, "    Mapper (lower): {}\n", (self.flags6 & 0b1111_0000) >> 4)?;

        write!(f, "  Flags 7\n")?;
        write!(f, "    VS Unisystem : {}\n", (self.flags7 & 0b0000_0001) >> 0)?;
        write!(f, "    PlayChoice-10 : {}\n", (self.flags7 & 0b0000_0010) >> 1)?;
        write!(f, "    NES 2.0 Format : {}\n", if (self.flags7 & 0b0000_1100) >> 2 == 2 { "true" } else { "false" })?;
        write!(f, "    Mapper (higher): {}\n", (self.flags6 & 0b1111_0000) >> 4)?;

        write!(f, "  Flags 8")?;
        write!(f, "    PRG RAM size: {}KB\n", if self.flags8 == 0 { 8 } else { &self.flags8 * 8 })?;

        write!(f, "  Flags 9\n")?;
        write!(f, "    TV System : {}\n", if (self.flags9 & 0b0000_0001) >> 0 == 0 { "NTSC" } else { "PAL" })?;

        write!(f, "  Flags 10\n")?;
        write!(f, "    TV System : {}\n", match (self.flags10 & 0b0000_0011) >> 0 {
            0 => "NTSC",
            2 => "PAL",
            _ => "Dual"
        })?;
        write!(f, "    PRG RAM : {}\n", if (self.flags10 & 0b0001_0000) >> 4 == 0 { "present" } else { "not present" })?;
        write!(f, "    Bus conflicts : {}\n", if (self.flags10 & 0b0010_0000) >> 5 == 0 { "false" } else { "true" })?;

        Ok(())
    }
}