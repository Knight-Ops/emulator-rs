use emulator_rs;

fn main() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(4096);
    cpu.load_image("tests/compliance-tests/add.bin".to_string());
    cpu.get_registers().set_pc(0x140);
    println!("{:?}", cpu);
    cpu.run();
}