use super::mem::Mem;
use super::mem::MemoryError;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DecodeError {
    RTypeExtract(u32, u32),
    ITypeExtract(u32, u32),
    STypeExtract(u32, u32),
    BTypeExtract(u32, u32),
    UTypeExtract(u32, u32),
    JTypeExtract(u32, u32),
    InvalidFence(u32, u32),
    InvalidInstruction(u32, u32),
}

type DecodeResult<T> = Result<T, DecodeError>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExecuteStatus {
    ECALL,
    EBREAK,
    CONTINUE,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExecuteError {
    InvalidExecutionInstruction,
    MisalignedAddress,
    MemoryError(MemoryError),
}

impl From<MemoryError> for ExecuteError {
    fn from(err: MemoryError) -> ExecuteError {
        ExecuteError::MemoryError(err)
    }
}

type ExecuteResult<T> = Result<T, ExecuteError>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Instruction {
    LUI(u32, u32),
    AUIPC(u32, u32),
    JAL(u32, u32),
    JALR(u32, u32, u32),
    BEQ(u32, u32, u32),
    BNE(u32, u32, u32),
    BLT(u32, u32, u32),
    BGE(u32, u32, u32),
    BLTU(u32, u32, u32),
    BGEU(u32, u32, u32),
    LB(u32, u32, u32),
    LH(u32, u32, u32),
    LW(u32, u32, u32),
    LBU(u32, u32, u32),
    LHU(u32, u32, u32),
    SB(u32, u32, u32),
    SH(u32, u32, u32),
    SW(u32, u32, u32),
    ADDI(u32, u32, u32),
    SLTI(u32, u32, u32),
    SLTIU(u32, u32, u32),
    XORI(u32, u32, u32),
    ORI(u32, u32, u32),
    ANDI(u32, u32, u32),
    SLLI(u32, u32, u32),
    SRLI(u32, u32, u32),
    SRAI(u32, u32, u32),
    ADD(u32, u32, u32),
    SUB(u32, u32, u32),
    SLL(u32, u32, u32),
    SLT(u32, u32, u32),
    SLTU(u32, u32, u32),
    XOR(u32, u32, u32),
    SRL(u32, u32, u32),
    SRA(u32, u32, u32),
    OR(u32, u32, u32),
    AND(u32, u32, u32),
    FENCE(u32, u32),
    FENCE_I,
    ECALL,
    EBREAK,
    CSRRW(u32, u32, u32),
    CSRRS(u32, u32, u32),
    CSRRC(u32, u32, u32),
    CSRRWI(u32, u32, u32),
    CSRRSI(u32, u32, u32),
    CSRRCI(u32, u32, u32),
}

impl Instruction {
    fn extract_rtype(instr: u32) -> (u32, u32, u32, u32, u32) {
        let rd = (instr >> 7) & 0x1F;
        let funct3 = (instr >> 12) & 0x7;
        let rs1 = (instr >> 15) & 0x1F;
        let rs2 = (instr >> 20) & 0x1F;
        let funct7 = (instr >> 25) & 0x7F;

        (rd, funct3, rs1, rs2, funct7)
    }

    fn extract_itype(instr: u32) -> (u32, u32, u32, u32) {
        let rd = (instr >> 7) & 0x1F;
        let funct3 = (instr >> 12) & 0x7;
        let rs1 = (instr >> 15) & 0x1F;
        let mut imm = (instr >> 20);

        // This is our sign extension shifting
        if (imm & 0x800) != 0 {
            imm = imm | 0xFFFF_F000;
        }

        (rd, funct3, rs1, imm)
    }

    fn extract_stype(instr: u32) -> (u32, u32, u32, u32) {
        let funct3 = (instr >> 12) & 0x7;
        let rs1 = (instr >> 15) & 0x1F;
        let rs2 = (instr >> 20) & 0x1F;
        let mut imm = (instr >> 7) & 0x1F | (instr >> 20) & 0xFE0;

        // This is our sign extension shifting
        if (imm & 0x800) != 0 {
            imm = imm | 0xFFFF_F000;
        }

        (funct3, rs1, rs2, imm)
    }

    fn extract_btype(instr: u32) -> (u32, u32, u32, u32) {
        let funct3 = (instr >> 12) & 0x7;
        let rs1 = (instr >> 15) & 0x1F;
        let rs2 = (instr >> 20) & 0x1F;
        let mut imm = (instr >> 7) & 0x1E
            | (instr >> 20) & 0x7E0
            | (instr << 4) & 0x800
            | (instr >> 19) & 0x1000;

        // This is our sign extension shifting
        if (imm & 0x1000) != 0 {
            imm = imm | 0xFFFF_E000;
        }

        (funct3, rs1, rs2, imm)
    }

    /// Extract the BU type from the instruction.
    /// This ignores opcode and rd since they are extracted prior to this function.
    fn extract_butype(instr: u32) -> (u32, u32) {
        let rd = (instr >> 7) & 0x1F;
        let imm = instr & 0xFFFF_F000;

        (rd, imm)
    }

    fn extract_jtype(instr: u32) -> (u32, u32) {
        let rd = (instr >> 7) & 0x1F;
        let mut imm = (instr >> 20) & 0x7FE
            | (instr >> 9) & 0x800
            | (instr >> 0) & 0xF_F000
            | (instr >> 10) & 0x10_0000;

        // This is our sign extension shifting
        if (imm & 0x10_000) != 0 {
            imm = imm | 0xFFE0_0000;
        }

        (rd, imm)
    }

    fn extract_fence(instr: u32) -> (u32, u32, u32, u32, u32, u32) {
        let rd = (instr >> 7) & 0x1F;
        let funct3 = (instr >> 12) & 0x7;
        let rs1 = (instr >> 15) & 0x1F;
        let succ = (instr >> 20) & 0xF;
        let pred = (instr >> 24) & 0xF;
        let imm = (instr >> 28) & 0xF;

        (rd, funct3, rs1, succ, pred, imm)
    }

    pub fn decode(instr: u32) -> DecodeResult<Self> {
        let opcode = instr & 0x7F;

        match opcode {
            0b0110111 => {
                let (rd, imm) = Instruction::extract_butype(instr);
                Ok(Instruction::LUI(rd, imm))
            }
            0b0010111 => {
                let (rd, imm) = Instruction::extract_butype(instr);
                Ok(Instruction::AUIPC(rd, imm))
            }
            0b1101111 => {
                let (rd, imm) = Instruction::extract_jtype(instr);
                Ok(Instruction::JAL(rd, imm))
            }
            0b1100111 => {
                let (rd, funct3, rs1, imm) = Instruction::extract_itype(instr);
                match funct3 {
                    0b000 => Ok(Instruction::JALR(rd, rs1, imm)),
                    _ => Err(DecodeError::ITypeExtract(instr, opcode)),
                }
            }
            0b1100011 => {
                let (funct3, rs1, rs2, imm) = Instruction::extract_btype(instr);
                match funct3 {
                    0b000 => Ok(Instruction::BEQ(rs1, rs2, imm)),
                    0b001 => Ok(Instruction::BNE(rs1, rs2, imm)),
                    0b100 => Ok(Instruction::BLT(rs1, rs2, imm)),
                    0b101 => Ok(Instruction::BGE(rs1, rs2, imm)),
                    0b110 => Ok(Instruction::BLTU(rs1, rs2, imm)),
                    0b111 => Ok(Instruction::BGEU(rs1, rs2, imm)),
                    _ => Err(DecodeError::BTypeExtract(instr, opcode)),
                }
            }
            0b0000011 => {
                let (rd, funct3, rs1, imm) = Instruction::extract_itype(instr);
                match funct3 {
                    0b000 => Ok(Instruction::LB(rd, rs1, imm)),
                    0b001 => Ok(Instruction::LH(rd, rs1, imm)),
                    0b010 => Ok(Instruction::LW(rd, rs1, imm)),
                    0b100 => Ok(Instruction::LBU(rd, rs1, imm)),
                    0b101 => Ok(Instruction::LHU(rd, rs1, imm)),
                    _ => Err(DecodeError::ITypeExtract(instr, opcode)),
                }
            }
            0b0100011 => {
                let (funct3, rs1, rs2, imm) = Instruction::extract_stype(instr);
                match funct3 {
                    0b000 => Ok(Instruction::SB(rs1, rs2, imm)),
                    0b001 => Ok(Instruction::SH(rs1, rs2, imm)),
                    0b010 => Ok(Instruction::SW(rs1, rs2, imm)),
                    _ => Err(DecodeError::STypeExtract(instr, opcode)),
                }
            }
            0b0010011 => {
                let (rd, funct3, rs1, imm) = Instruction::extract_itype(instr);
                match funct3 {
                    0b000 => Ok(Instruction::ADDI(rd, rs1, imm)),
                    0b010 => Ok(Instruction::SLTI(rd, rs1, imm)),
                    0b011 => Ok(Instruction::SLTIU(rd, rs1, imm)),
                    0b100 => Ok(Instruction::XORI(rd, rs1, imm)),
                    0b110 => Ok(Instruction::ORI(rd, rs1, imm)),
                    0b111 => Ok(Instruction::ANDI(rd, rs1, imm)),
                    0b001 => Ok(Instruction::SLLI(rd, rs1, imm & 0x1F)), // This is a strange edgecase where it is an R-Type instruction but we extracted it as an I-type
                    0b101 => {
                        // These instructions we need funct7 in order to determine which instruction it is
                        // Double extraction is a waste here, we could make it a special implementation to slightly speed it up
                        let (rd, funct3, rs1, rs2, funct7) = Instruction::extract_rtype(instr);
                        match funct7 {
                            0b0000000 => Ok(Instruction::SRLI(rd, rs1, rs2)),
                            0b0100000 => Ok(Instruction::SRAI(rd, rs1, rs2)),
                            _ => Err(DecodeError::RTypeExtract(instr, opcode)),
                        }
                    }
                    _ => Err(DecodeError::ITypeExtract(instr, opcode)),
                }
            }
            0b0110011 => {
                let (rd, funct3, rs1, rs2, funct7) = Instruction::extract_rtype(instr);
                match funct3 {
                    0b000 => match funct7 {
                        0b0000000 => Ok(Instruction::ADD(rd, rs1, rs2)),
                        0b0100000 => Ok(Instruction::SUB(rd, rs1, rs2)),
                        _ => Err(DecodeError::RTypeExtract(instr, opcode)),
                    },
                    0b001 => Ok(Instruction::SLL(rd, rs1, rs2)),
                    0b010 => Ok(Instruction::SLT(rd, rs1, rs2)),
                    0b011 => Ok(Instruction::SLTU(rd, rs1, rs2)),
                    0b100 => Ok(Instruction::XOR(rd, rs1, rs2)),
                    0b101 => match funct7 {
                        0b0000000 => Ok(Instruction::SRL(rd, rs1, rs2)),
                        0b0100000 => Ok(Instruction::SRA(rd, rs1, rs2)),
                        _ => Err(DecodeError::RTypeExtract(instr, opcode)),
                    },
                    0b110 => Ok(Instruction::OR(rd, rs1, rs2)),
                    0b111 => Ok(Instruction::AND(rd, rs1, rs2)),
                    _ => Err(DecodeError::RTypeExtract(instr, opcode)),
                }
            }
            0b0001111 => {
                let (rd, funct3, rs1, succ, pred, imm) = Instruction::extract_fence(instr);
                // This may not be a valid error, but the specification specifically shows these as 0
                if rd != 0 || rs1 != 0 || imm != 0 {
                    return Err(DecodeError::InvalidFence(instr, opcode));
                }
                match funct3 {
                    0b000 => Ok(Instruction::FENCE(succ, pred)),
                    0b001 => Ok(Instruction::FENCE_I),
                    _ => Err(DecodeError::InvalidFence(instr, opcode)),
                }
            }
            0b1110011 => {
                let (rd, funct3, rs1, imm) = Instruction::extract_itype(instr);
                match funct3 {
                    0b000 => match imm {
                        0b0 => Ok(Instruction::ECALL),
                        0b1 => Ok(Instruction::EBREAK),
                        _ => Err(DecodeError::ITypeExtract(instr, opcode)),
                    },
                    0b001 => Ok(Instruction::CSRRW(rd, rs1, imm)),
                    0b010 => Ok(Instruction::CSRRS(rd, rs1, imm)),
                    0b011 => Ok(Instruction::CSRRC(rd, rs1, imm)),
                    0b101 => Ok(Instruction::CSRRWI(rd, rs1, imm)),
                    0b110 => Ok(Instruction::CSRRSI(rd, rs1, imm)),
                    0b111 => Ok(Instruction::CSRRCI(rd, rs1, imm)),
                    _ => Err(DecodeError::ITypeExtract(instr, opcode)),
                }
            }
            _ => Err(DecodeError::InvalidInstruction(instr, opcode)),
        }
    }

    fn lui(&self, cpu: &mut CPU) -> ExecuteResult<ExecuteStatus> {
        if let &Instruction::LUI(rd, imm) = self {
            if rd != 0 {
                cpu.get_registers()[rd as usize] = imm;
            }
        } else {
            return Err(ExecuteError::InvalidExecutionInstruction);
        }

        cpu.get_registers().increment_pc();
        Ok(ExecuteStatus::CONTINUE)
    }

    fn auipc(&self, cpu: &mut CPU) -> ExecuteResult<ExecuteStatus> {
        if let &Instruction::AUIPC(rd, imm) = self {
            if rd != 0 {
                let (value, overflow) = cpu.get_registers().get_pc().overflowing_add(imm);
                cpu.get_registers()[rd as usize] = value;
            }
        } else {
            return Err(ExecuteError::InvalidExecutionInstruction);
        }

        cpu.get_registers().increment_pc();
        Ok(ExecuteStatus::CONTINUE)
    }

    fn jal(&self, cpu: &mut CPU) -> ExecuteResult<ExecuteStatus> {
        if let &Instruction::JAL(rd, imm) = self {
            if rd != 0 {
                cpu.get_registers()[rd as usize] = cpu.get_registers().get_pc() + 4;
            }

            if imm % 4 != 0 {
                return Err(ExecuteError::MisalignedAddress);
            }

            cpu.get_registers().add_to_pc(imm);
        } else {
            return Err(ExecuteError::InvalidExecutionInstruction);
        }

        // TODO : This instruction should return a misasigned exception if the address % 4 != 0
        Ok(ExecuteStatus::CONTINUE)
    }

    fn jalr(&self, cpu: &mut CPU) -> ExecuteResult<ExecuteStatus> {
        if let &Instruction::JALR(rd, rs1, imm) = self {
            if rd != 0 {
                cpu.get_registers()[rd as usize] = cpu.get_registers().get_pc() + 4;
            }
            let (mut address, overflow) = cpu.get_registers()[rs1 as usize].overflowing_add(imm);
            address = address & 0xFFFF_FFFE;

            if address % 4 != 0 {
                return Err(ExecuteError::MisalignedAddress);
            }

            cpu.get_registers().set_pc(address);
        } else {
            return Err(ExecuteError::InvalidExecutionInstruction);
        }

        // TODO : This instruction should return a misasigned exception if the address % 4 != 0
        Ok(ExecuteStatus::CONTINUE)
    }

    fn beq(&self, cpu: &mut CPU) -> ExecuteResult<ExecuteStatus> {
        if let &Instruction::BEQ(rs1, rs2, imm) = self {
            if cpu.get_registers()[rs1 as usize] == cpu.get_registers()[rs2 as usize] {
                cpu.get_registers().add_to_pc(imm)
            } else {
                cpu.get_registers().increment_pc();
            }
        } else {
            return Err(ExecuteError::InvalidExecutionInstruction);
        }

        Ok(ExecuteStatus::CONTINUE)
    }

    fn bne(&self, cpu: &mut CPU) -> ExecuteResult<ExecuteStatus> {
        if let &Instruction::BNE(rs1, rs2, imm) = self {
            if cpu.get_registers()[rs1 as usize] != cpu.get_registers()[rs2 as usize] {
                cpu.get_registers().add_to_pc(imm)
            } else {
                cpu.get_registers().increment_pc();
            }
        } else {
            return Err(ExecuteError::InvalidExecutionInstruction);
        }

        Ok(ExecuteStatus::CONTINUE)
    }

    fn blt(&self, cpu: &mut CPU) -> ExecuteResult<ExecuteStatus> {
        if let &Instruction::BLT(rs1, rs2, imm) = self {
            if (cpu.get_registers()[rs1 as usize] as i32)
                < (cpu.get_registers()[rs2 as usize] as i32)
            {
                cpu.get_registers().add_to_pc(imm)
            } else {
                cpu.get_registers().increment_pc();
            }
        } else {
            return Err(ExecuteError::InvalidExecutionInstruction);
        }

        Ok(ExecuteStatus::CONTINUE)
    }

    fn bltu(&self, cpu: &mut CPU) -> ExecuteResult<ExecuteStatus> {
        if let &Instruction::BLTU(rs1, rs2, imm) = self {
            if cpu.get_registers()[rs1 as usize] < cpu.get_registers()[rs2 as usize] {
                cpu.get_registers().add_to_pc(imm)
            } else {
                cpu.get_registers().increment_pc();
            }
        } else {
            return Err(ExecuteError::InvalidExecutionInstruction);
        }

        Ok(ExecuteStatus::CONTINUE)
    }

    fn bge(&self, cpu: &mut CPU) -> ExecuteResult<ExecuteStatus> {
        if let &Instruction::BGE(rs1, rs2, imm) = self {
            if (cpu.get_registers()[rs1 as usize] as i32)
                >= (cpu.get_registers()[rs2 as usize] as i32)
            {
                cpu.get_registers().add_to_pc(imm)
            } else {
                cpu.get_registers().increment_pc();
            }
        } else {
            return Err(ExecuteError::InvalidExecutionInstruction);
        }

        Ok(ExecuteStatus::CONTINUE)
    }

    fn bgeu(&self, cpu: &mut CPU) -> ExecuteResult<ExecuteStatus> {
        if let &Instruction::BGEU(rs1, rs2, imm) = self {
            if cpu.get_registers()[rs1 as usize] >= cpu.get_registers()[rs2 as usize] {
                cpu.get_registers().add_to_pc(imm)
            } else {
                cpu.get_registers().increment_pc();
            }
        } else {
            return Err(ExecuteError::InvalidExecutionInstruction);
        }

        Ok(ExecuteStatus::CONTINUE)
    }

    fn lb(&self, cpu: &mut CPU) -> ExecuteResult<ExecuteStatus> {
        if let &Instruction::LB(rd, rs1, imm) = self {
            let (address, overflow) = cpu.get_registers()[rs1 as usize].overflowing_add(imm);
            let mut val: u32 = cpu.get_memory().read_byte(address)? as u32;
            if rd != 0 {
                if val & 0x80 != 0 {
                    val = (val as u32) | 0xFFFF_FF00;
                }
                cpu.get_registers()[rd as usize] = val;
            }
        } else {
            return Err(ExecuteError::InvalidExecutionInstruction);
        }

        cpu.get_registers().increment_pc();
        Ok(ExecuteStatus::CONTINUE)
    }

    fn lbu(&self, cpu: &mut CPU) -> ExecuteResult<ExecuteStatus> {
        if let &Instruction::LBU(rd, rs1, imm) = self {
            let (address, overflow) = cpu.get_registers()[rs1 as usize].overflowing_add(imm);
            let mut val: u32 = cpu.get_memory().read_byte(address)? as u32;
            if rd != 0 {
                val = val & 0x0000_00FF;
                cpu.get_registers()[rd as usize] = val;
            }
        } else {
            return Err(ExecuteError::InvalidExecutionInstruction);
        }

        cpu.get_registers().increment_pc();
        Ok(ExecuteStatus::CONTINUE)
    }

    fn lh(&self, cpu: &mut CPU) -> ExecuteResult<ExecuteStatus> {
        if let &Instruction::LH(rd, rs1, imm) = self {
            let (address, overflow) = cpu.get_registers()[rs1 as usize].overflowing_add(imm);
            let mut val: u32 = cpu.get_memory().read_halfword(address)? as u32;
            if rd != 0 {
                if val & 0x8000 != 0 {
                    val = (val as u32) | 0xFFFF_0000;
                }
                cpu.get_registers()[rd as usize] = val;
            }
        } else {
            return Err(ExecuteError::InvalidExecutionInstruction);
        }

        cpu.get_registers().increment_pc();
        Ok(ExecuteStatus::CONTINUE)
    }

    fn lhu(&self, cpu: &mut CPU) -> ExecuteResult<ExecuteStatus> {
        if let &Instruction::LHU(rd, rs1, imm) = self {
            let (address, overflow) = cpu.get_registers()[rs1 as usize].overflowing_add(imm);
            let mut val: u32 = cpu.get_memory().read_halfword(address)? as u32;
            if rd != 0 {
                val = val & 0x0000_FFFF;
                cpu.get_registers()[rd as usize] = val;
            }
        } else {
            return Err(ExecuteError::InvalidExecutionInstruction);
        }

        cpu.get_registers().increment_pc();
        Ok(ExecuteStatus::CONTINUE)
    }

    fn lw(&self, cpu: &mut CPU) -> ExecuteResult<ExecuteStatus> {
        if let &Instruction::LW(rd, rs1, imm) = self {
            let (address, overflow) = cpu.get_registers()[rs1 as usize].overflowing_add(imm);
            let val: u32 = cpu.get_memory().read_word(address)? as u32;
            if rd != 0 {
                cpu.get_registers()[rd as usize] = val;
            }
        } else {
            return Err(ExecuteError::InvalidExecutionInstruction);
        }

        cpu.get_registers().increment_pc();
        Ok(ExecuteStatus::CONTINUE)
    }

    fn sb(&self, cpu: &mut CPU) -> ExecuteResult<ExecuteStatus> {
        if let &Instruction::SB(rs1, rs2, imm) = self {
            let (address, overflow) = cpu.get_registers()[rs1 as usize].overflowing_add(imm);
            let val: u8 = (rs2 as u8) & 0xFF;
            cpu.get_memory().write_byte(address, val)?;
        } else {
            return Err(ExecuteError::InvalidExecutionInstruction);
        }

        cpu.get_registers().increment_pc();
        Ok(ExecuteStatus::CONTINUE)
    }

    fn sh(&self, cpu: &mut CPU) -> ExecuteResult<ExecuteStatus> {
        if let &Instruction::SH(rs1, rs2, imm) = self {
            let (address, overflow) = cpu.get_registers()[rs1 as usize].overflowing_add(imm);
            let val: u16 = (rs2 as u16) & 0xFFFF;
            cpu.get_memory().write_halfword(address, val)?;
        } else {
            return Err(ExecuteError::InvalidExecutionInstruction);
        }

        cpu.get_registers().increment_pc();
        Ok(ExecuteStatus::CONTINUE)
    }
    fn sw(&self, cpu: &mut CPU) -> ExecuteResult<ExecuteStatus> {
        if let &Instruction::SW(rs1, rs2, imm) = self {
            let (address, overflow) = cpu.get_registers()[rs1 as usize].overflowing_add(imm);
            let val: u32 = rs2 & 0xFFFF_FFFF;
            cpu.get_memory().write_word(address, val)?;
        } else {
            return Err(ExecuteError::InvalidExecutionInstruction);
        }

        cpu.get_registers().increment_pc();
        Ok(ExecuteStatus::CONTINUE)
    }

    fn addi(&self, cpu: &mut CPU) -> ExecuteResult<ExecuteStatus> {
        if let &Instruction::ADDI(rd, rs1, imm) = self {
            if rd != 0 {
                let (value, _) = cpu.get_registers()[rs1 as usize].overflowing_add(imm);
                cpu.get_registers()[rd as usize] = value;
            }
        } else {
            return Err(ExecuteError::InvalidExecutionInstruction);
        }

        cpu.get_registers().increment_pc();
        Ok(ExecuteStatus::CONTINUE)
    }

    fn slti(&self, cpu: &mut CPU) -> ExecuteResult<ExecuteStatus> {
        if let &Instruction::SLTI(rd, rs1, imm) = self {
            if rd != 0 {
                if (cpu.get_registers()[rs1 as usize] as i32) < (imm as i32) {
                    cpu.get_registers()[rd as usize] = 1
                } else {
                    cpu.get_registers()[rd as usize] = 0
                }
            }
        } else {
            return Err(ExecuteError::InvalidExecutionInstruction);
        }

        cpu.get_registers().increment_pc();
        Ok(ExecuteStatus::CONTINUE)
    }

    fn sltiu(&self, cpu: &mut CPU) -> ExecuteResult<ExecuteStatus> {
        if let &Instruction::SLTIU(rd, rs1, imm) = self {
            if rd != 0 {
                if cpu.get_registers()[rs1 as usize] < imm {
                    cpu.get_registers()[rd as usize] = 1
                } else {
                    cpu.get_registers()[rd as usize] = 0
                }
            }
        } else {
            return Err(ExecuteError::InvalidExecutionInstruction);
        }

        cpu.get_registers().increment_pc();
        Ok(ExecuteStatus::CONTINUE)
    }

    fn xori(&self, cpu: &mut CPU) -> ExecuteResult<ExecuteStatus> {
        if let &Instruction::XORI(rd, rs1, imm) = self {
            if rd != 0 {
                cpu.get_registers()[rd as usize] = cpu.get_registers()[rs1 as usize] ^ imm;
            }
        } else {
            return Err(ExecuteError::InvalidExecutionInstruction);
        }

        cpu.get_registers().increment_pc();
        Ok(ExecuteStatus::CONTINUE)
    }

    fn ori(&self, cpu: &mut CPU) -> ExecuteResult<ExecuteStatus> {
        if let &Instruction::ORI(rd, rs1, imm) = self {
            if rd != 0 {
                cpu.get_registers()[rd as usize] = cpu.get_registers()[rs1 as usize] | imm;
            }
        } else {
            return Err(ExecuteError::InvalidExecutionInstruction);
        }

        cpu.get_registers().increment_pc();
        Ok(ExecuteStatus::CONTINUE)
    }

    fn andi(&self, cpu: &mut CPU) -> ExecuteResult<ExecuteStatus> {
        if let &Instruction::ANDI(rd, rs1, imm) = self {
            if rd != 0 {
                cpu.get_registers()[rd as usize] = cpu.get_registers()[rs1 as usize] & imm;
            }
        } else {
            return Err(ExecuteError::InvalidExecutionInstruction);
        }

        cpu.get_registers().increment_pc();
        Ok(ExecuteStatus::CONTINUE)
    }

    fn slli(&self, cpu: &mut CPU) -> ExecuteResult<ExecuteStatus> {
        if let &Instruction::SLLI(rd, rs1, imm) = self {
            if rd != 0 {
                cpu.get_registers()[rd as usize] = cpu.get_registers()[rs1 as usize] << imm;
            }
        } else {
            return Err(ExecuteError::InvalidExecutionInstruction);
        }

        cpu.get_registers().increment_pc();
        Ok(ExecuteStatus::CONTINUE)
    }

    fn srli(&self, cpu: &mut CPU) -> ExecuteResult<ExecuteStatus> {
        if let &Instruction::SRLI(rd, rs1, imm) = self {
            if rd != 0 {
                cpu.get_registers()[rd as usize] = cpu.get_registers()[rs1 as usize] >> imm;
            }
        } else {
            return Err(ExecuteError::InvalidExecutionInstruction);
        }

        cpu.get_registers().increment_pc();
        Ok(ExecuteStatus::CONTINUE)
    }

    fn srai(&self, cpu: &mut CPU) -> ExecuteResult<ExecuteStatus> {
        if let &Instruction::SRAI(rd, rs1, imm) = self {
            if rd != 0 {
                cpu.get_registers()[rd as usize] =
                    ((cpu.get_registers()[rs1 as usize] as i32) >> (imm as i32)) as u32;
            }
        } else {
            return Err(ExecuteError::InvalidExecutionInstruction);
        }

        cpu.get_registers().increment_pc();
        Ok(ExecuteStatus::CONTINUE)
    }

    fn add(&self, cpu: &mut CPU) -> ExecuteResult<ExecuteStatus> {
        if let &Instruction::ADD(rd, rs1, rs2) = self {
            if rd != 0 {
                let (value, overflow) = cpu.get_registers()[rs1 as usize]
                    .overflowing_add(cpu.get_registers()[rs2 as usize]);
                cpu.get_registers()[rd as usize] = value;
            }
        } else {
            return Err(ExecuteError::InvalidExecutionInstruction);
        }

        cpu.get_registers().increment_pc();
        Ok(ExecuteStatus::CONTINUE)
    }

    fn sub(&self, cpu: &mut CPU) -> ExecuteResult<ExecuteStatus> {
        if let &Instruction::SUB(rd, rs1, rs2) = self {
            if rd != 0 {
                let (value, overflow) = cpu.get_registers()[rs1 as usize]
                    .overflowing_sub(cpu.get_registers()[rs2 as usize]);
                cpu.get_registers()[rd as usize] = value;
            }
        } else {
            return Err(ExecuteError::InvalidExecutionInstruction);
        }

        cpu.get_registers().increment_pc();
        Ok(ExecuteStatus::CONTINUE)
    }

    fn sll(&self, cpu: &mut CPU) -> ExecuteResult<ExecuteStatus> {
        if let &Instruction::SLL(rd, rs1, rs2) = self {
            if rd != 0 {
                cpu.get_registers()[rd as usize] =
                    cpu.get_registers()[rs1 as usize] << (cpu.get_registers()[rs2 as usize] & 0x1F);
            }
        } else {
            return Err(ExecuteError::InvalidExecutionInstruction);
        }

        cpu.get_registers().increment_pc();
        Ok(ExecuteStatus::CONTINUE)
    }

    fn slt(&self, cpu: &mut CPU) -> ExecuteResult<ExecuteStatus> {
        if let &Instruction::SLT(rd, rs1, rs2) = self {
            if rd != 0 {
                if (cpu.get_registers()[rs1 as usize] as i32)
                    < (cpu.get_registers()[rs2 as usize] as i32)
                {
                    cpu.get_registers()[rd as usize] = 1;
                } else {
                    cpu.get_registers()[rd as usize] = 0;
                }
            }
        } else {
            return Err(ExecuteError::InvalidExecutionInstruction);
        }

        cpu.get_registers().increment_pc();
        Ok(ExecuteStatus::CONTINUE)
    }

    fn sltu(&self, cpu: &mut CPU) -> ExecuteResult<ExecuteStatus> {
        if let &Instruction::SLTU(rd, rs1, rs2) = self {
            if rd != 0 {
                if cpu.get_registers()[rs1 as usize] < cpu.get_registers()[rs2 as usize] {
                    cpu.get_registers()[rd as usize] = 1;
                } else {
                    cpu.get_registers()[rd as usize] = 0;
                }
            }
        } else {
            return Err(ExecuteError::InvalidExecutionInstruction);
        }

        cpu.get_registers().increment_pc();
        Ok(ExecuteStatus::CONTINUE)
    }

    fn xor(&self, cpu: &mut CPU) -> ExecuteResult<ExecuteStatus> {
        if let &Instruction::XOR(rd, rs1, rs2) = self {
            if rd != 0 {
                cpu.get_registers()[rd as usize] =
                    cpu.get_registers()[rs1 as usize] ^ cpu.get_registers()[rs2 as usize];
            }
        } else {
            return Err(ExecuteError::InvalidExecutionInstruction);
        }

        cpu.get_registers().increment_pc();
        Ok(ExecuteStatus::CONTINUE)
    }

    fn srl(&self, cpu: &mut CPU) -> ExecuteResult<ExecuteStatus> {
        if let &Instruction::SRL(rd, rs1, rs2) = self {
            if rd != 0 {
                cpu.get_registers()[rd as usize] =
                    cpu.get_registers()[rs1 as usize] >> (cpu.get_registers()[rs2 as usize] & 0x1F);
            }
        } else {
            return Err(ExecuteError::InvalidExecutionInstruction);
        }

        cpu.get_registers().increment_pc();
        Ok(ExecuteStatus::CONTINUE)
    }

    fn sra(&self, cpu: &mut CPU) -> ExecuteResult<ExecuteStatus> {
        if let &Instruction::SRA(rd, rs1, rs2) = self {
            if rd != 0 {
                cpu.get_registers()[rd as usize] = ((cpu.get_registers()[rs1 as usize] as i32)
                    >> ((cpu.get_registers()[rs2 as usize] & 0x1F) as i32))
                    as u32;
            }
        } else {
            return Err(ExecuteError::InvalidExecutionInstruction);
        }

        cpu.get_registers().increment_pc();
        Ok(ExecuteStatus::CONTINUE)
    }

    fn or(&self, cpu: &mut CPU) -> ExecuteResult<ExecuteStatus> {
        if let &Instruction::OR(rd, rs1, rs2) = self {
            if rd != 0 {
                cpu.get_registers()[rd as usize] =
                    cpu.get_registers()[rs1 as usize] | cpu.get_registers()[rs2 as usize];
            }
        } else {
            return Err(ExecuteError::InvalidExecutionInstruction);
        }

        cpu.get_registers().increment_pc();
        Ok(ExecuteStatus::CONTINUE)
    }

    fn and(&self, cpu: &mut CPU) -> ExecuteResult<ExecuteStatus> {
        if let &Instruction::AND(rd, rs1, rs2) = self {
            if rd != 0 {
                cpu.get_registers()[rd as usize] =
                    cpu.get_registers()[rs1 as usize] & cpu.get_registers()[rs2 as usize];
            }
        } else {
            return Err(ExecuteError::InvalidExecutionInstruction);
        }

        cpu.get_registers().increment_pc();
        Ok(ExecuteStatus::CONTINUE)
    }

    fn fence(&self, cpu: &mut CPU) -> ExecuteResult<ExecuteStatus> {
        if let &Instruction::FENCE(succ, pred) = self {
            //TODO
            println!("FENCE");
        } else {
            return Err(ExecuteError::InvalidExecutionInstruction);
        }

        cpu.get_registers().increment_pc();
        Ok(ExecuteStatus::CONTINUE)
    }

    fn fence_i(&self, cpu: &mut CPU) -> ExecuteResult<ExecuteStatus> {
        if let &Instruction::FENCE_I = self {
            //TODO
            println!("FENCE.I");
        } else {
            return Err(ExecuteError::InvalidExecutionInstruction);
        }

        cpu.get_registers().increment_pc();
        Ok(ExecuteStatus::CONTINUE)
    }

    fn ecall(&self, cpu: &mut CPU) -> ExecuteResult<ExecuteStatus> {
        if let &Instruction::ECALL = self {
            //TODO
            println!("ECALL");
        } else {
            return Err(ExecuteError::InvalidExecutionInstruction);
        }

        cpu.get_registers().increment_pc();
        Ok(ExecuteStatus::ECALL)
    }

    fn ebreak(&self, cpu: &mut CPU) -> ExecuteResult<ExecuteStatus> {
        if let &Instruction::EBREAK = self {
            //TODO
            println!("EBREAK");
        } else {
            return Err(ExecuteError::InvalidExecutionInstruction);
        }

        cpu.get_registers().increment_pc();
        Ok(ExecuteStatus::EBREAK)
    }

    fn csrrw(&self, cpu: &mut CPU) -> ExecuteResult<ExecuteStatus> {
        if let &Instruction::CSRRW(rd, rs1, csr) = self {
            //TODO
            println!("CSRRW");
        } else {
            return Err(ExecuteError::InvalidExecutionInstruction);
        }

        cpu.get_registers().increment_pc();
        Ok(ExecuteStatus::CONTINUE)
    }

    fn csrrs(&self, cpu: &mut CPU) -> ExecuteResult<ExecuteStatus> {
        if let &Instruction::CSRRS(rd, rs1, csr) = self {
            //TODO
            println!("CSRRS");
        } else {
            return Err(ExecuteError::InvalidExecutionInstruction);
        }

        cpu.get_registers().increment_pc();
        Ok(ExecuteStatus::CONTINUE)
    }

    fn csrrc(&self, cpu: &mut CPU) -> ExecuteResult<ExecuteStatus> {
        if let &Instruction::CSRRC(rd, rs1, csr) = self {
            //TODO
            println!("CSRRC");
        } else {
            return Err(ExecuteError::InvalidExecutionInstruction);
        }

        cpu.get_registers().increment_pc();
        Ok(ExecuteStatus::CONTINUE)
    }

    fn csrrwi(&self, cpu: &mut CPU) -> ExecuteResult<ExecuteStatus> {
        if let &Instruction::CSRRWI(rd, rs1, csr) = self {
            //TODO
            println!("CSRRWI");
        } else {
            return Err(ExecuteError::InvalidExecutionInstruction);
        }

        cpu.get_registers().increment_pc();
        Ok(ExecuteStatus::CONTINUE)
    }

    fn csrrsi(&self, cpu: &mut CPU) -> ExecuteResult<ExecuteStatus> {
        if let &Instruction::CSRRSI(rd, rs1, csr) = self {
            //TODO
            println!("CSRRSI");
        } else {
            return Err(ExecuteError::InvalidExecutionInstruction);
        }

        cpu.get_registers().increment_pc();
        Ok(ExecuteStatus::CONTINUE)
    }

    fn csrrci(&self, cpu: &mut CPU) -> ExecuteResult<ExecuteStatus> {
        if let &Instruction::CSRRCI(rd, rs1, csr) = self {
            //TODO
            println!("CSRRCI");
        } else {
            return Err(ExecuteError::InvalidExecutionInstruction);
        }

        cpu.get_registers().increment_pc();
        Ok(ExecuteStatus::CONTINUE)
    }
}

use super::cpu::CPU;
pub trait Executable {
    fn execute(&self, cpu: &mut CPU) -> ExecuteResult<ExecuteStatus>;
}

impl Executable for Instruction {
    fn execute(&self, cpu: &mut CPU) -> ExecuteResult<ExecuteStatus> {
        match self {
            &Instruction::LUI(_, _) => self.lui(cpu),
            &Instruction::AUIPC(_, _) => self.auipc(cpu),
            &Instruction::JAL(_, _) => self.jal(cpu),
            &Instruction::JALR(_, _, _) => self.jalr(cpu),
            &Instruction::BEQ(_, _, _) => self.beq(cpu),
            &Instruction::BNE(_, _, _) => self.bne(cpu),
            &Instruction::BLT(_, _, _) => self.blt(cpu),
            &Instruction::BGE(_, _, _) => self.bge(cpu),
            &Instruction::BLTU(_, _, _) => self.bltu(cpu),
            &Instruction::BGEU(_, _, _) => self.bgeu(cpu),
            &Instruction::LB(_, _, _) => self.lb(cpu),
            &Instruction::LH(_, _, _) => self.lh(cpu),
            &Instruction::LW(_, _, _) => self.lw(cpu),
            &Instruction::LBU(_, _, _) => self.lbu(cpu),
            &Instruction::LHU(_, _, _) => self.lhu(cpu),
            &Instruction::SB(_, _, _) => self.sb(cpu),
            &Instruction::SH(_, _, _) => self.sh(cpu),
            &Instruction::SW(_, _, _) => self.sw(cpu),
            &Instruction::ADDI(_, _, _) => self.addi(cpu),
            &Instruction::SLTI(_, _, _) => self.slti(cpu),
            &Instruction::SLTIU(_, _, _) => self.sltiu(cpu),
            &Instruction::XORI(_, _, _) => self.xori(cpu),
            &Instruction::ORI(_, _, _) => self.ori(cpu),
            &Instruction::ANDI(_, _, _) => self.andi(cpu),
            &Instruction::SLLI(_, _, _) => self.slli(cpu),
            &Instruction::SRLI(_, _, _) => self.srli(cpu),
            &Instruction::SRAI(_, _, _) => self.srai(cpu),
            &Instruction::ADD(_, _, _) => self.add(cpu),
            &Instruction::SUB(_, _, _) => self.sub(cpu),
            &Instruction::SLL(_, _, _) => self.sll(cpu),
            &Instruction::SLT(_, _, _) => self.slt(cpu),
            &Instruction::SLTU(_, _, _) => self.sltu(cpu),
            &Instruction::XOR(_, _, _) => self.xor(cpu),
            &Instruction::SRL(_, _, _) => self.srl(cpu),
            &Instruction::SRA(_, _, _) => self.sra(cpu),
            &Instruction::OR(_, _, _) => self.or(cpu),
            &Instruction::AND(_, _, _) => self.and(cpu),
            &Instruction::FENCE(_, _) => self.fence(cpu),
            &Instruction::FENCE_I => self.fence_i(cpu),
            &Instruction::ECALL => self.ecall(cpu),
            &Instruction::EBREAK => self.ebreak(cpu),
            &Instruction::CSRRW(_, _, _) => self.csrrw(cpu),
            &Instruction::CSRRS(_, _, _) => self.csrrs(cpu),
            &Instruction::CSRRC(_, _, _) => self.csrrc(cpu),
            &Instruction::CSRRWI(_, _, _) => self.csrrwi(cpu),
            &Instruction::CSRRSI(_, _, _) => self.csrrsi(cpu),
            &Instruction::CSRRCI(_, _, _) => self.csrrci(cpu),
            _ => Err(ExecuteError::InvalidExecutionInstruction),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addi() {
        let instr = Instruction::decode(0x7FF00193);
        assert_eq!(instr.unwrap(), Instruction::ADDI(3, 0, 2047))
    }

    #[test]
    fn addi2() {
        let instr = Instruction::decode(0x01400F13);
        assert_eq!(instr.unwrap(), Instruction::ADDI(30, 0, 20))
    }

    #[test]
    fn jal() {
        let instr = Instruction::decode(0x01000FEF);
        assert_eq!(instr.unwrap(), Instruction::JAL(31, 16))
    }
}
