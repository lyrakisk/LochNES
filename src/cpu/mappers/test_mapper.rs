use crate::memory::*;

#[derive(Debug, Clone, PartialEq)]
pub struct TestMapper {
    memory: [u8; 65536],
}

impl TestMapper {
    pub fn new() -> Self {
        TestMapper { memory: [0; 65536] }
    }
}

impl Memory for TestMapper {
    fn read_u8(&self, address: u16) -> u8 {
        return self.memory[address as usize];
    }

    fn write_u8(&mut self, address: u16, data: u8) {
        self.memory[address as usize] = data;
    }

    fn read_u16(&self, address: u16) -> u16 {
        let low_order_address = address;
        let high_order_address = address.wrapping_add(1);
        return u16::from_le_bytes([
            self.memory[low_order_address as usize],
            self.memory[high_order_address as usize],
        ]);
    }

    fn zero_page_read_u16(&self, address: u8) -> u16 {
        let low_order_address = address;
        let high_order_address = address.wrapping_add(1);
        return u16::from_le_bytes([
            self.memory[low_order_address as usize],
            self.memory[high_order_address as usize],
        ]);
    }

    fn write_u16(&mut self, address: u16, data: u16) {
        let bytes = data.to_le_bytes();
        let index = address as usize;
        self.memory[index] = bytes[0];
        self.memory[index + 1] = bytes[1];
    }
}

#[cfg(test)]
mod test_mapper {
    use super::*;

    #[test]
    fn test_mem_read() {
        let mut mapper = TestMapper::new();
        mapper.memory[0x00AA] = 12;
        assert_eq!(mapper.read_u8(0x00AA), 12);
    }

    #[test]
    fn test_mem_write() {
        let mut mapper = TestMapper::new();
        mapper.write_u8(0x00AA, 12);
        assert_eq!(mapper.memory[0x00AA], 12);
    }

    #[test]
    fn test_mem_write_u16() {
        let mut mapper = TestMapper::new();
        mapper.write_u16(0x00AA, 0x8000);
        assert_eq!(mapper.memory[0x00AA], 0x00);
        assert_eq!(mapper.memory[0x00AB], 0x80);
    }

    #[test]
    fn test_mem_read_u16() {
        let mut mapper = TestMapper::new();
        mapper.memory[0x00AA] = 0x00;
        mapper.memory[0x00AB] = 0x80;
        assert_eq!(mapper.read_u16(0x00AA), 0x8000);
    }
}
