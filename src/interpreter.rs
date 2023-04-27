use crate::types::{Instruction, Register, Either};

// As each register is indexed instead of 
// using HashMap<Register, i64> we can use Vec<i64>
pub struct Interpreter {
    state: Vec<i64>,
    curr_instruction: usize,
}

impl Interpreter {
    pub fn new(num_registers: usize) -> Self {
        let mut state = Vec::with_capacity(num_registers);
        state.resize(num_registers, 0);
        Self {
            state,
            curr_instruction: 0,
        }
    }

    pub fn execute(&mut self, instructions: &Vec<Instruction>) {
        self.curr_instruction = 0;
        while self.curr_instruction < instructions.len() {
            let instruction = &instructions[self.curr_instruction];
            let mut stop_next = false;
            match instruction {
                Instruction::Mov(x, y) => self.mov(x, y),
                Instruction::Inc(x) => self.inc(x),
                Instruction::Dec(x) => self.dec(x),
                Instruction::Jnz(x, y) => {
                    stop_next = self.jnz(x, y);
                },
            }
            self.curr_instruction += 1;
            if stop_next {
                self.curr_instruction -= 1;
            }
        }
    }

    pub fn get_state(&self) -> &Vec<i64> {
        &self.state
    }

    fn mov(&mut self, x: &Register, y: &Either) {
        self.state[*x] = match y {
            Either::Register(reg) => self.state[*reg],
            Either::Constant(con) => *con,
        }
    }

    fn inc(&mut self, x: &Register) {
        self.state[*x] += 1
    }

    fn dec(&mut self, x: &Register) {
        self.state[*x] -= 1
    }

    /// Returns true if needed to stop jumping to next instruction
    fn jnz(&mut self, x: &Either, y: &Either) -> bool {

        let x = match *x {
            Either::Constant(con) => con,
            Either::Register(x) => self.state[x],
        };
        if x == 0 {
            return false;
        }

        let steps = match y {
            Either::Register(reg) => self.state[*reg],
            Either::Constant(con) => *con,
        };
        
        if (self.curr_instruction as i64) < steps {
            return false;
        } else {
            self.curr_instruction = (self.curr_instruction as i64 + steps) as usize;
            return true;
        }
    }
}