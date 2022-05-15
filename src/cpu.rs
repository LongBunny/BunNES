pub mod instructions;
mod opcode;

// pub use crate::cpu::instructions::OpCodes;

use crate::cpu::instructions::{AddressingMode, Instruction};

#[allow(unused_variables)]
#[allow(dead_code)]
pub struct CPU {
    address: u16,
    data: u8,
    registers: Registers,

    rom: [u8; 32 * 1024],
    ram: [u8; 32 * 1024],
}

struct Registers {
    program_counter: u16,
    stack_pointer: u8,
    accumulator: u8,
    index_register_x: u8,
    index_register_y: u8,
    processor_status: u8,
}

#[allow(unused_variables)]
#[allow(dead_code)]
enum ProcessorStatus {
    Carry,
    Zero,
    InterruptDisable,
    Decimal,
    Break,
    Overflow,
    Negative,
}

impl CPU {
    pub fn new(rom: &[u8; 32 * 1024]) -> CPU {
        let ram = [0u8; 32 * 1024];
        CPU {
            address: 0x0000,
            data: 0x00,
            registers: Registers {
                program_counter: 0x00,
                stack_pointer: 0x00,
                accumulator: 0x00,
                index_register_x: 0x00,
                index_register_y: 0x00,
                processor_status: 0x00,
            },
            rom: *rom,
            ram: ram,
        }
    }

    pub fn run(&mut self) {
        println!("CPU run");
        let mut running = true;

        while running {
            running = !self.step();
        }

        println!("CPU stop");
    }

    pub fn step(&mut self) -> bool {
        println!("{}", format!("ACC:    {}    {}", format!("{:#04X}", self.registers.accumulator), format!("{:3}", self.registers.accumulator)));
        match self.fetch() {
            Some(code) => opcode::process(self, code),
            None => {
                eprintln!("Coulnd't fetch instruction");
                true
            },
        }
    }

    fn fetch(&mut self) -> Option<(Instruction, AddressingMode)> {
        let instruction = self.rom[self.registers.program_counter as usize];
        self.registers.program_counter = u16::wrapping_add(self.registers.program_counter, 1);
        Instruction::get(instruction)
    }
}

impl std::fmt::Display for CPU {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut format_string = String::from("");
        format_string.push_str(
            format!(
                "\n    ADDR:  {}  {}",
                format!("{:#06X}", self.address),
                format!("{:5}", self.address)
            )
            .as_str(),
        );
        format_string.push_str(
            format!(
                "\n    DATA:    {}    {}",
                format!("{:#04X}", self.data),
                format!("{:3}", self.data)
            )
            .as_str(),
        );
        format_string.push_str(
            format!(
                "\n      PC:    {}    {}",
                format!("{:#04X}", self.registers.program_counter),
                format!("{:3}", self.registers.program_counter)
            )
            .as_str(),
        );
        format_string.push_str(
            format!(
                "\n      SP:    {}    {}",
                format!("{:#04X}", self.registers.stack_pointer),
                format!("{:3}", self.registers.stack_pointer)
            )
            .as_str(),
        );
        format_string.push_str(
            format!(
                "\n     ACC:    {}    {}",
                format!("{:#04X}", self.registers.accumulator),
                format!("{:3}", self.registers.accumulator)
            )
            .as_str(),
        );
        format_string.push_str(
            format!(
                "\n     IRX:    {}    {}",
                format!("{:#04X}", self.registers.index_register_x),
                format!("{:3}", self.registers.index_register_x)
            )
            .as_str(),
        );
        format_string.push_str(
            format!(
                "\n     IRY:    {}    {}",
                format!("{:#04X}", self.registers.index_register_y),
                format!("{:3}", self.registers.index_register_y)
            )
            .as_str(),
        );
        format_string.push_str(
            format!(
                "\n      PS:    {}\n               _NOBDIZC",
                format!("{:#010b}", self.registers.processor_status))
            .as_str(),
        );

        write!(f, "Registers: {}", format_string)
    }
}
