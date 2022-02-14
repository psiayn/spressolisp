use crate::{
    ast::{Atom, AtomKind, Expr},
    env::Env,
    errors::{RuntimeError, SpressoError},
    eval::execute_single,
};

pub fn and(args: Vec<Expr>, env: &mut Env) -> Result<Expr, SpressoError> {
    if args.len() != 2 {
        return Err(SpressoError::from(RuntimeError::from(
            "`and` needs 2 arguments.",
        )));
    }

    let lhs = execute_single(args[0].clone(), env)?;
    let rhs = execute_single(args[1].clone(), env)?;
    match lhs {
        Expr::Atom(Atom {
            kind: AtomKind::Bool(lhs),
            ..
        }) => match rhs {
            Expr::Atom(Atom {
                kind: AtomKind::Bool(rhs),
                ..
            }) => return Ok(Expr::Atom(Atom::new_bool(lhs && rhs))),
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

    let lhs = execute_single(args[0].clone(), env)?;
    let rhs = execute_single(args[1].clone(), env)?;
    match lhs {
        Expr::Atom(Atom {
            kind: AtomKind::Bool(lhs),
            ..
        }) => match rhs {
            Expr::Atom(Atom {
                kind: AtomKind::Bool(rhs),
                ..
            }) => return Ok(Expr::Atom(Atom::new_bool(lhs || rhs))),
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

    let expr = execute_single(args[0].clone(), env)?;
    match expr {
        Expr::Atom(Atom {
            kind: AtomKind::Bool(arg),
            ..
        }) => return Ok(Expr::Atom(Atom::new_bool(!arg))),
        _ => {
            return Err(SpressoError::from(RuntimeError::from(
                "arg needs to be bool",
            )))
        }
    }
}
