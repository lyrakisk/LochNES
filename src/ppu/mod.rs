mod frame;
mod registers;
use core::panic;

use registers::data::Data;
use registers::write_toggle::WriteToggle;
use registers::{Register16, Register8};

use frame::Frame;
use registers::address::Address;
use registers::control::Control;
use registers::mask::Mask;
use registers::status::Status;

use registers::oam_address::OAMAddress;
#[rustfmt::skip]
pub static SYSTEM_PALLETE: [(u8,u8,u8); 64] = [
   (0x80, 0x80, 0x80), (0x00, 0x3D, 0xA6), (0x00, 0x12, 0xB0), (0x44, 0x00, 0x96), (0xA1, 0x00, 0x5E),
   (0xC7, 0x00, 0x28), (0xBA, 0x06, 0x00), (0x8C, 0x17, 0x00), (0x5C, 0x2F, 0x00), (0x10, 0x45, 0x00),
   (0x05, 0x4A, 0x00), (0x00, 0x47, 0x2E), (0x00, 0x41, 0x66), (0x00, 0x00, 0x00), (0x05, 0x05, 0x05),
   (0x05, 0x05, 0x05), (0xC7, 0xC7, 0xC7), (0x00, 0x77, 0xFF), (0x21, 0x55, 0xFF), (0x82, 0x37, 0xFA),
   (0xEB, 0x2F, 0xB5), (0xFF, 0x29, 0x50), (0xFF, 0x22, 0x00), (0xD6, 0x32, 0x00), (0xC4, 0x62, 0x00),
   (0x35, 0x80, 0x00), (0x05, 0x8F, 0x00), (0x00, 0x8A, 0x55), (0x00, 0x99, 0xCC), (0x21, 0x21, 0x21),
   (0x09, 0x09, 0x09), (0x09, 0x09, 0x09), (0xFF, 0xFF, 0xFF), (0x0F, 0xD7, 0xFF), (0x69, 0xA2, 0xFF),
   (0xD4, 0x80, 0xFF), (0xFF, 0x45, 0xF3), (0xFF, 0x61, 0x8B), (0xFF, 0x88, 0x33), (0xFF, 0x9C, 0x12),
   (0xFA, 0xBC, 0x20), (0x9F, 0xE3, 0x0E), (0x2B, 0xF0, 0x35), (0x0C, 0xF0, 0xA4), (0x05, 0xFB, 0xFF),
   (0x5E, 0x5E, 0x5E), (0x0D, 0x0D, 0x0D), (0x0D, 0x0D, 0x0D), (0xFF, 0xFF, 0xFF), (0xA6, 0xFC, 0xFF),
   (0xB3, 0xEC, 0xFF), (0xDA, 0xAB, 0xEB), (0xFF, 0xA8, 0xF9), (0xFF, 0xAB, 0xB3), (0xFF, 0xD2, 0xB0),
   (0xFF, 0xEF, 0xA6), (0xFF, 0xF7, 0x9C), (0xD7, 0xE8, 0x95), (0xA6, 0xED, 0xAF), (0xA2, 0xF2, 0xDA),
   (0x99, 0xFF, 0xFC), (0xDD, 0xDD, 0xDD), (0x11, 0x11, 0x11), (0x11, 0x11, 0x11)
];

pub struct PPU {
    control: Control,
    mask: Mask,
    status: Status,
    oamaddr: OAMAddress,
    oamdata: u16,
    ppuscroll: u16,
    address: Address,
    data: Data,
    pub frame: Frame,
    pub vram: [u8; 2048],
    oam_ram: [u8; 256],
    pallete_ram: [u8; 32],
    w: WriteToggle,
    pub cycles: u16,
    pub scanline: u16,
    pub nmi_triggered: bool,
    chr: Vec<u8>,
}

impl PPU {
    pub fn new(chr: Vec<u8>) -> Self {
        PPU {
            control: Control::new(0b0000_0000),
            mask: Mask::new(0b0000_0000),
            status: Status::new(0b1010_0000),
            oamaddr: OAMAddress::new(0b0000_0000),
            oamdata: 0b0000_0000,
            ppuscroll: 0b0000_0000,
            address: Address::new(0x0000),
            data: Data::new(0b0000_0000),
            frame: Frame::new(),
            vram: [0; 2048],
            oam_ram: [0; 256],
            pallete_ram: [0; 32],
            w: WriteToggle::FirstWrite,
            cycles: 0,
            scanline: 0,
            nmi_triggered: false,
            chr: chr,
        }
    }

