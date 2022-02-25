use crate::{
    ast::{Expr, ExprKind},
    env::Env,
    errors::{RuntimeError, SpressoError},
    eval::execute_single,
    eval::functions,
    TokenGiver, TokenHoarder,
};

/// Map function that iterates over a list and applies a lambda over it
/// # Usage
/// `(map list lambda)`
pub fn map(args: Vec<Expr>, env: &mut Env) -> Result<Expr, SpressoError> {
    if args.len() < 2 {
        return Err(SpressoError::from(RuntimeError::from(
            "Map should have a list and a lambda to evaluate",
        ))
        .maybe_with_tokens(args.get_tokens()));
    }
    // get the list
    let list = execute_single(args[0].clone(), env)?;

    // get the lambda
    let lambda = execute_single(args[1].clone(), env)?;

    // check if we got a lambda or something else
    if let ExprKind::Lambda(lambda) = lambda.kind {
        // check if we got a list or something else
        if let ExprKind::List(ref list) = list.kind {
            // apply the lambda over every element
            let res: Result<Vec<Expr>, SpressoError> = list
                .clone()
                .into_iter()
                .map(|ele| functions::execute_lambda(lambda.clone(), vec![ele], env))
                .collect();
            // handle errors and return the result
            match res {
                Ok(result) => return Ok(Expr::from(ExprKind::List(result))),
                Err(err) => return Err(err),
            };
        } else {
            return Err(SpressoError::from(RuntimeError::from(
                "Map: expected list as input got something else",
            ))
            .maybe_with_tokens(list.get_tokens()));
        }
    } else {
        return Err(SpressoError::from(RuntimeError::from(
            "Map: expected lambda got something else",
        ))
        .maybe_with_tokens(lambda.get_tokens()));
    }
}

pub fn append(args: Vec<Expr>, env: &mut Env) -> Result<Expr, SpressoError> {
    if args.len() < 2 {
        return Err(
            SpressoError::from(RuntimeError::from("Append should have two lists"))
                .maybe_with_tokens(args.get_tokens()),
        );
    }

    let list_1 = execute_single(args[0].clone(), env)?;
    let list_2 = execute_single(args[1].clone(), env)?;

    if let ExprKind::List(ref a) = list_1.kind {
        if let ExprKind::List(mut b) = list_2.kind {
            let mut initial_list = a.clone();
            initial_list.append(&mut b);
            return Ok(Expr::from(ExprKind::List(initial_list)));
        } else {
            return Err(SpressoError::from(RuntimeError::from(
                "Append: expected list as input got something else",
            ))
            .maybe_with_tokens(list_1.get_tokens()));
        }
    } else {
        return Err(SpressoError::from(RuntimeError::from(
            "Append: expected list as input got something else",
        ))
        .maybe_with_tokens(list_2.get_tokens()));
    }
}
