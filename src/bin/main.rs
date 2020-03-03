use emulator_rs;

fn main() {
    let mut cpu = emulator_rs::frontend::rv32i::cpu::CPU::new(0,4096);
    let entry_point = cpu.load_elf("src/bin/rv32i-sb".to_string());
    println!("Entry Point loaded");
    cpu.get_registers().set_pc(entry_point + 0x140);
    cpu.run().unwrap();
    println!("{:?}", cpu.get_registers()[10]);
}
