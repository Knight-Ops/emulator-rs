use std::ops::{DerefMut, Deref};

#[derive(Debug, Copy, Clone, Default)]
pub struct RV32Registers {
    inner : [u32; 32],
    pc : u32,
    // x0: u32,
    // x1: u32,
    // x2: u32,
    // x3: u32,
    // x4: u32,
    // x5: u32,
    // x6: u32,
    // x7: u32,
    // x8: u32,
    // x9: u32,
    // x10: u32,
    // x11: u32,
    // x12: u32,
    // x13: u32,
    // x14: u32,
    // x15: u32,
    // x16: u32,
    // x17: u32,
    // x18: u32,
    // x19: u32,
    // x20: u32,
    // x21: u32,
    // x22: u32,
    // x23: u32,
    // x24: u32,
    // x25: u32,
    // x26: u32,
    // x27: u32,
    // x28: u32,
    // x29: u32,
    // x30: u32,
    // x31: u32,
    // pc: u32,
}

impl Deref for RV32Registers {
    type Target = [u32; 32];

    fn deref(&self) -> &[u32; 32] {
        &self.inner
    }
}

impl DerefMut for RV32Registers {
    fn deref_mut(&mut self) -> &mut [u32; 32] {
        &mut self.inner
    }
}

impl RV32Registers {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get_pc(&self) -> u32 {
        self.pc
    }

    // This might have a signedness issue when we are adding
    pub fn add_to_pc(&mut self, val: u32) {
        let (value, overflow) = self.pc.overflowing_add(val);
        self.pc = value;
    }

    pub fn set_pc(&mut self, val: u32) {
        self.pc = val;
    }

    pub fn increment_pc(&mut self) {
        self.pc += 4;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let regs = RV32Registers::new();

        assert_eq!(*regs, [0; 32])

        // assert_eq!(regs.x0, 0);
        // assert_eq!(regs.x1, 0);
        // assert_eq!(regs.x2, 0);
        // assert_eq!(regs.x3, 0);
        // assert_eq!(regs.x4, 0);
        // assert_eq!(regs.x5, 0);
        // assert_eq!(regs.x6, 0);
        // assert_eq!(regs.x7, 0);
        // assert_eq!(regs.x8, 0);
        // assert_eq!(regs.x9, 0);
        // assert_eq!(regs.x10, 0);
        // assert_eq!(regs.x11, 0);
        // assert_eq!(regs.x12, 0);
        // assert_eq!(regs.x13, 0);
        // assert_eq!(regs.x14, 0);
        // assert_eq!(regs.x15, 0);
        // assert_eq!(regs.x16, 0);
        // assert_eq!(regs.x17, 0);
        // assert_eq!(regs.x18, 0);
        // assert_eq!(regs.x19, 0);
        // assert_eq!(regs.x20, 0);
        // assert_eq!(regs.x21, 0);
        // assert_eq!(regs.x22, 0);
        // assert_eq!(regs.x23, 0);
        // assert_eq!(regs.x24, 0);
        // assert_eq!(regs.x25, 0);
        // assert_eq!(regs.x26, 0);
        // assert_eq!(regs.x27, 0);
        // assert_eq!(regs.x28, 0);
        // assert_eq!(regs.x29, 0);
        // assert_eq!(regs.x30, 0);
        // assert_eq!(regs.x31, 0);
        // assert_eq!(regs.pc, 0);
    }

    #[test]
    fn inc_pc() {
        let mut regs = RV32Registers::new();
        assert_eq!(regs.pc, 0);
        regs.increment_pc();
        assert_eq!(regs.pc, 4);
    }

    #[test]
    fn get_pc() {
        let mut regs = RV32Registers::new();
        assert_eq!(regs.get_pc(), 0);
        regs.increment_pc();
        assert_eq!(regs.get_pc(), 4);
    }
}
