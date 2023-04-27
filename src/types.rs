#![allow(dead_code)]

use std::collections::HashMap;

/**
Each register has unique index
R - Register
C - Constant
*/
pub type Register = usize;
pub type Registers<'a> = HashMap<&'a str, Register>;
pub type Constant = i64;

#[derive(Debug, PartialEq, Eq)]
pub enum Either {
    Register(Register),
    Constant(Constant),
}

/**
 * mov x y - copies y (either a constant value or the content of a register) into register x
 * inc x - increases the content of the register x by one
 * dec x - decreases the content of the register x by one
 * jnz x y - jumps to an instruction y steps away (positive means forward, negative means backward, y can be a register or a constant), but only if x (a constant or a register) is not zero
*/
#[derive(Debug)]
pub enum Instruction {
    Mov(Register, Either),
    Inc(Register),
    Dec(Register),
    Jnz(Either, Either),
}

pub trait MyFromStr {
    fn from_str(from: &str, registers: &Registers) -> Self;
}

impl MyFromStr for Register {
    fn from_str<'a>(from: &'a str, registers: &Registers) -> Self {
        *registers.get(from)
            .expect("Mov instruction error: Register doesn't exist.")
    }
}

impl MyFromStr for Either {
    fn from_str<'a >(from: &'a str, registers: &Registers) -> Self {
        match from.parse::<Constant>() {
            Ok(val) => Either::Constant(val),
            _ => Either::Register(Register::from_str(from, registers))
        }
    }
}