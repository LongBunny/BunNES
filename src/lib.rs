use crate::cpu::{CPU, instructions::{Instruction, AddressingMode}};

pub mod cpu;


pub fn run() {
    let mut rom: [u8; 32 * 1024] = [0; 32 * 1024];

    rom[0x0000] = Instruction::LDA.mode(AddressingMode::Immediate);
    rom[0x0001] = 1;
    rom[0x0002] = Instruction::ADC.mode(AddressingMode::Immediate);
    rom[0x0003] = 1;
    rom[0x0004] = Instruction::BCS.mode(AddressingMode::Relative);
    rom[0x0005] = 3;
    rom[0x0006] = Instruction::JMP.mode(AddressingMode::Absolute);
    rom[0x0007] = 0x02;
    rom[0x0008] = 0x00;
    rom[0x0009] = Instruction::BRK.mode(AddressingMode::Implied);
    

    // rom[0x0000] = Instruction::LDA.mode(AddressingMode::Immediate);
    // rom[0x0001] = 12;
    // rom[0x0002] = Instruction::STA.mode(AddressingMode::Absolute);
    // rom[0x0003] = 0x00;
    // rom[0x0004] = 0x02;



    let mut cpu = CPU::new(&rom);
    cpu.run();

    // let mut line = String::new();
    // std::io::stdin().read_line(&mut line).unwrap();
    // println!("{}", line);
    // match line.as_str() {
    //     "y" => {
    //         println!("single step");
    //     }
    //     _ => {
    //         println!("automatic");
    //         cpu.run();
    //     }
    // }
    

    println!("{}", cpu);

}

