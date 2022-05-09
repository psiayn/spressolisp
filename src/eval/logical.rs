use crate::{
    ast::{Atom, Expr, ExprKind},
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
    match lhs.kind {
        ExprKind::Atom(Atom::Bool(lhs)) => match rhs.kind {
            ExprKind::Atom(Atom::Bool(rhs)) => Ok(ExprKind::Atom(Atom::Bool(lhs && rhs)).into()),
            _ => Err(SpressoError::from(RuntimeError::from(
                "RHS needs to be bool",
            ))),
        },
        _ => Err(SpressoError::from(RuntimeError::from(
            "LHS needs to be bool",
        ))),
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
    match lhs.kind {
        ExprKind::Atom(Atom::Bool(lhs)) => match rhs.kind {
            ExprKind::Atom(Atom::Bool(rhs)) => Ok(ExprKind::Atom(Atom::Bool(lhs || rhs)).into()),
            _ => Err(SpressoError::from(RuntimeError::from(
                "RHS needs to be bool",
            ))),
        },
        _ => Err(SpressoError::from(RuntimeError::from(
            "LHS needs to be bool",
        ))),
    }
}

pub fn not(args: Vec<Expr>, env: &mut Env) -> Result<Expr, SpressoError> {
    if args.len() != 1 {
        return Err(SpressoError::from(RuntimeError::from(
            "`not` needs only 1 arguments.",
        )));
    }

    let expr = execute_single(args[0].clone(), env)?;
    match expr.kind {
        ExprKind::Atom(Atom::Bool(arg)) => Ok(ExprKind::Atom(Atom::Bool(!arg)).into()),
        _ => Err(SpressoError::from(RuntimeError::from(
            "arg needs to be bool",
        ))),
    }
}
