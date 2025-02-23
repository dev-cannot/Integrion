use crate::environment::Environment;
use crate::parser::Expr;

pub fn eval(expr: &Expr, env: &mut Environment) -> i64 {
    match expr {
        Expr::Number(n) => *n,
        Expr::Ident(name) => env.get(name),
        Expr::Add(lhs, rhs) => eval(lhs, env) + eval(rhs, env),
        Expr::Sub(lhs, rhs) => eval(lhs, env) - eval(rhs, env),
        Expr::Mul(lhs, rhs) => eval(lhs, env) * eval(rhs, env),
        Expr::Div(lhs, rhs) => eval(lhs, env) / eval(rhs, env),

        Expr::FuncDef(name, params, body) => {
            env.set_func(name.clone(), params.clone(), *body.clone());
            0
        }
        Expr::FuncCall(name, args) => {
            let (params, body) = env.get_func(name);
            let mut local_env = env.clone();

            for (param, arg) in params.iter().zip(args.iter()) {
                local_env.set(param.clone(), eval(arg, env));
            }

            eval(&body, &mut local_env)
        }

        _ => panic!("Unsupported expression"),
    }
}
