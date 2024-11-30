use crate::ppu::registers::Register8;

pub struct Status {
    value: u8,
}

impl Status {
    pub fn new(value: u8) -> Self {
        return Status { value };
    }
    pub fn clear_v_blank(&mut self) {
        self.value = self.value & 0b0111_1111;
    }

    pub fn set_v_blank(&mut self) {
        self.value = self.value | 0b1000_0000;
    }

    pub fn is_in_v_blank(&self) -> bool {
        return self.value & 0b1000_0000 >> 7 == 1;
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
