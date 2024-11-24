use crate::ppu::registers::{ReadRegister, WriteRegister};

pub struct Control {
    value: u8,
}

impl Control {
    pub fn new(value: u8) -> Self {
        return Control { value };
    }
}

impl ReadRegister for Control {
    fn read(&self) -> u8 {
        return self.value;
    }
}

impl WriteRegister for Control {
    fn write(&mut self, data: u8) {
        self.value = data;
    }
}
