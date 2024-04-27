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
            acc: 0
        }
    }

    fn fetch_instruction(&mut self, memory: &[u8]) -> u8 {
        let instruction = memory[self.pc as usize];
        self.pc += 1;
        instruction
    }

    fn execute_instruction(&mut self, instruction: u8, memory: &mut [u8]) {
        println!("Executing instruction: {:X}", instruction);
        match instruction {
            0x01 => {
                self.acc = self.acc.wrapping_add(1);
                self.push(self.acc, memory);
                println!("Incremented ACC to {}", self.acc);
            },
            0x02 => {
                self.acc = self.pop(memory);
                println!("Popped to ACC, new ACC: {}", self.acc);
            },
            0x03 => {
                self.acc = self.acc.wrapping_sub(1);
                println!("Decremented ACC to {}", self.acc);
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
        if self.sp == memory.len() as u16 {
            panic!("Stack underflow");
        }
        self.sp += 1;
        memory[self.sp as usize - 1]
    }

    fn peek(&self, memory: &[u8]) -> u8 {
        if self.sp >= memory.len() as u16 {
            panic!("Invalid stack pointer access");
        }
        memory[self.sp as usize]
        
    }

    fn poke(&mut self, value: u8, memory: &mut [u8]) {
        if self.sp >= memory.len() as u16 {
            panic!("Invalid stack pointer access");
        }
        memory[self.sp as usize] = value;
        
    }

    fn reset(&mut self) {
        self.pc = 0;
        self.sp = 0xFF;
        self.acc = 0;
    }
}

fn main() {
    let mut cpu = CPU::new();
    let mut memory = vec![0; 256];
    memory[0] = 0x01; 
    memory[1] = 0x01; 
    memory[2] = 0x01; 
    memory[3] = 0x02;
    memory[4] = 0x03;
    memory[5] = 0x00;

    cpu.run(&mut memory);
    println!("Final accumulator value after run: {}", cpu.acc);

    cpu.poke(0xFE, &mut memory);
    println!("Value at the stack pointer after poke: {:X}", cpu.peek(&memory));

    cpu.reset();
    println!("Registers after reset: PC={:04X}, SP={:04X}, ACC={:02X}", cpu.pc, cpu.sp, cpu.acc);

    memory[3] = 0x03; 
    cpu.run(&mut memory);
    println!("Final accumulator value after modifying program: {}", cpu.acc);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu() {
        let mut cpu = CPU::new();
        let mut memory = vec![0; 256];
        memory[0] = 0x01;
        memory[1] = 0x01;
        memory[2] = 0x01;
        memory[3] = 0x00;
        cpu.run(&mut memory);
        assert_eq!(cpu.acc, 3);
    }

    #[test]
    fn test_cpu_push_pop() {
        let mut cpu = CPU::new();
        let mut memory = vec![0; 256];
        cpu.push(0x01, &mut memory);
        assert_eq!(cpu.pop(&memory), 0x01);
    }

    #[test]
    fn test_cpu_stack_overflow() {
        let mut cpu = CPU::new();
        let mut memory = vec![0; 256];
        for _ in 0..255 {
            cpu.push(0x01, &mut memory);
        }
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            cpu.push(0x01, &mut memory)
        }));
        assert!(result.is_err(), "Expected a stack overflow panic");
    }
    
    #[test]
    fn test_cpu_peek() {
        let mut cpu = CPU::new();
        let mut memory = vec![0; 256];
        cpu.push(0x01, &mut memory);
        assert_eq!(cpu.peek(&memory), 0x01);
    }

    #[test]
    fn test_cpu_poke() {
        let mut cpu = CPU::new();
        let mut memory = vec![0; 256];
        cpu.push(0x01, &mut memory);
        cpu.poke(0xFE, &mut memory);
        assert_eq!(cpu.peek(&memory), 0xFE);
    }


    #[test]
    fn test_reset() {
        let mut cpu = CPU::new();
        let mut memory = vec![0; 256];
        cpu.push(0x01, &mut memory);
        cpu.reset();
        assert_eq!(cpu.sp, 0xFF);
    }
}