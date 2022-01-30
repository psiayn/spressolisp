mod conditional;
mod functions;
mod logical;
mod number;
mod relational;
mod loops;

pub use conditional::*;
pub use functions::*;
pub use logical::*;
pub use number::*;
pub use relational::*;
pub use loops::*;

use crate::{
    ast::{Atom, Expr},
    env::Env,
    errors::{RuntimeError, SpressoError},
};

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
            let args: Result<Vec<Expr>, SpressoError> = exprs
                .into_iter()
                .map(|arg| execute(&mut vec![arg], env))
                .collect();
            let args = args?;

            if args.len() != lambda.params.len() {
                Err(SpressoError::from(RuntimeError::from(format!(
                    "Expected {} arguments, got {}",
                    lambda.params.len(),
                    args.len()
                ))))
            } else {
                env.in_new_scope(|env| {
                    args.into_iter().enumerate().for_each(|(i, arg)| {
                        env.insert(lambda.params[i].as_str(), arg);
                    });
                    execute(&lambda.body, env)
                })
            }
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
    println!("{}", result);
    Ok(result)
}
