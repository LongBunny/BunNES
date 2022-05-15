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
    use crate::cpu::{
        instructions::{AddressingMode, Instruction},
        ProcessorStatus, CPU,
    };

    impl CPU {
        pub fn process(&mut self, ins: Instruction, am: AddressingMode) -> bool {
            print!("Executing: {:?} {:?}", ins, am);

            let iri: IRI = match ins {
                Instruction::ADC => self.adc(am),
                Instruction::AND => self.and(am),
                Instruction::ASL => self.asl(am),
                Instruction::BCC => self.bcc(am),
                Instruction::BCS => self.bcs(am),
                Instruction::BEQ => self.beq(am),
                Instruction::BIT => self.bit(am),
                Instruction::BMI => self.bmi(am),
                Instruction::BNE => self.bne(am),
                Instruction::BPL => self.bpl(am),
                Instruction::BRK => self.brk(am),
                Instruction::BVC => self.bvc(am),
                Instruction::BVS => self.bvs(am),
                Instruction::CLC => self.clc(am),
                Instruction::CLD => self.cld(am),
                Instruction::CLI => self.cli(am),
                Instruction::CLV => self.clv(am),
                Instruction::CMP => self.cmp(am),
                Instruction::CPX => self.cpx(am),
                Instruction::CPY => self.cpy(am),
                Instruction::DEC => self.dec(am),
                Instruction::DEX => self.dex(am),
                Instruction::DEY => self.dey(am),
                Instruction::EOR => self.eor(am),
                Instruction::INC => self.inc(am),
                Instruction::INX => self.inx(am),
                Instruction::INY => self.iny(am),
                Instruction::JMP => self.jmp(am),
                Instruction::JSR => self.jsr(am),
                Instruction::LDA => self.lda(am),
                Instruction::LDX => self.ldx(am),
                Instruction::LDY => self.ldy(am),
                Instruction::LSR => self.lsr(am),
                Instruction::NOP => self.nop(am),
                Instruction::ORA => self.ora(am),
                Instruction::PHA => self.pha(am),
                Instruction::PHP => self.php(am),
                Instruction::PLA => self.pla(am),
                Instruction::PLP => self.plp(am),
                Instruction::ROL => self.rol(am),
                Instruction::ROR => self.ror(am),
                Instruction::RTI => self.rti(am),
                Instruction::RTS => self.rts(am),
                Instruction::SBC => self.sbc(am),
                Instruction::SEC => self.sec(am),
                Instruction::SED => self.sed(am),
                Instruction::SEI => self.sei(am),
                Instruction::STA => self.sta(am),
                Instruction::STX => self.stx(am),
                Instruction::STY => self.sty(am),
                Instruction::TAX => self.tax(am),
                Instruction::TAY => self.tay(am),
                Instruction::TSX => self.tsx(am),
                Instruction::TXA => self.txa(am),
                Instruction::TXS => self.txs(am),
                Instruction::TYA => self.tya(am),
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

        fn adc(&mut self, am: AddressingMode) -> IRI {
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

        fn and(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }

        fn asl(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }

        fn bcc(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }

        fn bcs(&mut self, am: AddressingMode) -> IRI {
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

        fn beq(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }

        fn bit(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }
        
        fn bmi(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }
        
        fn bne(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }
        
        fn bpl(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }
        
        fn brk(&mut self, am: AddressingMode) -> IRI {
            let iri = match am {
                AddressingMode::Implied => IRI::new_stop(7),
                _ => todo!("BRK: unrecognized AddressingMode: {:?}", am),
            };
            self.set_processor_status_bit(ProcessorStatus::Break, true);
            iri
        }

        fn bvc(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }
        
        fn bvs(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }
        
        fn clc(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }
        
        fn cld(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }
        
        fn cli(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }
        
        fn clv(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }
        
        fn cmp(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }
        
        fn cpx(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }
        
        fn cpy(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }
        
        fn dec(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }
        
        fn dex(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }
        
        fn dey(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }
        
        fn eor(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }
        
        fn inc(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }
        
        fn inx(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }
        
        fn iny(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }
        
        fn jmp(&mut self, am: AddressingMode) -> IRI {
            match am {
                AddressingMode::Absolute => {
                    self.registers.program_counter = self.get_next_word();
                    IRI::new(3)
                }
                _ => todo!("JMP: unrecognized AddressingMode: {:?}", am),
            }
        }
        
        fn jsr(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }
        
        fn lda(&mut self, am: AddressingMode) -> IRI {
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

        fn ldx(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }
        
        fn ldy(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }
        
        fn lsr(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }
        
        fn nop(&mut self, am: AddressingMode) -> IRI {
            match am {
                AddressingMode::Implied => IRI::new(2),
                _ => todo!("NOP: unrecognized AddressingMode: {:?}", am),
            }
        }

        fn ora(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }
        
        fn pha(&mut self, am: AddressingMode) -> IRI {
            match am {
                AddressingMode::Implied => {
                    self.address_bus[self.get_stack_location()] = self.registers.accumulator;
                    self.registers.stack_pointer =
                        u8::wrapping_sub(self.registers.stack_pointer, 1);
                    IRI::new(3)
                }
                _ => todo!("PHA: unrecognized AddressingMode: {:?}", am),
            }
        }

        fn php(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }
        
        fn pla(&mut self, am: AddressingMode) -> IRI {
            let iri = match am {
                AddressingMode::Implied => {
                    self.registers.accumulator = self.address_bus[self.get_stack_location()];
                    self.registers.stack_pointer =
                        u8::wrapping_add(self.registers.stack_pointer, 1);
                    IRI::new(3)
                }
                _ => todo!("PLA: unrecognized AddressingMode: {:?}", am),
            };

            iri
        }

        fn plp(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }
        
        fn rol(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }
        
        fn ror(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }
        
        fn rti(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }
        
        fn rts(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }
        
        fn sbc(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }
        
        fn sec(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }
        
        fn sed(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }
        
        fn sei(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }
        
        fn sta(&mut self, am: AddressingMode) -> IRI {
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
        
        fn stx(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }
        
        fn sty(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }
        
        fn tax(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }
        
        fn tay(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }
        
        fn tsx(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }
        
        fn txa(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }
        
        fn txs(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }
        
        fn tya(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }
        
    }
}
