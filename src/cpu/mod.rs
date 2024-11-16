mod addressing_modes;
mod instructions;
pub mod mappers;

use std::cell::RefCell;
use std::rc::Rc;

use crate::cpu::instructions::*;
use crate::memory::Memory;

pub const STATUS_FLAG_NEGATIVE: u8 = 0b10000000;
pub const STATUS_FLAG_OVERFLOW: u8 = 0b01000000;
pub const STATUS_FLAG_BREAK_COMMAND: u8 = 0b0001_0000;
pub const STATUS_FLAG_DECIMAL: u8 = 0b0000_1000;
pub const STATUS_FLAG_INTERRUPT_DISABLE: u8 = 0b0000_0100;
pub const STATUS_FLAG_ZERO: u8 = 0b00000010;
pub const STATUS_FLAG_CARRY: u8 = 0b00000001;

#[derive(Debug, PartialEq)]
enum FlagStates {
    CLEAR = 0,
    SET = 1,
}

// #[derive(Debug)]
pub struct CPU {
    register_a: u8,
    register_x: u8,
    register_y: u8,
    status: u8,
    program_counter: u16,
    stack_pointer: u8,
    mapper: Rc<RefCell<dyn Memory>>,
}

impl CPU {
    pub fn new(mapper: Rc<RefCell<dyn Memory>>) -> Self {
        CPU {
            register_a: 0,
            register_x: 0, // todo: check reference, should this be initialized?
            register_y: 0,
            status: 0, // todo: according to nesdev wiki, the 5th bit is always 1, https://www.nesdev.org/wiki/Status_flags
            program_counter: 0x8000,
            stack_pointer: 0xFF,
            mapper,
        }
    }

    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.status = 0;
        self.program_counter = self.mapper.borrow().read_u16(0xFFFC);
    }

    pub fn run(&mut self) {
        loop {
            self.execute_next_instruction();
        }
    }

    pub fn run_with_callback<F>(&mut self, mut callback: F)
    where
        F: FnMut(&mut CPU),
    {
        loop {
            callback(self);
            self.execute_next_instruction();
        }
    }

    pub fn execute_next_instruction(&mut self) -> InstructionResult {
        let opcode = self.fetch();

        let decoded_opcode = self.decode(opcode);

        match decoded_opcode {
            None => panic!("Could not decode opcode 0x{:02x}", opcode),
            Some(instruction) => {
                let instruction_result = instruction.execute(self);
                self.update_program_counter(instruction);
                return instruction_result;
            }
        }
    }

    fn update_program_counter(&mut self, instruction: Instruction) {
        let instr = vec!["JMP", "JSR"];
        if instr.contains(&instruction.name) {
            return;
        } else {
            self.program_counter = self
                .program_counter
                .wrapping_add(instruction.bytes as u16 - 1);
        }
    }

    fn fetch(&mut self) -> u8 {
        let opcode = self.mapper.borrow().read_u8(self.program_counter);
        self.program_counter = self.program_counter.wrapping_add(1);
        return opcode;
    }

    fn decode(&self, opcode: u8) -> Option<Instruction> {
        if !INSTRUCTIONS.contains_key(&opcode) {
            return None;
        } else {
            return Some(INSTRUCTIONS[&opcode].clone());
        }
    }

    pub fn load(&mut self, program: Vec<u8>) {
        for address in self.program_counter..=(self.program_counter - 1 + program.len() as u16) {
            let program_address = (address).wrapping_sub(self.program_counter) as usize;
            self.mapper
                .borrow_mut()
                .write_u8(address as u16, program[program_address]);
        }
    }

    fn stack_pop(&mut self) -> u8 {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        return self
            .mapper
            .borrow()
            .read_u8(0x0100 + (self.stack_pointer as u16));
    }
    fn stack_pop_u16(&mut self) -> u16 {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        let low_order_byte = self
            .mapper
            .borrow()
            .read_u8(0x0100 + (self.stack_pointer as u16));
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        let high_order_byte = self
            .mapper
            .borrow()
            .read_u8(0x0100 + (self.stack_pointer as u16));

        return u16::from_le_bytes([low_order_byte, high_order_byte]).wrapping_add(1);
    }

    fn stack_push(&mut self, data: u8) {
        self.mapper
            .borrow_mut()
            .write_u8(0x0100 + (self.stack_pointer as u16), data);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    }
    fn stack_push_u16(&mut self, data: u16) {
        let bytes = data.to_le_bytes();
        self.mapper
            .borrow_mut()
            .write_u8(0x0100 + (self.stack_pointer as u16), bytes[1]);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
        self.mapper
            .borrow_mut()
            .write_u8(0x0100 + (self.stack_pointer as u16), bytes[0]);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    }

    fn get_flag_state(&self, mask: u8) -> FlagStates {
        if self.status & mask == 0x0 {
            FlagStates::CLEAR
        } else {
            FlagStates::SET
        }
    }

    fn set_flag(&mut self, mask: u8) {
        self.status = self.status | mask;
    }

    fn clear_flag(&mut self, mask: u8) {
        self.status = self.status & (!mask);
    }

    // todo: separate the check for page crossing
    fn branch_off_program_counter(&mut self, distance: u8) -> bool {
        let page_crossed: bool;
        let le_bytes = self.program_counter.to_le_bytes();
        // low is incremented here, because we're at the second
        // cycle of the instruction which has incremented the PC
        // as described in https://www.nesdev.org/6502_cpu.txt
        let low = le_bytes[0].wrapping_add(1);
        if distance > 0x7F {
            let offset: u8 = 0xff_u8.wrapping_sub(distance).wrapping_add(1);
            (_, page_crossed) = low.overflowing_sub(offset as u8);
            self.program_counter = self.program_counter.wrapping_sub(offset as u16);
        } else {
            let distance = distance;
            (_, page_crossed) = low.overflowing_add(distance as u8);
            self.program_counter = self.program_counter.wrapping_add(distance as u16);
        }
        page_crossed
    }

    fn update_zero_flag(&mut self, data: u8) {
        if data == 0 {
            self.set_flag(STATUS_FLAG_ZERO);
        } else {
            self.clear_flag(STATUS_FLAG_ZERO);
        }
    }

    fn update_negative_flag(&mut self, data: u8) {
        if data & 0b1000_0000 != 0 {
            self.set_flag(STATUS_FLAG_NEGATIVE);
        } else {
            self.clear_flag(STATUS_FLAG_NEGATIVE);
        }
    }
}

