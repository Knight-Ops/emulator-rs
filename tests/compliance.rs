use emulator_rs::frontend::cpu::CPUError::DecodeError;
use emulator_rs::frontend::DecodeError::InvalidInstruction;

#[test]
fn test_add() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    cpu.load_image("tests/rv32i-compliance/add.bin".to_string());
    cpu.get_registers().set_pc(0x140);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_addi() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    cpu.load_image("tests/rv32i-compliance/addi.bin".to_string());
    cpu.get_registers().set_pc(0x140);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_and() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    cpu.load_image("tests/rv32i-compliance/and.bin".to_string());
    cpu.get_registers().set_pc(0x140);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_andi() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    cpu.load_image("tests/rv32i-compliance/andi.bin".to_string());
    cpu.get_registers().set_pc(0x140);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_auipc() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    cpu.load_image("tests/rv32i-compliance/auipc.bin".to_string());
    cpu.get_registers().set_pc(0x140);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_beq() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    cpu.load_image("tests/rv32i-compliance/beq.bin".to_string());
    cpu.get_registers().set_pc(0x140);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_bge() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    cpu.load_image("tests/rv32i-compliance/bge.bin".to_string());
    cpu.get_registers().set_pc(0x140);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_bgeu() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    cpu.load_image("tests/rv32i-compliance/bgeu.bin".to_string());
    cpu.get_registers().set_pc(0x140);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_blt() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    cpu.load_image("tests/rv32i-compliance/blt.bin".to_string());
    cpu.get_registers().set_pc(0x140);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_bltu() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    cpu.load_image("tests/rv32i-compliance/bltu.bin".to_string());
    cpu.get_registers().set_pc(0x140);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_bne() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    cpu.load_image("tests/rv32i-compliance/bne.bin".to_string());
    cpu.get_registers().set_pc(0x140);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_fence_i() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    cpu.load_image("tests/rv32i-compliance/fence_i.bin".to_string());
    cpu.get_registers().set_pc(0x198);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_jal() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    cpu.load_image("tests/rv32i-compliance/jal.bin".to_string());
    cpu.get_registers().set_pc(0x140);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_jalr() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    cpu.load_image("tests/rv32i-compliance/jalr.bin".to_string());
    cpu.get_registers().set_pc(0x140);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_lb() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    cpu.load_image("tests/rv32i-compliance/lb.bin".to_string());
    cpu.get_registers().set_pc(0x140);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_lbu() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    cpu.load_image("tests/rv32i-compliance/lbu.bin".to_string());
    cpu.get_registers().set_pc(0x140);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_lh() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    cpu.load_image("tests/rv32i-compliance/lh.bin".to_string());
    cpu.get_registers().set_pc(0x140);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_lhu() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    cpu.load_image("tests/rv32i-compliance/lhu.bin".to_string());
    cpu.get_registers().set_pc(0x140);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_lui() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    cpu.load_image("tests/rv32i-compliance/lui.bin".to_string());
    cpu.get_registers().set_pc(0x140);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_lw() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    cpu.load_image("tests/rv32i-compliance/lw.bin".to_string());
    cpu.get_registers().set_pc(0x140);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_or() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    cpu.load_image("tests/rv32i-compliance/or.bin".to_string());
    cpu.get_registers().set_pc(0x140);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_ori() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    cpu.load_image("tests/rv32i-compliance/ori.bin".to_string());
    cpu.get_registers().set_pc(0x140);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_sb() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    cpu.load_image("tests/rv32i-compliance/sb.bin".to_string());
    cpu.get_registers().set_pc(0x140);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_sh() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    cpu.load_image("tests/rv32i-compliance/sh.bin".to_string());
    cpu.get_registers().set_pc(0x140);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_simple() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    cpu.load_image("tests/rv32i-compliance/simple.bin".to_string());
    cpu.get_registers().set_pc(0x140);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_sll() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    cpu.load_image("tests/rv32i-compliance/sll.bin".to_string());
    cpu.get_registers().set_pc(0x140);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_slli() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    cpu.load_image("tests/rv32i-compliance/slli.bin".to_string());
    cpu.get_registers().set_pc(0x140);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_slt() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    cpu.load_image("tests/rv32i-compliance/slt.bin".to_string());
    cpu.get_registers().set_pc(0x140);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_slti() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    cpu.load_image("tests/rv32i-compliance/slti.bin".to_string());
    cpu.get_registers().set_pc(0x140);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_sltiu() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    cpu.load_image("tests/rv32i-compliance/sltiu.bin".to_string());
    cpu.get_registers().set_pc(0x140);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_sltu() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    cpu.load_image("tests/rv32i-compliance/sltu.bin".to_string());
    cpu.get_registers().set_pc(0x140);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_sra() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    cpu.load_image("tests/rv32i-compliance/sra.bin".to_string());
    cpu.get_registers().set_pc(0x140);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_srai() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    cpu.load_image("tests/rv32i-compliance/srai.bin".to_string());
    cpu.get_registers().set_pc(0x140);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_srl() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    cpu.load_image("tests/rv32i-compliance/srl.bin".to_string());
    cpu.get_registers().set_pc(0x140);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_srli() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    cpu.load_image("tests/rv32i-compliance/srli.bin".to_string());
    cpu.get_registers().set_pc(0x140);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_sub() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    cpu.load_image("tests/rv32i-compliance/sub.bin".to_string());
    cpu.get_registers().set_pc(0x140);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_sw() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    cpu.load_image("tests/rv32i-compliance/sw.bin".to_string());
    cpu.get_registers().set_pc(0x140);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_xor() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    cpu.load_image("tests/rv32i-compliance/xor.bin".to_string());
    cpu.get_registers().set_pc(0x140);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}

#[test]
fn test_xori() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0, 16384);
    cpu.load_image("tests/rv32i-compliance/xori.bin".to_string());
    cpu.get_registers().set_pc(0x140);
    println!("CPU : {:X?}", cpu);
    cpu.run().unwrap();
    assert_eq!(cpu.get_registers()[10], 0);
}
