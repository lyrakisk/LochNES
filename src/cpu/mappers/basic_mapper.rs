use core::panic;

use crate::memory::*;
use crate::rom::*;

const RAM_START: u16 = 0x0000;
const RAM_MIRRORS_END: u16 = 0x1FFF;
const PPU_REGISTERS_START: u16 = 0x2000;
const PPU_REGISTERS_MIRRORS_END: u16 = 0x3FFF;
const ROM_START: u16 = 0x8000;
const ROM_END: u16 = 0xFFFF;
pub struct BasicMapper {
    ram: [u8; 2048],
    rom: Rom,
}

impl BasicMapper {
    pub fn new(rom: Rom) -> Self {
        BasicMapper {
            ram: [0; 2048],
            rom: rom,
        }
    }

    fn calculate_rom_address(&self, mut address: u16) -> u16 {
        address -= ROM_START;
        if self.rom.prg_rom.len() == 0x4000 && address >= 0x4000 {
            //mirror if needed
            address = address % 0x4000;
        }
        return address;
    }
}

impl Memory for BasicMapper {
    fn read_u8(&self, address: u16) -> u8 {
        match address {
            RAM_START..=RAM_MIRRORS_END => {
                let mirror_down_address = address & 0b00000111_11111111;
                return self.ram[mirror_down_address as usize];
            }
            PPU_REGISTERS_START..=PPU_REGISTERS_MIRRORS_END => {
                todo!("PPU registers not implemented yet")
            }
            ROM_START..=ROM_END => self.rom.prg_rom[self.calculate_rom_address(address) as usize],
            _ => panic!("Can't read address {}", address),
        }
    }

    fn write_u8(&mut self, address: u16, data: u8) {
        match address {
            RAM_START..=RAM_MIRRORS_END => self.ram[address as usize] = data,
            _ => panic!("Attempt to write read-only memory at address {}", address),
        }
    }

    fn read_u16(&self, address: u16) -> u16 {
        match address {
            RAM_START..=RAM_MIRRORS_END => {
                let low_order_address = address;
                let high_order_address = address.wrapping_add(1);
                return u16::from_le_bytes([
                    self.ram[low_order_address as usize],
                    self.ram[high_order_address as usize],
                ]);
            }
            PPU_REGISTERS_START..=PPU_REGISTERS_MIRRORS_END => {
                todo!("PPU registers not implemented yet")
            }
            ROM_START..=ROM_END => {
                let rom_address = self.calculate_rom_address(address);
                // duplicate code
                let low_order_address = rom_address;
                let high_order_address = rom_address.wrapping_add(1);
                return u16::from_le_bytes([
                    self.rom.prg_rom[low_order_address as usize],
                    self.rom.prg_rom[high_order_address as usize],
                ]);
            }
            _ => panic!("Can't read address {}", address),
        }
    }

    fn zero_page_read_u16(&self, address: u8) -> u16 {
        let low_order_address = address;
        let high_order_address = address.wrapping_add(1);
        return u16::from_le_bytes([
            self.ram[low_order_address as usize],
            self.ram[high_order_address as usize],
        ]);
    }

    fn write_u16(&mut self, address: u16, data: u16) {
        match address {
            RAM_START..=RAM_MIRRORS_END => {
                let bytes = data.to_le_bytes();
                let index = address as usize;
                self.ram[index] = bytes[0];
                self.ram[index + 1] = bytes[1];
            }
            _ => panic!("Attempt to write read-only memory at address {}", address),
        }
    }
}
