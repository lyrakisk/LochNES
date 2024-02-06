use std::collections::HashMap;
use once_cell::sync::Lazy;

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPage_X,
    ZeroPage_Y,
    Absolute,
    Absolute_X,
    Absolute_Y,
    Implied,
    Indirect,
    Indirect_X,
    Indirect_Y,
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

pub static INSTRUCTIONS: Lazy<HashMap<u8, Instruction>> = Lazy::new(|| {
    vec![
        Instruction {
            opcode: 0x00,
            name: "BRK",
            bytes: 1,
            addressing_mode: AddressingMode::Implied,
        },
        Instruction {
            opcode: 0xA9,
            name: "LDA",
            bytes: 2,
            addressing_mode: AddressingMode::Immediate,
        },
        Instruction {
            opcode: 0xA5,
            name: "LDA",
            bytes: 2,
            addressing_mode: AddressingMode::ZeroPage,
        },
        Instruction {
            opcode: 0xB5,
            name: "LDA",
            bytes: 2,
            addressing_mode: AddressingMode::ZeroPage_X,
        },
        Instruction {
            opcode: 0xAD,
            name: "LDA",
            bytes: 3,
            addressing_mode: AddressingMode::Absolute,
        },
        Instruction {
            opcode: 0xBD,
            name: "LDA",
            bytes: 3,
            addressing_mode: AddressingMode::Absolute_X,
        },
        Instruction {
            opcode: 0xB9,
            name: "LDA",
            bytes: 3,
            addressing_mode: AddressingMode::Absolute_Y,
        },
        Instruction {
            opcode: 0xA1,
            name: "LDA",
            bytes: 2,
            addressing_mode: AddressingMode::Indirect_X,
        },
        Instruction {
            opcode: 0xB1,
            name: "LDA",
            bytes: 2,
            addressing_mode: AddressingMode::Indirect_Y,
        },
        Instruction {
            opcode: 0xAA,
            name: "TAX",
            bytes: 1,
            addressing_mode: AddressingMode::Implied,
        },
        Instruction {
            opcode: 0xE8,
            name: "INX",
            bytes: 1,
            addressing_mode: AddressingMode::Implied,
        },
    ]
    .into_iter()
    .map(|instruction| (instruction.opcode, instruction))
    .collect()
});
