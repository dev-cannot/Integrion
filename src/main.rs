mod parser;
mod evaluator;
mod environment;

use parser::{parse_program, Expr};
use evaluator::eval;
use environment::Environment;

fn main() {
    let input = "fn add(x, y) { x + y } add(2, 3)";
    let (_, expr) = parse_program(input).expect("Failed to parse program");

    let mut env = Environment::new();
    let result = eval(&expr, &mut env);

    println!("Result: {}", result);
}
