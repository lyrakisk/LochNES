use LochNES::cpu::mappers::basic_mapper::*;
use LochNES::cpu::*;
use LochNES::memory::Memory;
use LochNES::ppu::PPU;
use LochNES::rom::*;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::EventPump;

use std::cell::RefCell;
use std::env;
use std::fs::read;
use std::path::PathBuf;
use std::rc::Rc;
fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Pacman", (256.0 * 3.0) as u32, (240.0 * 3.0) as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    canvas.set_scale(1.0, 1.0).unwrap();

    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_target(PixelFormatEnum::RGB24, 256, 240)
        .unwrap();

    let rom_bytes = read(PathBuf::from("examples/pacman/pacman.nes")).unwrap();
    let rom = Rom::try_from(&rom_bytes).unwrap();
    println!("prg rom len: {}", rom.prg_rom.len());
    println!("chr rom len: {}", rom.chr_rom.len());
    let ppu = Rc::new(RefCell::new(PPU::new(rom.chr_rom.clone())));
    let cpu_mapper = Rc::new(RefCell::new(BasicMapper::new(rom, ppu.clone())));
    let mut cpu = CPU::new(cpu_mapper.clone());

    cpu.reset();

    let mut total_cycles: usize = 0;
    loop {
        handle_user_input(cpu_mapper.clone(), &mut event_pump);

        if total_cycles % 34100 == 0 {
            texture
                .update(None, &ppu.borrow().frame.bytes, 256 * 3)
                .unwrap();
            canvas.copy(&texture, None, None).unwrap();
            canvas.present();
        }

        let instruction_result = cpu.execute_next_instruction();

        for _ in 0..instruction_result.executed_cycles {
            ppu.borrow_mut().tick();
        }

        total_cycles += instruction_result.executed_cycles as usize;
    }
}

fn handle_user_input(mapper: Rc<RefCell<BasicMapper>>, event_pump: &mut EventPump) {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => std::process::exit(0),
            Event::KeyDown {
                keycode: Some(Keycode::W),
                ..
            } => {
                mapper.borrow_mut().write_u8(0xff, 0x77);
            }
            Event::KeyDown {
                keycode: Some(Keycode::S),
                ..
            } => {
                mapper.borrow_mut().write_u8(0xff, 0x73);
            }
            Event::KeyDown {
                keycode: Some(Keycode::A),
                ..
            } => {
                mapper.borrow_mut().write_u8(0xff, 0x61);
            }
            Event::KeyDown {
                keycode: Some(Keycode::D),
                ..
            } => {
                mapper.borrow_mut().write_u8(0xff, 0x64);
            }
            _ => { /* do nothing */ }
        }
    }
}
