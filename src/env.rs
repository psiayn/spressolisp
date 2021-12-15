use std::collections::HashMap;

use crate::ast::{Expr, Atom, Number, SyntaxError};

pub fn standard_env() -> HashMap<String, Expr> {
    let mut env = HashMap::<String, Expr>::new();
    env.insert(
        "+".to_string(),
        Expr::Func(add),
    );
    return env;
}

fn add(args: Vec<Expr>) -> Result<Expr, SyntaxError> {
    let res = args.into_iter().try_fold(Number::Int(0), |x, y| {
        match y {
            Expr::Atom(Atom::Number(num)) => Ok(x + num),
            _ => Err(SyntaxError{ err: "TF you adding you retard".to_string() }),
        }
    });
    match res {
        Ok(result) => Ok(Expr::Atom(Atom::Number(result))),
        Err(err) => Err(err),
    }
}

pub fn execute(expr: &mut Vec<Expr>, env: HashMap<String, Expr>) -> Result<Expr, SyntaxError> {
    let first_arg = expr.remove(0);
    match first_arg {
        Expr::Func(func) => func(expr.to_vec()),
        Expr::Atom(Atom::Symbol(symbol)) => {
            if env.contains_key(symbol.as_str()) {
                let func = &env[symbol.as_str()];
                expr.insert(0, func.clone());
                execute(expr, env)
            } else {
                Err(SyntaxError{ err: "Symbol not found".to_string() })
            }
        },
        _ => Err(SyntaxError{ err: "Why you calling something else, when it's not function".to_string() })
    }
}