#[cfg(test)]
mod test_cpu {
    use super::*;
    use crate::cpu::mappers::TestMapper;
    use json::JsonValue;
    use test_case::test_case;

    #[test]
    fn test_load() {
        let mapper = Rc::new(RefCell::new((TestMapper::new())));
        let mut cpu = CPU::new(mapper);
        cpu.program_counter = 0x8000;
        let program = vec![0xAA, 0x35, 0xFF, 0x00];
        cpu.load(program);
        assert_eq!(0xAA, cpu.mapper.borrow().read_u8(0x8000));
        assert_eq!(0x35, cpu.mapper.borrow().read_u8(0x8001));
        assert_eq!(0xFF, cpu.mapper.borrow().read_u8(0x8002));
        assert_eq!(0x00, cpu.mapper.borrow().read_u8(0x8003));
    }

    #[test_case(0b0, 0b0000_0010)]
    #[test_case(0b10, 0b0)]
    fn test_update_zero_flag(register: u8, expected: u8) {
        let mapper = Rc::new(RefCell::new(TestMapper::new()));
        let mut cpu = CPU::new(mapper);
        cpu.update_zero_flag(register);
        assert_eq!(cpu.status, expected);
    }

    #[test_case("submodules/65x02/nes6502/v1/00.json")]
    #[test_case("submodules/65x02/nes6502/v1/01.json")]
    #[test_case("submodules/65x02/nes6502/v1/03.json")]
    #[test_case("submodules/65x02/nes6502/v1/04.json")]
    #[test_case("submodules/65x02/nes6502/v1/05.json")]
    #[test_case("submodules/65x02/nes6502/v1/06.json")]
    #[test_case("submodules/65x02/nes6502/v1/07.json")]
    #[test_case("submodules/65x02/nes6502/v1/08.json")]
    #[test_case("submodules/65x02/nes6502/v1/09.json")]
    #[test_case("submodules/65x02/nes6502/v1/0a.json")]
    #[test_case("submodules/65x02/nes6502/v1/0b.json")]
    #[test_case("submodules/65x02/nes6502/v1/0c.json")]
    #[test_case("submodules/65x02/nes6502/v1/0d.json")]
    #[test_case("submodules/65x02/nes6502/v1/0e.json")]
    #[test_case("submodules/65x02/nes6502/v1/0f.json")]
    #[test_case("submodules/65x02/nes6502/v1/10.json")]
    #[test_case("submodules/65x02/nes6502/v1/11.json")]
    #[test_case("submodules/65x02/nes6502/v1/13.json")]
    #[test_case("submodules/65x02/nes6502/v1/14.json")]
    #[test_case("submodules/65x02/nes6502/v1/15.json")]
    #[test_case("submodules/65x02/nes6502/v1/16.json")]
    #[test_case("submodules/65x02/nes6502/v1/17.json")]
    #[test_case("submodules/65x02/nes6502/v1/18.json")]
    #[test_case("submodules/65x02/nes6502/v1/19.json")]
    #[test_case("submodules/65x02/nes6502/v1/1a.json")]
    #[test_case("submodules/65x02/nes6502/v1/1b.json")]
    #[test_case("submodules/65x02/nes6502/v1/1c.json")]
    #[test_case("submodules/65x02/nes6502/v1/1d.json")]
    #[test_case("submodules/65x02/nes6502/v1/1e.json")]
    #[test_case("submodules/65x02/nes6502/v1/1f.json")]
    #[test_case("submodules/65x02/nes6502/v1/20.json")]
    #[test_case("submodules/65x02/nes6502/v1/21.json")]
    #[test_case("submodules/65x02/nes6502/v1/23.json")]
    #[test_case("submodules/65x02/nes6502/v1/24.json")]
    #[test_case("submodules/65x02/nes6502/v1/25.json")]
    #[test_case("submodules/65x02/nes6502/v1/26.json")]
    #[test_case("submodules/65x02/nes6502/v1/27.json")]
    #[test_case("submodules/65x02/nes6502/v1/28.json")]
    #[test_case("submodules/65x02/nes6502/v1/29.json")]
    #[test_case("submodules/65x02/nes6502/v1/2a.json")]
    #[test_case("submodules/65x02/nes6502/v1/2b.json")]
    #[test_case("submodules/65x02/nes6502/v1/2c.json")]
    #[test_case("submodules/65x02/nes6502/v1/2d.json")]
    #[test_case("submodules/65x02/nes6502/v1/2e.json")]
    #[test_case("submodules/65x02/nes6502/v1/2f.json")]
    #[test_case("submodules/65x02/nes6502/v1/30.json")]
    #[test_case("submodules/65x02/nes6502/v1/31.json")]
    #[test_case("submodules/65x02/nes6502/v1/33.json")]
    #[test_case("submodules/65x02/nes6502/v1/34.json")]
    #[test_case("submodules/65x02/nes6502/v1/35.json")]
    #[test_case("submodules/65x02/nes6502/v1/36.json")]
    #[test_case("submodules/65x02/nes6502/v1/37.json")]
    #[test_case("submodules/65x02/nes6502/v1/38.json")]
    #[test_case("submodules/65x02/nes6502/v1/39.json")]
    #[test_case("submodules/65x02/nes6502/v1/3a.json")]
    #[test_case("submodules/65x02/nes6502/v1/3b.json")]
    #[test_case("submodules/65x02/nes6502/v1/3c.json")]
    #[test_case("submodules/65x02/nes6502/v1/3d.json")]
    #[test_case("submodules/65x02/nes6502/v1/3e.json")]
    #[test_case("submodules/65x02/nes6502/v1/3f.json")]
    #[test_case("submodules/65x02/nes6502/v1/40.json")]
    #[test_case("submodules/65x02/nes6502/v1/41.json")]
    #[test_case("submodules/65x02/nes6502/v1/44.json")]
    #[test_case("submodules/65x02/nes6502/v1/45.json")]
    #[test_case("submodules/65x02/nes6502/v1/46.json")]
    #[test_case("submodules/65x02/nes6502/v1/48.json")]
    #[test_case("submodules/65x02/nes6502/v1/49.json")]
    #[test_case("submodules/65x02/nes6502/v1/4a.json")]
    #[test_case("submodules/65x02/nes6502/v1/4c.json")]
    #[test_case("submodules/65x02/nes6502/v1/4d.json")]
    #[test_case("submodules/65x02/nes6502/v1/4e.json")]
    #[test_case("submodules/65x02/nes6502/v1/50.json")]
    #[test_case("submodules/65x02/nes6502/v1/51.json")]
    #[test_case("submodules/65x02/nes6502/v1/54.json")]
    #[test_case("submodules/65x02/nes6502/v1/55.json")]
    #[test_case("submodules/65x02/nes6502/v1/56.json")]
    #[test_case("submodules/65x02/nes6502/v1/58.json")]
    #[test_case("submodules/65x02/nes6502/v1/59.json")]
    #[test_case("submodules/65x02/nes6502/v1/5a.json")]
    #[test_case("submodules/65x02/nes6502/v1/5c.json")]
    #[test_case("submodules/65x02/nes6502/v1/5d.json")]
    #[test_case("submodules/65x02/nes6502/v1/5e.json")]
    #[test_case("submodules/65x02/nes6502/v1/60.json")]
    #[test_case("submodules/65x02/nes6502/v1/61.json")]
    #[test_case("submodules/65x02/nes6502/v1/64.json")]
    #[test_case("submodules/65x02/nes6502/v1/65.json")]
    #[test_case("submodules/65x02/nes6502/v1/66.json")]
    #[test_case("submodules/65x02/nes6502/v1/68.json")]
    #[test_case("submodules/65x02/nes6502/v1/69.json")]
    #[test_case("submodules/65x02/nes6502/v1/6a.json")]
    #[test_case("submodules/65x02/nes6502/v1/6c.json")]
    #[test_case("submodules/65x02/nes6502/v1/6d.json")]
    #[test_case("submodules/65x02/nes6502/v1/6e.json")]
    #[test_case("submodules/65x02/nes6502/v1/70.json")]
    #[test_case("submodules/65x02/nes6502/v1/71.json")]
    #[test_case("submodules/65x02/nes6502/v1/74.json")]
    #[test_case("submodules/65x02/nes6502/v1/75.json")]
    #[test_case("submodules/65x02/nes6502/v1/76.json")]
    #[test_case("submodules/65x02/nes6502/v1/78.json")]
    #[test_case("submodules/65x02/nes6502/v1/79.json")]
    #[test_case("submodules/65x02/nes6502/v1/7a.json")]
    #[test_case("submodules/65x02/nes6502/v1/7c.json")]
    #[test_case("submodules/65x02/nes6502/v1/7d.json")]
    #[test_case("submodules/65x02/nes6502/v1/7e.json")]
    #[test_case("submodules/65x02/nes6502/v1/80.json")]
    #[test_case("submodules/65x02/nes6502/v1/81.json")]
    #[test_case("submodules/65x02/nes6502/v1/82.json")]
    #[test_case("submodules/65x02/nes6502/v1/84.json")]
    #[test_case("submodules/65x02/nes6502/v1/85.json")]
    #[test_case("submodules/65x02/nes6502/v1/86.json")]
    #[test_case("submodules/65x02/nes6502/v1/88.json")]
    #[test_case("submodules/65x02/nes6502/v1/89.json")]
    #[test_case("submodules/65x02/nes6502/v1/8a.json")]
    #[test_case("submodules/65x02/nes6502/v1/8c.json")]
    #[test_case("submodules/65x02/nes6502/v1/8d.json")]
    #[test_case("submodules/65x02/nes6502/v1/8e.json")]
    #[test_case("submodules/65x02/nes6502/v1/90.json")]
    #[test_case("submodules/65x02/nes6502/v1/91.json")]
    #[test_case("submodules/65x02/nes6502/v1/94.json")]
    #[test_case("submodules/65x02/nes6502/v1/95.json")]
    #[test_case("submodules/65x02/nes6502/v1/96.json")]
    #[test_case("submodules/65x02/nes6502/v1/98.json")]
    #[test_case("submodules/65x02/nes6502/v1/99.json")]
    #[test_case("submodules/65x02/nes6502/v1/9a.json")]
    #[test_case("submodules/65x02/nes6502/v1/9d.json")]
    #[test_case("submodules/65x02/nes6502/v1/a0.json")]
    #[test_case("submodules/65x02/nes6502/v1/a1.json")]
    #[test_case("submodules/65x02/nes6502/v1/a2.json")]
    #[test_case("submodules/65x02/nes6502/v1/a4.json")]
    #[test_case("submodules/65x02/nes6502/v1/a5.json")]
    #[test_case("submodules/65x02/nes6502/v1/a6.json")]
    #[test_case("submodules/65x02/nes6502/v1/a8.json")]
    #[test_case("submodules/65x02/nes6502/v1/a9.json")]
    #[test_case("submodules/65x02/nes6502/v1/aa.json")]
    #[test_case("submodules/65x02/nes6502/v1/ac.json")]
    #[test_case("submodules/65x02/nes6502/v1/ad.json")]
    #[test_case("submodules/65x02/nes6502/v1/ae.json")]
    #[test_case("submodules/65x02/nes6502/v1/b0.json")]
    #[test_case("submodules/65x02/nes6502/v1/b1.json")]
    #[test_case("submodules/65x02/nes6502/v1/b4.json")]
    #[test_case("submodules/65x02/nes6502/v1/b5.json")]
    #[test_case("submodules/65x02/nes6502/v1/b6.json")]
    #[test_case("submodules/65x02/nes6502/v1/b8.json")]
    #[test_case("submodules/65x02/nes6502/v1/b9.json")]
    #[test_case("submodules/65x02/nes6502/v1/ba.json")]
    #[test_case("submodules/65x02/nes6502/v1/bc.json")]
    #[test_case("submodules/65x02/nes6502/v1/bd.json")]
    #[test_case("submodules/65x02/nes6502/v1/be.json")]
    #[test_case("submodules/65x02/nes6502/v1/c0.json")]
    #[test_case("submodules/65x02/nes6502/v1/c1.json")]
    #[test_case("submodules/65x02/nes6502/v1/c2.json")]
    #[test_case("submodules/65x02/nes6502/v1/c4.json")]
    #[test_case("submodules/65x02/nes6502/v1/c5.json")]
    #[test_case("submodules/65x02/nes6502/v1/c6.json")]
    #[test_case("submodules/65x02/nes6502/v1/c8.json")]
    #[test_case("submodules/65x02/nes6502/v1/c9.json")]
    #[test_case("submodules/65x02/nes6502/v1/ca.json")]
    #[test_case("submodules/65x02/nes6502/v1/cc.json")]
    #[test_case("submodules/65x02/nes6502/v1/cd.json")]
    #[test_case("submodules/65x02/nes6502/v1/ce.json")]
    #[test_case("submodules/65x02/nes6502/v1/d0.json")]
    #[test_case("submodules/65x02/nes6502/v1/d1.json")]
    #[test_case("submodules/65x02/nes6502/v1/d4.json")]
    #[test_case("submodules/65x02/nes6502/v1/d5.json")]
    #[test_case("submodules/65x02/nes6502/v1/d6.json")]
    #[test_case("submodules/65x02/nes6502/v1/d8.json")]
    #[test_case("submodules/65x02/nes6502/v1/d9.json")]
    #[test_case("submodules/65x02/nes6502/v1/da.json")]
    #[test_case("submodules/65x02/nes6502/v1/dc.json")]
    #[test_case("submodules/65x02/nes6502/v1/dd.json")]
    #[test_case("submodules/65x02/nes6502/v1/de.json")]
    #[test_case("submodules/65x02/nes6502/v1/e0.json")]
    #[test_case("submodules/65x02/nes6502/v1/e1.json")]
    #[test_case("submodules/65x02/nes6502/v1/e2.json")]
    #[test_case("submodules/65x02/nes6502/v1/e4.json")]
    #[test_case("submodules/65x02/nes6502/v1/e5.json")]
    #[test_case("submodules/65x02/nes6502/v1/e6.json")]
    #[test_case("submodules/65x02/nes6502/v1/e8.json")]
    #[test_case("submodules/65x02/nes6502/v1/e9.json")]
    #[test_case("submodules/65x02/nes6502/v1/ea.json")]
    #[test_case("submodules/65x02/nes6502/v1/ec.json")]
    #[test_case("submodules/65x02/nes6502/v1/ed.json")]
    #[test_case("submodules/65x02/nes6502/v1/ee.json")]
    #[test_case("submodules/65x02/nes6502/v1/f0.json")]
    #[test_case("submodules/65x02/nes6502/v1/f1.json")]
    #[test_case("submodules/65x02/nes6502/v1/f4.json")]
    #[test_case("submodules/65x02/nes6502/v1/f5.json")]
    #[test_case("submodules/65x02/nes6502/v1/f6.json")]
    #[test_case("submodules/65x02/nes6502/v1/f8.json")]
    #[test_case("submodules/65x02/nes6502/v1/f9.json")]
    #[test_case("submodules/65x02/nes6502/v1/fa.json")]
    #[test_case("submodules/65x02/nes6502/v1/fc.json")]
    #[test_case("submodules/65x02/nes6502/v1/fd.json")]
    #[test_case("submodules/65x02/nes6502/v1/fe.json")]
    fn run_test_from_json(path: &str) {
        let tests_string = std::fs::read_to_string(path).unwrap();
        let tests = json::parse(tests_string.as_str()).unwrap();

        for i in 0..tests.len() {
            run_test(&tests[i]);
        }
    }