    pub fn write_control(&mut self, data: u8) {
        let current_nmi_enable = self.control.nmi_enable();
        self.control.write_u8(data);
        let next_nmi_enable = self.control.nmi_enable();
        if !current_nmi_enable && next_nmi_enable && self.status.is_in_v_blank() {
            // println!("nmi triggered by writing to control register");
            self.nmi_triggered = true;
        }
    }

    pub fn read_status(&mut self) -> u8 {
        let result = self.status.read_u8();
        self.status.clear_v_blank();
        return result;
    }

    pub fn write_oam_address(&mut self, data: u8) {
        self.oamaddr.write_u8(data);
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
            0x0000..=0x1FFF => self.chr[address as usize],
            // todo: get nametable mirroring from cartrige, for now we assume it's vertical
            0x2000..=0x2FFF => self.vram[(address & 0b10011111111111 - 0x2000) as usize],
            0x3000..=0x3EFF => panic!("Can't access address {}", address),
            0x3F00..=0x3F1F => self.pallete_ram[(address - 0x3f00) as usize],
            0x3F20..=0x3FFF => todo!("Mirroring not implemented!"),
            _ => panic!("Address {} is out of bounds", address),
        }
    }

    pub fn write_data(&mut self, data: u8) {
        // println!("Write {} to address {:0x}", data, self.address.read_u16());
        self.mem_write_u8(self.address.read_u16(), data);
        self.increment_address();
    }

    pub fn dma_write(&mut self, data: &[u8]) {
        assert!(data.len() == 256);
        self.oam_ram.clone_from_slice(data);
    }

    fn mem_write_u8(&mut self, address: u16, data: u8) {
        match address {
            0x000..=0x1FFF => panic!("CHR Rom is read-only."),
            // todo: get nametable mirroring from cartrige, for now we assume it's vertical
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
        // println!("Control {:08b}",. self.control.read_u8());
        // println!("cycle: {}, scanline: {}", self.cycles, self.scanline);
        match self.cycles {
            0 => (),
            1..=256 => {
                if self.scanline < 240 {
                    self.render_pixel(self.cycles - 1, self.scanline);
                }
            }
            257..=340 => (),
            _ => {
                self.cycles = self.cycles - 341;
                self.scanline += 1;

                // todo: Implement sprite rendering with proper timing
                if self.scanline == 240 {
                    self.render_sprites();
                }

                if self.scanline == 241 {
                    self.status.set_v_blank();
                    self.scanline += 1;
                    if self.control.nmi_enable() {
                        // println!("nmi triggered, from scanline 241");
                        self.nmi_triggered = true;
                    }
                }

                if self.scanline == 261 {
                    {
                        self.scanline = 0;
                    }
                }
            }
        }
        self.cycles += 1;
    }

    fn render_pixel(&mut self, x: u16, y: u16) {
        let bank = (self.control.background_pattern_table_address() as u16) * 0x1000;
        // println!("bank: {}", bank);
        assert!(bank == 0 || bank == 0x1000);
        let nametable_x = x / 8;
        let nametable_y = y / 8;
        let nametable_base = self.control.nametable_base();
        let nametable_index = nametable_base + nametable_x + nametable_y * 32;
        assert!(nametable_index - nametable_base < 0x400);
        let nametable_byte = self.mem_read_u8(nametable_index) as u16;
        // println!("Render pixel (x: {}, y: {}, bank: {}, n_x: {}, n_y: {}, n_base: {:0x}, n_index: {:0x},  n_byte: {})", x, y, bank, nametable_x, nametable_y, nametable_base, nametable_index, nametable_byte);

        let tile = &self.chr
            [(bank + nametable_byte * 16) as usize..=(bank + nametable_byte * 16 + 15) as usize];
        let shift = vec![7, 6, 5, 4, 3, 2, 1, 0];

        let mut upper = tile[(y % 8) as usize];
        let mut lower = tile[((y % 8) + 8) as usize];

        upper = upper >> (shift[(x % 8) as usize]);
        lower = lower >> (shift[(x % 8) as usize]);

        let value = (1 & upper) << 1 | (1 & lower);
        let rgb = match value {
            0 => SYSTEM_PALLETE[0x01],
            1 => SYSTEM_PALLETE[0x23],
            2 => SYSTEM_PALLETE[0x27],
            3 => SYSTEM_PALLETE[0x30],
            _ => panic!("can't be"),
        };

        self.frame.set_pixel(x as usize, y as usize, rgb);
    }

    fn render_sprites(&mut self) {
        for i in (0..self.oam_ram.len()).step_by(4).rev() {
            let tile_idx = self.oam_ram[i + 1] as u16;
            let tile_x = self.oam_ram[i + 3] as usize;
            let tile_y = self.oam_ram[i] as usize;

            let flip_vertical = if self.oam_ram[i + 2] >> 7 & 1 == 1 {
                true
            } else {
                false
            };
            let flip_horizontal = if self.oam_ram[i + 2] >> 6 & 1 == 1 {
                true
            } else {
                false
            };
            let pallette_idx = self.oam_ram[i + 2] & 0b11;
            let sprite_palette = self.sprite_palette(pallette_idx);

            let bank: u16 = self.control.sprite_pattern_table_address();
            assert!(bank == 0 || bank == 0x1000);
            let tile =
                &self.chr[(bank + tile_idx * 16) as usize..=(bank + tile_idx * 16 + 15) as usize];

            for y in 0..=7 {
                let mut upper = tile[y];
                let mut lower = tile[y + 8];
                'ololo: for x in (0..=7).rev() {
                    let value = (1 & lower) << 1 | (1 & upper);
                    upper = upper >> 1;
                    lower = lower >> 1;
                    let rgb = match value {
                        0 => continue 'ololo, // skip coloring the pixel
                        1 => SYSTEM_PALLETE[sprite_palette[1] as usize],
                        2 => SYSTEM_PALLETE[sprite_palette[2] as usize],
                        3 => SYSTEM_PALLETE[sprite_palette[3] as usize],
                        _ => panic!("can't be"),
                    };
                    match (flip_horizontal, flip_vertical) {
                        (false, false) => self.frame.set_pixel(tile_x + x, tile_y + y, rgb),
                        (true, false) => self.frame.set_pixel(tile_x + 7 - x, tile_y + y, rgb),
                        (false, true) => self.frame.set_pixel(tile_x + x, tile_y + 7 - y, rgb),
                        (true, true) => self.frame.set_pixel(tile_x + 7 - x, tile_y + 7 - y, rgb),
                    }
                }
            }
        }
    }

    fn sprite_palette(&self, pallete_idx: u8) -> [u8; 4] {
        let start = 0x11 + (pallete_idx * 4) as usize;
        // todo: get correct colors from frame pallette
        return [0, 0x20, 0x25, 0x35];
    }
}

