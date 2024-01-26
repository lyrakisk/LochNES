pub struct CPU {
    pub register_a: u8,
    pub status: u8,
    pub program_counter: u16,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            status: 0, // todo: according to nesdev wiki, the 5th bit is always 1, https://www.nesdev.org/wiki/Status_flags
            program_counter: 0,
        }
    }

    pub fn interpret(&mut self, program: Vec<u8>) {
        self.program_counter = 0;

        loop {
            let opscode = program[self.program_counter as usize];
            self.program_counter += 1;

            match opscode {
                0x00 => {
                    return;
                }

                // todo: extract method
                0xA9 => {
                    self.register_a = program[self.program_counter as usize];
                    self.program_counter += 1;

                    if self.register_a == 0 {
                        self.status = self.status | 0b0000_0010;
                    } else {
                        self.status = self.status & 0b1111_1101;
                    }

                    if self.register_a & 0b1000_0000 != 0 {
                        self.status = self.status | 0b1000_0000;
                    } else {
                        self.status = self.status & 0b0111_1111;
                    }
                }
                _ => {
                    todo!();
                }
            }
        }
    }
}

#[cfg(test)]
mod test_cpu {
    use super::*;

    #[test]
    fn lda_correctly_sets_negative_flag() {
        let program = vec![0xa9, 0x05, 0x00];
        let mut cpu = CPU::new();
        cpu.interpret(program);
        assert!(cpu.status & 0b0000_0010 == 0b00);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn lda_correctly_sets_zero_flag() {
        let program = vec![0xa9, 0x00, 0x00];
        let mut cpu = CPU::new();
        cpu.interpret(program);
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }

    // todo: add test to check lda loads to register_a
}