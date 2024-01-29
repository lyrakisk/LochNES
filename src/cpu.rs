pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub status: u8,
    pub program_counter: u16,
    memory: [u8; 0xFFFF],
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0, // todo: check reference, should this be initialized? 
            status: 0, // todo: according to nesdev wiki, the 5th bit is always 1, https://www.nesdev.org/wiki/Status_flags
            program_counter: 0,
            memory: [0; 0xFFFF], // should everything be initialized to zero? 
        }
    }


    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.run();
    }

    fn run(&mut self) {
        loop {
            let opscode = self.memory[self.program_counter as usize];
            self.program_counter += 1;

            match opscode {
                0x00 => {
                    return;
                }

                0xA9 => {
                    self.lda();
                }

                0xAA => {
                    self.tax();
                }
                
                0xE8 => {
                    self.inx();
                }
                _ => {
                    todo!();
                }
            }
        }
    }

    fn load(&mut self, program: Vec<u8>) {
        self.memory[0x8000 .. (0x8000 + program.len())].copy_from_slice(&program[..]);
        self.program_counter = 0x8000;
    }

    fn mem_read(&self, address: u16)-> u8 {
        return self.memory[address as usize];
    }


    fn mem_write(&mut self, address: u16, data: u8) {
        self.memory[address as usize] = data;
    }


    fn lda(&mut self) {
        self.register_a = self.memory[self.program_counter as usize]; 

        // consider moving this to interpret() so that only one method can manipulate the program counter
        self.program_counter += 1;

        self.status = CPU::update_zero_flag(self.status, self.register_a);
        self.status = self.update_negative_flag(self.status, self.register_a);                    
    }


    fn tax(&mut self) {
        self.register_x = self.register_a;

        self.status = CPU::update_zero_flag(self.status, self.register_x);
        self.status = self.update_negative_flag(self.status, self.register_x);
    }

    fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.status = CPU::update_zero_flag(self.status, self.register_a);
        self.status = self.update_negative_flag(self.status, self.register_a);                    
    }


    fn update_zero_flag(status_register: u8, register: u8)-> u8 {
        if register == 0 {
            return status_register | 0b0000_0010;
        } else {
            return status_register & 0b1111_1101;
        }
    }


    fn update_negative_flag(&self, status_register: u8, register: u8)-> u8 {
        if register & 0b1000_0000 != 0 {
            return status_register | 0b1000_0000;
        } else {
            return status_register & 0b0111_1111;
        }
    }

}

#[cfg(test)]
mod test_cpu {
    use super::*;

    fn update_zero_flag_test_case(status_register: u8, register: u8, expected: u8)-> Result<(), String> {
        let result = CPU::update_zero_flag(status_register, register);
        if  result != expected {
            Err(format!(
                "Input ({}, {}) was expected to return {}, but returned {}", status_register, register, expected, result
            ))
        } else {
            Ok(())
        }
    }


    #[test]
    fn run_update_zero_flag_tests() -> Result<(), String> {
        let _examples = [
            (0b0, 0b0, 0b0000_0010),
            (0b0000_0010, 0b10, 0b0),
        ]
        .into_iter()        
        .try_for_each(| (status_register, register, expected)| update_zero_flag_test_case(status_register, register, expected))?;
        Ok(())
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
        let program = vec![0xa9, 010, 0xAA, 0x00];
        let mut cpu = CPU::new();
        cpu.load_and_run(program);
        assert!(cpu.register_a == cpu.register_x);
    }

    #[test]
    fn tax_correctly_sets_zero_flag() {
        let mut cpu = CPU::new();
        cpu.register_a = 0;
        let program = vec![0xAA, 0x00];
        cpu.load_and_run(program);
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }

    #[test]
    fn tax_correctly_sets_negative_flag() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x05;
        let program = vec![0xAA, 0x00];
        cpu.load_and_run(program);
        assert!(cpu.status & 0b0000_0010 == 0b00);
        assert!(cpu.status & 0b1000_0000 == 0);
        // todo: add test case where the negative flag is 1
    }

    #[test]
    fn inx_increments_the_x_register() {
        let mut cpu = CPU::new();
        cpu.register_x = 0x00;
        let program= vec![0xe8, 0x00];
        cpu.load_and_run(program);
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
        cpu.load_and_run(vec![0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 1)
    }

}