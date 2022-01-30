use crate::{
    ast::{Atom, Expr},
    env::Env,
    errors::{RuntimeError, SpressoError},
    eval::execute,
};

pub fn and(args: Vec<Expr>, env: &mut Env) -> Result<Expr, SpressoError> {
    if args.len() != 2 {
        return Err(SpressoError::from(RuntimeError::from(
            "`and` needs 2 arguments.",
        )));
    }

    let lhs = execute(&mut vec![args[0].clone()], env)?;
    let rhs = execute(&mut vec![args[1].clone()], env)?;
    match lhs {
        Expr::Atom(Atom::Bool(lhs)) => match rhs {
            Expr::Atom(Atom::Bool(rhs)) => return Ok(Expr::Atom(Atom::Bool(lhs && rhs))),
            _ => {
                return Err(SpressoError::from(RuntimeError::from(
                    "RHS needs to be bool",
                )))
            }
        },
        _ => {
            return Err(SpressoError::from(RuntimeError::from(
                "LHS needs to be bool",
            )))
        }
    }
}

pub fn or(args: Vec<Expr>, env: &mut Env) -> Result<Expr, SpressoError> {
    if args.len() != 2 {
        return Err(SpressoError::from(RuntimeError::from(
            "`or` needs 2 arguments.",
        )));
    }

    let lhs = execute(&mut vec![args[0].clone()], env)?;
    let rhs = execute(&mut vec![args[1].clone()], env)?;
    match lhs {
        Expr::Atom(Atom::Bool(lhs)) => match rhs {
            Expr::Atom(Atom::Bool(rhs)) => return Ok(Expr::Atom(Atom::Bool(lhs || rhs))),
            _ => {
                return Err(SpressoError::from(RuntimeError::from(
                    "RHS needs to be bool",
                )))
            }
        },
        _ => {
            return Err(SpressoError::from(RuntimeError::from(
                "LHS needs to be bool",
            )))
        }
    }
}

pub fn not(args: Vec<Expr>, env: &mut Env) -> Result<Expr, SpressoError> {
    if args.len() != 1 {
        return Err(SpressoError::from(RuntimeError::from(
            "`not` needs only 1 arguments.",
        )));
    }

    let expr = execute(&mut vec![args[0].clone()], env)?;
    match expr {
        Expr::Atom(Atom::Bool(arg)) => return Ok(Expr::Atom(Atom::Bool(!arg))),
        _ => {
            return Err(SpressoError::from(RuntimeError::from(
                "arg needs to be bool",
            )))
        }
    }
}
