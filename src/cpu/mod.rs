mod instructions;

use std::ops::{Add, Shl};

use crate::cpu::instructions::*;

type ExitCode = u8;

#[derive(Debug)]
pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: u8,
    pub program_counter: u16,
    memory: [u8; 0xFFFF],
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0, // todo: check reference, should this be initialized?
            register_y: 0,
            status: 0, // todo: according to nesdev wiki, the 5th bit is always 1, https://www.nesdev.org/wiki/Status_flags
            program_counter: 0,
            memory: [0; 0xFFFF], // should everything be initialized to zero?
        }
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run();
    }

    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.status = 0;
        self.program_counter = self.mem_read_u16(0xFFFC);
    }

    fn run(&mut self) {
        loop {
            let opcode = self.memory[self.program_counter as usize];
            self.program_counter += 1;

            let instruction = &INSTRUCTIONS[&opcode];

            if let Some(_) = self.execute(&instruction) {
                return;
            }
            self.program_counter += (instruction.bytes as u16) - 1;
        }
    }

    fn execute(&mut self, instruction: &Instruction) -> Option<ExitCode> {
        match instruction.name {
            "ADC" => {
                self.adc(&instruction.addressing_mode);
                None
            }
            "AND" => {
                self.and(&instruction.addressing_mode);
                None
            }
            "BRK" => Some(0),
            "LDA" => {
                self.lda(&instruction.addressing_mode);
                None
            }
            "TAX" => {
                self.tax();
                None
            }
            "INX" => {
                self.inx();
                None
            }
            _ => {
                todo!();
            }
        }
    }

    fn load(&mut self, program: Vec<u8>) {
        self.memory[0x8000..(0x8000 + program.len())].copy_from_slice(&program[..]);
        self.mem_write_u16(0xFFFC, 0x8000);
    }

    fn mem_read(&self, address: u16) -> u8 {
        return self.memory[address as usize];
    }

    fn mem_write(&mut self, address: u16, data: u8) {
        self.memory[address as usize] = data;
    }

    fn mem_read_u16(&self, address: u16) -> u16 {
        let index = address as usize;
        return u16::from_le_bytes([self.memory[index], self.memory[index + 1]]);
    }

    fn mem_write_u16(&mut self, address: u16, data: u16) {
        let bytes = data.to_le_bytes();
        let index = address as usize;
        self.memory[index] = bytes[0];
        println!("Writing {:#01x} to address {:#01x}", bytes[0], index);
        self.memory[index + 1] = bytes[1];
        println!("Writing {:#01x} to address {:#01x}", bytes[1], index + 1);
    }

    fn set_carry_flag(&mut self) {
        self.status = self.status | 0b0000_0001;
    }

    fn get_carry_flag(&self) -> u8 {
        self.status & 0b0000_0001
    }

    fn clear_carry_flag(&mut self) {
        self.status = self.status & 0b1111_1110;
    }

    fn get_operand_address(&mut self, addressing_mode: &AddressingMode) -> u16 {
        match addressing_mode {
            AddressingMode::Implicit => {
                panic!("Cannot get operand address when the Addressing Mode is Implicit");
            }
            AddressingMode::Accumulator => {
                panic!("Cannot get operand address when the Addressing Mode is Accumulator");
            }
            AddressingMode::Immediate => self.program_counter,
            AddressingMode::ZeroPage => self.mem_read(self.program_counter) as u16,
            AddressingMode::ZeroPage_X => self
                .mem_read(self.program_counter)
                .wrapping_add(self.register_x) as u16,
            AddressingMode::ZeroPage_Y => self
                .mem_read(self.program_counter)
                .wrapping_add(self.register_y) as u16,
            AddressingMode::Absolute => self.mem_read_u16(self.program_counter),
            AddressingMode::Absolute_X => self
                .mem_read_u16(self.program_counter)
                .wrapping_add(self.register_x as u16),
            AddressingMode::Absolute_Y => self
                .mem_read_u16(self.program_counter)
                .wrapping_add(self.register_y as u16),
            AddressingMode::Indirect => {
                let indirect_address = self.mem_read_u16(self.program_counter);
                self.mem_read_u16(indirect_address)
            }
            AddressingMode::Indexed_Idirect_X => {
                let indirect_address = self
                    .mem_read(self.program_counter)
                    .wrapping_add(self.register_x);
                self.mem_read_u16(indirect_address as u16)
            }
            AddressingMode::Indexed_Idirect_Y => {
                let indirect_address = self
                    .mem_read(self.program_counter)
                    .wrapping_add(self.register_y);
                self.mem_read_u16(indirect_address as u16)
            }
            AddressingMode::Indirect_indexed_X => {
                let indirect_address = self.mem_read_u16(self.program_counter);
                self.mem_read_u16(indirect_address)
                    .wrapping_add(self.register_x as u16)
            }
            AddressingMode::Indirect_indexed_Y => {
                let indirect_address = self.mem_read_u16(self.program_counter);
                self.mem_read_u16(indirect_address)
                    .wrapping_add(self.register_y as u16)
            }
        }
    }

    fn adc(&mut self, addressing_mode: &AddressingMode) {
        let index = self.get_operand_address(addressing_mode) as usize;
        let carry = self.get_carry_flag();
        let (temp_sum, overflow_occured_on_first_addition) =
            self.register_a.overflowing_add(self.memory[index]);
        let (final_sum, overflow_occured_on_second_addition) = temp_sum.overflowing_add(carry);
        self.register_a = final_sum;
        if (overflow_occured_on_first_addition || overflow_occured_on_second_addition) {
            self.set_carry_flag();
        } else {
            self.clear_carry_flag()
        };
        self.update_negative_flag(self.register_a);
        self.update_zero_flag(self.register_a);
    }

    fn and(&mut self, addressing_mode: &AddressingMode) {
        let index = self.get_operand_address(addressing_mode);
        self.register_a = self.register_a & self.memory[index as usize];
        self.update_zero_flag(self.register_a);
        self.update_negative_flag(self.register_a);
    }

    fn lda(&mut self, addressing_mode: &AddressingMode) {
        let index = self.get_operand_address(addressing_mode);
        self.register_a = self.memory[index as usize];
        self.update_zero_flag(self.register_a);
        self.update_negative_flag(self.register_a);
    }

    fn tax(&mut self) {
        self.register_x = self.register_a;

        self.update_zero_flag(self.register_x);
        self.update_negative_flag(self.register_x);
    }

    fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.update_zero_flag(self.register_a);
        self.update_negative_flag(self.register_a);
    }

    fn update_zero_flag(&mut self, register: u8) {
        if register == 0 {
            self.status = self.status | 0b0000_0010;
        } else {
            self.status = self.status & 0b1111_1101;
        }
    }

    fn update_negative_flag(&mut self, register: u8) {
        if register & 0b1000_0000 != 0 {
            self.status = self.status | 0b1000_0000;
        } else {
            self.status = self.status & 0b0111_1111;
        }
    }
}

