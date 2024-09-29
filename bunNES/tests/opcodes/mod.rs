
mod load;
mod jump;
mod logic;
mod arithmetic;
mod misc;

#[macro_use]
mod helpers {
    use bunNES::nes::bus::Bus;
    use bunNES::nes::cpu::{Cpu, ProcessorStatus};
    use bunNES::nes::opcodes::{op_code_from_instruction, AddrMode, Instruction, OpCode};
    use bunNES::nes::rom::Cartridge;
    
    #[derive(Eq, PartialEq)]
    struct Registers {
        acc: u8,
        x: u8,
        y: u8,
        ps: ProcessorStatus
    }
    
    
    pub fn get_cpu(mut code: Vec<u8>) -> Cpu {
        code.resize(0x4000, 0);
        // interrupt vector
        code[0xFFFE] = 0x00;
        code[0xFFFF] = 0x01;
        let cartridge = Cartridge::test_cartride(code);

        Cpu {
            pc: 0x8000,
            sp: 0xFF,
            acc: 0,
            x: 0,
            y: 0,
            ps: ProcessorStatus::new(),
            bus: Bus::new(cartridge),
            cycles_to_finish: 0,
        }
    }

    pub fn instruction(op_code: OpCode, addr_mode: AddrMode) -> u8 {
        let instruction = Instruction {
            op_code,
            addr_mode,
            size: 1
        };
        if let Some(byte_code) = op_code_from_instruction(instruction) {
            byte_code as u8
        } else {
            panic!("Invalid instruction: {} {}", op_code, addr_mode)
        }
    }
    
    pub fn registers(cpu: &Cpu) {
        
    }
}