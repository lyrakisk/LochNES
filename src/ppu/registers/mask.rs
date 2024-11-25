use crate::ppu::registers::Register8;

pub struct Mask {
    value: u8,
}

impl Mask {
    pub fn new(value: u8) -> Self {
        return Mask { value };
    }
}

impl Register8 for Mask {
    fn read_u8(&self) -> u8 {
        return self.value;
    }

    fn write_u8(&mut self, data: u8) {
        self.value = data;
    }
}
