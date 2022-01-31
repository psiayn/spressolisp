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
    errors::SpressoError,
};

pub fn execute(exprs: &mut Vec<Expr>, env: &mut Env) -> Result<Expr, SpressoError> {
    let first_arg = exprs[0].clone();
    match first_arg {
        Expr::Func(func) => func(exprs[1..].to_vec(), env),
        Expr::Atom(Atom::Symbol(symbol)) => {
            let sym = env.get_symbol(symbol.as_str())?;

            if exprs.len() > 1 {
                exprs[0] = sym;
                execute(exprs, env)
            } else {
                Ok(sym)
            }
        }
        Expr::Lambda(lambda) => execute_lambda(lambda, exprs[1..].to_vec(), env),
        _ => execute_single(first_arg, env),
    }
}

pub fn execute_single(expr: Expr, env: &mut Env) -> Result<Expr, SpressoError> {
    match expr {
        Expr::Func(func) => func(vec![], env),
        Expr::Atom(Atom::Symbol(symbol)) => Ok(env.get_symbol(symbol.as_str())?),
        Expr::List(mut exprs) => execute(&mut exprs, env),
        Expr::Lambda(lambda) => execute_lambda(lambda, vec![], env),
        Expr::Atom(_) => Ok(expr),
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
