use crate::cpu::*;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub enum AddressingModes {
    Implicit,
    Accumulator,
    Relative,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndexedIndirectX,
    IndirectIndexedY,
}

impl AddressingModes {
    pub fn get_operand_address(&self, cpu: &CPU) -> u16 {
        match self {
            AddressingModes::Immediate => cpu.program_counter,
            AddressingModes::ZeroPage => cpu.mapper.borrow().read_u8(cpu.program_counter) as u16,
            AddressingModes::ZeroPageX => cpu
                .mapper
                .borrow()
                .read_u8(cpu.program_counter)
                .wrapping_add(cpu.register_x) as u16,
            AddressingModes::ZeroPageY => cpu
                .mapper
                .borrow()
                .read_u8(cpu.program_counter)
                .wrapping_add(cpu.register_y) as u16,
            AddressingModes::Absolute => cpu.mapper.borrow().read_u16(cpu.program_counter),
            AddressingModes::AbsoluteX => cpu
                .mapper
                .borrow()
                .read_u16(cpu.program_counter)
                .wrapping_add(cpu.register_x as u16),
            AddressingModes::AbsoluteY => cpu
                .mapper
                .borrow()
                .read_u16(cpu.program_counter)
                .wrapping_add(cpu.register_y as u16),
            AddressingModes::Indirect => {
                let indirect_adress = cpu.mapper.borrow().read_u16(cpu.program_counter);
                let low_order_address = indirect_adress;

                /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
                //  An original 6502 has does not correctly fetch the target address if the indirect vector falls on a page boundary. (source: NES DEV wiki)
                //  E.g. If the indirect vector falls on $02FF, then the first byte is found at $02FF as expected,
                //  but the second byte will be at $0200 instead of $0300.
                //  From the extensive test cases on 6c.json, check example `6c ff f5`.
                //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
                let high_order_address = match indirect_adress & 0x00FF {
                    0xFF => indirect_adress & 0b1111_1111_0000_0000,
                    _ => indirect_adress.wrapping_add(1),
                };

                let low_order_byte = cpu.mapper.borrow().read_u8(low_order_address);
                let high_order_byte = cpu.mapper.borrow().read_u8(high_order_address);
                u16::from_le_bytes([low_order_byte, high_order_byte])
            }
            AddressingModes::IndexedIndirectX => {
                let indirect_address = cpu.mapper.borrow().read_u8(cpu.program_counter);
                let inc_address = indirect_address.wrapping_add(cpu.register_x);
                let address = cpu.mapper.borrow().zero_page_read_u16(inc_address);
                return address;
            }
            AddressingModes::IndirectIndexedY => {
                let indirect_address = cpu.mapper.borrow().read_u8(cpu.program_counter);
                cpu.mapper
                    .borrow()
                    .zero_page_read_u16(indirect_address)
                    .wrapping_add(cpu.register_y as u16)
            }
            _ => {
                panic!(
                    "Cannot get operand address when the Addressing Mode is {:?}",
                    self
                );
            }
        }
    }

    pub fn get_operand(&self, cpu: &CPU) -> u8 {
        match self {
            AddressingModes::Accumulator => cpu.register_a,
            _ => {
                let index = self.get_operand_address(cpu);
                cpu.mapper.borrow().read_u8(index)
            }
        }
    }

    pub fn is_page_crossed(&self, cpu: &CPU) -> bool {
        match self {
            AddressingModes::Implicit => false,
            AddressingModes::Accumulator => false,
            AddressingModes::Relative => false,
            AddressingModes::Immediate => false,
            AddressingModes::ZeroPage => false,
            AddressingModes::ZeroPageX => false,
            AddressingModes::ZeroPageY => false,
            AddressingModes::Absolute => false,
            AddressingModes::AbsoluteX => {
                let low = cpu.mapper.borrow().read_u8(cpu.program_counter);
                let (_, page_crossed) = low.overflowing_add(cpu.register_x);
                return page_crossed;
            }
            AddressingModes::AbsoluteY => {
                let low = cpu.mapper.borrow().read_u8(cpu.program_counter);
                let (_, page_crossed) = low.overflowing_add(cpu.register_y);
                return page_crossed;
            }
            AddressingModes::Indirect => false,
            AddressingModes::IndexedIndirectX => false,
            AddressingModes::IndirectIndexedY => {
                let indirect_address = cpu.mapper.borrow().read_u8(cpu.program_counter);
                let (_, page_crossed) = cpu
                    .mapper
                    .borrow()
                    .read_u8(indirect_address as u16)
                    .overflowing_add(cpu.register_y);
                return page_crossed;
            }
        }
    }
}

