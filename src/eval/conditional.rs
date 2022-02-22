use crate::{
    ast::{Atom, Expr, ExprKind},
    env::Env,
    errors::{RuntimeError, SpressoError},
    eval::execute_single,
    TokenGiver, TokenHoarder,
};

pub fn if_cond(args: Vec<Expr>, env: &mut Env) -> Result<Expr, SpressoError> {
    if !(args.len() == 2 || args.len() == 3) {
        return Err(SpressoError::from(RuntimeError::from("If statement should have a condition, expression to evaluate when true and optionally an expression to evaluate when false.")).maybe_with_tokens(args.get_tokens()));
    }

    let mut args = args.clone();
    let cond = args.remove(0);

    let cond = execute_single(cond, env)?;

    if let ExprKind::Atom(Atom::Bool(boolean)) = cond.kind {
        if boolean {
            // execute true
            let true_cond = args.remove(0);
            execute_single(true_cond, env)
        } else {
            // execute false
            if args.len() > 1 {
                let false_cond = args.pop().unwrap();
                execute_single(false_cond, env)
            } else {
                Ok(ExprKind::Atom(Atom::Bool(false)).into())
            }
        }
    } else {
        Err(SpressoError::from(RuntimeError::from(
            "Trying to use a non bool for condition",
        )).maybe_with_tokens(cond.get_tokens()))
    }
}
