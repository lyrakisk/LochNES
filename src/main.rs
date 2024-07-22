mod cpu;

fn main() {
    let mut cpu = cpu::CPU::new();
    cpu.run();
}
