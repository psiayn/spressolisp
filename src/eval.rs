use crate::{ast::{Number, Expr, SyntaxError, Atom}, env::EnvType};

pub fn add(args: Vec<Expr>, env: &mut EnvType) -> Result<Expr, SyntaxError> {
    let res = args.into_iter().try_fold(Number::Int(0), |x, y| {
        match y {
            Expr::Atom(Atom::Number(num)) => Ok(x + num),
            Expr::List(mut exprs) => match execute(&mut exprs, env) {
                Ok(res) => match add(vec![Expr::Atom(Atom::Number(x)), res], env) {
                    Ok(Expr::Atom(Atom::Number(num))) => Ok(num),
                    _ => Err(SyntaxError{ err: "brrrrrrrr this should not happen".to_string() })
                },
                Err(e) => Err(e),
            },
            _ => Err(SyntaxError{ err: "TF you adding you retard".to_string() }),
        }
    });
    match res {
        Ok(result) => Ok(Expr::Atom(Atom::Number(result))),
        Err(err) => Err(err),
    }
}

pub fn mul(args: Vec<Expr>, env: &mut EnvType) -> Result<Expr, SyntaxError> {
    let res = args.into_iter().try_fold(Number::Int(1), |x, y| {
        match y {
            Expr::Atom(Atom::Number(num)) => Ok(x * num),
            Expr::List(mut exprs) => match execute(&mut exprs, env) {
                Ok(res) => match add(vec![Expr::Atom(Atom::Number(x)), res], env) {
                    Ok(Expr::Atom(Atom::Number(num))) => Ok(num),
                    _ => Err(SyntaxError{ err: "brrrrrrrr this should not happen".to_string() })
                },
                Err(e) => Err(e),
            },
            _ => Err(SyntaxError{ err: "TF you adding you retard".to_string() }),
        }
    });
    match res {
        Ok(result) => Ok(Expr::Atom(Atom::Number(result))),
        Err(err) => Err(err),
    }
}

pub fn execute(expr: &mut Vec<Expr>, env: &mut EnvType) -> Result<Expr, SyntaxError> {
    let first_arg = expr.remove(0);
    match first_arg {
        Expr::Func(func) => func(expr.to_vec(), env),
        Expr::Atom(Atom::Symbol(symbol)) => {
            if env.contains_key(symbol.as_str()) {
                let func = &env[symbol.as_str()];
                expr.insert(0, func.clone());
                execute(expr, env)
            } else {
                Err(SyntaxError{ err: "Symbol not found".to_string() })
            }
        },
        Expr::List(mut exprs) => execute(&mut exprs, env),
        _ => Err(SyntaxError{ err: "Why you calling something else, when it's not function".to_string() })
    }
}
