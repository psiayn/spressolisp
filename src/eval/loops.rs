use crate::{
    ast::{Atom, Expr, ExprKind},
    env::Env,
    errors::{RuntimeError, SpressoError},
    eval::execute_single,
    TokenGiver, TokenHoarder,
};

pub fn while_loop(args: &mut [Expr], env: &mut Env) -> Result<Expr, SpressoError> {
    if args.len() < 2 {
        return Err(SpressoError::from(RuntimeError::from(
            "Loop statement should have a condition and a list of expressions to evaluate",
        ))
        .maybe_with_tokens(args.get_tokens()));
    }
    let (condition, body) = args.split_first_mut().unwrap();
    let first_condition_result = execute_single(condition, env)?;

    if let ExprKind::Atom(Atom::Bool(mut cond)) = first_condition_result.kind {
        while cond {
            for expr in body.iter_mut() {
                execute_single(expr, env)?;
            }
            if let ExprKind::Atom(Atom::Bool(boolean)) = execute_single(condition, env)?.kind {
                cond = boolean;
            } else {
                return Err(SpressoError::from(RuntimeError::from(
                    "Trying to use a non bool for condition",
                ))
                .maybe_with_tokens(condition.get_tokens())
                .maybe_with_tokens(args[0].get_tokens()));
            }
        }
        Ok(ExprKind::Atom(Atom::Unit).into())
    } else {
        Err(
            SpressoError::from(RuntimeError::from("Trying to use a non bool for condition"))
                .maybe_with_tokens(first_condition_result.get_tokens())
                .maybe_with_tokens(args[0].get_tokens()),
        )
    }
}
