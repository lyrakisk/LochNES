use crate::ppu::registers::Register8;

pub struct Control {
    value: u8,
}

impl Control {
    pub fn new(value: u8) -> Self {
        return Control { value };
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
