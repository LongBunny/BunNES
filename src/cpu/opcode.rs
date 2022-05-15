
// instruction return info

#[allow(unused_variables)]
#[allow(dead_code)]
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


mod cpu_functions {
    use super::IRI;
    use crate::cpu::{instructions::{AddressingMode, Instruction}, ProcessorStatus, CPU};

    impl CPU {
        pub fn process(&mut self, ins: (Instruction, AddressingMode)) -> bool {
            print!("Executing: {:?} {:?}", ins.0, ins.1);
            
            
            let iri: IRI = match ins.0 {
                Instruction::ADC => self.adc(ins.1),
                Instruction::BCS => self.bcs(ins.1),
                Instruction::BRK => self.brk(ins.1),
                Instruction::JMP => self.jmp(ins.1),
                Instruction::LDA => self.lda(ins.1),
                Instruction::NOP => self.nop(ins.1),
                Instruction::PHA => self.pha(ins.1),
                Instruction::PLA => self.pla(ins.1),
                Instruction::STA => self.sta(ins.1),
                not_implemented => {
                    panic!("Not implemented instruction {:?}", not_implemented);
                }
            };
            print!("\n");
            iri.stop
        }


        fn get_next_byte(&mut self) -> u8 {
            let ret = self.address_bus[(self.registers.program_counter + 32 * 1024) as usize];
            self.registers.program_counter = u16::wrapping_add(self.registers.program_counter, 1);
            ret
        }

        fn get_next_word(&mut self) -> u16 {
            let ret: [u8; 2] = [
                self.address_bus[(self.registers.program_counter + 32 * 1024 + 0) as usize],
                self.address_bus[(self.registers.program_counter + 32 * 1024 + 1) as usize],
            ];
            self.registers.program_counter = u16::wrapping_add(self.registers.program_counter, 2);
            ((ret[1] as u16) << 8) ^ (ret[0] as u16)
        }

        fn overflow_add(&mut self, val: u8) -> bool {
            let (new_val, overflow) = u8::overflowing_add(self.registers.accumulator, val);
            self.registers.accumulator = new_val;
            overflow
        }

        pub fn adc(&mut self, am: AddressingMode) -> IRI {
            let val: u8;
            let iri = match am {
                AddressingMode::Immediate => {
                    val = self.get_next_byte();
                    print!(" - value: {}", val);
                    IRI::new(2)
                }
                AddressingMode::Zeropage => {
                    let addr = self.get_next_byte();
                    val = self.get_ram_value_zp(addr);
                    print!(" - value: {}", val);
                    IRI::new(3)
                }
                AddressingMode::ZeropageX => {
                    let addr = self.get_next_byte();
                    val = self.get_ram_value_zp(addr + self.registers.index_register_x);
                    print!(" - value: {}", val);
                    IRI::new(4)
                }
                _ => todo!("ADC: unrecognized AddressingMode: {:?}", am),
            };
            let sign_before = self.registers.accumulator & 0b1000_0000;
            let overflow = self.overflow_add(val);

            self.set_flag_carry(overflow);
            self.set_flag_zero();
            self.set_flag_overflow(sign_before);
            self.set_flag_negative();

            iri
        }

        pub fn bcs(&mut self, am: AddressingMode) -> IRI {
            match am {
                AddressingMode::Relative => {
                    let rel_jump = self.get_next_byte();
                    if ProcessorStatus::Carry.is_set(&self) {
                        self.registers.program_counter =
                            u16::wrapping_add(self.registers.program_counter, rel_jump as u16);
                    }
                    IRI::new(3)
                }
                _ => todo!("BCS: unrecognized AddressingMode: {:?}", am),
            }
        }

        pub fn brk(&mut self, am: AddressingMode) -> IRI {
            let iri = match am {
                AddressingMode::Implied => IRI::new_stop(7),
                _ => todo!("BRK: unrecognized AddressingMode: {:?}", am),
            };
            self.set_processor_status_bit(ProcessorStatus::Break, true);
            iri
        }

        pub fn jmp(&mut self, am: AddressingMode) -> IRI {
            match am {
                AddressingMode::Absolute => {
                    self.registers.program_counter = self.get_next_word();
                    IRI::new(3)
                }
                _ => todo!("JMP: unrecognized AddressingMode: {:?}", am),
            }
        }

        pub fn lda(&mut self, am: AddressingMode) -> IRI {
            let iri = match am {
                AddressingMode::Immediate => {
                    let next_val = self.get_next_byte();
                    print!(" - value: {}", next_val);
                    self.registers.accumulator = next_val;
                    IRI::new(2)
                }
                AddressingMode::Zeropage => {
                    let addr = self.get_next_byte() as usize;
                    self.registers.accumulator = self.address_bus[addr];
                    IRI::new(2)
                }
                _ => todo!("LDA: unrecognized AddressingMode: {:?}", am),
            };
            self.set_flag_zero();
            self.set_flag_negative();
            iri
        }

        pub fn nop(&mut self, am: AddressingMode) -> IRI {
            match am {
                AddressingMode::Implied => IRI::new(2),
                _ => todo!("NOP: unrecognized AddressingMode: {:?}", am),
            }
        }

        pub fn pha(&mut self, am: AddressingMode) -> IRI {
            match am {
                AddressingMode::Implied => {
                    self.address_bus[self.get_stack_location()] = self.registers.accumulator;
                    self.registers.stack_pointer = u8::wrapping_sub(self.registers.stack_pointer, 1);
                    IRI::new(3)
                }
                _ => todo!("PHA: unrecognized AddressingMode: {:?}", am),
            }
        }

        pub fn pla(&mut self, am: AddressingMode) -> IRI {
            let iri = match am {
                AddressingMode::Implied => {
                    self.registers.accumulator = self.address_bus[self.get_stack_location()];
                    self.registers.stack_pointer = u8::wrapping_add(self.registers.stack_pointer, 1);
                    IRI::new(3)
                }
                _ => todo!("PLA: unrecognized AddressingMode: {:?}", am),
            };

            iri
        }

        pub fn sta(&mut self, am: AddressingMode) -> IRI {
            match am {
                AddressingMode::Zeropage => {
                    let addr = self.get_next_byte() as usize;
                    print!(" - addr: {}", addr);
                    self.address_bus[addr] = self.registers.accumulator;
                    IRI::new(2)
                }
                _ => todo!("STA: unrecognized AddressingMode: {:?}", am),
            }
        }
    }
}
