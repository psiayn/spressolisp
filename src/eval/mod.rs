mod conditional;
mod functions;
mod logical;
mod loops;
mod number;
mod relational;

pub use conditional::*;
pub use functions::*;
pub use logical::*;
pub use loops::*;
pub use number::*;
pub use relational::*;

use crate::{
    ast::{Atom, Expr},
    env::Env,
    errors::{RuntimeError, SpressoError},
};

pub fn execute(exprs: &Vec<Expr>, env: &mut Env) -> Result<Expr, SpressoError> {
    let mut exprs = exprs.clone();

    let first_arg = exprs.remove(0);
    match first_arg {
        Expr::Func(func) => func(exprs.to_vec(), env),
        Expr::Atom(Atom::Symbol(symbol)) => {
            if env.contains_key(symbol.as_str()) {
                let sym = env[symbol.as_str()].clone();

                if !exprs.is_empty() {
                    exprs.insert(0, sym);
                    execute(&exprs, env)
                } else {
                    Ok(sym)
                }
            } else {
                Err(SpressoError::from(RuntimeError::from(format!(
                    "Symbol not found: {}",
                    symbol
                ))))
            }
        }
        Expr::List(mut exprs) => execute(&mut exprs, env),
        Expr::Lambda(lambda) => execute_lambda(lambda, exprs, env),
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
    println!("{}", result);
    Ok(result)
}
