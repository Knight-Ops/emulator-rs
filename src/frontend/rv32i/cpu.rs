use super::instructions;
use super::mem;
use super::registers;

use instructions::Instruction;
use mem::Mem;
use instructions::Executable;

#[derive(Debug, Clone)]
pub struct CPU {
    registers: registers::RV32Registers,
    memory: mem::RAM,
}

impl CPU {
    pub fn new(memory_size: u32) -> Self {
        CPU {
            registers: registers::RV32Registers::new(),
            memory: mem::RAM::new(memory_size),
        }
    }

    fn fetch(&self) -> u32 {
        self.memory.read_dword(self.registers.get_pc())
    }

    fn decode(&self, instr: u32) -> Instruction {
        Instruction::decode(instr)
    }

    fn execute(&mut self, instr: Instruction) {
        instr.execute(self);
        self.registers.increment_pc();
    }

    pub fn get_registers(&mut self) -> &mut registers::RV32Registers {
        &mut self.registers
    }

    pub fn get_memory(&mut self) -> &mut mem::RAM {
        &mut self.memory
    }

    pub fn load_image(&mut self, path: String) {
        use std::io::Read;

        let mut file = std::fs::File::open(path).unwrap();
        let mut buf = [0u8; 1];
        let mut bytes_read = 1;
        let mut index = 0;

        while bytes_read != 0 {
            bytes_read = file.read(&mut buf).unwrap();
            self.get_memory().write_byte(index, buf[0]);
            index += 1;
        }
    }

    pub fn run(&mut self) {
        loop {
            let fetch = self.fetch();
            let decode = self.decode(fetch);
            self.execute(decode);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cpu_fetch() {
        let mut cpu = CPU::new(1024);

        cpu.memory.write_dword(0, 0x11335577);
        assert_eq!(cpu.fetch(), 0x11335577);
    }

    #[test]
    fn cpu_execute() {
        let mut cpu = CPU::new(1024);
        cpu.memory.write_dword(0, 0x7FF00193);
        let instr_fetch = cpu.fetch();
        assert_eq!(instr_fetch, 0x7FF00193);
        let instr_decode = cpu.decode(instr_fetch);
        assert_eq!(instr_decode, Instruction::ADDI(3, 0, 2047));
        instr_decode.execute(&mut cpu);
        assert_eq!(cpu.get_registers()[3], 2047);
    }
}
