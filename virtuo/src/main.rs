// src/main.rs

struct CPU {
    pc: u16,
    sp: u16,
    acc: u8,
}

impl CPU {
    fn new() -> CPU {
        CPU {
            pc: 0,
            sp: 0xFF,
            acc: 0,
        }
    }

    fn fetch_instruction(&mut self, memory: &[u8]) -> u8 {
        let instruction = memory[self.pc as usize];
        self.pc += 1;
        instruction
    }

    fn execute_instruction(&mut self, instruction: u8, memory: &mut [u8]) {
        match instruction {
            0x01 => {
                self.acc = self.acc.wrapping_add(1);
                self.push(self.acc, memory);
            },
            0x02 => {
                self.acc = self.pop(memory);
            },
            _ => println!("Unknown instruction"),
        }
    }

    fn run(&mut self, memory: &mut [u8]) {
        loop {
            let instruction = self.fetch_instruction(memory);
            if instruction == 0x00 {
                break;
            }
            self.execute_instruction(instruction, memory);
        }
    }
    fn push(&mut self, value: u8, memory: &mut [u8]) {
        if self.sp == 0 {
            panic!("Stack overflow");
        }
        self.sp -= 1;
        memory[self.sp as usize] = value;
    }
    
    fn pop(&mut self, memory: &[u8]) -> u8 {
        if self.sp >= memory.len() as u16 - 1 {
            panic!("Stack underflow");
        }
        self.sp += 1;
        memory[self.sp as usize]
    }    
}

fn main() {
    let mut cpu = CPU::new();
    let mut memory = vec![0; 256];
    memory[0] = 0x01;
    memory[1] = 0x01;
    memory[2] = 0x01;
    memory[3] = 0x00;
    cpu.run(&mut memory);
    println!("Final accumulator value: {}", cpu.acc);
}
