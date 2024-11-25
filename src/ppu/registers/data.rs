use super::Register8;

pub struct Data {
    value: u8,
}

impl Data {
    pub fn new(value: u8) -> Self {
        Data { value: value }
    }
}
impl Register8 for Data {
    fn read_u8(&self) -> u8 {
        return self.value;
    }

    fn write_u8(&mut self, data: u8) {
        self.value = data;
    }
}
