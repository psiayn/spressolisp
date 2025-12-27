use crate::{
    ast::{Atom, Expr, ExprKind},
    env::Env,
    errors::{RuntimeError, SpressoError},
    eval::execute_single,
};

pub fn and(args: &mut [Expr], env: &mut Env) -> Result<Expr, SpressoError> {
    if args.len() != 2 {
        return Err(SpressoError::from(RuntimeError::from(
            "`and` needs 2 arguments.",
        )));
    }

    let lhs = execute_single(&mut args[0], env)?;
    let rhs = execute_single(&mut args[1], env)?;
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

pub fn or(args: &mut [Expr], env: &mut Env) -> Result<Expr, SpressoError> {
    if args.len() != 2 {
        return Err(SpressoError::from(RuntimeError::from(
            "`or` needs 2 arguments.",
        )));
    }

    let lhs = execute_single(&mut args[0], env)?;
    let rhs = execute_single(&mut args[1], env)?;
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

pub fn not(args: &mut [Expr], env: &mut Env) -> Result<Expr, SpressoError> {
    if args.len() != 1 {
        return Err(SpressoError::from(RuntimeError::from(
            "`not` needs only 1 arguments.",
        )));
    }

    let expr = execute_single(&mut args[0], env)?;
    match expr.kind {
        ExprKind::Atom(Atom::Bool(arg)) => Ok(ExprKind::Atom(Atom::Bool(!arg)).into()),
        _ => Err(SpressoError::from(RuntimeError::from(
            "arg needs to be bool",
        ))),
    }
}
