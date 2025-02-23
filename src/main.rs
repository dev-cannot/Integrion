mod parser;
mod evaluator;
mod environment;

use parser::parse_expr;
use evaluator::eval;
use environment::Environment;
use std::io::{self, Write};

fn main() {
    let mut env = Environment::new();
    
    println!("Integrion Interpreter - Type 'exit' to quit.");
    
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Error reading input.");
            continue;
        }
        
        if input.trim() == "exit" {
            break;
        }
        
        match parse_expr(&input) {
            Ok((_remaining, expr)) => {
                let result = eval(&expr, &mut env);
                println!("Result: {}", result);
            }
            Err(err) => println!("Error: {:?}", err),
        }
    }
}
