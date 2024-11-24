use core::panic;
use std::cell::RefCell;
use std::rc::Rc;

use crate::memory::*;
use crate::ppu::PPU;
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
    ppu: Rc<RefCell<PPU>>,
}

impl BasicMapper {
    pub fn new(rom: Rom, ppu: Rc<RefCell<PPU>>) -> Self {
        BasicMapper {
            ram: [0; 2048],
            rom: rom,
            ppu: ppu,
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
            0x2000 => panic!("Control register is write-only!"),
            0x2001 => panic!("Mask register is write-only!"),
            0x2002 => self.ppu.borrow().read_status(),
            0x2003 => todo!("OAMADDR register is not implemented yet!"),
            0x2004 => todo!("OAMDATA register is not implemented yet!"),
            0x2005 => panic!("Scroll register is write-only!"),
            0x2006 => panic!("Address register is not implemented yet!"),
            0x2007 => panic!("Data register is not implemented yet!"),
            ROM_START..=ROM_END => self.rom.prg_rom[self.calculate_rom_address(address) as usize],
            _ => panic!("Can't read address {}", address),
        }
    }

    fn write_u8(&mut self, address: u16, data: u8) {
        match address {
            RAM_START..=RAM_MIRRORS_END => self.ram[address as usize] = data,
            0x2000 => self.ppu.borrow_mut().write_control(data),
            0x2001 => self.ppu.borrow_mut().write_mask(data),
            0x2002 => panic!("Status register is read-only!"),
            0x2003 => todo!("OAMADDR register is not implemented yet!"),
            0x2004 => todo!("OAMDATA register is not implemented yet!"),
            0x2005 => panic!("Scroll register is not implemented yet!"),
            0x2006 => panic!("Address register is not implemented yet!"),
            0x2007 => panic!("Data register is not implemented yet!"),

            0x4016..=0x4017 => {
                println!("Joypads not implemented yet")
            }
            _ => println!(
                "Attempt to write read-only memory at address {:0x}",
                address
            ),
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
                panic!("Can't read 2 bytes from PPU registers")
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
            PPU_REGISTERS_START..=PPU_REGISTERS_MIRRORS_END => {
                todo!("Can't write 2 bytes to a PPU register")
            }
            _ => panic!(
                "Attempt to write read-only memory at address {:0x}",
                address
            ),
        }
    }
}
