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
    use crate::cpu::{instructions::AddressingMode, instructions::Instruction};
    use crate::cpu::cpu::CPU;

    #[allow(unused_variables)]
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


        // TODO: this
        fn zeropage() {

        }

        fn zeropage_x() {

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

        pub fn and(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }

        pub fn asl(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }

        pub fn bcc(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
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

        pub fn beq(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }

        pub fn bit(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }

        pub fn bmi(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }

        pub fn bne(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }

        pub fn bpl(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }

        pub fn brk(&mut self, am: AddressingMode) -> IRI {
            let iri = match am {
                AddressingMode::Implied => IRI::new_stop(7),
                _ => todo!("BRK: unrecognized AddressingMode: {:?}", am),
            };
            self.set_processor_status_bit(ProcessorStatus::Break, true);
            iri
        }

        pub fn bvc(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }

        pub fn bvs(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }

        pub fn clc(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }

        pub fn cld(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }

        pub fn cli(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }

        pub fn clv(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }

        pub fn cmp(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }

        pub fn cpx(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }

        pub fn cpy(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }

        pub fn dec(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }

        pub fn dex(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }

        pub fn dey(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }

        pub fn eor(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }

        pub fn inc(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }

        pub fn inx(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }

        pub fn iny(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
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

        pub fn jsr(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
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
                    let addr = self.get_next_byte();
                    self.registers.accumulator = self.get_ram_value_zp(addr);
                    IRI::new(3)
                },
                AddressingMode::ZeropageX => {
                    self.registers.accumulator = self.get_ram_value_zp(self.registers.index_register_x);
                    IRI::new(4)
                }
                _ => todo!("LDA: unrecognized AddressingMode: {:?}", am),
            };
            self.set_flag_zero();
            self.set_flag_negative();
            iri
        }

        pub fn ldx(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }

        pub fn ldy(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }

        pub fn lsr(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }

        pub fn nop(&mut self, am: AddressingMode) -> IRI {
            match am {
                AddressingMode::Implied => IRI::new(2),
                _ => todo!("NOP: unrecognized AddressingMode: {:?}", am),
            }
        }

        pub fn ora(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }

        pub fn pha(&mut self, am: AddressingMode) -> IRI {
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

        pub fn php(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }

        pub fn pla(&mut self, am: AddressingMode) -> IRI {
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

        pub fn plp(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }

        pub fn rol(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }

        pub fn ror(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }

        pub fn rti(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }

        pub fn rts(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }

        pub fn sbc(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }

        pub fn sec(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }

        pub fn sed(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }

        pub fn sei(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
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

        pub fn stx(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }

        pub fn sty(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }

        pub fn tax(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }

        pub fn tay(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }

        pub fn tsx(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }

        pub fn txa(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }

        pub fn txs(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }

        pub fn tya(&mut self, am: AddressingMode) -> IRI {
            IRI::new(0)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::CPU;

    fn get_cpu() -> CPU {
        let binary = [0u8; 64 * 1024];
        CPU::new(binary)
    }

    mod cpu_tests {
        use super::get_cpu;

        #[test]
        fn cpu_acc_empty() {
            let cpu = get_cpu();
            assert_eq!(cpu.registers.accumulator, 0x00);
        }
    }

    mod opcode_tests {
        use super::get_cpu;
        use crate::cpu::{
            instructions::{self, AddressingMode, Instruction},
            CPU,
        };

        fn prep_command(
            cpu: &mut CPU,
            ins: Instruction,
            am: AddressingMode,
            value: Option<u8>,
            value2: Option<u8>,
        ) {
            cpu.registers.program_counter = 0x0000;
            let rom_start = 32 * 1024;
            cpu.address_bus[rom_start + 0x0000] = Instruction::mode(&ins, am);
            if let Some(value) = value {
                cpu.address_bus[rom_start + 0x0001] = value;
            }
            if let Some(value2) = value2 {
                cpu.address_bus[rom_start + 0x0002] = value2;
            }

            cpu.registers.program_counter = u16::wrapping_add(cpu.registers.program_counter, 1);
        }

        fn prep_memory_zp(cpu: &mut CPU, addr: u8, value: u8) {
            cpu.address_bus[(0x0000 + addr) as usize] = value;
        }

        fn prep_memory(cpu: &mut CPU, addr: u16, value: u8) {
            cpu.address_bus[(0x0000 + addr) as usize] = value;
        }

        mod opcode_tests_lda {
            use super::*;

            #[test]
            fn cpu_lda_immediate() {
                let adm = AddressingMode::Immediate;
                let mut cpu = get_cpu();
                // immediate
                prep_command(
                    &mut cpu,
                    Instruction::LDA,
                    adm,
                    Some(2),
                    None,
                );
                cpu.lda(adm);
                assert_eq!(cpu.registers.accumulator, 0x02);
            }

            #[test]
            fn cpu_lda_zeropage() {
                let adm = AddressingMode::Zeropage;
                let mut cpu = get_cpu();
                prep_memory_zp(&mut cpu, 0x02, 0x08);
                prep_command(
                    &mut cpu,
                    Instruction::LDA,
                    adm,
                    Some(2),
                    None,
                );
                cpu.lda(adm);
                assert_eq!(cpu.registers.accumulator, 0x08);
            }

            #[test]
            fn cpu_lda_zeropage_x() {
                let adm = AddressingMode::ZeropageX;
                let mut cpu = get_cpu();
                cpu.registers.index_register_x = 0x02;
                prep_memory_zp(&mut cpu, 0x02, 0x08);
                prep_command(
                    &mut cpu,
                    Instruction::LDA,
                    adm,
                    Some(0),
                    None,
                );
                println!("{}", cpu);
                cpu.lda(adm);
                assert_eq!(cpu.registers.accumulator, 0x08);
            }
            
            #[test]
            fn cpu_lda_absolute() {
                let adm = AddressingMode::Absolute;
                let mut cpu = get_cpu();
                prep_memory(&mut cpu, 0x4000, 0x08);
                prep_command(
                    &mut cpu,
                    Instruction::LDA,
                    adm,
                    Some(0x00),
                    Some(0x40),
                );
                cpu.lda(adm);
                assert_eq!(cpu.registers.accumulator, 0x08);
            }
            
            #[test]
            fn cpu_lda_absolute_x() {
                let adm = AddressingMode::AbsoluteX;
                let mut cpu = get_cpu();
                cpu.registers.index_register_x = 0x02;
                prep_memory(&mut cpu, 0x4000, 0x08);
                prep_command(
                    &mut cpu,
                    Instruction::LDA,
                    adm,
                    Some(0x00),
                    Some(0x00),
                );
                cpu.lda(adm);
                assert_eq!(cpu.registers.accumulator, 0x08);
            }
            
            #[test]
            fn cpu_lda_absolute_y() {
                let adm = AddressingMode::AbsoluteY;
                let mut cpu = get_cpu();
                cpu.registers.index_register_y = 0x02;
                prep_memory(&mut cpu, 0x4000, 0x08);
                prep_command(
                    &mut cpu,
                    Instruction::LDA,
                    adm,
                    Some(0x00),
                    Some(0x00),
                );
                cpu.lda(adm);
                assert_eq!(cpu.registers.accumulator, 0x08);
            }
            
            #[test]
            fn cpu_lda_indirect_x() {
                let adm = AddressingMode::IndirectX;
                let mut cpu = get_cpu();
                cpu.registers.index_register_x = 0x02;
                prep_memory(&mut cpu, 0x4000, 0x08);
                prep_command(
                    &mut cpu,
                    Instruction::LDA,
                    adm,
                    Some(0x00),
                    Some(0x00),
                );
                cpu.lda(adm);
                assert_eq!(cpu.registers.accumulator, 0x08);
            }

            
            #[test]
            fn cpu_lda_indirect_y() {
                let adm = AddressingMode::IndirectY;
                let mut cpu = get_cpu();
                cpu.registers.index_register_y = 0x02;
                prep_memory(&mut cpu, 0x4000, 0x08);
                prep_command(
                    &mut cpu,
                    Instruction::LDA,
                    adm,
                    Some(0x00),
                    Some(0x00),
                );
                cpu.lda(adm);
                assert_eq!(cpu.registers.accumulator, 0x08);
            }
        }
    }
}


