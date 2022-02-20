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
    ast::{Atom, Expr, ExprKind},
    env::Env,
    errors::{RuntimeError, SpressoError},
    TokenGiver, TokenHoarder,
};

pub fn execute(exprs: &mut Vec<Expr>, env: &mut Env) -> Result<Expr, SpressoError> {
    let first_arg = exprs[0].clone();
    match first_arg.kind {
        ExprKind::Func(func) => {
            func(exprs[1..].to_vec(), env).maybe_with_tokens(first_arg.get_tokens())
        }
        ExprKind::Atom(Atom::Symbol(ref symbol)) => {
            let sym = env
                .get_symbol(symbol.as_str())?
                .maybe_with_tokens(first_arg.get_tokens());

            if exprs.len() > 1 {
                exprs[0] = sym;
                execute(exprs, env)
            } else {
                Ok(sym)
            }
        }
        ExprKind::Lambda(lambda) => execute_lambda(lambda, exprs[1..].to_vec(), env),
        _ => execute_single(first_arg, env),
    }
}

pub fn execute_single(expr: Expr, env: &mut Env) -> Result<Expr, SpressoError> {
    match expr.kind {
        ExprKind::Func(func) => func(vec![], env).maybe_with_tokens(expr.get_tokens()),
        ExprKind::Atom(Atom::Symbol(symbol)) => Ok(env.get_symbol(symbol.as_str())?),
        ExprKind::List(mut exprs) => execute(&mut exprs, env),
        ExprKind::Lambda(lambda) => execute_lambda(lambda, vec![], env),
        ExprKind::Atom(_) => Ok(expr),
    }
}

pub fn define(args: Vec<Expr>, env: &mut Env) -> Result<Expr, SpressoError> {
    if args.len() != 2 {
        return Err(SpressoError::from(RuntimeError::from(
            "define needs a variable name and a value to assign to it.",
        ))
        .maybe_with_tokens(args.get_tokens()));
    }

    let variable_name = args[0].clone();
    let result = execute_single(args[1].clone(), env)?.maybe_with_tokens(args.get_tokens());
    env.insert(&variable_name.to_string().trim(), result.clone());
    Ok(result)
}

pub fn print(args: Vec<Expr>, env: &mut Env) -> Result<Expr, SpressoError> {
    let mut args = args.clone();
    let result = execute(&mut args, env)?;
    println!("{}", result);
    Ok(result)
}

pub fn lists(args: Vec<Expr>, _: &mut Env) -> Result<Expr, SpressoError> {
    if args.len() != 1 {
        return Err(SpressoError::from(RuntimeError::from(
            "' only needs one arg",
        )));
    }
    Ok(args[0].clone())
}
