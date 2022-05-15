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

    address_bus: [u8; 64 * 1024],
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
#[derive(Clone, Copy, Debug)]
enum ProcessorStatus {
    Carry,
    Zero,
    InterruptDisable,
    Decimal,
    Break,
    Overflow,
    Negative,
}

impl ProcessorStatus {
    // TODO: refactor
    pub fn is_set(&self, cpu: &CPU) -> bool {
        cpu.registers.processor_status & (1 << (*self as i8)) == 1
    }
}

impl CPU {
    pub fn new(rom: &[u8; 64 * 1024]) -> CPU {
        // TODO: refactor
        let mut addressable_memory: [u8; 64 * 1024] = [0x00; 64 * 1024];

        // load rom into addressable memory
        for (i, o) in rom.iter().enumerate() {
            addressable_memory[i] = *o;
        }

        CPU {
            address: 0x0000,
            data: 0x00,
            registers: Registers {
                program_counter: 0xFFFC,
                stack_pointer: 0x80,
                accumulator: 0x00,
                index_register_x: 0x00,
                index_register_y: 0x00,
                processor_status: 0x00,
            },
            address_bus: addressable_memory,
        }
    }

    pub fn run(&mut self) {
        println!("CPU run");
        let mut running = true;

        let mut reset_vector = 0xFFFC;
        // reset vector
        reset_vector = reset_vector
            & (self.address_bus[(self.registers.program_counter + 0) as usize] as u16);
        reset_vector = reset_vector
            & (self.address_bus[(self.registers.program_counter + 1) as usize] as u16) << 8;
        self.registers.program_counter = reset_vector;
        println!("reset vector: {}", reset_vector);

        while running {
            running = !self.step();
        }

        println!("CPU stop");
    }

    pub fn step(&mut self) -> bool {
        println!("------------------------------------");
        println!("{}", self);
        print!("STACK: [");
        for i in 0x0100..0x01ff {
            print!("{:#04X}, ", self.address_bus[i]);
        }
        print!("]\n");

        match self.fetch() {
            Some(code) => opcode::process(self, code),
            None => {
                eprintln!("Coulnd't fetch instruction");
                true
            }
        }
    }

    fn fetch(&mut self) -> Option<(Instruction, AddressingMode)> {
        println!("{:#04X}", (self.registers.program_counter + 32 * 1024) as usize);
        let instruction =
            self.address_bus[(self.registers.program_counter + 32 * 1024) as usize];
        self.registers.program_counter = u16::wrapping_add(self.registers.program_counter, 1);
        Instruction::get(instruction)
    }

    
    fn set_processor_status_bit(&mut self, ps: ProcessorStatus, value: bool) {
        let mask: u8 = 1 << (ps as u8);
        self.registers.processor_status =
        (self.registers.processor_status & !mask) | ((value as u8) << (ps as u8));
    }

    pub fn set_flag_carry(&mut self, overflow: bool) {
        self.set_processor_status_bit(ProcessorStatus::Carry, overflow);
    }

    pub fn set_flag_zero(&mut self) {
        self.set_processor_status_bit(ProcessorStatus::Zero, self.registers.accumulator == 0);
    }

    pub fn set_flag_overflow(&mut self, sign_before: u8) {
        self.set_processor_status_bit(
            ProcessorStatus::Overflow,
            self.registers.accumulator & 0b1000_0000 != sign_before,
        );
    }

    pub fn set_flag_negative(&mut self) {
        self.set_processor_status_bit(
            ProcessorStatus::Negative,
            self.registers.accumulator & 0b1000_0000 == 1,
        );
    }


    pub fn get_stack_location(&mut self) -> usize {
        usize::wrapping_add(0x0100, (self.registers.stack_pointer) as usize)
    }

    pub fn get_ram_value_zp(&mut self, addr: u8) -> u8 {
        self.address_bus[addr as usize]
    }

    pub fn get_ram_value(&mut self, addr: u16) -> u8 {
        self.address_bus[addr as usize]
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
                format!("{:#010b}", self.registers.processor_status)
            )
            .as_str(),
        );

        write!(f, "Registers: {}", format_string)
    }
}
