// src/evaluator.rs

use crate::parser::Expr;

/// Evaluates the given arithmetic expression and returns the result as i64.
pub fn eval(expr: &Expr) -> i64 {
    match expr {
        Expr::Number(n) => *n,
        Expr::Add(lhs, rhs) => eval(lhs) + eval(rhs),
        Expr::Sub(lhs, rhs) => eval(lhs) - eval(rhs),
        Expr::Mul(lhs, rhs) => eval(lhs) * eval(rhs),
        Expr::Div(lhs, rhs) => eval(lhs) / eval(rhs),
    }
}
