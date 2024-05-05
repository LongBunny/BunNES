use bit::BitIndex;
use crate::opcodes::OP_CODES;
use crate::rom::Rom;

const RAM_CAP: usize = 2 * 1024;

pub struct Emulator {
    cpu: Cpu,
    rom: Rom,
    ram: [u8; RAM_CAP],
}

impl Emulator {
    pub fn new(rom: Rom) -> Emulator {
        Emulator {
            rom,
            ram: [0u8; RAM_CAP],
            cpu: Cpu::new(),
        }
    }

    pub fn run(mut self) {
        println!("prg size: {}", self.rom.prg().len());

        let reset: u16 = self.map_addr(0xFFFC) as u16 | (self.map_addr(0xFFFD) as u16) << 8;
        println!("reset vector: {:#04X}", reset);
        self.cpu.pc = reset;
        self.cpu.step();
        // loop {
        //     self.cpu.step(&self);
        //     panic!("eh");
        // }
    }

    fn parse_rom(&mut self) {

    }

    fn map_addr(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x07FF => self.ram[addr as usize],
            0x0800..=0x1FFF => unimplemented!("RAM mirrors"),
            0x2000..=0x2007 => unimplemented!("PPU"),
            0x2008..=0x3FFF => unimplemented!("PPU mirrors"),
            0x4000..=0x4017 => unimplemented!("APU"),
            0x4018..=0x401F => panic!("APU and IO. Should be disabled"),
            0x4020..=0xFFFF => {
                match addr {
                    0x6000..=0x7FFF => panic!("Cartridge: Family Basic only"),
                    0x8000..=0xFFFF => {

                        match self.rom.rom_len() {
                            0x4000 => {
                                // 16k rom
                                // subtract rom location and mirror upper 0x4000 bytes
                                let mut addr = addr as usize -0x8000;
                                if addr > 0x4000 {
                                    addr -= 0x4000;
                                }
                                self.rom.prg()[addr]
                            },
                            0x8000 => {
                                // 32k rom
                                // subtract rom location
                                let addr = addr as usize - 0x8000;
                                self.rom.prg()[addr]
                            },
                            _ => panic!("Cartridge: Rom: no matching address")
                        }
                    }
                    _ => panic!("Cartridge: no matching address")
                }
            },
            _ => panic!("no matching address")
        }
    }
}

struct Bus {

}

impl Bus {
    fn read(addr: u16) -> u8 {
        0
    }

    fn write(addr: u16) {

    }
}

struct ProcessorStatus {
    /// [0] carry
    /// [1] zero
    /// [2] irqb disable
    /// [3] decimal
    /// [4] break
    /// [5] unavailable
    /// [6] overflow
    /// [7] negative
    reg: u8,
}

impl ProcessorStatus {
    fn new() -> ProcessorStatus {
        ProcessorStatus {
            reg: 0
        }
    }

    fn carry(&self) -> bool {
        self.reg.bit(0)
    }

    fn set_carry(&mut self, value: bool) {
        self.reg.set_bit(0, value);
    }

    fn zero(&self) -> bool {
        self.reg.bit(1)
    }

    fn set_zero(&mut self, value: bool) {
        self.reg.set_bit(1, value);
    }

    fn irqb(&self) -> bool {
        self.reg.bit(2)
    }

    fn set_irqb(&mut self, value: bool) {
        self.reg.set_bit(2, value);
    }

    fn decimal(&self) -> bool {
        self.reg.bit(3)
    }

    fn set_decimal(&mut self, value: bool) {
        self.reg.set_bit(3, value);
    }

    fn brk(&self) -> bool {
        self.reg.bit(4)
    }

    fn set_brk(&mut self, value: bool) {
        self.reg.set_bit(4, value);
    }

    fn overflow(&self) -> bool {
        self.reg.bit(6)
    }

    fn set_overflow(&mut self, value: bool) {
        self.reg.set_bit(6, value);
    }

    fn negative(&self) -> bool {
        self.reg.bit(7)
    }

    fn set_negative(&mut self, value: bool) {
        self.reg.set_bit(7, value);
    }

}

#[allow(unused_variables, dead_code)]
struct Cpu {
    /// program counter
    pc: u16,
    /// stack pointer
    sp: u8,
    /// accumulator
    acc: u8,
    /// index register x
    x: u8,
    /// index register y
    y: u8,
    /// processor status
    ps: ProcessorStatus,
}


#[derive(Debug, Copy, Clone)]
pub(crate) enum AddrMode {
    Abs,
    Implicit
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum OpCode {
    Sei(AddrMode)
}

type Cycle = u8;

impl Cpu {
    fn new() -> Cpu {
        Cpu {
            pc: 0,
            sp: 0,
            acc: 0,
            x: 0,
            y: 0,
            ps: ProcessorStatus::new(),
        }
    }

    fn step(&mut self) {
        let inst_num = map_addr(self.pc);
        let inst = OP_CODES[inst_num as usize];
        let cycles = match inst {
            Some(OpCode::Sei(addrMode)) => {
                self.sei(addrMode)
            }
            _ => panic!("unknown instruction: {inst_num:#04X} {inst:?}")
        };
    }

    /// set interrupt disable
    fn sei(&mut self, addr_mode: AddrMode) -> Cycle {
        match addr_mode {
            AddrMode::Implicit => {
                self.ps.set_irqb(false);
                2
            }
            _ => panic!("unimplemented: SEI {addr_mode:?}")
        }

    }
}

