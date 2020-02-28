use super::instruction_formats::InstructionFormats;

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

    /// Extract the BU type from the instruction.
    /// This ignores opcode and rd since they are extracted prior to this function.
    fn extract_butype(instr: u32) -> (u32, u32) {
        let rd = (instr >> 7) & 0x1F;
        let imm = instr & 0xFFFF_F000;

        (rd, imm)
    }

    fn extract_itype(instr: u32) -> (u32, u32, u32, u32) {
        let rd = (instr >> 7) & 0x1F;
        let funct3 = (instr >> 12) & 0x7;
        let rs1 = (instr >> 15) & 0x1F;
        let imm = (instr >> 20);

        (rd, funct3, rs1, imm)
    }

    fn extract_btype(instr: u32) -> (u32, u32, u32, u32) {
        let funct3 = (instr >> 12) & 0x7;
        let rs1 = (instr >> 15) & 0x1F;
        let rs2 = (instr >> 20) & 0x1F;
        let imm = (instr >> 7) & 0x1E |
                    (instr >> 20) & 0x7E0 |
                    (instr << 4) & 0x800 |
                    (instr >> 19) & 0x1000;

        (funct3, rs1, rs2, imm)
    }

    pub fn decode(instr: u32) -> Self {
        let opcode = instr & 0x7F;
        

        let instr_type = match opcode {
            0b0110111 => {
                let (rd, imm) = Instruction::extract_butype(instr);
                Instruction::LUI(rd, imm)
            },
            0b1101111 => {
                let (rd, imm) = Instruction::extract_butype(instr);
                Instruction::AUIPC(rd, imm)
            },
            0b1100111 => {
                let (rd, funct3, rs1, imm) = Instruction::extract_itype(instr);
                match funct3 {
                    0b000 => Instruction::JALR(rd, rs1, imm),
                    _ => panic!("Invalid I-Type Instruction while decoding Opcode : {:b}", opcode)
                }
            },
            0b1100011 => {
                let (funct3, rs1, rs2, imm) = Instruction::extract_btype(instr);
                match funct3 {
                    0b000 => Instruction::BEQ(rs1, rs2, imm),
                    0b001 => Instruction::BNE(rs1, rs2, imm),
                    0b100 => Instruction::BLT(rs1, rs2, imm),
                    0b101 => Instruction::BGE(rs1, rs2, imm),
                    0b110 => Instruction::BLTU(rs1, rs2, imm),
                    0b111 => Instruction::BGEU(rs1, rs2, imm),
                    _ => panic!("Invalid B-Type Instruction while decoding Opcode : {:b}", opcode)
                }
            },
            0b0000011 => {
                let (rd, funct3, rs1, imm) = Instruction::extract_itype(instr);
                match funct3 {
                    0b000 => Instruction::LB(rd, rs1, imm),
                    0b010 => Instruction::LH(rd, rs1, imm),
                    0b010 => Instruction::LW(rd, rs1, imm),
                    0b100 => Instruction::LBU(rd, rs1, imm),
                    0b101 => Instruction::LHU(rd, rs1, imm),
                    _ => panic!("Invalid I-Type Instruction while decoding Opcode : {:b}", opcode)
                }
            },
            0b0100011 => Instruction::FENCE_I,
            0b0010011 => {
                let (rd, funct3, rs1, imm) = Instruction::extract_itype(instr);
                match funct3 {
                    0b000 => Instruction::ADDI(rd, rs1, imm),
                    0b010 => Instruction::SLTI(rd, rs1, imm),
                    0b011 => Instruction::SLTIU(rd, rs1, imm),
                    0b100 => Instruction::XORI(rd, rs1, imm),
                    0b110 => Instruction::ORI(rd, rs1, imm),
                    0b111 => Instruction::ANDI(rd, rs1, imm),
                    _ => panic!("Invalid I-Type Instruction while decoding Opcode : {:b}", opcode)
                }
            },
            _ => {
                panic!("Unreachable!")
            }
        };
        println!("Opcode is : {:b}|{:X}", opcode, opcode);
        instr_type
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let instr = Instruction::decode(0x7FF00193);
        assert_eq!(instr, Instruction::ADDI(3, 0, 2047))
    }
}