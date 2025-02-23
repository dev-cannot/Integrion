use crate::environment::Environment;
use crate::parser::Expr;

pub fn eval(expr: &Expr, env: &mut Environment) -> i64 {
    match expr {
        Expr::Number(n) => *n,
        Expr::Add(lhs, rhs) => eval(lhs, env) + eval(rhs, env),
        Expr::Sub(lhs, rhs) => eval(lhs, env) - eval(rhs, env),
        Expr::Mul(lhs, rhs) => eval(lhs, env) * eval(rhs, env),
        Expr::Div(lhs, rhs) => eval(lhs, env) / eval(rhs, env),
        Expr::Var(name) => env.get(name).unwrap_or(0),
        Expr::Assign(name, expr) => {
            let value = eval(expr, env);
            env.set(name, value);
            value
        }
        Expr::Block(statements) => {
            let mut last_value = 0;
            for stmt in statements {
                last_value = eval(stmt, env);
            }
            last_value
        }
    }
}