    fn run_test(test: &JsonValue) {
        let name = &test["name"];
        println!("Testing with instructions: {}", name);

        let (final_cpu, final_mapper) = parse_json_value(&test["final"]);

        let (mut cpu, mapper) = parse_json_value(&test["initial"]);

        let executed_cycles = cpu.execute_next_instruction().executed_cycles;
        let expected_cycles = test["cycles"].members().count() as u8;
        assert_eq!(
            executed_cycles, expected_cycles,
            "Executed cycles don't match\n expected: {}\n   actual: {}",
            expected_cycles, executed_cycles
        );

        assert_eq!(
            cpu.program_counter, final_cpu.program_counter,
            "Program counter values don't match\n expected: {}\n   actual: {}",
            final_cpu.program_counter, cpu.program_counter
        );
        assert_eq!(
            cpu.register_a, final_cpu.register_a,
            "Register a values don't match\n expected: {}\n   actual: {}",
            final_cpu.register_a, cpu.register_a
        );

        assert_eq!(cpu.status, final_cpu.status, "\nStatus flags don't match\n            NV_BDIZC\n expected : {:08b}\n   actual : {:08b}", final_cpu.status, cpu.status);

        assert_eq!(
            cpu.register_x, final_cpu.register_x,
            "Register x values don't match\n expected: {}\n   actual: {}",
            final_cpu.register_x, cpu.register_x
        );
        assert_eq!(
            cpu.register_y, final_cpu.register_y,
            "Register y values don't match"
        );

        assert_eq!(
            cpu.stack_pointer, final_cpu.stack_pointer,
            "Stack pointer values don't match\n expected: {}\n   actual: {}",
            final_cpu.stack_pointer, cpu.stack_pointer
        );

        assert_eq!(mapper, final_mapper, "Memories don't match!",);
    }

