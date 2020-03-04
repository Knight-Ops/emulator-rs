use emulator_rs::frontend::cpu::CPUError::DecodeError;
use emulator_rs::frontend::DecodeError::InvalidInstruction;

#[test]
fn test_add() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    let entry_point = cpu.load_elf("tests/rv32i-compliance/add".to_string());
    cpu.get_registers().set_pc(entry_point + 0x100);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_addi() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    let entry_point = cpu.load_elf("tests/rv32i-compliance/addi".to_string());
    cpu.get_registers().set_pc(entry_point+0x100);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_and() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    let entry_point = cpu.load_elf("tests/rv32i-compliance/and".to_string());
    cpu.get_registers().set_pc(entry_point+0x100);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_andi() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    let entry_point = cpu.load_elf("tests/rv32i-compliance/andi".to_string());
    cpu.get_registers().set_pc(entry_point+0x100);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_auipc() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    let entry_point = cpu.load_elf("tests/rv32i-compliance/auipc".to_string());
    cpu.get_registers().set_pc(entry_point+0x100);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_beq() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    let entry_point = cpu.load_elf("tests/rv32i-compliance/beq".to_string());
    cpu.get_registers().set_pc(entry_point+0x100);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_bge() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    let entry_point = cpu.load_elf("tests/rv32i-compliance/bge".to_string());
    cpu.get_registers().set_pc(entry_point+0x100);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_bgeu() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    let entry_point = cpu.load_elf("tests/rv32i-compliance/bgeu".to_string());
    cpu.get_registers().set_pc(entry_point+0x100);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_blt() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    let entry_point = cpu.load_elf("tests/rv32i-compliance/blt".to_string());
    cpu.get_registers().set_pc(entry_point+0x100);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_bltu() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    let entry_point = cpu.load_elf("tests/rv32i-compliance/bltu".to_string());
    cpu.get_registers().set_pc(entry_point+0x100);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_bne() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    let entry_point = cpu.load_elf("tests/rv32i-compliance/bne".to_string());
    cpu.get_registers().set_pc(entry_point+0x100);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_fence_i() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    let entry_point = cpu.load_elf("tests/rv32i-compliance/fence_i".to_string());
    cpu.get_registers().set_pc(entry_point+0x100);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_jal() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    let entry_point = cpu.load_elf("tests/rv32i-compliance/jal".to_string());
    cpu.get_registers().set_pc(entry_point+0x100);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_jalr() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    let entry_point = cpu.load_elf("tests/rv32i-compliance/jalr".to_string());
    cpu.get_registers().set_pc(entry_point+0x100);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_lb() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    let entry_point = cpu.load_elf("tests/rv32i-compliance/lb".to_string());
    cpu.get_registers().set_pc(entry_point+0x100);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_lbu() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    let entry_point = cpu.load_elf("tests/rv32i-compliance/lbu".to_string());
    cpu.get_registers().set_pc(entry_point+0x100);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_lh() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    let entry_point = cpu.load_elf("tests/rv32i-compliance/lh".to_string());
    cpu.get_registers().set_pc(entry_point+0x100);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_lhu() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    let entry_point = cpu.load_elf("tests/rv32i-compliance/lhu".to_string());
    cpu.get_registers().set_pc(entry_point+0x100);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_lui() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    let entry_point = cpu.load_elf("tests/rv32i-compliance/lui".to_string());
    cpu.get_registers().set_pc(entry_point+0x100);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_lw() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    let entry_point = cpu.load_elf("tests/rv32i-compliance/lw".to_string());
    cpu.get_registers().set_pc(entry_point+0x100);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_or() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    let entry_point = cpu.load_elf("tests/rv32i-compliance/or".to_string());
    cpu.get_registers().set_pc(entry_point+0x100);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_ori() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    let entry_point = cpu.load_elf("tests/rv32i-compliance/ori".to_string());
    cpu.get_registers().set_pc(entry_point+0x100);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_sb() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    let entry_point = cpu.load_elf("tests/rv32i-compliance/sb".to_string());
    cpu.get_registers().set_pc(entry_point+0x100);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_sh() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    let entry_point = cpu.load_elf("tests/rv32i-compliance/sh".to_string());
    cpu.get_registers().set_pc(entry_point+0x100);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_simple() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    let entry_point = cpu.load_elf("tests/rv32i-compliance/simple".to_string());
    cpu.get_registers().set_pc(entry_point+0x100);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_sll() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    let entry_point = cpu.load_elf("tests/rv32i-compliance/sll".to_string());
    cpu.get_registers().set_pc(entry_point+0x100);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_slli() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    let entry_point = cpu.load_elf("tests/rv32i-compliance/slli".to_string());
    cpu.get_registers().set_pc(entry_point+0x100);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_slt() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    let entry_point = cpu.load_elf("tests/rv32i-compliance/slt".to_string());
    cpu.get_registers().set_pc(entry_point+0x100);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_slti() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    let entry_point = cpu.load_elf("tests/rv32i-compliance/slti".to_string());
    cpu.get_registers().set_pc(entry_point+0x100);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_sltiu() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    let entry_point = cpu.load_elf("tests/rv32i-compliance/sltiu".to_string());
    cpu.get_registers().set_pc(entry_point+0x100);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_sltu() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    let entry_point = cpu.load_elf("tests/rv32i-compliance/sltu".to_string());
    cpu.get_registers().set_pc(entry_point+0x100);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_sra() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    let entry_point = cpu.load_elf("tests/rv32i-compliance/sra".to_string());
    cpu.get_registers().set_pc(entry_point+0x100);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_srai() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    let entry_point = cpu.load_elf("tests/rv32i-compliance/srai".to_string());
    cpu.get_registers().set_pc(entry_point+0x100);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_srl() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    let entry_point = cpu.load_elf("tests/rv32i-compliance/srl".to_string());
    cpu.get_registers().set_pc(entry_point+0x100);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_srli() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    let entry_point = cpu.load_elf("tests/rv32i-compliance/srli".to_string());
    cpu.get_registers().set_pc(entry_point+0x100);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_sub() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    let entry_point = cpu.load_elf("tests/rv32i-compliance/sub".to_string());
    cpu.get_registers().set_pc(entry_point+0x100);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_sw() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    let entry_point = cpu.load_elf("tests/rv32i-compliance/sw".to_string());
    cpu.get_registers().set_pc(entry_point+0x100);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_xor() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    let entry_point = cpu.load_elf("tests/rv32i-compliance/xor".to_string());
    cpu.get_registers().set_pc(entry_point+0x100);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_xori() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    let entry_point = cpu.load_elf("tests/rv32i-compliance/xori".to_string());
    cpu.get_registers().set_pc(entry_point+0x100);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}
