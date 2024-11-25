mod frame;
mod registers;
use registers::write_toggle::WriteToggle;
use registers::{Register16, Register8};

use crate::ppu::frame::Frame;
use crate::ppu::registers::address::Address;
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
    address: Address,
    ppudata: u16,
    pub frame: Frame,
    pub vram: [u8; 2048],
    w: WriteToggle,
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
            address: Address::new(0x0000),
            ppudata: 0b0000_0000,
            frame: Frame::new(),
            vram: [0; 2048],
            w: WriteToggle::FirstWrite,
        }
    }

    pub fn write_control(&mut self, data: u8) {
        self.control.write_u8(data);
    }

    pub fn read_status(&self) -> u8 {
        self.status.read_u8()
    }

    pub fn write_mask(&mut self, data: u8) {
        self.mask.write_u8(data)
    }

    pub fn write_address(&mut self, data: u8) {
        match self.w {
            WriteToggle::FirstWrite => {
                self.address.write_u16((data as u16) << 8);
            }
            WriteToggle::SecondWrite => {
                self.address
                    .write_u16((self.address.read_u16() & 0xFF00) | (data as u16));
            }
        }
        self.w.toggle();
    }

    pub fn tick(&mut self) {
        todo!()
    }
}

#[cfg(test)]
mod test_ppu {
    use super::*;
    use crate::ppu::registers::Register8;

    #[test]
    fn test_power_up_state() {
        // Test Power-up state as documented in https://www.nesdev.org/wiki/PPU_power_up_state
        let ppu = PPU::new();
        assert_eq!(0b0000_0000, ppu.control.read_u8());
        assert_eq!(0b0000_0000, ppu.mask.read_u8());
        assert_eq!(0b1010_0000, ppu.status.read_u8());
        assert_eq!(0b0000_0000, ppu.ppuscroll);
        assert_eq!(0b0000_0000, ppu.oamaddr);
        assert_eq!(0x0000, ppu.address.read_u16());
        assert_eq!(0b0000_0000, ppu.ppudata);
        assert_eq!([0; 2048], ppu.vram)
    }
}

#[test]
fn test_address_register_first_write() {
    let mut ppu = PPU::new();

    ppu.write_address(0xAA);
    assert_eq!(0xAA00, ppu.address.read_u16());
    assert_eq!(WriteToggle::SecondWrite, ppu.w);
}

#[test]
fn test_address_register_second_write() {
    let mut ppu = PPU::new();
    ppu.w = WriteToggle::SecondWrite;

    ppu.write_address(0xAA);
    assert_eq!(0x00AA, ppu.address.read_u16());
    assert_eq!(WriteToggle::FirstWrite, ppu.w);
}