#[cfg(test)]
mod test_addressing_modes {
    use super::*;
    use crate::cpu::mappers::TestMapper;
    #[test]
    fn test_addressing_mode_immediate() {
        let mapper = Rc::new(RefCell::new(TestMapper::new()));
        let mut cpu = CPU::new(mapper);
        cpu.program_counter = 0x8000;
        let result = AddressingModes::Immediate.get_operand_address(&cpu);
        assert_eq!(result, 0x8000);
    }

    #[test]
    fn test_addressing_mode_zero_page() {
        let mapper = Rc::new(RefCell::new(TestMapper::new()));
        let mut cpu = CPU::new(mapper);
        cpu.program_counter = 0xAAAA;
        cpu.mapper.borrow_mut().write_u8(0xAAAA, 0xAA);
        let result = AddressingModes::ZeroPage.get_operand_address(&cpu);
        assert_eq!(result, 0xAA);
    }

    #[test]
    fn test_addressing_mode_zero_page_x() {
        let mapper = Rc::new(RefCell::new(TestMapper::new()));
        let mut cpu = CPU::new(mapper);
        cpu.program_counter = 0xAAAA;
        cpu.mapper.borrow_mut().write_u8(0xAAAA, 0x80);
        cpu.register_x = 0xFF;
        let result = AddressingModes::ZeroPageX.get_operand_address(&cpu);
        assert_eq!(result, 0x7F);
    }

    #[test]
    fn test_addressing_mode_zero_page_y() {
        let mapper = Rc::new(RefCell::new(TestMapper::new()));
        let mut cpu = CPU::new(mapper);
        cpu.program_counter = 0xAAAA;
        cpu.mapper.borrow_mut().write_u8(0xAAAA, 0x80);
        cpu.register_y = 0xFF;
        let result = AddressingModes::ZeroPageY.get_operand_address(&cpu);
        assert_eq!(result, 0x7F);
    }

    #[test]
    fn test_addressing_mode_absolute() {
        let mapper = Rc::new(RefCell::new(TestMapper::new()));
        let mut cpu = CPU::new(mapper);
        cpu.program_counter = 0x0;
        cpu.mapper.borrow_mut().write_u8(0x0, 0x9e);
        cpu.mapper.borrow_mut().write_u8(0x1, 0x5e);
        let result = AddressingModes::Absolute.get_operand_address(&cpu);
        assert_eq!(result, 0x5e9e);
    }

    #[test]
    fn test_addressing_mode_absolute_x() {
        let mapper = Rc::new(RefCell::new(TestMapper::new()));
        let mut cpu = CPU::new(mapper);
        cpu.program_counter = 0x0;
        cpu.mapper.borrow_mut().write_u16(0x00, 2000);
        cpu.register_x = 82;
        let result = AddressingModes::AbsoluteX.get_operand_address(&cpu);
        assert_eq!(result, 2082);
    }

    #[test]
    fn test_addressing_mode_absolute_y() {
        let mapper = Rc::new(RefCell::new(TestMapper::new()));
        let mut cpu = CPU::new(mapper);
        cpu.program_counter = 0x0;
        cpu.mapper.borrow_mut().write_u16(0x00, 2000);
        cpu.register_y = 82;
        let result = AddressingModes::AbsoluteY.get_operand_address(&cpu);
        assert_eq!(result, 2082);
    }

    #[test]
    fn test_addressing_mode_indexed_indirect_x() {
        let mapper = Rc::new(RefCell::new(TestMapper::new()));
        let mut cpu = CPU::new(mapper);
        cpu.program_counter = 0x8000;
        cpu.mapper.borrow_mut().write_u8(0x8000, 0x20);
        cpu.mapper.borrow_mut().write_u16(0x0021, 0xBAFC);
        cpu.register_x = 0x01;
        let result = AddressingModes::IndexedIndirectX.get_operand_address(&cpu);
        assert_eq!(result, 0xBAFC);
    }

    #[test]
    fn test_addressing_mode_indirect_indexed_y() {
        let mapper = Rc::new(RefCell::new(TestMapper::new()));
        let mut cpu = CPU::new(mapper);
        cpu.program_counter = 0x8000;
        cpu.mapper.borrow_mut().write_u8(0x8000, 0x52);
        cpu.mapper.borrow_mut().write_u16(0x0052, 0xEF05);
        cpu.register_y = 0x03;

        let result = AddressingModes::IndirectIndexedY.get_operand_address(&cpu);
        assert_eq!(result, 0xEF08);
    }

    #[test]
    fn test_get_operand() {
        let mapper = Rc::new(RefCell::new(TestMapper::new()));
        let mut cpu = CPU::new(mapper);
        cpu.register_a = 0x80;
        let result = AddressingModes::Accumulator.get_operand(&cpu);
        assert_eq!(result, 0x80);
    }
}
