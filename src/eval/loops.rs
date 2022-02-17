use crate::{
    ast::{Atom, Expr, ExprKind},
    env::Env,
    errors::{RuntimeError, SpressoError},
    eval::execute_single,
};

pub fn while_loop(args: Vec<Expr>, env: &mut Env) -> Result<Expr, SpressoError> {
    if args.len() < 2 {
        return Err(SpressoError::from(RuntimeError::from(
            "Loop statement should have a condition and a list of expressions to evaluate",
        )));
    }
    let condition = execute_single(args[0].clone(), env)?;
    let body = args[1..].to_vec();

    if let ExprKind::Atom(Atom::Bool(mut cond)) = condition.kind {
        while cond {
            for expr in body.clone() {
                execute_single(expr, env)?;
            }
            if let ExprKind::Atom(Atom::Bool(boolean)) = execute_single(args[0].clone(), env)?.kind {
                cond = boolean;
            } else {
                return Err(SpressoError::from(RuntimeError::from(
                    "Trying to use a non bool for condition",
                )));
            }
        }
        return Ok(ExprKind::Atom(Atom::Bool(true)).into());
    } else {
        Err(SpressoError::from(RuntimeError::from(
            "Trying to use a non bool for condition",
        )))
    }
}
