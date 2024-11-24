use crate::ppu::registers::Register;

pub struct Mask {
    value: u8,
}

impl Mask {
    pub fn new(value: u8) -> Self {
        return Mask { value };
    }
}

impl Register for Mask {
    fn read(&self) -> u8 {
        return self.value;
    }

    fn write(&mut self, data: u8) {
        self.value = data;
    }
}
