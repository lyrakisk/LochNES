use crate::ppu::registers::Register8;

pub struct Control {
    value: u8,
}

impl Control {
    pub fn new(value: u8) -> Self {
        return Control { value };
    }

    pub fn nametable_base(&self) -> u16 {
        match self.value & 0b0000_0011 {
            0 => 0x2000,
            1 => 0x2400,
            2 => 0x2800,
            3 => 0x2c00,
            _ => panic!("Not possible"),
        }
    }

    pub fn vram_increment(&self) -> u16 {
        if (self.value & 0b0000_0100) >> 2 == 0 {
            return 1;
        } else {
            return 32;
        }
    }

    pub fn nmi_enable(&self) -> bool {
        return (self.value & 0b1000_0000) >> 7 == 1;
    }
}

impl Register8 for Control {
    fn read_u8(&self) -> u8 {
        return self.value;
    }

    fn write_u8(&mut self, data: u8) {
        self.value = data;
    }
}
