use super::registers;
use super::decoder;
use super::mem;

use decoder::Instruction;
use mem::Mem;

struct CPU {
    registers : registers::RV32Registers,
    memory: mem::RAM,
}

impl CPU {
    fn new(memory_size: u32) -> Self {
        CPU {
            registers: registers::RV32Registers::new(),
            memory: mem::RAM::new(memory_size),
        }
    }

    fn fetch(&self) -> u32 {
        self.memory.read_dword(self.registers.get_pc())
    }

    fn decode(instr: u32) -> Instruction {
        Instruction::decode(instr)
    }

    fn execute(&mut self, instr: Instruction) {
        // TODO
        self.registers.increment_pc();
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
}