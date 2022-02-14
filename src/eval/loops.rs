use crate::{
    ast::{Atom, AtomKind, Expr},
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

    if let Expr::Atom(Atom {
        kind: AtomKind::Bool(mut cond),
        ..
    }) = condition
    {
        while cond {
            for expr in body.clone() {
                execute_single(expr, env)?;
            }
            if let Expr::Atom(Atom {
                kind: AtomKind::Bool(boolean),
                ..
            }) = execute_single(args[0].clone(), env)?
            {
                cond = boolean;
            } else {
                return Err(SpressoError::from(RuntimeError::from(
                    "Trying to use a non bool for condition",
                )));
            }
        }
        return Ok(Expr::Atom(Atom::new_bool(true)));
    } else {
        Err(SpressoError::from(RuntimeError::from(
            "Trying to use a non bool for condition",
        )))
    }
}
