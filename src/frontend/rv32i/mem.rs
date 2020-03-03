use std::ops::{Deref, DerefMut};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MemoryError {
    UnmappedRegion,
    AlreadyMappedRegion,
}

type MemoryResult<T> = Result<T, MemoryError>;

pub trait Mem {
    fn read_byte(&self, addr: u32) -> MemoryResult<u8>;
    fn write_byte(&mut self, addr: u32, val: u8) -> MemoryResult<()>;

    fn read_halfword(&self, addr: u32) -> MemoryResult<u16> {
        Ok(self.read_byte(addr)? as u16 | (self.read_byte(addr + 1)? as u16) << 8)
    }

    fn write_halfword(&mut self, addr: u32, val: u16) -> MemoryResult<()> {
        self.write_byte(addr, (val & 0xFF) as u8)?;
        self.write_byte(addr + 1, ((val >> 8) & 0xFF) as u8)?;
        Ok(())
    }

    fn read_word(&self, addr: u32) -> MemoryResult<u32> {
        Ok((self.read_byte(addr)? as u32)
            | (self.read_byte(addr + 1)? as u32) << 8
            | (self.read_byte(addr + 2)? as u32) << 16
            | (self.read_byte(addr + 3)? as u32) << 24)
    }

    fn write_word(&mut self, addr: u32, val: u32) -> MemoryResult<()> {
        self.write_byte(addr, (val & 0xFF) as u8)?;
        self.write_byte(addr + 1, ((val >> 8) & 0xFF) as u8)?;
        self.write_byte(addr + 2, ((val >> 16) & 0xFF) as u8)?;
        self.write_byte(addr + 3, ((val >> 24) & 0xFF) as u8)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct RAM {
    mapping: HashMap<(u32, u32), Vec<u8>>,
    // pub inner: Vec<u8>,
}

impl RAM {
    pub fn new(base: u32, size: u32) -> Self {
        let mut hm = HashMap::new();
        hm.insert((base, size), vec![0; size as usize]);
        RAM {
            mapping: hm,
            // inner: vec![0; size as usize],
        }
    }

    pub fn add_segment(&mut self, base: u32, size: u32) -> MemoryResult<()> {
        for entry in self.mapping.keys() {
            if base >= entry.0 && base < (entry.0 + entry.1) {
                return Err(MemoryError::AlreadyMappedRegion)
            }
        }
        // println!("Adding segment @ {:X} with size {:X}", base, size);
        self.mapping.insert((base, size), vec![0; size as usize]);
        Ok(())
    }
}

// impl Deref for RAM {
//     type Target = Vec<u8>;

//     fn deref(&self) -> &HashMap<(u32, u32), Vec<u8>> {
//         &self.mapping
//     }
// }

// impl DerefMut for RAM {
//     fn deref_mut(&mut self) -> &mut HashMap<(u32, u32), Vec<u8>> {
//         &mut self.mapping
//     }
// }

impl Mem for RAM {
    // TODO We need to implement memory protections here so we can't read everything
    fn read_byte(&self, addr: u32) -> MemoryResult<u8> {
        for entry in self.mapping.keys() {
            if addr >= entry.0 && addr < (entry.0 + entry.1) {
                return Ok(self.mapping.get(entry).unwrap()[(addr - entry.0) as usize])
            }
        }
        Err(MemoryError::UnmappedRegion)
        // self.inner[addr as usize]
    }

    // TODO We need to implement memory protections here so we can't read everything
    fn write_byte(&mut self, addr: u32, val: u8) -> MemoryResult<()> {

        self.mapping.iter_mut().for_each(|(key, value)| {
            if addr >= key.0 && addr < (key.0 + key.1) {
                value[(addr - key.0) as usize] = val;
            }
        });
        Ok(())
        // self.inner[addr as usize] = val;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ram_new() {
        let ram = RAM::new(0, 1024);
        assert_eq!(ram.mapping.values().next().unwrap().len(), 1024);
    }

    #[test]
    fn read_zero() {
        let ram = RAM::new(0, 1024);
        assert_eq!(ram.read_byte(0).unwrap(), 0);
    }

    #[test]
    fn write_read_byte() {
        let mut ram = RAM::new(0, 1024);
        ram.write_byte(527, 83);
        assert_eq!(ram.read_byte(527).unwrap(), 83);
    }

    #[test]
    fn write_read_halfword() {
        let mut ram = RAM::new(0, 1024);
        ram.write_halfword(527, 0x1234);
        assert_eq!(ram.read_halfword(527).unwrap(), 0x1234);
    }

    #[test]
    fn write_read_word() {
        let mut ram = RAM::new(0, 1024);
        ram.write_word(527, 0x12345678);
        assert_eq!(ram.read_word(527).unwrap(), 0x12345678);
    }

    #[test]
    #[should_panic]
    fn read_past_memory() {
        let mut ram = RAM::new(0, 1024);
        assert_eq!(ram.read_byte(2048).unwrap(), 0);
    }
}
