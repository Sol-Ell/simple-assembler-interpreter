#![allow(dead_code)]

use std::collections::HashMap;

use crate::types::{Instruction, Either, Register, Registers, MyFromStr};

#[derive(Debug)]
pub struct Parser<'a> {
    registers: Registers<'a>,
    new_reg: Register,
    instructions: Vec<Instruction>,
}

impl<'a> Parser<'a> {
    /// Parses program and generates instruction list.
    pub fn new() -> Self {
        Self {
            registers: HashMap::new(),
            new_reg: 0,
            instructions: vec![]
        }
    }

    /// Use it to preallocate memory for instructions
    pub fn with_instructions(cnt: usize) -> Self {
        Self {
            registers: HashMap::new(),
            new_reg: 0,
            instructions: Vec::with_capacity(cnt),
        }
    }

    /// Parses instructions and returns number of errorneous
    pub fn parse(&mut self, program: Vec<&'a str>) -> usize {
        let mut errorneous = 0;
        for line in program {
            if let Some(instruction) = self.parse_line(line) {
                self.instructions.push(instruction);
            } else {
                errorneous += 1;
            }
        }

        errorneous
    }

    pub fn get_instructions(&self) -> &Vec<Instruction> {
        &self.instructions
    }

    pub fn get_registers(&self) -> &HashMap<&'a str, Register> {
        &self.registers
    }

    fn parse_mov(&mut self, x: &'a str, y: &'a str) -> Instruction {
        let register: Register = 
            if let Some(key) = self.registers.get(x) {
                *key
            } else {
                self.registers.insert(x, self.new_reg);
                self.new_reg += 1;
                self.new_reg - 1
            };
        
        Instruction::Mov(register, Either::from_str(y, &self.registers))
    }

    fn parse_inc(&self, x: &str) -> Instruction {
        Instruction::Inc(Register::from_str(x, &self.registers))
    }

    fn parse_dec(&self, x: &str) -> Instruction {
        Instruction::Dec(Register::from_str(x, &self.registers))
    }

    fn parse_jnz(&self, x: &str, y: &str) -> Option<Instruction> {
        // Mini optimization:
        // as "jnz 0 1" 
        //         | |
        //         ^ ^- never executes 
        //         ^--- because always stay 0
        let x = Either::from_str(x, &self.registers);
        if x == Either::Constant(0) {
            return None;
        }
        let y = Either::from_str(y, &self.registers);
        Some(Instruction::Jnz(x, y))
    }
    
    fn parse_line(&mut self, line: &'a str) -> Option<Instruction> {
        let tokens = line.split_whitespace().collect::<Vec<&str>>();
        let len = tokens.len();
        
        match (tokens[0], len) {
            ("mov", 3) => Some(self.parse_mov(tokens[1], tokens[2])),
            ("inc", 2) => Some(self.parse_inc(tokens[1])),
            ("dec", 2) => Some(self.parse_dec(tokens[1])),
            ("jnz", 3) => self.parse_jnz(tokens[1], tokens[2]),
            _ => {
                println!("Wrong instruction: {:?}", tokens);
                None
            },
        }
    }
}