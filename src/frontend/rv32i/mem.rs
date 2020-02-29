use std::ops::{Deref, DerefMut};

pub trait Mem {
    fn read_byte(&self, addr: u32) -> u8;
    fn write_byte(&mut self, addr: u32, val: u8);

    fn read_word(&self, addr: u32) -> u16 {
        self.read_byte(addr) as u16 | (self.read_byte(addr + 1) as u16) << 8
    }

    fn write_word(&mut self, addr: u32, val: u16) {
        self.write_byte(addr, (val & 0xFF) as u8);
        self.write_byte(addr + 1, ((val >> 8) & 0xFF) as u8);
    }

    fn read_dword(&self, addr: u32) -> u32 {
        (self.read_byte(addr) as u32)
            | (self.read_byte(addr + 1) as u32) << 8
            | (self.read_byte(addr + 2) as u32) << 16
            | (self.read_byte(addr + 3) as u32) << 24
    }

    fn write_dword(&mut self, addr: u32, val: u32) {
        self.write_byte(addr, (val & 0xFF) as u8);
        self.write_byte(addr + 1, ((val >> 8) & 0xFF) as u8);
        self.write_byte(addr + 2, ((val >> 16) & 0xFF) as u8);
        self.write_byte(addr + 3, ((val >> 24) & 0xFF) as u8);
    }
}

#[derive(Debug, Clone)]
pub struct RAM {
    pub inner: Vec<u8>,
}

impl RAM {
    pub fn new(size: u32) -> Self {
        RAM {
            inner: vec![0; size as usize],
        }
    }
}

impl Deref for RAM {
    type Target = Vec<u8>;

    fn deref(&self) -> &Vec<u8> {
        &self.inner
    }
}

impl DerefMut for RAM {
    fn deref_mut(&mut self) -> &mut Vec<u8> {
        &mut self.inner
    }
}

impl Mem for RAM {
    // TODO We need to implement memory protections here so we can't read everything
    fn read_byte(&self, addr: u32) -> u8 {
        self.inner[addr as usize]
    }

    // TODO We need to implement memory protections here so we can't read everything
    fn write_byte(&mut self, addr: u32, val: u8) {
        self.inner[addr as usize] = val;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ram_new() {
        let ram = RAM::new(1024);
        assert_eq!(ram.inner.len(), 1024);
    }

    #[test]
    fn read_zero() {
        let ram = RAM::new(1024);
        assert_eq!(ram.read_byte(0), 0);
    }

    #[test]
    fn write_read_byte() {
        let mut ram = RAM::new(1024);
        ram.write_byte(527, 83);
        assert_eq!(ram.read_byte(527), 83);
    }

    #[test]
    fn write_read_word() {
        let mut ram = RAM::new(1024);
        ram.write_word(527, 0x1234);
        assert_eq!(ram.read_word(527), 0x1234);
    }

    #[test]
    fn write_read_dword() {
        let mut ram = RAM::new(1024);
        ram.write_dword(527, 0x12345678);
        assert_eq!(ram.read_dword(527), 0x12345678);
    }

    #[test]
    #[should_panic]
    fn read_past_memory() {
        let ram = RAM::new(1024);
        assert_eq!(ram.read_byte(2048), 0);
    }
}
