use emulator_rs;

fn main() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(4096);
    cpu.load_image("/home/carl/git/riscv-tests/isa/test.bin".to_string());
    cpu.get_registers().set_pc(0x140);
    println!("{:?}", cpu);
    cpu.run();
}