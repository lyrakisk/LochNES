use crate::cpu::mappers::*;
use crate::cpu::*;
use crate::ppu::*;

use std::sync::{Arc, Mutex};

pub fn emulate_game() {
    // let cpu_bus = Arc::new(Mutex::new(Bus::new()));
    // let ppu_bus = Arc::new(Mutex::new(Bus::new()));
    // let mut cpu = CPU::new(cpu_bus.clone());
    // let gui = GUI::new();
    // let mut ppu = PPU::new(ppu_bus.clone());
    // let apu = APU::new();
    // let controller = Controller::new(&cpu_bus);

    // let mut ppu_cycles_budet = 0;
    // let mut apu_cycles_budget = 0;

    // gui.handle_user_input(&controller, cpu_bus); // update current controller status

    // let executed_cpu_cycles = cpu.execute_next_instruction().executed_cycles;

    // ppu_cycles_budet += executed_cpu_cycles * 3;
    // apu_cycles_budget += executed_cpu_cycles / 2; // this will miss apu cycles, which will probably mess up audio

    // for _ in 0..ppu_cycles_budet {
    //     ppu.tick();
    // }

    // for 0..apu_cycles_budget {
    //     apu.tick();
    // }

    // gui.render(ppu.frame);
    // gui.play_sound() // ??????
}

// pub fn load()
// pub fn reset()
