mod frame;
mod registers;
use registers::Register;

use crate::ppu::frame::Frame;
use crate::ppu::registers::control::Control;
use crate::ppu::registers::mask::Mask;
use crate::ppu::registers::status::Status;

pub struct PPU {
    control: Control,
    mask: Mask,
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
            mask: Mask::new(0b0000_0000),
            status: Status::new(0b1010_0000),
            oamaddr: 0b0000_0000,
            oamdata: 0b0000_0000,
            ppuscroll: 0b0000_0000,
            ppuaddr: 0b0000_0000,
            ppudata: 0b0000_0000,
            frame: Frame::new(),
        }
    }

    pub fn write_control(&mut self, data: u8) {
        self.control.write(data);
    }

    pub fn read_status(&self) -> u8 {
        self.status.read()
    }

    pub fn write_mask(&mut self, data: u8) {
        self.mask.write(data)
    }
}

#[cfg(test)]
mod test_ppu {
    use super::*;
    use crate::ppu::registers::Register;

    #[test]
    fn test_power_up_state() {
        // Test Power-up state as documented in https://www.nesdev.org/wiki/PPU_power_up_state
        let ppu = PPU::new();
        assert_eq!(0b0000_0000, ppu.control.read());
        assert_eq!(0b0000_0000, ppu.mask.read());
        assert_eq!(0b1010_0000, ppu.status.read());
        assert_eq!(0b0000_0000, ppu.ppuscroll);
        assert_eq!(0b0000_0000, ppu.ppuaddr);
        assert_eq!(0b0000_0000, ppu.ppudata);
    }
}