    fn parse_json_value(json_value: &JsonValue) -> (CPU, Rc<RefCell<TestMapper>>) {
        let mapper = Rc::new(RefCell::new(TestMapper::new()));
        for ram_tuple in json_value["ram"].members() {
            mapper.borrow_mut().write_u8(
                ram_tuple[0].as_u16().unwrap(),
                ram_tuple[1].as_u8().unwrap(),
            );
        }
        let mut cpu = CPU::new(mapper.clone());

        cpu.program_counter = json_value["pc"].as_u16().unwrap();
        cpu.status = json_value["p"].as_u8().unwrap();
        cpu.register_a = json_value["a"].as_u8().unwrap();
        cpu.register_x = json_value["x"].as_u8().unwrap();
        cpu.register_y = json_value["y"].as_u8().unwrap();
        cpu.stack_pointer = json_value["s"].as_u8().unwrap();

        return (cpu, mapper.clone());
    }

    #[test]
    fn test_example() {
        let test_case  = "{ \"name\": \"03 6e 78\", \"initial\": { \"pc\": 63085, \"s\": 16, \"a\": 140, \"x\": 122, \"y\": 205, \"p\": 38, \"ram\": [ [63085, 3], [63086, 110], [63087, 120], [110, 248], [232, 110], [233, 246]]}, \"final\": { \"pc\": 63087, \"s\": 16, \"a\": 220, \"x\": 122, \"y\": 205, \"p\": 164, \"ram\": [ [110, 248], [232, 110], [233, 246], [63085, 3], [63086, 220], [63087, 120]]}, \"cycles\": [ [63085, 3, \"read\"], [63086, 110, \"read\"], [110, 248, \"read\"], [232, 110, \"read\"], [233, 246, \"read\"], [63086, 110, \"read\"], [63086, 110, \"write\"], [63086, 220, \"write\"]] }";
        let json_value = json::parse(test_case).unwrap();
        run_test(&json_value);
    }
}
