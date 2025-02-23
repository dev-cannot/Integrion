// src/evaluator.rs

use crate::parser::Expr;
use crate::environment::Environment;

pub fn eval(expr: &Expr, env: &mut Environment) -> i64 {
    match expr {
        Expr::Number(n) => *n,
        Expr::Add(lhs, rhs) => eval(lhs, env) + eval(rhs, env),
        Expr::Sub(lhs, rhs) => eval(lhs, env) - eval(rhs, env),
        Expr::Mul(lhs, rhs) => eval(lhs, env) * eval(rhs, env),
        Expr::Div(lhs, rhs) => eval(lhs, env) / eval(rhs, env),
        Expr::Var(name) => {
            env.get(name).expect(&format!("Undefined variable: {}", name))
        },
        Expr::Assign(name, expr) => {
            let value = eval(expr, env);
            env.set(name.clone(), value);
            value
        },
    }
}
