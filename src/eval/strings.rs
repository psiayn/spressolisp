use crate::{
    ast::{Atom, Expr, ExprKind},
    env::Env,
    errors::{RuntimeError, SpressoError},
    eval::execute_single,
    TokenGiver, TokenHoarder,
};

pub fn split(args: Vec<Expr>, env: &mut Env) -> Result<Expr, SpressoError> {
    match &args[..] {
        [string] => {
            let input_string = execute_single(string.clone(), env)?;
            if let ExprKind::Atom(Atom::String(str)) = input_string.kind {
                Ok(ExprKind::List(
                    str.split("")
                        .filter(|x| !x.is_empty())
                        .map(|x| ExprKind::Atom(Atom::String(x.to_string())).into())
                        .collect(),
                )
                .into())
            } else {
                Err(
                    SpressoError::from(RuntimeError::from("split: can only split a string"))
                        .maybe_with_tokens(string.get_tokens()),
                )
            }
        }
        [string, split_pattern] => {
            let input_string = execute_single(string.clone(), env)?;
            let split = execute_single(split_pattern.clone(), env)?;
            if let ExprKind::Atom(Atom::String(str)) = input_string.kind {
                if let ExprKind::Atom(Atom::String(split_str)) = split.kind {
                    Ok(ExprKind::List(
                        str.split(split_str.as_str())
                            .filter(|x| !x.is_empty())
                            .map(|x| ExprKind::Atom(Atom::String(x.to_string())).into())
                            .collect(),
                    )
                    .into())
                } else {
                    Err(SpressoError::from(RuntimeError::from(
                        "split: second argument must be a pattern to split the provided string with",
                    ))
                    .maybe_with_tokens(split.get_tokens()))
                }
            } else {
                Err(
                    SpressoError::from(RuntimeError::from("split: can only split a string"))
                        .maybe_with_tokens(input_string.get_tokens()),
                )
            }
        }
        _ => Err(
            SpressoError::from(RuntimeError::from("split: invalid number of arguments"))
                .maybe_with_tokens(args.get_tokens()),
        ),
    }
}

pub fn concat(args: Vec<Expr>, env: &mut Env) -> Result<Expr, SpressoError> {
    if args.len() < 2 {
        return Err(SpressoError::from(RuntimeError::from(
            "concat should have two strings to concat",
        )))
        .maybe_with_tokens(args.get_tokens());
    }

    let str_1 = execute_single(args[0].clone(), env)?;

    let str_2 = execute_single(args[1].clone(), env)?;

    if let ExprKind::Atom(Atom::String(first_str)) = str_1.kind {
        if let ExprKind::Atom(Atom::String(second_str)) = str_2.kind {
            let res = first_str + second_str.as_str();

            Ok(Expr::from(ExprKind::Atom(Atom::String(res))))
        } else {
            Err(SpressoError::from(RuntimeError::from(
                "Concat: expected strings as input got something else",
            ))
            .maybe_with_tokens(str_2.get_tokens()))
        }
    } else {
        Err(SpressoError::from(RuntimeError::from(
            "Concat: expected strings as input got something else",
        ))
        .maybe_with_tokens(str_1.get_tokens()))
    }
}
