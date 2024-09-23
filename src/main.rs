mod bus;
mod cpu;

use std::sync::{Arc, Mutex};

fn main() {
    let mut bus = bus::Bus::new();
    let mut cpu = cpu::CPU::new(Arc::new(Mutex::new(bus)));
    cpu.run();
}
