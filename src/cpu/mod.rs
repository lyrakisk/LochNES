mod instructions;

use crate::cpu::instructions::*;

const STATUS_FLAG_MASK_NEGATIVE: u8 = 0b10000000;
const STATUS_FLAG_MASK_OVERFLOW: u8 = 0b01000000;
const STATUS_FLAG_MASK_BREAK_COMMAND: u8 = 0b0001_0000;
const STATUS_FLAG_MASK_DECIMAL: u8 = 0b0000_1000;
const STATUS_FLAG_INTERRUPT_DISABLE: u8 = 0b0000_0100;
const STATUS_FLAG_MASK_ZERO: u8 = 0b00000010;
const STATUS_FLAG_MASK_CARRY: u8 = 0b00000001;

#[derive(Debug, PartialEq)]
enum FlagStates {
    CLEAR = 0,
    SET = 1,
}

enum InstructionExecutionError {
    INTERRUPT_HANDLING_NOT_IMPLEMENTED,
}

#[derive(Clone, Debug)]
pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: u8,
    pub program_counter: u16,
    pub stack_pointer: u8,
    memory: [u8; 65536],
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0, // todo: check reference, should this be initialized?
            register_y: 0,
            status: 0, // todo: according to nesdev wiki, the 5th bit is always 1, https://www.nesdev.org/wiki/Status_flags
            program_counter: 0x8000,
            stack_pointer: 0xFF,
            memory: [0; 65536], // should everything be initialized to zero?
        }
    }

    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.status = 0;
        self.program_counter = self.mem_read_u16(0xFFFC);
    }

    pub fn run(&mut self) {
        loop {
            match self.execute_next_instruction() {
                Err(_) => {
                    return;
                }
                Ok(()) => {
                    continue;
                }
            }
        }
    }

    fn execute_next_instruction(&mut self) -> Result<(), u8> {
        let opcode = self.fetch();

        let instruction = self.decode(opcode);

        if instruction.is_none() {
            return Err(1);
        }

        let instruction_unwrapped = instruction.unwrap();

        match self.execute(instruction_unwrapped.clone()) {
            Ok(()) => (),
            _ => {
                return Ok(());
            }
        }

        self.update_program_counter(instruction_unwrapped);
        Ok(())
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
        let opcode = self.memory[self.program_counter as usize];
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

    fn execute(&mut self, instruction: Instruction) -> Result<(), InstructionExecutionError> {
        match instruction.name {
            "ADC" => {
                self.adc(&instruction.addressing_mode);
                Ok(())
            }
            "AND" => {
                self.and(&instruction.addressing_mode);
                Ok(())
            }
            "ASL" => {
                self.asl(&instruction.addressing_mode);
                Ok(())
            }
            "BCC" => {
                self.bcc();
                Ok(())
            }
            "BCS" => {
                self.bcs();
                Ok(())
            }
            "BEQ" => {
                self.beq();
                Ok(())
            }
            "BIT" => {
                self.bit(&instruction.addressing_mode);
                Ok(())
            }
            "BMI" => {
                self.bmi();
                Ok(())
            }
            "BNE" => {
                self.bne();
                Ok(())
            }
            "BPL" => {
                self.bpl();
                Ok(())
            }
            "BVC" => {
                self.bvc();
                Ok(())
            }
            "BVS" => {
                self.bvs();
                Ok(())
            }
            "CLC" => {
                self.clc();
                Ok(())
            }
            "CLD" => {
                self.cld();
                Ok(())
            }
            "CLI" => {
                self.cli();
                Ok(())
            }
            "CLV" => {
                self.clv();
                Ok(())
            }
            "CMP" => {
                self.cmp(&instruction.addressing_mode);
                Ok(())
            }
            "CPX" => {
                self.cpx(&instruction.addressing_mode);
                Ok(())
            }
            "CPY" => {
                self.cpy(&instruction.addressing_mode);
                Ok(())
            }
            "DEC" => {
                self.dec(&instruction.addressing_mode);
                Ok(())
            }
            "DEX" => {
                self.dex();
                Ok(())
            }
            "EOR" => {
                self.eor(&instruction.addressing_mode);
                Ok(())
            }
            "BRK" => {
                self.brk();
                Ok(())
            }
            "LDA" => {
                self.lda(&instruction.addressing_mode);
                Ok(())
            }
            "LDX" => {
                self.ldx(&instruction.addressing_mode);
                Ok(())
            }
            "LDY" => {
                self.ldy(&instruction.addressing_mode);
                Ok(())
            }
            "LSR" => {
                self.lsr(&instruction.addressing_mode);
                Ok(())
            }
            "NOP" => Ok(()),
            "JMP" => {
                self.jmp(&instruction.addressing_mode);
                Ok(())
            }
            "JSR" => {
                self.jsr(&instruction.addressing_mode);
                Ok(())
            }
            "ORA" => {
                self.ora(&instruction.addressing_mode);
                Ok(())
            }
            "PHP" => {
                self.php();
                Ok(())
            }
            "PLP" => {
                self.plp();
                Ok(())
            }
            "INC" => {
                self.inc(&instruction.addressing_mode);
                Ok(())
            }
            "INX" => {
                self.inx();
                Ok(())
            }
            "ROL" => {
                self.rol(&instruction.addressing_mode);
                Ok(())
            }
            "RTS" => {
                self.rts();
                Ok(())
            }
            "SBC" => {
                self.sbc(&instruction.addressing_mode);
                Ok(())
            }
            "SEC" => {
                self.sec();
                Ok(())
            }
            "STA" => {
                self.sta(&instruction.addressing_mode);
                Ok(())
            }
            "STY" => {
                self.sty(&instruction.addressing_mode);
                Ok(())
            }
            "TAX" => {
                self.tax();
                Ok(())
            }
            "TAY" => {
                self.tay();
                Ok(())
            }
            "TXA" => {
                self.txa();
                Ok(())
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

    fn stack_pop(&mut self) -> u8 {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        return self.mem_read(0x0100 + (self.stack_pointer as u16));
    }
    fn stack_pop_u16(&mut self) -> u16 {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        let low_order_byte = self.mem_read(0x0100 + (self.stack_pointer as u16));
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        let high_order_byte = self.mem_read(0x0100 + (self.stack_pointer as u16));

        return u16::from_le_bytes([low_order_byte, high_order_byte]).wrapping_add(1);
    }

    fn stack_push(&mut self, data: u8) {
        self.mem_write(0x0100 + (self.stack_pointer as u16), data);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    }
    fn stack_push_u16(&mut self, data: u16) {
        let bytes = data.to_le_bytes();
        self.mem_write(0x0100 + (self.stack_pointer as u16), bytes[1]);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
        self.mem_write(0x0100 + (self.stack_pointer as u16), bytes[0]);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    }

    // consider returning reference to memory instead of copying,
    // because some instructions need to update it in place
    pub fn mem_read(&self, address: u16) -> u8 {
        return self.memory[address as usize];
    }

    pub fn mem_write(&mut self, address: u16, data: u8) {
        self.memory[address as usize] = data;
    }

    fn mem_read_u16(&self, address: u16) -> u16 {
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

    fn mem_write_u16(&mut self, address: u16, data: u16) {
        let bytes = data.to_le_bytes();
        let index = address as usize;
        self.memory[index] = bytes[0];
        println!("Writing {:#01x} to address {:#01x}", bytes[0], index);
        self.memory[index + 1] = bytes[1];
        println!("Writing {:#01x} to address {:#01x}", bytes[1], index + 1); // TODO: wrapping add
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

    fn get_operand_address(&mut self, addressing_mode: &AddressingMode) -> u16 {
        match addressing_mode {
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
                let indirect_adress = self.mem_read_u16(self.program_counter);
                let low_order_address = indirect_adress;

                /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
                //  An original 6502 has does not correctly fetch the target address if the indirect vector falls on a page boundary. (source: NES DEV wiki)
                //  E.g. If the indirect vector falls on $02FF, then the first byte is found at $02FF as expected,
                //  but the second byte will be at $0200 instead of $0300.
                //  From the extensive test cases on 6c.json, check example `6c ff f5`.
                //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
                let high_order_address = match indirect_adress & 0b0000_0000_1111_1111 {
                    0xFF => indirect_adress & 0b1111_1111_0000_0000,
                    _ => indirect_adress.wrapping_add(1),
                };

                u16::from_le_bytes([
                    self.memory[low_order_address as usize],
                    self.memory[high_order_address as usize],
                ])
            }
            AddressingMode::Indexed_Indirect_X => {
                let indirect_address = self
                    .mem_read(self.program_counter)
                    .wrapping_add(self.register_x);
                self.zero_page_read_u16(indirect_address)
            }
            AddressingMode::Indexed_Indirect_Y => {
                let indirect_address = self
                    .mem_read(self.program_counter)
                    .wrapping_add(self.register_y);
                self.zero_page_read_u16(indirect_address)
            }
            AddressingMode::Indirect_indexed_X => {
                let indirect_address = self.mem_read(self.program_counter);
                self.zero_page_read_u16(indirect_address)
                    .wrapping_add(self.register_x as u16)
            }
            AddressingMode::Indirect_indexed_Y => {
                let indirect_address = self.mem_read(self.program_counter);
                self.zero_page_read_u16(indirect_address)
                    .wrapping_add(self.register_y as u16)
            }
            _ => {
                panic!(
                    "Cannot get operand address when the Addressing Mode is {:?}",
                    addressing_mode
                );
            }
        }
    }

    fn get_operand(&mut self, addressing_mode: &AddressingMode) -> u8 {
        match addressing_mode {
            AddressingMode::Accumulator => self.register_a,
            _ => {
                let index = self.get_operand_address(addressing_mode);
                self.memory[index as usize]
            }
        }
    }

    fn branch_off_program_counter(&mut self, distance: u16) {
        if distance > 0x7F {
            let distance = 0xff_u16.wrapping_sub(distance).wrapping_add(1);
            self.program_counter = self.program_counter.wrapping_sub(distance);
        } else {
            let distance = distance;
            self.program_counter = self.program_counter.wrapping_add(distance);
        }
    }

    fn adc(&mut self, addressing_mode: &AddressingMode) {
        let operand = self.get_operand(addressing_mode);
        let register_a_sign = self.register_a & 0b1000_0000;
        let operand_sign = operand & 0b1000_0000;
        let carry = self.get_flag_state(STATUS_FLAG_MASK_CARRY);
        let (temp_sum, overflow_occured_on_first_addition) =
            self.register_a.overflowing_add(operand);
        let (final_sum, overflow_occured_on_second_addition) =
            temp_sum.overflowing_add(carry as u8);
        self.register_a = final_sum;
        if overflow_occured_on_first_addition || overflow_occured_on_second_addition {
            self.set_flag(STATUS_FLAG_MASK_CARRY);
            self.set_flag(STATUS_FLAG_MASK_OVERFLOW);
        } else {
            self.clear_flag(STATUS_FLAG_MASK_CARRY)
        };

        let result_sign = self.register_a & 0b1000_0000;
        if register_a_sign == operand_sign && result_sign != register_a_sign {
            self.set_flag(STATUS_FLAG_MASK_OVERFLOW);
        } else {
            self.clear_flag(STATUS_FLAG_MASK_OVERFLOW);
        }

        self.update_negative_flag(self.register_a);
        self.update_zero_flag(self.register_a);
    }

    fn and(&mut self, addressing_mode: &AddressingMode) {
        let operand = self.get_operand(addressing_mode);
        self.register_a = self.register_a & operand;
        self.update_zero_flag(self.register_a);
        self.update_negative_flag(self.register_a);
    }

    fn asl(&mut self, addressing_mode: &AddressingMode) {
        // code duplication, almost identical to lsr

        let operand: &mut u8;
        match addressing_mode {
            AddressingMode::Accumulator => {
                operand = &mut self.register_a;
            }
            _ => {
                let index = self.get_operand_address(addressing_mode) as usize;
                operand = &mut self.memory[index];
            }
        }
        let operand_most_significant_bit = ((*operand) & 0b1000_0000) >> 7;
        let result = *operand << 1;

        *operand = result;

        self.update_zero_flag(result);
        self.update_negative_flag(result);

        if operand_most_significant_bit == 1 {
            self.set_flag(STATUS_FLAG_MASK_CARRY);
        } else {
            self.clear_flag(STATUS_FLAG_MASK_CARRY);
        }
    }

    fn bcc(&mut self) {
        match self.get_flag_state(STATUS_FLAG_MASK_CARRY) {
            FlagStates::CLEAR => {
                let distance = self.mem_read(self.program_counter);
                self.branch_off_program_counter(distance as u16);
            }
            FlagStates::SET => {
                return;
            }
        }
    }

    fn bcs(&mut self) {
        match self.get_flag_state(STATUS_FLAG_MASK_CARRY) {
            FlagStates::SET => {
                let distance = self.mem_read(self.program_counter);
                self.branch_off_program_counter(distance as u16);
            }
            FlagStates::CLEAR => {
                return;
            }
        }
    }

    fn beq(&mut self) {
        match self.get_flag_state(STATUS_FLAG_MASK_ZERO) {
            FlagStates::SET => {
                let distance = self.mem_read(self.program_counter);
                self.branch_off_program_counter(distance as u16);
            }
            FlagStates::CLEAR => {
                return;
            }
        }
    }

    fn bit(&mut self, addressing_mode: &AddressingMode) {
        let operand = self.get_operand(addressing_mode);
        let result = self.register_a & operand;

        self.update_zero_flag(result);
        self.update_negative_flag(operand);

        if (operand & 0b0100_0000 == 0b0100_0000) {
            self.set_flag(STATUS_FLAG_MASK_OVERFLOW);
        } else {
            self.clear_flag(STATUS_FLAG_MASK_OVERFLOW);
        }
    }

    fn bmi(&mut self) {
        match self.get_flag_state(STATUS_FLAG_MASK_NEGATIVE) {
            FlagStates::SET => {
                let distance = self.mem_read(self.program_counter);
                self.branch_off_program_counter(distance as u16);
            }
            FlagStates::CLEAR => {
                return;
            }
        }
    }

    fn bne(&mut self) {
        match self.get_flag_state(STATUS_FLAG_MASK_ZERO) {
            FlagStates::CLEAR => {
                let distance = self.mem_read(self.program_counter);
                self.branch_off_program_counter(distance as u16);
            }
            FlagStates::SET => {
                return;
            }
        }
    }

    fn bpl(&mut self) {
        match self.get_flag_state(STATUS_FLAG_MASK_NEGATIVE) {
            FlagStates::CLEAR => {
                let distance = self.mem_read(self.program_counter);
                self.branch_off_program_counter(distance as u16);
            }
            FlagStates::SET => {
                return;
            }
        }
    }

    fn bvc(&mut self) {
        match self.get_flag_state(STATUS_FLAG_MASK_OVERFLOW) {
            FlagStates::CLEAR => {
                let distance = self.mem_read(self.program_counter);
                self.branch_off_program_counter(distance as u16);
            }
            FlagStates::SET => {
                return;
            }
        }
    }

    fn bvs(&mut self) {
        match self.get_flag_state(STATUS_FLAG_MASK_OVERFLOW) {
            FlagStates::SET => {
                let distance = self.mem_read(self.program_counter);
                self.branch_off_program_counter(distance as u16);
            }
            FlagStates::CLEAR => {
                return;
            }
        }
    }

    fn brk(&mut self) {
        let interrupt_vector = self.mem_read_u16(0xFFFE);
        self.stack_push_u16(self.program_counter.wrapping_add(1));
        self.stack_push(self.status | 0b0001_0000);
        self.set_flag(STATUS_FLAG_INTERRUPT_DISABLE);
        self.program_counter = interrupt_vector;
    }

    fn clc(&mut self) {
        self.clear_flag(STATUS_FLAG_MASK_CARRY);
    }

    fn cld(&mut self) {
        self.clear_flag(STATUS_FLAG_MASK_DECIMAL);
    }

    fn cli(&mut self) {
        self.clear_flag(STATUS_FLAG_INTERRUPT_DISABLE);
    }
    fn clv(&mut self) {
        self.clear_flag(STATUS_FLAG_MASK_OVERFLOW);
    }

    fn cmp(&mut self, addressing_mode: &AddressingMode) {
        let (result, overflow_occured) = self
            .register_a
            .overflowing_sub(self.get_operand(&addressing_mode));

        if overflow_occured {
            self.clear_flag(STATUS_FLAG_MASK_CARRY);
        } else {
            self.set_flag(STATUS_FLAG_MASK_CARRY);
        }

        self.update_zero_flag(result);
        self.update_negative_flag(result);
    }

    fn cpx(&mut self, addressing_mode: &AddressingMode) {
        // todo: remove code duplication, almost similar to cmp, cpy
        let (result, overflow_occured) = self
            .register_x
            .overflowing_sub(self.get_operand(&addressing_mode));

        if overflow_occured {
            self.clear_flag(STATUS_FLAG_MASK_CARRY);
        } else {
            self.set_flag(STATUS_FLAG_MASK_CARRY);
        }

        self.update_zero_flag(result);
        self.update_negative_flag(result);
    }

    fn cpy(&mut self, addressing_mode: &AddressingMode) {
        // todo: remove code duplication, almost similar to cmp, cpx
        let (result, overflow_occured) = self
            .register_y
            .overflowing_sub(self.get_operand(&addressing_mode));

        if overflow_occured {
            self.clear_flag(STATUS_FLAG_MASK_CARRY);
        } else {
            self.set_flag(STATUS_FLAG_MASK_CARRY);
        }

        self.update_zero_flag(result);
        self.update_negative_flag(result);
    }

    fn dec(&mut self, addressing_mode: &AddressingMode) {
        let address = self.get_operand_address(addressing_mode);
        let result = self.mem_read(address).wrapping_sub(1);
        self.mem_write(address, result);
        self.update_zero_flag(result);
        self.update_negative_flag(result);
    }

    fn dex(&mut self) {
        self.register_x = self.register_x.wrapping_sub(1);
        self.update_zero_flag(self.register_x);
        self.update_negative_flag(self.register_x);
    }

    fn eor(&mut self, addressing_mode: &AddressingMode) {
        let operand = self.get_operand(addressing_mode);
        self.register_a = self.register_a ^ operand;
        self.update_zero_flag(self.register_a);
        self.update_negative_flag(self.register_a);
    }

    fn lda(&mut self, addressing_mode: &AddressingMode) {
        // todo: remove duplicate code, same as ldx() and ldy()
        let operand = self.get_operand(addressing_mode);
        self.register_a = operand;
        self.update_zero_flag(self.register_a);
        self.update_negative_flag(self.register_a);
    }

    fn ldx(&mut self, addressing_mode: &AddressingMode) {
        let operand = self.get_operand(addressing_mode);
        self.register_x = operand;
        self.update_zero_flag(self.register_x);
        self.update_negative_flag(self.register_x);
    }

    fn ldy(&mut self, addressing_mode: &AddressingMode) {
        let operand = self.get_operand(addressing_mode);
        self.register_y = operand;
        self.update_zero_flag(self.register_y);
        self.update_negative_flag(self.register_y);
    }

    fn lsr(&mut self, addressing_mode: &AddressingMode) {
        let operand: &mut u8;
        match addressing_mode {
            AddressingMode::Accumulator => {
                operand = &mut self.register_a;
            }
            _ => {
                let index = self.get_operand_address(addressing_mode) as usize;
                operand = &mut self.memory[index];
            }
        }
        let operand_least_significant_bit = ((*operand) & 0b0000_0001);
        let result = *operand >> 1;

        *operand = result;

        self.update_zero_flag(result);
        self.update_negative_flag(result);

        if operand_least_significant_bit == 1 {
            self.set_flag(STATUS_FLAG_MASK_CARRY);
        } else {
            self.clear_flag(STATUS_FLAG_MASK_CARRY);
        }
    }

    fn ora(&mut self, addressing_mode: &AddressingMode) {
        let operand = self.get_operand(addressing_mode);
        self.register_a = self.register_a | operand;
        self.update_zero_flag(self.register_a);
        self.update_negative_flag(self.register_a);
    }

    fn php(&mut self) {
        self.stack_push(self.status | 0b0001_0000);
    }

    fn plp(&mut self) {
        self.status = self.stack_pop() | 0b0010_0000;
        // NesDev reference says that this flag should be set from stack,
        // but the test suite only passes if I clear it here.
        self.clear_flag(STATUS_FLAG_MASK_BREAK_COMMAND);
    }

    fn inc(&mut self, addressing_mode: &AddressingMode) {
        let address = self.get_operand_address(addressing_mode);
        let result = self.mem_read(address).wrapping_add(1);
        self.mem_write(address, result);
        self.update_zero_flag(result);
        self.update_negative_flag(result);
    }

    fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.update_zero_flag(self.register_x);
        self.update_negative_flag(self.register_x);
    }

    fn rol(&mut self, addressing_mode: &AddressingMode) {
        let carry = self.get_flag_state(STATUS_FLAG_MASK_CARRY);

        let operand: &mut u8;
        match addressing_mode {
            AddressingMode::Accumulator => {
                operand = &mut self.register_a;
            }
            _ => {
                let index = self.get_operand_address(addressing_mode) as usize;
                operand = &mut self.memory[index];
            }
        }
        let operand_most_significant_bit = ((*operand) & 0b1000_0000) >> 7;
        let mut result = *operand << 1;

        match carry {
            FlagStates::SET => {
                result = result | 0b0000_0001;
            }
            FlagStates::CLEAR => {
                result = result & 0b1111_1110;
            }
        }
        *operand = result;

        self.update_zero_flag(result);
        self.update_negative_flag(result);

        if operand_most_significant_bit == 1 {
            self.set_flag(STATUS_FLAG_MASK_CARRY);
        } else {
            self.clear_flag(STATUS_FLAG_MASK_CARRY);
        }
    }

    fn rts(&mut self) {
        self.program_counter = self.stack_pop_u16();
    }

    fn jmp(&mut self, addressing_mode: &AddressingMode) {
        self.program_counter = self.get_operand_address(addressing_mode);
    }

    fn jsr(&mut self, addressing_mode: &AddressingMode) {
        // Program counter is incremented instead of decremented as requested in the nesdev reference
        self.stack_push_u16(self.program_counter.wrapping_add(1));
        self.program_counter = self.get_operand_address(addressing_mode);
    }

    fn sbc(&mut self, addressing_mode: &AddressingMode) {
        let operand = !self.get_operand(addressing_mode);
        let register_a_sign = self.register_a & 0b1000_0000;
        let operand_sign = operand & 0b1000_0000;
        let carry = (self.get_flag_state(STATUS_FLAG_MASK_CARRY) as u8);
        let (temp_sum, overflow_occured_on_first_addition) =
            self.register_a.overflowing_add(operand);
        let (final_sum, overflow_occured_on_second_addition) = temp_sum.overflowing_add(carry);
        self.register_a = final_sum;
        if overflow_occured_on_first_addition || overflow_occured_on_second_addition {
            self.set_flag(STATUS_FLAG_MASK_CARRY);
            self.set_flag(STATUS_FLAG_MASK_OVERFLOW);
        } else {
            self.clear_flag(STATUS_FLAG_MASK_CARRY)
        };

        let result_sign = self.register_a & 0b1000_0000;
        if register_a_sign == operand_sign && result_sign != register_a_sign {
            self.set_flag(STATUS_FLAG_MASK_OVERFLOW);
        } else {
            self.clear_flag(STATUS_FLAG_MASK_OVERFLOW);
        }

        self.update_negative_flag(self.register_a);
        self.update_zero_flag(self.register_a);
    }

    fn sec(&mut self) {
        self.set_flag(STATUS_FLAG_MASK_CARRY);
    }
    fn sta(&mut self, addressing_mode: &AddressingMode) {
        let address = self.get_operand_address(addressing_mode);
        self.mem_write(address, self.register_a);
    }
    fn sty(&mut self, addressing_mode: &AddressingMode) {
        let address = self.get_operand_address(addressing_mode);
        self.mem_write(address, self.register_y);
    }

    fn tax(&mut self) {
        self.register_x = self.register_a;

        self.update_zero_flag(self.register_x);
        self.update_negative_flag(self.register_x);
    }

    fn tay(&mut self) {
        self.register_y = self.register_a;

        self.update_zero_flag(self.register_y);
        self.update_negative_flag(self.register_y);
    }

    fn txa(&mut self) {
        self.register_a = self.register_x;
        self.update_zero_flag(self.register_a);
        self.update_negative_flag(self.register_a);
    }

    fn update_zero_flag(&mut self, data: u8) {
        if data == 0 {
            self.set_flag(STATUS_FLAG_MASK_ZERO);
        } else {
            self.clear_flag(STATUS_FLAG_MASK_ZERO);
        }
    }

    fn update_negative_flag(&mut self, data: u8) {
        if data & 0b1000_0000 != 0 {
            self.set_flag(STATUS_FLAG_MASK_NEGATIVE);
        } else {
            self.clear_flag(STATUS_FLAG_MASK_NEGATIVE);
        }
    }
}

#[cfg(test)]
mod test_cpu {
    use std::ops::Add;

    use super::*;
    use json::JsonValue;
    use test_case::test_case;

    #[test_case(0b0, 0b0000_0010)]
    #[test_case(0b10, 0b0)]
    fn test_update_zero_flag(register: u8, expected: u8) {
        let mut cpu = CPU::new();
        cpu.update_zero_flag(register);
        assert_eq!(cpu.status, expected);
    }

    // TODO: test that illegal opcodes are ignored

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
        assert_eq!(cpu.status & 0b0000_0001, expected_carry_flag) // todo: use get_carry_flag()
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

    #[test_case(0b0001_0010, 0b0000_0001, 0b0010_0100, 0b0000_0000)]
    #[test_case(0b1001_0010, 0b0000_0000, 0b0010_0100, 0b0000_0001)]
    fn test_asl_accumulator(acc: u8, status: u8, expected_acc: u8, expected_status: u8) {
        let mut cpu = CPU::new();
        cpu.register_a = acc;
        cpu.status = status;
        cpu.asl(&AddressingMode::Accumulator);
        assert_eq!(cpu.register_a, expected_acc);
        assert_eq!(cpu.status, expected_status);
    }

    #[test_case(0b0001_0010, 0b0000_0001, 0b0010_0100, 0b0000_0000)]
    #[test_case(0b1001_0010, 0b0000_0000, 0b0010_0100, 0b0000_0001)]
    fn test_asl_memory(operand: u8, status: u8, epxected_operand: u8, expected_status: u8) {
        let mut cpu = CPU::new();
        cpu.status = status;
        cpu.program_counter = 0x0000;
        cpu.mem_write(cpu.program_counter, 0x80);
        cpu.mem_write(0x80, operand);
        cpu.asl(&AddressingMode::ZeroPage);
        assert_eq!(cpu.mem_read(0x80), epxected_operand);
        assert_eq!(cpu.status, expected_status);
    }

    #[test_case(0b0000_0001, 0x8080, 0x8080, 0x06)]
    #[test_case(0b0000_0000, 0xE004, 0xE00A, 0x06)]
    #[test_case(0b0000_0000, 0xE009, 0xE003, 0xFA)]
    fn test_bcc(status: u8, program_counter: u16, expected_program_counter: u16, distance: u8) {
        let mut cpu = CPU::new();
        cpu.status = status;
        cpu.program_counter = program_counter;
        cpu.memory[cpu.program_counter as usize] = distance;
        cpu.bcc();
        assert_eq!(cpu.program_counter, expected_program_counter);
    }

    #[test_case(0b0000_0000, 0x8080, 0x8080, 0x06)]
    #[test_case(0b0000_0001, 0xE004, 0xE00A, 0x06)]
    #[test_case(0b0000_0001, 0xE009, 0xE003, 0xFA)]
    fn test_bcs(status: u8, program_counter: u16, expected_program_counter: u16, distance: u8) {
        let mut cpu = CPU::new();
        cpu.status = status;
        cpu.program_counter = program_counter;
        cpu.memory[cpu.program_counter as usize] = distance;
        cpu.bcs();
        assert_eq!(cpu.program_counter, expected_program_counter);
    }

    #[test_case(0b0000_0000, 0x8080, 0x8080, 0x06)]
    #[test_case(0b0000_0010, 0xE004, 0xE00A, 0x06)]
    #[test_case(0b0000_0010, 0xE009, 0xE003, 0xFA)]
    fn test_beq(status: u8, program_counter: u16, expected_program_counter: u16, distance: u8) {
        let mut cpu = CPU::new();
        cpu.status = status;
        cpu.program_counter = program_counter;
        cpu.memory[cpu.program_counter as usize] = distance;
        cpu.beq();
        assert_eq!(cpu.program_counter, expected_program_counter);
    }

    #[test_case(0b0000_0000, 0x8080, 0x8080, 0x06)]
    #[test_case(0b1000_0000, 0xE004, 0xE00A, 0x06)]
    #[test_case(0b1000_0000, 0xE009, 0xE003, 0xFA)]
    fn test_bmi(status: u8, program_counter: u16, expected_program_counter: u16, distance: u8) {
        let mut cpu = CPU::new();
        cpu.status = status;
        cpu.program_counter = program_counter;
        cpu.memory[cpu.program_counter as usize] = distance;
        cpu.bmi();
        assert_eq!(cpu.program_counter, expected_program_counter);
    }

    #[test_case(0b0000_0010, 0x8080, 0x8080, 0x06)]
    #[test_case(0b0000_0000, 0xE004, 0xE00A, 0x06)]
    #[test_case(0b0000_0000, 0xE009, 0xE003, 0xFA)]
    fn test_bne(status: u8, program_counter: u16, expected_program_counter: u16, distance: u8) {
        let mut cpu = CPU::new();
        cpu.status = status;
        cpu.program_counter = program_counter;
        cpu.memory[cpu.program_counter as usize] = distance;
        cpu.bne();
        assert_eq!(cpu.program_counter, expected_program_counter);
    }

    #[test_case(0b0100_0000, 0x8080, 0x8080, 0x06)]
    #[test_case(0b0000_0000, 0xE004, 0xE00A, 0x06)]
    #[test_case(0b0000_0000, 0xE009, 0xE003, 0xFA)]
    fn test_bvc(status: u8, program_counter: u16, expected_program_counter: u16, distance: u8) {
        let mut cpu = CPU::new();
        cpu.status = status;
        cpu.program_counter = program_counter;
        cpu.memory[cpu.program_counter as usize] = distance;
        cpu.bvc();
        assert_eq!(cpu.program_counter, expected_program_counter);
    }

    #[test_case(0b0000_0000, 0x8080, 0x8080, 0x06)]
    #[test_case(0b0100_0000, 0xE004, 0xE00A, 0x06)]
    #[test_case(0b0100_0000, 0xE009, 0xE003, 0xFA)]
    fn test_bvs(status: u8, program_counter: u16, expected_program_counter: u16, distance: u8) {
        let mut cpu = CPU::new();
        cpu.status = status;
        cpu.program_counter = program_counter;
        cpu.memory[cpu.program_counter as usize] = distance;
        cpu.bvs();
        assert_eq!(cpu.program_counter, expected_program_counter);
    }

    #[test_case(0b0000_0001, 0b0000_0000)]
    #[test_case(0b0000_0000, 0b0000_0000)]
    #[test_case(0b0100_0001, 0b0100_0000)]
    #[test_case(0b0100_0000, 0b0100_0000)]
    fn test_clc(status: u8, expected_status: u8) {
        let mut cpu = CPU::new();
        cpu.status = status;
        cpu.clc();
        assert_eq!(cpu.status, expected_status)
    }

    #[test_case(0b0100_0000, 0b0000_0000)]
    #[test_case(0b0000_0000, 0b0000_0000)]
    #[test_case(0b0100_0001, 0b0000_0001)]
    #[test_case(0b0000_0001, 0b0000_0001)]
    fn test_clv(status: u8, expected_status: u8) {
        let mut cpu = CPU::new();
        cpu.status = status;
        cpu.clv();
        assert_eq!(cpu.status, expected_status)
    }

    #[test_case(0x15, 0x10, FlagStates::SET, FlagStates::CLEAR, FlagStates::CLEAR)]
    #[test_case(0x15, 0x15, FlagStates::SET, FlagStates::SET, FlagStates::CLEAR)]
    #[test_case(0x15, 0x35, FlagStates::CLEAR, FlagStates::CLEAR, FlagStates::SET)]
    #[test_case(0xFA, 0x00, FlagStates::SET, FlagStates::CLEAR, FlagStates::SET)]
    #[test_case(0xFA, 0xFA, FlagStates::SET, FlagStates::SET, FlagStates::CLEAR)]
    #[test_case(0xFA, 0xFB, FlagStates::CLEAR, FlagStates::CLEAR, FlagStates::SET)]
    fn test_cmp(
        accumulator: u8,
        memory_value: u8,
        expected_carry_flag_state: FlagStates,
        expected_zero_flag_state: FlagStates,
        expected_negative_flag_state: FlagStates,
    ) {
        let mut cpu = CPU::new();
        cpu.register_a = accumulator;
        cpu.program_counter = 0x8001;
        cpu.mem_write(0x8001, memory_value);

        cpu.cmp(&AddressingMode::Immediate);
        let carry_flag_state = cpu.get_flag_state(STATUS_FLAG_MASK_CARRY);
        let zero_flag_state = cpu.get_flag_state(STATUS_FLAG_MASK_ZERO);
        let negative_flag_state = cpu.get_flag_state(STATUS_FLAG_MASK_NEGATIVE);
        assert_eq!(
            carry_flag_state, expected_carry_flag_state,
            "Expected carry flag {:?}, but got {:?}",
            expected_carry_flag_state, carry_flag_state
        );
        assert_eq!(
            zero_flag_state, expected_zero_flag_state,
            "Expected zero flag {:?}, but got {:?}",
            expected_zero_flag_state, zero_flag_state
        );
        assert_eq!(
            negative_flag_state, expected_negative_flag_state,
            "Expected negative flag {:?}, but got {:?}",
            expected_negative_flag_state, negative_flag_state
        );
    }

    #[test_case(0x15, 0x10, FlagStates::SET, FlagStates::CLEAR, FlagStates::CLEAR)]
    #[test_case(0x15, 0x15, FlagStates::SET, FlagStates::SET, FlagStates::CLEAR)]
    #[test_case(0x15, 0x35, FlagStates::CLEAR, FlagStates::CLEAR, FlagStates::SET)]
    #[test_case(0xFA, 0x00, FlagStates::SET, FlagStates::CLEAR, FlagStates::SET)]
    #[test_case(0xFA, 0xFA, FlagStates::SET, FlagStates::SET, FlagStates::CLEAR)]
    #[test_case(0xFA, 0xFB, FlagStates::CLEAR, FlagStates::CLEAR, FlagStates::SET)]

    fn test_cpx(
        register_x: u8,
        memory_value: u8,
        expected_carry_flag_state: FlagStates,
        expected_zero_flag_state: FlagStates,
        expected_negative_flag_state: FlagStates,
    ) {
        let mut cpu = CPU::new();
        cpu.register_x = register_x;
        cpu.program_counter = 0x8001;
        cpu.mem_write(0x8001, memory_value);

        cpu.cpx(&AddressingMode::Immediate);
        let carry_flag_state = cpu.get_flag_state(STATUS_FLAG_MASK_CARRY);
        let zero_flag_state = cpu.get_flag_state(STATUS_FLAG_MASK_ZERO);
        let negative_flag_state = cpu.get_flag_state(STATUS_FLAG_MASK_NEGATIVE);
        assert_eq!(
            carry_flag_state, expected_carry_flag_state,
            "Expected carry flag {:?}, but got {:?}",
            expected_carry_flag_state, carry_flag_state
        );
        assert_eq!(
            zero_flag_state, expected_zero_flag_state,
            "Expected zero flag {:?}, but got {:?}",
            expected_zero_flag_state, zero_flag_state
        );
        assert_eq!(
            negative_flag_state, expected_negative_flag_state,
            "Expected negative flag {:?}, but got {:?}",
            expected_negative_flag_state, negative_flag_state
        );
    }

    #[test_case(0b0000_0001, FlagStates::CLEAR, FlagStates::CLEAR)]
    #[test_case(0b0000_0000, FlagStates::SET, FlagStates::CLEAR)]
    #[test_case(0b1000_0000, FlagStates::CLEAR, FlagStates::SET)]
    fn test_ldy(
        register_y_value: u8,
        expected_zero_flag_state: FlagStates,
        expected_negative_flag: FlagStates,
    ) {
        let mut cpu = CPU::new();
        cpu.program_counter = 0xAAAA;
        cpu.register_y = 151;
        cpu.mem_write(cpu.program_counter, register_y_value);
        cpu.ldy(&AddressingMode::Immediate);
        assert_eq!(cpu.register_y, register_y_value);
        assert_eq!(
            cpu.get_flag_state(STATUS_FLAG_MASK_ZERO),
            expected_zero_flag_state
        );
        assert_eq!(
            cpu.get_flag_state(STATUS_FLAG_MASK_NEGATIVE),
            expected_negative_flag
        );
    }

    #[test_case(0b0000_0001, 0x5, 0x4, 0x1, 0b0000_0001)]
    #[test_case(0b0000_0001, 0x5, 0x5, 0x0, 0b0000_0011)]
    #[test_case(0b0000_0001, 0x0, 0x1, 0xFF, 0b1000_0000)]
    #[test_case(0b0000_0001, 0x80, 0x1, 0x7F, 0b0100_0001)]
    #[test_case(0b0000_0001, 0x7F, 0xFF, 0x80, 0b1100_0000)]
    fn test_sbc(status: u8, acc: u8, nn: u8, expected_acc: u8, expected_status: u8) {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x8000;
        cpu.register_a = acc;
        cpu.memory[0x8000] = nn;
        cpu.status = status;
        cpu.sbc(&AddressingMode::Immediate);
        assert_eq!(cpu.register_a, expected_acc);
        assert_eq!(cpu.status, expected_status)
    }

    #[test_case(0b0000_0000, 0x7A, 0b0000_0000)]
    #[test_case(0b0000_0000, 0x0, 0b0000_0010)]
    #[test_case(0b0000_0000, 0xFF, 0b1000_0000)]
    fn test_txa(status: u8, x: u8, expected_status: u8) {
        let mut cpu = CPU::new();
        cpu.status = status;
        cpu.register_x = x;
        cpu.txa();

        assert_eq!(cpu.register_a, x);
        assert_eq!(cpu.status, expected_status);
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
        cpu.program_counter = 0x0;
        cpu.memory[0x0] = 0x9e;
        cpu.memory[0x1] = 0x5e;
        let result = cpu.get_operand_address(&AddressingMode::Absolute);
        assert_eq!(result, 0x5e9e);
    }

    #[test]
    fn test_addressing_mode_absolute_x() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x0;
        cpu.mem_write_u16(0x00, 2000);
        cpu.register_x = 82;
        let result = cpu.get_operand_address(&AddressingMode::Absolute_X);
        assert_eq!(result, 2082);
    }

    #[test]
    fn test_addressing_mode_absolute_y() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x0;
        cpu.mem_write_u16(0x00, 2000);
        cpu.register_y = 82;
        let result = cpu.get_operand_address(&AddressingMode::Absolute_Y);
        assert_eq!(result, 2082);
    }

    #[test]
    fn test_addressing_mode_indexed_indirect_x() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x8000;
        cpu.mem_write(0x8000, 0x20);
        cpu.mem_write_u16(0x0021, 0xBAFC);
        cpu.register_x = 0x01;
        let result = cpu.get_operand_address(&AddressingMode::Indexed_Indirect_X);
        assert_eq!(result, 0xBAFC);
    }

    #[test]
    fn test_addressing_mode_indexed_indirect_y() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x8000;
        cpu.mem_write(0x8000, 0x20);
        cpu.mem_write_u16(0x0021, 0xBAFC);
        cpu.register_y = 0x01;
        let result = cpu.get_operand_address(&AddressingMode::Indexed_Indirect_Y);
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

    #[test]
    fn test_get_operand() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x80;
        let result = cpu.get_operand(&AddressingMode::Accumulator);
        assert_eq!(result, 0x80);
    }

    #[test_case("submodules/65x02/nes6502/v1/00.json")]
    #[test_case("submodules/65x02/nes6502/v1/01.json")]
    #[test_case("submodules/65x02/nes6502/v1/05.json")]
    #[test_case("submodules/65x02/nes6502/v1/06.json")]
    #[test_case("submodules/65x02/nes6502/v1/08.json")]
    #[test_case("submodules/65x02/nes6502/v1/09.json")]
    #[test_case("submodules/65x02/nes6502/v1/0a.json")]
    #[test_case("submodules/65x02/nes6502/v1/0d.json")]
    #[test_case("submodules/65x02/nes6502/v1/0e.json")]
    #[test_case("submodules/65x02/nes6502/v1/10.json")]
    #[test_case("submodules/65x02/nes6502/v1/11.json")]
    #[test_case("submodules/65x02/nes6502/v1/15.json")]
    #[test_case("submodules/65x02/nes6502/v1/16.json")]
    #[test_case("submodules/65x02/nes6502/v1/18.json")]
    #[test_case("submodules/65x02/nes6502/v1/19.json")]
    #[test_case("submodules/65x02/nes6502/v1/1d.json")]
    #[test_case("submodules/65x02/nes6502/v1/1e.json")]
    #[test_case("submodules/65x02/nes6502/v1/20.json")]
    #[test_case("submodules/65x02/nes6502/v1/21.json")]
    #[test_case("submodules/65x02/nes6502/v1/24.json")]
    #[test_case("submodules/65x02/nes6502/v1/25.json")]
    #[test_case("submodules/65x02/nes6502/v1/26.json")]
    #[test_case("submodules/65x02/nes6502/v1/28.json")]
    #[test_case("submodules/65x02/nes6502/v1/29.json")]
    #[test_case("submodules/65x02/nes6502/v1/2a.json")]
    #[test_case("submodules/65x02/nes6502/v1/2c.json")]
    #[test_case("submodules/65x02/nes6502/v1/2d.json")]
    #[test_case("submodules/65x02/nes6502/v1/2e.json")]
    #[test_case("submodules/65x02/nes6502/v1/31.json")]
    #[test_case("submodules/65x02/nes6502/v1/35.json")]
    #[test_case("submodules/65x02/nes6502/v1/36.json")]
    #[test_case("submodules/65x02/nes6502/v1/38.json")]
    #[test_case("submodules/65x02/nes6502/v1/39.json")]
    #[test_case("submodules/65x02/nes6502/v1/3d.json")]
    #[test_case("submodules/65x02/nes6502/v1/3e.json")]
    #[test_case("submodules/65x02/nes6502/v1/41.json")]
    #[test_case("submodules/65x02/nes6502/v1/45.json")]
    #[test_case("submodules/65x02/nes6502/v1/46.json")]
    #[test_case("submodules/65x02/nes6502/v1/49.json")]
    #[test_case("submodules/65x02/nes6502/v1/4a.json")]
    #[test_case("submodules/65x02/nes6502/v1/4c.json")]
    #[test_case("submodules/65x02/nes6502/v1/4d.json")]
    #[test_case("submodules/65x02/nes6502/v1/4e.json")]
    #[test_case("submodules/65x02/nes6502/v1/51.json")]
    #[test_case("submodules/65x02/nes6502/v1/55.json")]
    #[test_case("submodules/65x02/nes6502/v1/56.json")]
    #[test_case("submodules/65x02/nes6502/v1/58.json")]
    #[test_case("submodules/65x02/nes6502/v1/59.json")]
    #[test_case("submodules/65x02/nes6502/v1/5d.json")]
    #[test_case("submodules/65x02/nes6502/v1/5e.json")]
    #[test_case("submodules/65x02/nes6502/v1/60.json")]
    #[test_case("submodules/65x02/nes6502/v1/61.json")]
    #[test_case("submodules/65x02/nes6502/v1/65.json")]
    #[test_case("submodules/65x02/nes6502/v1/69.json")]
    #[test_case("submodules/65x02/nes6502/v1/6c.json")]
    #[test_case("submodules/65x02/nes6502/v1/6d.json")]
    #[test_case("submodules/65x02/nes6502/v1/71.json")]
    #[test_case("submodules/65x02/nes6502/v1/75.json")]
    #[test_case("submodules/65x02/nes6502/v1/79.json")]
    #[test_case("submodules/65x02/nes6502/v1/7d.json")]
    #[test_case("submodules/65x02/nes6502/v1/81.json")]
    #[test_case("submodules/65x02/nes6502/v1/84.json")]
    #[test_case("submodules/65x02/nes6502/v1/85.json")]
    #[test_case("submodules/65x02/nes6502/v1/8a.json")]
    #[test_case("submodules/65x02/nes6502/v1/8c.json")]
    #[test_case("submodules/65x02/nes6502/v1/8d.json")]
    #[test_case("submodules/65x02/nes6502/v1/90.json")]
    #[test_case("submodules/65x02/nes6502/v1/91.json")]
    #[test_case("submodules/65x02/nes6502/v1/94.json")]
    #[test_case("submodules/65x02/nes6502/v1/95.json")]
    #[test_case("submodules/65x02/nes6502/v1/99.json")]
    #[test_case("submodules/65x02/nes6502/v1/9d.json")]
    #[test_case("submodules/65x02/nes6502/v1/a0.json")]
    #[test_case("submodules/65x02/nes6502/v1/a2.json")]
    #[test_case("submodules/65x02/nes6502/v1/a5.json")]
    #[test_case("submodules/65x02/nes6502/v1/a6.json")]
    #[test_case("submodules/65x02/nes6502/v1/a9.json")]
    #[test_case("submodules/65x02/nes6502/v1/aa.json")]
    #[test_case("submodules/65x02/nes6502/v1/ae.json")]
    #[test_case("submodules/65x02/nes6502/v1/b0.json")]
    #[test_case("submodules/65x02/nes6502/v1/b5.json")]
    #[test_case("submodules/65x02/nes6502/v1/b6.json")]
    #[test_case("submodules/65x02/nes6502/v1/be.json")]
    #[test_case("submodules/65x02/nes6502/v1/c0.json")]
    #[test_case("submodules/65x02/nes6502/v1/c1.json")]
    #[test_case("submodules/65x02/nes6502/v1/c4.json")]
    #[test_case("submodules/65x02/nes6502/v1/c5.json")]
    #[test_case("submodules/65x02/nes6502/v1/c6.json")]
    #[test_case("submodules/65x02/nes6502/v1/c9.json")]
    #[test_case("submodules/65x02/nes6502/v1/ca.json")]
    #[test_case("submodules/65x02/nes6502/v1/cc.json")]
    #[test_case("submodules/65x02/nes6502/v1/cd.json")]
    #[test_case("submodules/65x02/nes6502/v1/ce.json")]
    #[test_case("submodules/65x02/nes6502/v1/d0.json")]
    #[test_case("submodules/65x02/nes6502/v1/d1.json")]
    #[test_case("submodules/65x02/nes6502/v1/d5.json")]
    #[test_case("submodules/65x02/nes6502/v1/d6.json")]
    #[test_case("submodules/65x02/nes6502/v1/d8.json")]
    #[test_case("submodules/65x02/nes6502/v1/d9.json")]
    #[test_case("submodules/65x02/nes6502/v1/dd.json")]
    #[test_case("submodules/65x02/nes6502/v1/de.json")]
    #[test_case("submodules/65x02/nes6502/v1/e0.json")]
    #[test_case("submodules/65x02/nes6502/v1/e1.json")]
    #[test_case("submodules/65x02/nes6502/v1/e4.json")]
    #[test_case("submodules/65x02/nes6502/v1/e5.json")]
    #[test_case("submodules/65x02/nes6502/v1/e6.json")]
    #[test_case("submodules/65x02/nes6502/v1/e8.json")]
    #[test_case("submodules/65x02/nes6502/v1/e9.json")]
    #[test_case("submodules/65x02/nes6502/v1/ea.json")]
    #[test_case("submodules/65x02/nes6502/v1/ed.json")]
    #[test_case("submodules/65x02/nes6502/v1/ee.json")]
    #[test_case("submodules/65x02/nes6502/v1/f0.json")]
    #[test_case("submodules/65x02/nes6502/v1/f1.json")]
    #[test_case("submodules/65x02/nes6502/v1/f5.json")]
    #[test_case("submodules/65x02/nes6502/v1/f6.json")]
    #[test_case("submodules/65x02/nes6502/v1/f9.json")]
    #[test_case("submodules/65x02/nes6502/v1/fd.json")]
    #[test_case("submodules/65x02/nes6502/v1/fe.json")]
    fn run_test_from_json(path: &str) {
        let tests_string = std::fs::read_to_string(path).unwrap();
        let tests = json::parse(tests_string.as_str()).unwrap();

        for i in 0..tests.len() {
            run_test(&tests[i]);
        }
    }

    // todo: derive from file, one test per line
    fn run_test(test: &JsonValue) {
        let name = &test["name"];
        println!("Testing with instructions: {}", name);

        let initial_cpu = cpu_from_json_value(&test["initial"]);

        let final_cpu = cpu_from_json_value(&test["final"]);

        let mut cpu = initial_cpu.clone();

        cpu.execute_next_instruction();

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

        assert_eq!(
            cpu.memory,
            final_cpu.memory,
            "Memories don't match: \n{}",
            array_diff(cpu.memory.to_vec(), final_cpu.memory.to_vec())
        );
    }

    fn array_diff(left: Vec<u8>, right: Vec<u8>) -> String {
        let mut display_diff = String::from("");

        for (i, (a, b)) in left.iter().zip(right.iter()).enumerate() {
            if a != b {
                display_diff.push_str(&format!("Difference at position {i}: {a} != {b}\n"));
            }
        }

        return display_diff;
    }

    fn cpu_from_json_value(json_value: &JsonValue) -> CPU {
        let mut cpu = CPU::new();
        cpu.program_counter = json_value["pc"].as_u16().unwrap();
        cpu.status = json_value["p"].as_u8().unwrap();
        cpu.register_a = json_value["a"].as_u8().unwrap();
        cpu.register_x = json_value["x"].as_u8().unwrap();
        cpu.register_y = json_value["y"].as_u8().unwrap();
        cpu.stack_pointer = json_value["s"].as_u8().unwrap();

        for ram_tuple in json_value["ram"].members() {
            cpu.mem_write(
                ram_tuple[0].as_u16().unwrap(),
                ram_tuple[1].as_u8().unwrap(),
            );
        }

        return cpu;
    }
}
