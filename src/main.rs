use std::io::{self, Write};

mod parser;
mod evaluator;
mod environment;

use parser::parse_program;
use evaluator::eval;
use environment::Environment;

fn main() {
    let mut env = Environment::new();
    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "exit" {
            break;
        }

        match parse_program(&input) {
            Ok((_, expr)) => {
                let result = eval(&expr, &mut env);
                println!("Result: {}", result);
            }
            Err(err) => println!("Error: {:?}", err),
        }
    }
}
