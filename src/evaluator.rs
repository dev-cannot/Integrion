use crate::parser::Expr;
use crate::environment::Environment;

pub fn eval(expr: &Expr, env: &mut Environment) -> i64 {
    match expr {
        Expr::Number(n) => *n,
        Expr::Ident(name) | Expr::Var(name) => env.get(name),
        Expr::Assign(name, value) => {
            let val = eval(value, env);
            env.set(name.as_str(), val);
            val
        }
        Expr::Add(left, right) => eval(left, env) + eval(right, env),
        Expr::Sub(left, right) => eval(left, env) - eval(right, env),
        Expr::Mul(left, right) => eval(left, env) * eval(right, env),
        Expr::Div(left, right) => eval(left, env) / eval(right, env),
        Expr::If(condition, then_block, else_block) => {
            if eval(condition, env) != 0 {
                eval(then_block, env)
            } else {
                else_block.as_ref().map_or(0, |block| eval(block, env))
            }
        }
        Expr::While(condition, body) => {
            let mut result = 0;
            while eval(condition, env) != 0 {
                result = eval(body, env);
            }
            result
        }
        Expr::Block(exprs) => {
            let mut last_val = 0;
            for e in exprs {
                last_val = eval(e, env);
            }
            last_val
        }
    }
}