#[cfg(test)]
mod test_cpu {
    use super::*;
    use test_case::test_case;

    #[test_case(0b0, 0b0, 0b0000_0010)]
    #[test_case(0b0000_0010, 0b10, 0b0)]
    fn update_zero_flag_test_case(status: u8, register: u8, expected: u8) {
        let mut cpu = CPU::new();
        cpu.update_zero_flag(register);
        assert_eq!(cpu.status, expected);
    }

    #[test]
    fn lda_correctly_sets_negative_flag() {
        let program = vec![0xa9, 0x05, 0x00];
        let mut cpu = CPU::new();
        cpu.load_and_run(program);
        assert!(cpu.status & 0b0000_0010 == 0b00);
        assert!(cpu.status & 0b1000_0000 == 0);
        // todo: add test case where the negative flag is 1
    }

    // todo: add test to check lda loads to register_a

    #[test]
    fn tax_correctly_updates_register_x() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x010;
        cpu.tax();
        assert!(cpu.register_a == cpu.register_x);
    }

    #[test]
    fn tax_correctly_sets_zero_flag() {
        let mut cpu = CPU::new();
        cpu.register_a = 0;
        cpu.tax();
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }

    #[test]
    fn tax_correctly_sets_negative_flag() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x05;
        cpu.tax();
        assert!(cpu.status & 0b0000_0010 == 0b00);
        assert!(cpu.status & 0b1000_0000 == 0);
        // todo: add test case where the negative flag is 1
    }

    #[test]
    fn inx_increments_the_x_register() {
        let mut cpu = CPU::new();
        cpu.register_x = 0x00;
        cpu.inx();
        assert_eq!(cpu.register_x, 0x01);
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);
        assert_eq!(cpu.register_x, 0xc1)
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.register_x = 0xff;
        cpu.inx();
        cpu.inx();
        assert_eq!(cpu.register_x, 1)
    }

    #[test]
    fn test_mem_read() {
        let mut cpu = CPU::new();
        cpu.memory[0x00AA] = 12;
        assert_eq!(cpu.mem_read(0x00AA), 12);
    }

    #[test]
    fn test_mem_write() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x00AA, 12);
        assert_eq!(cpu.memory[0x00AA], 12);
    }

    #[test]
    fn test_mem_write_u16() {
        let mut cpu = CPU::new();
        cpu.mem_write_u16(0x00AA, 0x8000);
        assert_eq!(cpu.memory[0x00AA], 0x00);
        assert_eq!(cpu.memory[0x00AB], 0x80);
    }

    #[test]
    fn test_mem_read_u16() {
        let mut cpu = CPU::new();
        cpu.memory[0x00AA] = 0x00;
        cpu.memory[0x00AB] = 0x80;
        assert_eq!(cpu.mem_read_u16(0x00AA), 0x8000);
    }

    #[test_case(0b0000_0000, 0x85, 0x03, 0x88, 0b0000_0000)]
    #[test_case(0b0000_0000, 0x85, 0x99, 0x1E, 0b0000_0001)]
    #[test_case(0b0000_0001, 0x5F, 0x42, 0xA2, 0b0000_0000)]
    fn test_adc(status: u8, acc: u8, nn: u8, expected_acc: u8, expected_carry_flag: u8) {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x8000;
        cpu.register_a = acc;
        cpu.memory[0x8000] = nn;
        cpu.status = status;

        cpu.adc(&AddressingMode::Immediate);
        assert_eq!(cpu.register_a, expected_acc);
        assert_eq!(cpu.status & 0b0000_0001, expected_carry_flag)
    }

    #[test]
    fn test_and() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b1001_1001;
        cpu.memory[0xFF00 as usize] = 0b1111_1111;
        cpu.program_counter = 0xFF00;

        let addressing_mode = AddressingMode::Immediate;
        cpu.and(&addressing_mode);

        assert_eq!(cpu.register_a, 0b1001_1001);
    }

    #[test]
    fn test_addressing_mode_immediate() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x8000;
        let result = cpu.get_operand_address(&AddressingMode::Immediate);
        assert_eq!(result, 0x8000);
    }

    #[test]
    fn test_addressing_mode_zero_page() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0xAAAA;
        cpu.mem_write(0xAAAA, 0xAA);
        let result = cpu.get_operand_address(&AddressingMode::ZeroPage);
        assert_eq!(result, 0xAA);
    }

    #[test]
    fn test_addressing_mode_zero_page_x() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0xAAAA;
        cpu.mem_write(0xAAAA, 0x80);
        cpu.register_x = 0xFF;
        let result = cpu.get_operand_address(&AddressingMode::ZeroPage_X);
        assert_eq!(result, 0x7F);
    }

    #[test]
    fn test_addressing_mode_zero_page_y() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0xAAAA;
        cpu.mem_write(0xAAAA, 0x80);
        cpu.register_y = 0xFF;
        let result = cpu.get_operand_address(&AddressingMode::ZeroPage_Y);
        assert_eq!(result, 0x7F);
    }

    #[test]
    fn test_addressing_mode_absolute() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0xAAAA;
        cpu.mem_write_u16(0xAAAA, 0x8000);
        let result = cpu.get_operand_address(&AddressingMode::Absolute);
        assert_eq!(result, 0x8000);
    }

    #[test]
    fn test_addressing_mode_absolute_x() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0xAAAA;
        cpu.register_x = 0x80;
        cpu.mem_write_u16(0xAAAA, 0x8000);
        let result = cpu.get_operand_address(&AddressingMode::Absolute_X);
        assert_eq!(result, 0x8080);
    }

    #[test]
    fn test_addressing_mode_absolute_y() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0xAAAA;
        cpu.register_y = 0x80;
        cpu.mem_write_u16(0xAAAA, 0x8000);
        let result = cpu.get_operand_address(&AddressingMode::Absolute_Y);
        assert_eq!(result, 0x8080);
    }

    #[test]
    fn test_addressing_mode_indirect() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x8000;
        cpu.mem_write_u16(0x8000, 0x0120);
        cpu.mem_write_u16(0x0120, 0xBAFC);
        let result = cpu.get_operand_address(&AddressingMode::Indirect);
        assert_eq!(result, 0xBAFC);
    }

    #[test]
    fn test_addressing_mode_indexed_indirect_x() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x8000;
        cpu.mem_write(0x8000, 0x20);
        cpu.mem_write_u16(0x0021, 0xBAFC);
        cpu.register_x = 0x01;
        let result = cpu.get_operand_address(&AddressingMode::Indexed_Idirect_X);
        assert_eq!(result, 0xBAFC);
    }

    #[test]
    fn test_addressing_mode_indexed_indirect_y() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x8000;
        cpu.mem_write(0x8000, 0x20);
        cpu.mem_write_u16(0x0021, 0xBAFC);
        cpu.register_y = 0x01;
        let result = cpu.get_operand_address(&AddressingMode::Indexed_Idirect_Y);
        assert_eq!(result, 0xBAFC);
    }

    #[test]
    fn test_addressing_mode_indirect_indexed_x() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x8000;
        cpu.mem_write(0x8000, 0x52);
        cpu.mem_write_u16(0x0052, 0xEF05);
        cpu.register_x = 0x03;

        let result = cpu.get_operand_address(&AddressingMode::Indirect_indexed_X);
        assert_eq!(result, 0xEF08);
    }

    #[test]
    fn test_addressing_mode_indirect_indexed_y() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x8000;
        cpu.mem_write(0x8000, 0x52);
        cpu.mem_write_u16(0x0052, 0xEF05);
        cpu.register_y = 0x03;

        let result = cpu.get_operand_address(&AddressingMode::Indirect_indexed_Y);
        assert_eq!(result, 0xEF08);
    }
}
