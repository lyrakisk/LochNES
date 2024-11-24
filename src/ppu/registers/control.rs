use crate::ppu::registers::Register;

pub struct Control {
    value: u8,
}

impl Control {
    pub fn new(value: u8) -> Self {
        return Control { value };
    }
}

impl Register for Control {
    fn read(&self) -> u8 {
        return self.value;
    }

    fn write(&mut self, data: u8) {
        self.value = data;
    }
}
