use crate::ppu::registers::Register8;

pub struct Status {
    value: u8,
}

impl Status {
    pub fn new(value: u8) -> Self {
        return Status { value };
    }
}

impl Register8 for Status {
    fn read_u8(&self) -> u8 {
        return self.value;
    }

    fn write_u8(&mut self, data: u8) {
        self.value = data;
    }
}
