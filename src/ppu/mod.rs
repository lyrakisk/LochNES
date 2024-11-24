mod frame;
mod registers;
use registers::{ReadRegister, WriteRegister};

use crate::ppu::frame::Frame;
use crate::ppu::registers::control::Control;
use crate::ppu::registers::status::Status;

pub struct PPU {
    control: Control,
    ppumask: u16,
    status: Status,
    oamaddr: u16,
    oamdata: u16,
    ppuscroll: u16,
    ppuaddr: u16,
    ppudata: u16,
    pub frame: Frame,
}

impl PPU {
    pub fn new() -> Self {
        PPU {
            control: Control::new(0b0000_0000),
            ppumask: 0b0000_0000,
            status: Status::new(0b1010_0000),
            oamaddr: 0b0000_0000,
            oamdata: 0b0000_0000,
            ppuscroll: 0b0000_0000,
            ppuaddr: 0b0000_0000,
            ppudata: 0b0000_0000,
            frame: Frame::new(),
        }
    }

    pub fn read_register(&self, address: u16) -> u8 {
        match address {
            0x2000 => {
                return self.control.read();
            }
            0x2002 => {
                return self.status.read();
            }
            _ => {
                println!("Register {:0x} not implemented", address);
                return 0;
            }
        }
    }

    pub fn write_register(&mut self, address: u16, data: u8) {
        match address {
            0x2000 => {
                self.control.write(data);
            }
            _ => {
                println!("Register {:0x} not implemented", address);
            }
        }
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
    use crate::ppu::registers::ReadRegister;

    #[test]
    fn test_power_up_state() {
        // Test Power-up state as documented in https://www.nesdev.org/wiki/PPU_power_up_state
        let ppu = PPU::new();
        assert_eq!(0b0000_0000, ppu.control.read());
        assert_eq!(0b0000_0000, ppu.ppumask);
        assert_eq!(0b1010_0000, ppu.status.read());
        assert_eq!(0b0000_0000, ppu.ppuscroll);
        assert_eq!(0b0000_0000, ppu.ppuaddr);
        assert_eq!(0b0000_0000, ppu.ppudata);
    }

    // todo: test reset
}