#[cfg(test)]
mod test_ppu {
    use super::*;
    use crate::ppu::registers::Register8;

    #[test]
    fn test_power_up_state() {
        // Test Power-up state as documented in https://www.nesdev.org/wiki/PPU_power_up_state
        let ppu = PPU::new(vec![]);
        assert_eq!(0b0000_0000, ppu.control.read_u8());
        assert_eq!(0b0000_0000, ppu.mask.read_u8());
        assert_eq!(0b1010_0000, ppu.status.read_u8());
        assert_eq!(0b0000_0000, ppu.ppuscroll);
        assert_eq!(0b0000_0000, ppu.oamaddr.read_u8());
        assert_eq!(0x0000, ppu.address.read_u16());
        assert_eq!(0b0000_0000, ppu.data.read_u8());
        assert_eq!([0; 2048], ppu.vram)
    }
}

#[test]
fn test_address_register_first_write() {
    let mut ppu = PPU::new(vec![]);

    ppu.write_address(0xAA);
    assert_eq!(0xAA00, ppu.address.read_u16());
    assert_eq!(WriteToggle::SecondWrite, ppu.w);
}

#[test]
fn test_address_register_second_write() {
    let mut ppu = PPU::new(vec![]);
    ppu.w = WriteToggle::SecondWrite;

    ppu.write_address(0xAA);
    assert_eq!(0x00AA, ppu.address.read_u16());
    assert_eq!(WriteToggle::FirstWrite, ppu.w);
}
