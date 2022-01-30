use crate::{
    ast::{Atom, Expr},
    env::Env,
    errors::{RuntimeError, SpressoError},
    eval::execute,
};

pub fn if_cond(args: Vec<Expr>, env: &mut Env) -> Result<Expr, SpressoError> {
    if !(args.len() == 2 || args.len() == 3) {
        return Err(SpressoError::from(RuntimeError::from("If statement should have a condition, expression to evaluate when true and optionally an expression to evaluate when false.")));
    }

    let mut args = args.clone();
    let cond = args.remove(0);

    let cond = execute(&mut vec![cond], env)?;

    if let Expr::Atom(Atom::Bool(boolean)) = cond {
        if boolean {
            // execute true
            let true_cond = args.remove(0);
            execute(&mut vec![true_cond], env)
        } else {
            // execute false
            if args.len() > 1 {
                let false_cond = args.pop().unwrap();
                execute(&mut vec![false_cond], env)
            } else {
                Ok(Expr::Atom(Atom::Bool(false)))
            }
        }
    } else {
        Err(SpressoError::from(RuntimeError::from(
            "Trying to use a non bool for condition",
        )))
    }
}
