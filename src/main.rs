// src/main.rs

mod parser;
mod evaluator;
mod environment;

use parser::parse_expr;
use evaluator::eval;
use environment::Environment;

fn main() {
    let mut env = Environment::new();

    // Test assignment: "x = 3 + 4 * (2 - 1)"
    let input = "x = 3 + 4 * (2 - 1)";
    match parse_expr(input) {
        Ok((remaining, expr)) if remaining.trim().is_empty() => {
            println!("Parsed expression: {:?}", expr);
            let result = eval(&expr, &mut env);
            println!("Assignment result: {}", result);
        },
        Ok((remaining, _)) => println!("Unparsed input remains: {:?}", remaining),
        Err(err) => println!("Error parsing input: {:?}", err),
    }

    // Test variable lookup: "x"
    let input2 = "x";
    match parse_expr(input2) {
        Ok((remaining, expr)) if remaining.trim().is_empty() => {
            println!("Parsed variable expression: {:?}", expr);
            let value = eval(&expr, &mut env);
            println!("Value of x: {}", value);
        },
        Ok((remaining, _)) => println!("Unparsed input remains: {:?}", remaining),
        Err(err) => println!("Error parsing input: {:?}", err),
    }
}
