mod frame;
mod registers;
use registers::data::Data;
use registers::write_toggle::WriteToggle;
use registers::{address, Register16, Register8};

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
    data: Data,
    pub frame: Frame,
    pub vram: [u8; 2048],
    pallete_ram: [u8; 32],
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
            data: Data::new(0b0000_0000),
            frame: Frame::new(),
            vram: [0; 2048],
            pallete_ram: [0; 32],
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

    pub fn read_data(&mut self) -> u8 {
        let result = self.data.read_u8();
        self.data
            .write_u8(self.mem_read_u8(self.address.read_u16()));
        self.increment_address();
        return result;
    }

    fn mem_read_u8(&self, address: u16) -> u8 {
        match address {
            0x000..=0x0FFF => todo!("CHR Rom not implemented!"),
            0x2000..=0x2FFF => self.vram[(address - 0x2000) as usize],
            0x3000..=0x3EFF => panic!("Can't access address {}", address),
            0x3F00..=0x3F1F => self.pallete_ram[(address - 0x3f00) as usize],
            0x3F20..=0x3FFF => todo!("Mirroring not implemented!"),
            _ => panic!("Address {} is out of bounds", address),
        }
    }

    pub fn write_data(&mut self, data: u8) {
        println!("Write {} to address {:0x}", data, self.address.read_u16());
        self.mem_write_u8(self.address.read_u16(), data);
        self.increment_address();
    }

    fn mem_write_u8(&mut self, address: u16, data: u8) {
        match address {
            0x000..=0x0FFF => todo!("CHR Rom is write-only."),
            // todo: get mirroring from cartrige
            0x2000..=0x2FFF => self.vram[(address & 0b10011111111111 - 0x2000) as usize] = data,
            0x3000..=0x3EFF => panic!("Can't access address {}", address),
            0x3F00..=0x3F1F => self.pallete_ram[(address - 0x3f00) as usize] = data,
            0x3F20..=0x3FFF => {
                self.pallete_ram[((address & 0b11111100011111) - 0x3f00) as usize] = data
            }
            _ => panic!("Address {} is out of bounds", address),
        }
    }

    fn increment_address(&mut self) {
        self.address.write_u16(
            self.address
                .read_u16()
                .wrapping_add(self.control.vram_increment()),
        );
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
        assert_eq!(0b0000_0000, ppu.data.read_u8());
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
