use crate::{
    ast::{Expr, ExprKind, Atom},
    env::Env,
    errors::{SpressoError, RuntimeError},
    TokenGiver,
    TokenHoarder
};

use super::extract_num;

pub fn cast_as_num(args: Vec<Expr>, env: &mut Env) -> Result<Expr, SpressoError> {
    if args.len() != 1 {
        return Err(SpressoError::from(RuntimeError::from(
            "number needs an expression to cast into a number",
        )).maybe_with_tokens(args.get_tokens()));
    }
    return Ok(Expr::from(ExprKind::Atom(Atom::Number(extract_num(args[0].clone(), env)?))));
}
