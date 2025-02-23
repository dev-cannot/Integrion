// src/main.rs
mod parser;
mod evaluator; // Include the evaluator module

use parser::{parse_expr, Expr};
use evaluator::eval;

fn main() {
    let input = "3 + 4 * (2 - 1)";
    match parse_expr(input) {
        Ok((remaining, expr)) if remaining.trim().is_empty() => {
            println!("Parsed expression: {:?}", expr);
            // Evaluate the parsed AST
            let result = eval(&expr);
            println!("Evaluation result: {}", result);
        },
        Ok((remaining, _)) => println!("Unparsed input remains: {:?}", remaining),
        Err(err) => println!("Error parsing input: {:?}", err),
    }
}
