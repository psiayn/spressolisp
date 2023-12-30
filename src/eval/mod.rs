mod conditional;
mod functions;
mod lists;
mod logical;
mod loops;
mod number;
mod relational;
mod types;

use std::io;

pub use conditional::*;
pub use functions::*;
pub use lists::*;
pub use logical::*;
pub use loops::*;
pub use number::*;
pub use relational::*;
pub use types::*;

use crate::{
    ast::{Atom, Expr, ExprKind},
    env::Env,
    errors::{RuntimeError, SpressoError},
    TokenGiver, TokenHoarder,
};

pub fn execute(exprs: &mut Vec<Expr>, env: &mut Env) -> Result<Expr, SpressoError> {
    let first_arg = exprs[0].clone();
    match first_arg.kind {
        ExprKind::Func(func) => func(exprs[1..].to_vec(), env),
        ExprKind::List(mut list) => {
            let res = execute(&mut list, env)?;
            let mut evaluated = exprs[1..].to_vec();
            evaluated.insert(0, res);
            execute(&mut evaluated, env)
        }
        ExprKind::Atom(Atom::Symbol(ref symbol)) => {
            let value = env
                .get_symbol(symbol.as_str())
                .maybe_with_tokens(first_arg.get_tokens());

            exprs[0] = value?;
            execute(exprs, env)
        }
        ExprKind::Atom(Atom::String(_)) => {
            Ok(first_arg)
        }
        ExprKind::Atom(Atom::Number(_)) => {
            Ok(first_arg)
        }
        ExprKind::Lambda(lambda) => execute_lambda(lambda, exprs[1..].to_vec(), env),
        _ => Err(SpressoError::from(RuntimeError::from(format!(
            "this is not something I can execute: {}",
            first_arg
        )))
        .maybe_with_tokens(first_arg.get_tokens())),
    }
}

pub fn execute_single(expr: Expr, env: &mut Env) -> Result<Expr, SpressoError> {
    let res = match expr.kind {
        ExprKind::Func(func) => func(vec![], env),
        ExprKind::Atom(Atom::Symbol(ref symbol)) => env
            .get_symbol(symbol.as_str())
            .maybe_with_tokens(expr.get_tokens()),
        ExprKind::List(mut exprs) => execute(&mut exprs, env),
        ExprKind::Lambda(lambda) => execute_lambda(lambda, vec![], env),
        ExprKind::Atom(_) => Ok(expr),
    };

    env.cleanup();

    res
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
    env.insert(variable_name.to_string().trim(), result.clone());
    Ok(result)
}

pub fn print(args: Vec<Expr>, env: &mut Env) -> Result<Expr, SpressoError> {
    let mut args = args;
    let result = execute(&mut args, env)?;
    println!("{}", result);
    Ok(Expr::from(ExprKind::Atom(Atom::Unit)))
}

pub fn input(_args: Vec<Expr>, _env: &mut Env) -> Result<Expr, SpressoError> {
    if _args.len() > 0 {
        print(_args, _env)?;
    }
    let mut buffer = String::new();
    if let Err(err) = io::stdin().read_line(&mut buffer) {
        return Err(SpressoError::from(RuntimeError::from(format!("{}", err))));
    }
    buffer = buffer.trim().to_string();
    Ok(Expr::from(ExprKind::Atom(Atom::String(buffer))))
}

pub fn list(args: Vec<Expr>, _: &mut Env) -> Result<Expr, SpressoError> {
    if args.len() != 1 {
        return Err(
            SpressoError::from(RuntimeError::from("' only needs one arg"))
                .maybe_with_tokens(args.get_tokens()),
        );
    }
    Ok(args[0].clone())
}
