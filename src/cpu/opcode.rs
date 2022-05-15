use crate::cpu::ProcessorStatus;

use super::{
    instructions::{AddressingMode, Instruction},
    CPU,
};

// instruction return info
pub struct IRI {
    cycles_taken: u16,
    stop: bool,
}

impl IRI {
    fn new(cycles_taken: u16) -> IRI {
        IRI {
            cycles_taken,
            stop: false,
        }
    }

    fn new_stop(cycles_taken: u16) -> IRI {
        IRI {
            cycles_taken,
            stop: true,
        }
    }
}


pub fn process(cpu: &mut CPU, ins: (Instruction, AddressingMode)) -> bool {
    print!("Executing: {:?} {:?}", ins.0, ins.1);
    let iri: IRI = match ins.0 {
        Instruction::ADC => cpu_functions::adc(cpu, ins.1),
        Instruction::BCS => cpu_functions::bcs(cpu, ins.1),
        Instruction::BRK => cpu_functions::brk(cpu, ins.1),
        Instruction::JMP => cpu_functions::jmp(cpu, ins.1),
        Instruction::LDA => cpu_functions::lda(cpu, ins.1),
        Instruction::NOP => cpu_functions::nop(cpu, ins.1),
        Instruction::PHA => cpu_functions::pha(cpu, ins.1),
        Instruction::PLA => cpu_functions::pla(cpu, ins.1),
        Instruction::STA => cpu_functions::sta(cpu, ins.1),
        not_implemented => {
            panic!("Not implemented instruction {:?}", not_implemented);
        }
    };
    print!("\n");
    iri.stop
}

mod cpu_functions {

    use crate::cpu::{instructions::AddressingMode, ProcessorStatus, CPU};
    use super::IRI;
    
    fn get_next_byte(cpu: &mut CPU) -> u8 {
        let ret = cpu.address_bus[(cpu.registers.program_counter + 32 * 1024) as usize];
        cpu.registers.program_counter = u16::wrapping_add(cpu.registers.program_counter, 1);
        ret
    }

    fn get_next_word(cpu: &mut CPU) -> u16 {
        let ret: [u8; 2] = [
            cpu.address_bus[(cpu.registers.program_counter + 32 * 1024 + 0) as usize],
            cpu.address_bus[(cpu.registers.program_counter + 32 * 1024 + 1) as usize],
        ];
        cpu.registers.program_counter = u16::wrapping_add(cpu.registers.program_counter, 2);
        ((ret[1] as u16) << 8) ^ (ret[0] as u16)
    }

    fn overflow_add(cpu: &mut CPU, val: u8) -> bool {
        let (new_val, overflow) = u8::overflowing_add(cpu.registers.accumulator, val);
        cpu.registers.accumulator = new_val;
        overflow
    }

    



    pub fn adc(cpu: &mut CPU, am: AddressingMode) -> IRI {
        let val: u8;
        let iri = match am {
            AddressingMode::Immediate => {
                val = get_next_byte(cpu);
                print!(" - value: {}", val);
                IRI::new(2)
            },
            AddressingMode::Zeropage => {
                let addr = get_next_byte(cpu);
                val = cpu.get_ram_value_zp(addr);
                print!(" - value: {}", val);
                IRI::new(3)
            },
            AddressingMode::ZeropageX => {
                let addr = get_next_byte(cpu);
                val = cpu.get_ram_value_zp(addr + cpu.registers.index_register_x);
                print!(" - value: {}", val);
                IRI::new(4)
            },
            _ => todo!("ADC: unrecognized AddressingMode: {:?}", am),
        };
        let sign_before = cpu.registers.accumulator & 0b1000_0000;
        let overflow = overflow_add(cpu, val);

        cpu.set_flag_carry(overflow);
        cpu.set_flag_zero();
        cpu.set_flag_overflow(sign_before);
        cpu.set_flag_negative();

        iri
    }

    pub fn bcs(cpu: &mut CPU, am: AddressingMode) -> IRI {
        match am {
            AddressingMode::Relative => {
                let rel_jump = get_next_byte(cpu);
                if ProcessorStatus::Carry.is_set(&cpu) {
                    cpu.registers.program_counter =
                        u16::wrapping_add(cpu.registers.program_counter, rel_jump as u16);
                }
                IRI::new(3)
            }
            _ => todo!("BCS: unrecognized AddressingMode: {:?}", am),
        }
    }

    pub fn brk(cpu: &mut CPU, am: AddressingMode) -> IRI {
        let iri = match am {
            AddressingMode::Implied => IRI::new_stop(7),
            _ => todo!("BRK: unrecognized AddressingMode: {:?}", am),
        };
        cpu.set_processor_status_bit(ProcessorStatus::Break, true);
        iri
    }
    
    pub fn jmp(cpu: &mut CPU, am: AddressingMode) -> IRI {
        match am {
            AddressingMode::Absolute => {
                cpu.registers.program_counter = get_next_word(cpu);
                IRI::new(3)
            }
            _ => todo!("JMP: unrecognized AddressingMode: {:?}", am),
        }
    }
    
    pub fn lda(cpu: &mut CPU, am: AddressingMode) -> IRI {
        let iri = match am {
            AddressingMode::Immediate => {
                let next_val = get_next_byte(cpu);
                print!(" - value: {}", next_val);
                cpu.registers.accumulator = next_val;
                IRI::new(2)
            },
            AddressingMode::Zeropage => {
                let addr = get_next_byte(cpu) as usize;
                cpu.registers.accumulator = cpu.address_bus[addr];
                IRI::new(2)
            }
            _ => todo!("LDA: unrecognized AddressingMode: {:?}", am),
        };
        cpu.set_flag_zero();
        cpu.set_flag_negative();
        iri
    }
    
    pub fn nop(cpu: &mut CPU, am: AddressingMode) -> IRI {
        match am {
            AddressingMode::Implied => IRI::new(2),
            _ => todo!("NOP: unrecognized AddressingMode: {:?}", am),
        }
    }

    pub fn pha(cpu: &mut CPU, am: AddressingMode) -> IRI {
        match am {
            AddressingMode::Implied => {
                cpu.address_bus[cpu.get_stack_location()] = cpu.registers.accumulator;
                cpu.registers.stack_pointer = u8::wrapping_sub(cpu.registers.stack_pointer, 1);
                IRI::new(3)
            }
            _ => todo!("PHA: unrecognized AddressingMode: {:?}", am),
        }
    }

    pub fn pla(cpu: &mut CPU, am: AddressingMode) -> IRI {
        let iri = match am {
            AddressingMode::Implied => {
                cpu.registers.accumulator = cpu.address_bus[cpu.get_stack_location()];
                cpu.registers.stack_pointer = u8::wrapping_add(cpu.registers.stack_pointer, 1);
                IRI::new(3)
            },
            _ => todo!("PLA: unrecognized AddressingMode: {:?}", am),
        };
        
        iri
    }

    pub fn sta(cpu: &mut CPU, am: AddressingMode) -> IRI {
        match am {
            AddressingMode::Zeropage => {
                let addr = get_next_byte(cpu) as usize;
                print!(" - addr: {}", addr);
                cpu.address_bus[addr] = cpu.registers.accumulator;
                IRI::new(2)
            }
            _ => todo!("STA: unrecognized AddressingMode: {:?}", am),
        }
    }


}
