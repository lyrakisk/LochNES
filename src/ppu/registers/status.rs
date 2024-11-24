use crate::ppu::registers::ReadRegister;

pub struct Status {
    value: u8,
}

impl Status {
    pub fn new(value: u8) -> Self {
        return Status { value };
    }
}

impl ReadRegister for Status {
    fn read(&self) -> u8 {
        return self.value;
    }
}
