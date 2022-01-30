use crate::{
    ast::{Atom, Expr},
    env::Env,
    errors::{RuntimeError, SpressoError},
    eval::{execute, execute_single},
};

pub fn while_loop(args: Vec<Expr>, env: &mut Env) -> Result<Expr, SpressoError> {
    if args.len() < 2 {
        return Err(SpressoError::from(RuntimeError::from(
            "Loop statement should have a condition and a list of expressions to evaluate",
        )));
    }
    let condition = execute_single(args[0].clone(), env)?;
    let mut body = args[1..].to_vec();

    if let Expr::Atom(Atom::Bool(mut cond)) = condition {
        while cond {
            execute(&mut body, env)?;
            if let Expr::Atom(Atom::Bool(boolean)) = execute_single(args[0].clone(), env)? {
                cond = boolean;
            } else {
                return Err(SpressoError::from(RuntimeError::from(
                    "Trying to use a non bool for condition",
                )));
            }
        }
        return Ok(Expr::Atom(Atom::Bool(true)));
    } else {
        Err(SpressoError::from(RuntimeError::from(
            "Trying to use a non bool for condition",
        )))
    }
}
