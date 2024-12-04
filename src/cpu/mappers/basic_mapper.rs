use core::panic;
use std::cell::RefCell;
use std::rc::Rc;


use crate::memory::*;
use crate::ppu::PPU;
use crate::rom::*;
use crate::controller::*;

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
    controller: Rc<RefCell<Controller>>,
}

impl BasicMapper {
    pub fn new(rom: Rom, ppu: Rc<RefCell<PPU>>, controller: Rc<RefCell<Controller>>) -> Self {
        BasicMapper {
            ram: [0; 2048],
            rom: rom,
            ppu: ppu,
            controller: controller,
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
    fn nmi_occured(&self) -> bool {
        let nmi_occured = self.ppu.borrow().nmi_triggered;
        if nmi_occured {
            self.ppu.borrow_mut().nmi_triggered = false;
        }
        return nmi_occured;
    }

    fn read_u8(&self, address: u16) -> u8 {
        match address {
            RAM_START..=RAM_MIRRORS_END => {
                let mirror_down_address = address & 0b00000111_11111111;
                return self.ram[mirror_down_address as usize];
            }
            PPU_REGISTERS_START..=PPU_REGISTERS_MIRRORS_END => {
                let mirror_down_address = address & 0b00100000_00000111;
                match mirror_down_address {
                    0x2000 => panic!("Control register is write-only!"),
                    0x2001 => panic!("Mask register is write-only!"),
                    0x2002 => self.ppu.borrow_mut().read_status(),
                    0x2003 => todo!("OAMADDR register is not implemented yet!"),
                    0x2004 => panic!("OAMDATA register is not implemented yet!"),
                    0x2005 => panic!("Scroll register is write-only!"),
                    0x2006 => panic!("Address register is write-only!"),
                    0x2007 => self.ppu.borrow_mut().read_data(),
                    _ => panic!("Impossible"),
                }
            }
            0x4000..=0x4015 | 0x4017 => {
                println!(
                    "Ignoring read from {:0x}, APU and IO are not implemented yet, returning 0",
                    address
                );
                return 0;
            }
            0x4016 => self.controller.borrow_mut().read_u8(),
            ROM_START..=ROM_END => self.rom.prg_rom[self.calculate_rom_address(address) as usize],
            _ => panic!("Can't read address {:0x}", address),
        }
    }

    fn write_u8(&mut self, address: u16, data: u8) {
        match address {
            RAM_START..=RAM_MIRRORS_END => {
                let mirror_down_address = address & 0b00000111_11111111;
                self.ram[mirror_down_address as usize] = data;
            }
            PPU_REGISTERS_START..=PPU_REGISTERS_MIRRORS_END => {
                let mirror_down_address = address & 0b00100000_00000111;
                match mirror_down_address {
                    0x2000 => self.ppu.borrow_mut().write_control(data),
                    0x2001 => self.ppu.borrow_mut().write_mask(data),
                    0x2002 => panic!("Status register is read-only!"),
                    0x2003 => println!("OAMADDR register is not implemented yet!"),
                    0x2004 => println!("OAMDATA register is not implemented yet!"),
                    0x2005 => println!("Scroll register is not implemented yet!"),
                    0x2006 => self.ppu.borrow_mut().write_address(data),
                    0x2007 => self.ppu.borrow_mut().write_data(data),
                    _ => panic!("Impossible!"),
                }
            }
            0x4000..=0x4015 | 0x4017 => {
                println!(
                    "Ignoring write to {:0x}, APU and IO are not implemented yet",
                    address
                );
            }
            0x4016 => {
                self.controller.borrow_mut().write(data);
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
                let mirror_down_address = address & 0b00000111_11111111;
                let low_order_address = mirror_down_address;
                let high_order_address = mirror_down_address.wrapping_add(1);
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
            _ => panic!("Can't read address {:0x}", address),
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
                let mirror_down_address = address & 0b00000111_11111111;
                let bytes = data.to_le_bytes();
                let index = mirror_down_address as usize;
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
