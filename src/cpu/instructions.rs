use once_cell::sync::Lazy;
use std::collections::HashMap;

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum AddressingMode {
    Implicit,
    Accumulator,
    Relative,
    Immediate,
    ZeroPage,
    ZeroPage_X,
    ZeroPage_Y,
    Absolute,
    Absolute_X,
    Absolute_Y,
    Indirect,
    Indexed_Idirect_X,
    Indexed_Idirect_Y,
    Indirect_indexed_X,
    Indirect_indexed_Y,
}

pub struct Instruction {
    pub opcode: u8,
    pub name: &'static str,
    pub bytes: u8,
    pub addressing_mode: AddressingMode,
}

impl Instruction {
    pub fn new(opcode: u8, name: &'static str, bytes: u8, addressing_mode: AddressingMode) -> Self {
        Instruction {
            opcode: opcode,
            name: name,
            bytes: bytes,
            addressing_mode: addressing_mode,
        }
    }
}

#[rustfmt::skip]
pub static INSTRUCTIONS: Lazy<HashMap<u8, Instruction>> = Lazy::new(|| {
    vec![
        Instruction {opcode: 0x69, name: "ADC", bytes: 2, addressing_mode: AddressingMode::Immediate},
        Instruction {opcode: 0x65, name: "ADC", bytes: 2, addressing_mode: AddressingMode::ZeroPage},
        Instruction {opcode: 0x75, name: "ADC", bytes: 2, addressing_mode: AddressingMode::ZeroPage_X},
        Instruction {opcode: 0x6D, name: "ADC", bytes: 3, addressing_mode: AddressingMode::Absolute},
        Instruction {opcode: 0x7D, name: "ADC", bytes: 3, addressing_mode: AddressingMode::Absolute_X},
        Instruction {opcode: 0x79, name: "ADC", bytes: 3, addressing_mode: AddressingMode::Absolute_Y},
        Instruction {opcode: 0x61, name: "ADC", bytes: 2, addressing_mode: AddressingMode::Indexed_Idirect_X},
        Instruction {opcode: 0x71, name: "ADC", bytes: 2, addressing_mode: AddressingMode::Indirect_indexed_Y},
        Instruction {opcode: 0x29, name: "AND", bytes: 2, addressing_mode: AddressingMode::Immediate},
        Instruction {opcode: 0x25, name: "AND", bytes: 2, addressing_mode: AddressingMode::ZeroPage},
        Instruction {opcode: 0x35, name: "AND", bytes: 2, addressing_mode: AddressingMode::ZeroPage_X},
        Instruction {opcode: 0x2D, name: "AND", bytes: 3, addressing_mode: AddressingMode::Absolute},
        Instruction {opcode: 0x3D, name: "AND", bytes: 3, addressing_mode: AddressingMode::Absolute_X},
        Instruction {opcode: 0x39, name: "AND", bytes: 3, addressing_mode: AddressingMode::Absolute_Y},
        Instruction {opcode: 0x21, name: "AND", bytes: 2, addressing_mode: AddressingMode::Indexed_Idirect_X},
        Instruction {opcode: 0x31, name: "AND", bytes: 2, addressing_mode: AddressingMode::Indirect_indexed_Y},
        Instruction {opcode: 0x0A, name: "ASL", bytes: 1, addressing_mode: AddressingMode::Accumulator},
        Instruction {opcode: 0x06, name: "ASL", bytes: 2, addressing_mode: AddressingMode::ZeroPage},
        Instruction {opcode: 0x16, name: "ASL", bytes: 2, addressing_mode: AddressingMode::ZeroPage_X},
        Instruction {opcode: 0x0E, name: "ASL", bytes: 3, addressing_mode: AddressingMode::Absolute},
        Instruction {opcode: 0x1E, name: "ASL", bytes: 3, addressing_mode: AddressingMode::Absolute_X},
        Instruction {opcode: 0x90, name: "BCC", bytes: 2, addressing_mode: AddressingMode::Relative},
        Instruction {opcode: 0xB0, name: "BCS", bytes: 2, addressing_mode: AddressingMode::Relative},
        Instruction {opcode: 0xF0, name: "BEQ", bytes: 2, addressing_mode: AddressingMode::Relative},
        Instruction {opcode: 0x30, name: "BMI", bytes: 2, addressing_mode: AddressingMode::Relative},
        Instruction {opcode: 0xD0, name: "BNE", bytes: 2, addressing_mode: AddressingMode::Relative},
        Instruction {opcode: 0x10, name: "BPL", bytes: 2, addressing_mode: AddressingMode::Relative},
        Instruction {opcode: 0x00, name: "BRK", bytes: 1, addressing_mode: AddressingMode::Implicit},
        Instruction {opcode: 0x50, name: "BVC", bytes: 2, addressing_mode: AddressingMode::Relative},
        Instruction {opcode: 0x70, name: "BVS", bytes: 2, addressing_mode: AddressingMode::Relative},
        Instruction {opcode: 0x18, name: "CLC", bytes: 1, addressing_mode: AddressingMode::Implicit},
        Instruction {opcode: 0xB8, name: "CLV", bytes: 1, addressing_mode: AddressingMode::Implicit},
        Instruction {opcode: 0xA9, name: "LDA", bytes: 2, addressing_mode: AddressingMode::Immediate},
        Instruction {opcode: 0xA5, name: "LDA", bytes: 2, addressing_mode: AddressingMode::ZeroPage},
        Instruction {opcode: 0xB5, name: "LDA", bytes: 2, addressing_mode: AddressingMode::ZeroPage_X},
        Instruction {opcode: 0xAD, name: "LDA", bytes: 3, addressing_mode: AddressingMode::Absolute},
        Instruction {opcode: 0xBD, name: "LDA", bytes: 3, addressing_mode: AddressingMode::Absolute_X},
        Instruction {opcode: 0xB9, name: "LDA", bytes: 3, addressing_mode: AddressingMode::Absolute_Y},
        Instruction {opcode: 0xA1, name: "LDA", bytes: 2, addressing_mode: AddressingMode::Indexed_Idirect_X},
        Instruction {opcode: 0xB1, name: "LDA", bytes: 2, addressing_mode: AddressingMode::Indirect_indexed_Y},
        Instruction {opcode: 0xAA, name: "TAX", bytes: 1, addressing_mode: AddressingMode::Implicit},
        Instruction {opcode: 0xE8, name: "INX", bytes: 1, addressing_mode: AddressingMode::Implicit},
        Instruction {opcode: 0xE9, name: "SBC", bytes: 2, addressing_mode: AddressingMode::Immediate},
        Instruction {opcode: 0xE5, name: "SBC", bytes: 2, addressing_mode: AddressingMode::ZeroPage},
        Instruction {opcode: 0xF5, name: "SBC", bytes: 2, addressing_mode: AddressingMode::ZeroPage_X},
        Instruction {opcode: 0xED, name: "SBC", bytes: 2, addressing_mode: AddressingMode::Absolute},
        Instruction {opcode: 0xFD, name: "SBC", bytes: 2, addressing_mode: AddressingMode::Absolute_X},
        Instruction {opcode: 0xF9, name: "SBC", bytes: 2, addressing_mode: AddressingMode::Absolute_Y},
        Instruction {opcode: 0xE1, name: "SBC", bytes: 2, addressing_mode: AddressingMode::Indexed_Idirect_X},
        Instruction {opcode: 0xF1, name: "SBC", bytes: 2, addressing_mode: AddressingMode::Indirect_indexed_Y},
    ]
    .into_iter()
    .map(|instruction| (instruction.opcode, instruction))
    .collect()
});
