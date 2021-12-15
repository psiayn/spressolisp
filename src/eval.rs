use crate::{
    ast::{Atom, Expr, Number, RuntimeError},
    env::EnvType,
};

pub fn add(args: Vec<Expr>, env: &mut EnvType) -> Result<Expr, RuntimeError> {
    let res = args.into_iter().try_fold(Number::Int(0), |x, y| match y {
        Expr::Atom(Atom::Number(num)) => Ok(x + num),
        Expr::List(mut exprs) => match execute(&mut exprs, env) {
            Ok(res) => match add(vec![Expr::Atom(Atom::Number(x)), res], env) {
                Ok(Expr::Atom(Atom::Number(num))) => Ok(num),
                _ => Err(RuntimeError {
                    err: "brrrrrrrr this should not happen".to_string(),
                }),
            },
            Err(e) => Err(e),
        },
        _ => Err(RuntimeError {
            err: "TF you adding you retard".to_string(),
        }),
    });
    match res {
        Ok(result) => Ok(Expr::Atom(Atom::Number(result))),
        Err(err) => Err(err),
    }
}

pub fn mul(args: Vec<Expr>, env: &mut EnvType) -> Result<Expr, RuntimeError> {
    let res = args.into_iter().try_fold(Number::Int(1), |x, y| match y {
        Expr::Atom(Atom::Number(num)) => Ok(x * num),
        Expr::List(mut exprs) => match execute(&mut exprs, env) {
            Ok(res) => match add(vec![Expr::Atom(Atom::Number(x)), res], env) {
                Ok(Expr::Atom(Atom::Number(num))) => Ok(num),
                _ => Err(RuntimeError {
                    err: "brrrrrrrr this should not happen".to_string(),
                }),
            },
            Err(e) => Err(e),
        },
        _ => Err(RuntimeError {
            err: "TF you adding you retard".to_string(),
        }),
    });
    match res {
        Ok(result) => Ok(Expr::Atom(Atom::Number(result))),
        Err(err) => Err(err),
    }
}

pub fn sub(args: Vec<Expr>, env: &mut EnvType) -> Result<Expr, RuntimeError> {
    let mut args = args.clone();
    let first = args.remove(0);
    let result = match first {
        Expr::Atom(Atom::Number(number)) => Ok(number),
        Expr::List(mut exprs) => match execute(&mut exprs, env) {
            Ok(res) => match res {
                Expr::Atom(Atom::Number(num)) => Ok(num),
                _ => Err(RuntimeError {
                    err: "trying to perform arithmetic on non-number".to_string(),
                }),
            },
            _ => Err(RuntimeError {
                err: "trying to perform arithmetic on non-number".to_string(),
            }),
        },
        _ => Err(RuntimeError {
            err: "trying to perform arithmetic operation on non-number".to_string(),
        }),
    };
    let start = match result {
        Ok(res) => res,
        Err(err) => return Err(err),
    };
    let res = args.into_iter().try_fold(start, |x, y| match y {
        Expr::Atom(Atom::Number(num)) => Ok(x - num),
        Expr::List(mut exprs) => match execute(&mut exprs, env) {
            Ok(res) => match add(vec![Expr::Atom(Atom::Number(x)), res], env) {
                Ok(Expr::Atom(Atom::Number(num))) => Ok(num),
                _ => Err(RuntimeError {
                    err: "brrrrrrrr this should not happen".to_string(),
                }),
            },
            Err(e) => Err(e),
        },
        _ => Err(RuntimeError {
            err: "TF you adding you retard".to_string(),
        }),
    });
    match res {
        Ok(result) => Ok(Expr::Atom(Atom::Number(result))),
        Err(err) => Err(err),
    }
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
