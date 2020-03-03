use super::instructions;
use super::mem;
use super::registers;

use instructions::Executable;
use instructions::{DecodeError, ExecuteError, Instruction, ExecuteStatus};
use mem::Mem;
use mem::MemoryError;

use xmas_elf::{ElfFile, header, program};
use xmas_elf::sections;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CPUError {
    FetchError,
    DecodeError(DecodeError),
    ExecuteError(ExecuteError),
    MemoryError(MemoryError)
}

impl From<DecodeError> for CPUError {
    fn from(err: DecodeError) -> CPUError {
        CPUError::DecodeError(err)
    }
}

impl From<ExecuteError> for CPUError {
    fn from(err: ExecuteError) -> CPUError {
        CPUError::ExecuteError(err)
    }
}

impl From<MemoryError> for CPUError {
    fn from(err: MemoryError) -> CPUError {
        CPUError::MemoryError(err)
    }
}

type CPUResult<T> = Result<T, CPUError>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CPUStatus {
    Continue,
    Halt,
}

impl From<ExecuteStatus> for CPUStatus {
    fn from(status: ExecuteStatus) -> CPUStatus {
        match status {
            ExecuteStatus::CONTINUE => CPUStatus::Continue,
            _ => CPUStatus::Halt,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CPU {
    registers: registers::RV32Registers,
    memory: mem::RAM,
}

impl CPU {
    pub fn new(memory_base: u32, memory_size: u32) -> Self {
        CPU {
            registers: registers::RV32Registers::new(),
            memory: mem::RAM::new(memory_base, memory_size),
        }
    }

    fn fetch(&self) -> CPUResult<u32> {
        let fetch = self.memory.read_word(self.registers.get_pc())?;
        Ok(fetch)
    }

    fn decode(&self, instr: u32) -> CPUResult<Instruction> {
        let decode = Instruction::decode(instr)?;
        Ok(decode)
    }

    fn execute(&mut self, instr: Instruction) -> CPUResult<CPUStatus> {
        let res = instr.execute(self)?;
        Ok(res.into())
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
        let mut index = 0x40;

        while bytes_read != 0 {
            bytes_read = file.read(&mut buf).unwrap();
            self.get_memory().write_byte(index, buf[0]);
            index += 1;
        }
    }

    pub fn load_raw_assembly(&mut self, path: String) {
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

    pub fn load_elf(&mut self, path: String) -> u32 {
        use std::io::Read;

        let binary_blob = std::fs::read(path).expect("Can't read binary");
        let elf_file = ElfFile::new(&binary_blob).expect("What is happening");
        
        let mut sect_iter = elf_file.section_iter();
        sect_iter.next();

        
        for sect in sect_iter {
            let section_type = sect.get_type();

            if section_type.is_ok() {
                if let Ok(result) = section_type {
                    match result {
                        sections::ShType::ProgBits => self.memory.add_segment(sect.address() as u32, sect.size() as u32).unwrap(),
                        _ => {}
                    }  
                }
            }

            let offset = sect.offset() as u32;
            let data = &binary_blob[offset as usize..(offset + sect.size() as u32) as usize];

            println!("{:?}", sect);
            let virt_addr = sect.address() as u32;
            let mut virtual_offset = 0;
            for byte in data {
                self.get_memory().write_byte(virt_addr + virtual_offset, *byte);
                virtual_offset += 1;
            }
        }

        elf_file.header.pt2.entry_point() as u32
    }

    pub fn run(&mut self) -> CPUResult<CPUStatus> {
        loop {
            println!("PC is : {:X}", self.get_registers().get_pc());
            let fetch = self.fetch()?;
            println!("Fetch : {:X?}", fetch);
            let decode = self.decode(fetch)?;
            println!("Decode : {:X?}", decode);
            if let Ok(CPUStatus::Continue) = self.execute(decode){

            } else {
                return Ok(CPUStatus::Halt)
            }
        }
    }

    pub fn run_for_steps(&mut self, steps: usize) -> CPUResult<CPUStatus> {
        let mut step_result = Ok(CPUStatus::Continue);
        for _ in 0..steps {
            // println!("PC is : {:X}", self.get_registers().get_pc());
            let fetch = self.fetch()?;
            // println!("Fetch : {:X?}", fetch);
            let decode = self.decode(fetch)?;
            // println!("Decode : {:X?}", decode);
            step_result = self.execute(decode);
        }

        step_result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cpu_fetch() {
        let mut cpu = CPU::new(0, 1024);

        cpu.memory.write_word(0, 0x11335577);
        assert_eq!(cpu.fetch().unwrap(), 0x11335577);
    }

    #[test]
    fn cpu_execute() {
        let mut cpu = CPU::new(0, 1024);
        cpu.memory.write_word(0, 0x7FF00193);
        let instr_fetch = cpu.fetch().unwrap();
        assert_eq!(instr_fetch, 0x7FF00193);
        let instr_decode = cpu.decode(instr_fetch).unwrap();
        assert_eq!(instr_decode, Instruction::ADDI(3, 0, 2047));
        instr_decode.execute(&mut cpu);
        assert_eq!(cpu.get_registers()[3], 2047);
    }
}
