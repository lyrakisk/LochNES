#[derive(Debug, Clone, PartialEq)]
pub struct Bus {
    memory: [u8; 65536],
}

impl Bus {
    pub fn new() -> Self {
        Bus { memory: [0; 65536] }
    }

    pub fn mem_read(&self, address: u16) -> u8 {
        return self.memory[address as usize];
    }

    pub fn mem_write(&mut self, address: u16, data: u8) {
        self.memory[address as usize] = data;
    }

    pub fn mem_read_u16(&self, address: u16) -> u16 {
        let low_order_address = address;
        let high_order_address = address.wrapping_add(1);
        return u16::from_le_bytes([
            self.memory[low_order_address as usize],
            self.memory[high_order_address as usize],
        ]);
    }

    pub fn zero_page_read_u16(&self, address: u8) -> u16 {
        let low_order_address = address;
        let high_order_address = address.wrapping_add(1);
        return u16::from_le_bytes([
            self.memory[low_order_address as usize],
            self.memory[high_order_address as usize],
        ]);
    }

    pub fn mem_write_u16(&mut self, address: u16, data: u16) {
        let bytes = data.to_le_bytes();
        let index = address as usize;
        self.memory[index] = bytes[0];
        self.memory[index + 1] = bytes[1];
    }
}

#[cfg(test)]
mod test_bus {
    use super::*;

    #[test]
    fn test_mem_read() {
        let mut bus = Bus::new();
        bus.memory[0x00AA] = 12;
        assert_eq!(bus.mem_read(0x00AA), 12);
    }

    #[test]
    fn test_mem_write() {
        let mut bus = Bus::new();
        bus.mem_write(0x00AA, 12);
        assert_eq!(bus.memory[0x00AA], 12);
    }

    #[test]
    fn test_mem_write_u16() {
        let mut bus = Bus::new();
        bus.mem_write_u16(0x00AA, 0x8000);
        assert_eq!(bus.memory[0x00AA], 0x00);
        assert_eq!(bus.memory[0x00AB], 0x80);
    }

    #[test]
    fn test_mem_read_u16() {
        let mut bus = Bus::new();
        bus.memory[0x00AA] = 0x00;
        bus.memory[0x00AB] = 0x80;
        assert_eq!(bus.mem_read_u16(0x00AA), 0x8000);
    }
}
