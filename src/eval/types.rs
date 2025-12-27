use crate::{
    ast::{Atom, Expr, ExprKind},
    env::Env,
    errors::{RuntimeError, SpressoError},
    TokenGiver, TokenHoarder,
};

use super::extract_num;

pub fn cast_as_num(args: &mut [Expr], env: &mut Env) -> Result<Expr, SpressoError> {
    if args.len() != 1 {
        return Err(SpressoError::from(RuntimeError::from(
            "number needs an expression to cast into a number",
        ))
        .maybe_with_tokens(args.get_tokens()));
    }
    Ok(Expr::from(ExprKind::Atom(Atom::Number(extract_num(
        &mut args[0],
        env,
    )?))))
}
