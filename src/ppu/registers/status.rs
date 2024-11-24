use crate::ppu::registers::Register;

pub struct Status {
    value: u8,
}

impl Status {
    pub fn new(value: u8) -> Self {
        return Status { value };
    }
}

impl Register for Status {
    fn read(&self) -> u8 {
        return self.value;
    }

    fn write(&mut self, data: u8) {
        self.value = data;
    }
}