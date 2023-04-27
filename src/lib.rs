use std::collections::HashMap;
use parser::Parser;

use crate::interpreter::Interpreter;

mod tests;

mod types;
mod parser;
mod interpreter;

fn simple_assembler(program: Vec<&str>) -> HashMap<String, i64> {
    let mut parser = Parser::with_instructions(program.len());
    parser.parse(program);

    let mut executor = Interpreter::new(parser.get_registers().len());
    executor.execute(parser.get_instructions());
    println!("{:?}", parser.get_instructions());
    
    let mut result = HashMap::new();
    let state = executor.get_state();
    
    for pair in parser.get_registers() {
        result.insert(String::from(pair.0.clone()), state[*pair.1]);
    }

    println!("State: {:?}\n\n", result);

    result
}