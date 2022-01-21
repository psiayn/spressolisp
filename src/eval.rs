use crate::{
    ast::{Atom, Expr, Lambda, Number},
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
        Expr::Atom(Atom::Symbol(symbol)) => {
            if env.contains_key(symbol.as_str()) {
                let sym = env[&symbol.as_str()].clone();
                match sym {
                    Expr::Atom(Atom::Number(num)) => Ok(num),
                    _ => Err(SpressoError::from(NumericError {
                        err: "Tried to extract num from variable but failed".to_string(),
                    })),
                }
            } else {
                return Err(SpressoError::from(RuntimeError::from(format!(
                    "Symbol not found: {}",
                    symbol
                ))));
            }
        }
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

pub fn execute(exprs: &Vec<Expr>, env: &mut Env) -> Result<Expr, SpressoError> {
    let mut exprs = exprs.clone();

    if exprs.len() == 1 {
        if let Expr::Atom(atom) = exprs[0].clone() {
            // return Ok(expr[0].clone());
            // extract symbol and return value if any
            match atom {
                Atom::Symbol(symbol) => {
                    if env.contains_key(&symbol.as_str()) {
                        return Ok(env[&symbol.as_str()].clone());
                    } else {
                        return Err(SpressoError::from(RuntimeError::from(format!(
                            "Symbol not found: {}",
                            symbol
                        ))));
                    }
                }
                _ => return Ok(Expr::Atom(atom)),
            }
        }
    }

    let first_arg = exprs.remove(0);
    match first_arg {
        Expr::Func(func) => func(exprs.to_vec(), env),
        Expr::Atom(Atom::Symbol(symbol)) => {
            if env.contains_key(symbol.as_str()) {
                let func = &env[symbol.as_str()];
                exprs.insert(0, func.clone());
                execute(&exprs, env)
            } else {
                Err(SpressoError::from(RuntimeError::from(format!(
                    "Symbol not found: {}",
                    symbol
                ))))
            }
        }
        Expr::List(mut exprs) => execute(&mut exprs, env),
        Expr::Lambda(lambda) => {
            // TODO: multiple args
            let arg = execute(&mut vec![exprs[0].clone()], env)?;
            env.in_new_scope(|env| {
                env.insert(lambda.params[0].as_str(), arg.clone());
                execute(&mut vec![*lambda.body.clone()], env)
            })
        }
        _ => Err(SpressoError::from(RuntimeError::from(format!(
            "Why you calling something else, when it's not function: {}",
            first_arg
        )))),
    }
}

pub fn define(args: Vec<Expr>, env: &mut Env) -> Result<Expr, SpressoError> {
    let mut args = args.clone();
    let variable_name = args.remove(0);
    let result = execute(&mut args, env)?;
    env.insert(&variable_name.to_string().trim(), result.clone());
    Ok(result)
}

pub fn print(args: Vec<Expr>, env: &mut Env) -> Result<Expr, SpressoError> {
    let mut args = args.clone();
    let result = execute(&mut args, env)?;
    Ok(result)
}

pub fn lambda(args: Vec<Expr>, _env: &mut Env) -> Result<Expr, SpressoError> {
    if args.len() != 2 {
        return Err(SpressoError::from(RuntimeError::from(
            "A lambda definition must have a param list and a body",
        )));
    }

    let fn_params = args[0].clone();
    let body = args[1].clone();

    // TODO: support multiple parameters (tuple of args)
    // for now only single parameters

    if let Expr::Atom(Atom::Symbol(fn_param)) = fn_params {
        Ok(Expr::Lambda(Lambda {
            params: vec![fn_param],
            body: Box::new(body),
        }))
    } else {
        Err(SpressoError::from(RuntimeError::from(
            "lambda parameters must be a symbol",
        )))
    }
}
