use std::sync::{Arc, Mutex};

use crate::bus::Bus;

// registers
const PPUCTRL: u16 = 0x2000;
const PPUMASK: u16 = 0x2001;
const PPUSTATUS: u16 = 0x2002;
const OAMADDR: u16 = 0x2003;
const OAMDATA: u16 = 0x2004;
const PPUSCROLL: u16 = 0x2005;
const PPUADDR: u16 = 0x2006;
const PPUDATA: u16 = 0x2007;

pub struct PPU {
    bus: Arc<Mutex<Bus>>,
}

impl PPU {
    pub fn new(bus: Arc<Mutex<Bus>>) -> Self {
        bus.lock().unwrap().mem_write(PPUSTATUS, 0b1010_0000);
        PPU { bus: bus }
    }
    pub fn load() {
        todo!()
    }

    pub fn tick() {
        todo!()
    }

    pub fn render_pixel() {
        todo!()
    }

    pub fn evaluate_sprite() {
        todo!()
    }
}

#[cfg(test)]
mod test_ppu {
    use super::*;

    #[test]
    fn test_power_up_state() {
        // Test Power-up state as documented in https://www.nesdev.org/wiki/PPU_power_up_state
        let bus = Arc::new(Mutex::new(Bus::new()));
        let ppu = PPU::new(bus.clone());

        assert_eq!(0b0000_0000, ppu.bus.lock().unwrap().mem_read(PPUCTRL));
        assert_eq!(0b0000_0000, ppu.bus.lock().unwrap().mem_read(PPUMASK));
        assert_eq!(0b1010_0000, ppu.bus.lock().unwrap().mem_read(PPUSTATUS));
        assert_eq!(0b0000_0000, ppu.bus.lock().unwrap().mem_read(PPUSCROLL));
        assert_eq!(0b0000_0000, ppu.bus.lock().unwrap().mem_read(PPUADDR));
        assert_eq!(0b0000_0000, ppu.bus.lock().unwrap().mem_read(PPUDATA));
    }

    // todo: test reset
}
