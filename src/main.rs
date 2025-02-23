// src/main.rs
mod parser; // tells Rust to include the parser.rs file

use parser::{parse_expr, Expr};

fn main() {
    let input = "3 + 4 * (2 - 1)";
    match parse_expr(input) {
        Ok((remaining, expr)) if remaining.trim().is_empty() => {
            println!("Parsed expression: {:?}", expr);
            // Future work: Pass the AST to an evaluator function
        },
        Ok((remaining, _)) => println!("Unparsed input remains: {:?}", remaining),
        Err(err) => println!("Error parsing input: {:?}", err),
    }
}
