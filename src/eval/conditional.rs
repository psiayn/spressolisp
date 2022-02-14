use crate::{
    ast::{Atom, Expr, AtomKind},
    env::Env,
    errors::{RuntimeError, SpressoError},
    eval::execute_single,
};

pub fn if_cond(args: Vec<Expr>, env: &mut Env) -> Result<Expr, SpressoError> {
    if !(args.len() == 2 || args.len() == 3) {
        return Err(SpressoError::from(RuntimeError::from("If statement should have a condition, expression to evaluate when true and optionally an expression to evaluate when false.")));
    }

    let mut args = args.clone();
    let cond = args.remove(0);

    let cond = execute_single(cond, env)?;

    if let Expr::Atom(Atom{kind: AtomKind::Bool(boolean), ..}) = cond {
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
                // TODO: maybe return a unit type () here
                Ok(Expr::Atom(Atom::new_bool(false)))
            }
        }
    } else {
        Err(SpressoError::from(RuntimeError::from(
            "Trying to use a non bool for condition",
        )))
    }
}
