use super::Register8;

pub struct OAMAddress {
    value: u8,
}

impl OAMAddress {
    pub fn new(value: u8) -> Self {
        return OAMAddress { value: value };
    }
}
impl Register8 for OAMAddress {
    fn read_u8(&self) -> u8 {
        return self.value;
    }

    fn write_u8(&mut self, data: u8) {
        self.value = data;
    }
}
