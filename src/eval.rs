use crate::{
    ast::{Atom, Expr, Number, RuntimeError},
    env::EnvType,
};

fn number_op(
    args: Vec<Expr>,
    env: &mut EnvType,
    init: Number,
    op: fn(Number, Number) -> Number,
) -> Result<Expr, RuntimeError> {
    let res = args.into_iter().try_fold(init, |x, y| {
        let num = extract_num(y, env)?;
        Ok(op(x, num))
    });
    Ok(Expr::Atom(Atom::Number(res?)))
}

fn extract_num(expr: Expr, env: &mut EnvType) -> Result<Number, RuntimeError> {
    match expr {
        Expr::Atom(Atom::Number(number)) => Ok(number),
        Expr::List(mut exprs) => {
            let res = execute(&mut exprs, env)?;
            match res {
                Expr::Atom(Atom::Number(num)) => Ok(num),
                _ => Err(RuntimeError {
                    err: format!("trying to perform arithmetic on non-number: {}", res).to_string(),
                }),
            }
        }
        _ => Err(RuntimeError {
            err: format!("trying to perform arithmetic on non-number: {}", expr).to_string(),
        }),
    }
}

pub fn add(args: Vec<Expr>, env: &mut EnvType) -> Result<Expr, RuntimeError> {
    number_op(args, env, Number::Int(0), |x, y| x + y)
}

pub fn mul(args: Vec<Expr>, env: &mut EnvType) -> Result<Expr, RuntimeError> {
    number_op(args, env, Number::Int(1), |x, y| x * y)
}

pub fn sub(args: Vec<Expr>, env: &mut EnvType) -> Result<Expr, RuntimeError> {
    let mut args = args.clone();
    let start = extract_num(args.remove(0), env)?;
    number_op(args, env, start, |x, y| x - y)
}

pub fn execute(expr: &mut Vec<Expr>, env: &mut EnvType) -> Result<Expr, RuntimeError> {
    let first_arg = expr.remove(0);
    match first_arg {
        Expr::Func(func) => func(expr.to_vec(), env),
        Expr::Atom(Atom::Symbol(symbol)) => {
            if env.contains_key(symbol.as_str()) {
                let func = &env[symbol.as_str()];
                expr.insert(0, func.clone());
                execute(expr, env)
            } else {
                Err(RuntimeError {
                    err: "Symbol not found".to_string(),
                })
            }
        }
        Expr::List(mut exprs) => execute(&mut exprs, env),
        _ => Err(RuntimeError {
            err: "Why you calling something else, when it's not function".to_string(),
        }),
    }
}
