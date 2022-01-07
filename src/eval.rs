use crate::{
    ast::{Atom, Expr, Number},
    env::Env,
    errors::{NumericError, RuntimeError, SpressoError},
};

fn number_op(
    args: Vec<Expr>,
    env: &mut Env,
    init: Number,
    op: fn(Number, Number) -> Result<Number, SpressoError>,
) -> Result<Expr, SpressoError> {
    Ok(Expr::Atom(Atom::Number(
        args.into_iter()
            .try_fold::<_, _, Result<Number, SpressoError>>(init, |x, y| {
                let num = extract_num(y, env)?;
                op(x, num)
            })?,
    )))
}

pub fn extract_num(expr: Expr, env: &mut Env) -> Result<Number, SpressoError> {
    match expr {
        Expr::Atom(Atom::Number(number)) => Ok(number),
        Expr::List(mut exprs) => {
            let res = execute(&mut exprs, env)?;
            match res {
                Expr::Atom(Atom::Number(num)) => Ok(num),
                _ => Err(SpressoError::Numeric(NumericError::from(format!(
                    "trying to perform arithmetic on non-number: {}",
                    res
                )))),
            }
        }
        _ => Err(SpressoError::from(NumericError::from(format!(
            "trying to perform arithmetic on non-number: {}",
            expr
        )))),
    }
}

pub fn add(args: Vec<Expr>, env: &mut Env) -> Result<Expr, SpressoError> {
    number_op(args, env, Number::Int(0), |x, y| x + y)
}

pub fn mul(args: Vec<Expr>, env: &mut Env) -> Result<Expr, SpressoError> {
    number_op(args, env, Number::Int(1), |x, y| x * y)
}

pub fn sub(args: Vec<Expr>, env: &mut Env) -> Result<Expr, SpressoError> {
    let mut args = args.clone();
    let start = extract_num(args.remove(0), env)?;
    number_op(args, env, start, |x, y| x - y)
}

pub fn div(args: Vec<Expr>, env: &mut Env) -> Result<Expr, SpressoError> {
    let mut args = args.clone();
    let start = extract_num(args.remove(0), env)?;
    number_op(args, env, start, |x, y| x / y)
}

pub fn execute(expr: &mut Vec<Expr>, env: &mut Env) 
    -> Result<Expr, SpressoError> {
    let first_arg = expr.remove(0);
    match first_arg {
        Expr::Func(func) => func(expr.to_vec(), env),
        Expr::Atom(Atom::Symbol(symbol)) => {
            if env.contains_key(symbol.as_str()) {
                let func = &env[symbol.as_str()];
                expr.insert(0, func.clone());
                execute(expr, env)
            } else {
                Err(SpressoError::from(RuntimeError::from(format!(
                    "Symbol not found: {}",
                    symbol
                ))))
            }
        }
        Expr::List(mut exprs) => execute(&mut exprs, env),
        _ => Err(SpressoError::from(RuntimeError::from(format!(
            "Why you calling something else, when it's not function: {}",
            first_arg
        )))),
    }
}