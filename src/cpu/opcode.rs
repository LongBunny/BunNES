use super::{
    instructions::{AddressingMode, Instruction},
    CPU,
};

// instruction return info
struct IRI {
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
    let iri = match ins.0 {
        Instruction::BRK => brk(cpu, ins.1),
        Instruction::NOP => nop(cpu, ins.1),
        Instruction::LDA => lda(cpu, ins.1),
        Instruction::ADC => adc(cpu, ins.1),
        Instruction::JMP => jmp(cpu, ins.1),
        not_implemented => {
            panic!("Not implemented instruction {:?}", not_implemented);
        }
    };
    print!("\n");
    iri.stop
}

fn get_next_byte(cpu: &mut CPU) -> u8 {
    let ret = cpu.rom[(cpu.registers.program_counter) as usize];
    cpu.registers.program_counter = u16::wrapping_add(cpu.registers.program_counter, 1);
    ret
}

fn get_next_two_bytes(cpu: &mut CPU) -> u16 {
    let ret: [u8; 2] = [
        cpu.rom[(cpu.registers.program_counter + 0) as usize],
        cpu.rom[(cpu.registers.program_counter + 1) as usize],
    ];
    cpu.registers.program_counter = u16::wrapping_add(cpu.registers.program_counter, 2);
    ((ret[1] as u16) << 8) ^ (ret[0] as u16)
}

fn brk(cpu: &mut CPU, am: AddressingMode) -> IRI {
    match am {
        AddressingMode::Implied => IRI::new_stop(7),
        _ => panic!("BRK: unrecognized AddressingMode: {:?}", am),
    }
}

fn nop(cpu: &mut CPU, am: AddressingMode) -> IRI {
    match am {
        AddressingMode::Implied => IRI::new_stop(2),
        _ => panic!("NOP: unrecognized AddressingMode: {:?}", am),
    }
}

fn lda(cpu: &mut CPU, am: AddressingMode) -> IRI {
    match am {
        AddressingMode::Immediate => {
            let next_val = get_next_byte(cpu);
            print!(" - value: {}", next_val);
            cpu.registers.accumulator = next_val;
            IRI::new(2)
        },
        _ => panic!("LDA: unrecognized AddressingMode: {:?}", am),
    }
}

fn adc(cpu: &mut CPU, am: AddressingMode) -> IRI {
    match am {
        AddressingMode::Immediate => {
            let next_val = get_next_byte(cpu);
            print!(" - value: {}", next_val);
            cpu.registers.accumulator = u8::wrapping_add(cpu.registers.accumulator, next_val);
            IRI::new(2)
        },
        _ => panic!("ADC: unrecognized AddressingMode: {:?}", am),
    }
}

fn jmp(cpu: &mut CPU, am: AddressingMode) -> IRI {
    match am {
        AddressingMode::Absolute => {
            cpu.registers.program_counter = get_next_two_bytes(cpu);
            IRI::new(3)
        },
        _ => panic!("ADC: unrecognized AddressingMode: {:?}", am),
    }
}
