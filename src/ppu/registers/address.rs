use crate::ppu::registers::Register16;

pub struct Address {
    pub value: u16,
}

impl Address {
    pub fn new(value: u16) -> Self {
        Address { value: value }
    }
}

impl Register16 for Address {
    fn read_u16(&self) -> u16 {
        return self.value;
    }

    fn write_u16(&mut self, data: u16) {
        self.value = data;
    }
}
