use crate::{
    ast::{Atom, Expr, ExprKind, Number},
    env::Env,
    errors::{RuntimeError, SpressoError},
    eval::execute_single,
    eval::functions,
    TokenGiver, TokenHoarder,
};

/// Map function that iterates over a list and applies a lambda over it
/// # Usage
/// `(map list lambda)`
pub fn map(args: &mut [Expr], env: &mut Env) -> Result<Expr, SpressoError> {
    if args.len() < 2 {
        return Err(SpressoError::from(RuntimeError::from(
            "Map should have a list and a lambda to evaluate",
        ))
        .maybe_with_tokens(args.get_tokens()));
    }
    // get the list
    let list = execute_single(&mut args[0], env)?;

    // get the lambda
    let lambda = execute_single(&mut args[1], env)?;

    // check if we got a lambda or something else
    if let ExprKind::Lambda(lambda) = lambda.kind {
        // check if we got a list or something else
        if let ExprKind::List(ref list) = list.kind {
            // apply the lambda over every element
            let res: Result<Vec<Expr>, SpressoError> = list
                // TODO: no clone
                .clone()
                .into_iter()
                .map(|ele| functions::execute_lambda(&lambda, &mut [ele], env))
                .collect();
            // handle errors and return the result
            Ok(Expr::from(ExprKind::List(res?)))
        } else {
            Err(SpressoError::from(RuntimeError::from(
                "Map: expected list as input got something else",
            ))
            .maybe_with_tokens(list.get_tokens()))
        }
    } else {
        Err(SpressoError::from(RuntimeError::from(
            "Map: expected lambda got something else",
        ))
        .maybe_with_tokens(lambda.get_tokens()))
    }
}

pub fn append(args: &mut [Expr], env: &mut Env) -> Result<Expr, SpressoError> {
    if args.len() < 2 {
        return Err(
            SpressoError::from(RuntimeError::from("Append should have two lists"))
                .maybe_with_tokens(args.get_tokens()),
        );
    }

    let list_1 = execute_single(&mut args[0], env)?;
    let list_2 = execute_single(&mut args[1], env)?;

    if let ExprKind::List(ref a) = list_1.kind {
        if let ExprKind::List(mut b) = list_2.kind {
            // TODO: no clone
            let mut initial_list = a.clone();
            initial_list.append(&mut b);
            Ok(Expr::from(ExprKind::List(initial_list)))
        } else {
            Err(SpressoError::from(RuntimeError::from(
                "Append: expected list as input got something else",
            ))
            .maybe_with_tokens(list_2.get_tokens()))
        }
    } else {
        Err(SpressoError::from(RuntimeError::from(
            "Append: expected list as input got something else",
        ))
        .maybe_with_tokens(list_1.get_tokens()))
    }
}

/// Get an element from a list by index
/// # Usage
/// `(nth list index)`
pub fn nth(args: &mut [Expr], env: &mut Env) -> Result<Expr, SpressoError> {
    if args.len() != 2 {
        return Err(
            SpressoError::from(RuntimeError::from("nth needs a list and an index"))
                .maybe_with_tokens(args.get_tokens()),
        );
    }

    let list = execute_single(&mut args[0], env)?;
    let index = execute_single(&mut args[1], env)?;

    if let ExprKind::List(mut lst) = list.kind {
        if let ExprKind::Atom(Atom::Number(Number::Int(idx))) = index.kind {
            if idx < 0 || idx as usize >= lst.len() {
                return Err(SpressoError::from(RuntimeError::from(format!(
                    "Index {} out of bounds for list of length {}",
                    idx,
                    lst.len()
                )))
                .maybe_with_tokens(index.get_tokens()));
            }
            let expr = lst.swap_remove(idx as usize);
            Ok(expr)
        } else {
            Err(
                SpressoError::from(RuntimeError::from("nth: index must be an integer"))
                    .maybe_with_tokens(index.get_tokens()),
            )
        }
    } else {
        Err(
            SpressoError::from(RuntimeError::from("nth: first argument must be a list"))
                .maybe_with_tokens(list.get_tokens()),
        )
    }
}

/// Get the rest of the list after the first element
/// # Usage
/// `(rest list)`
pub fn rest(args: &mut [Expr], env: &mut Env) -> Result<Expr, SpressoError> {
    if args.len() != 1 {
        return Err(
            SpressoError::from(RuntimeError::from("rest needs a list argument"))
                .maybe_with_tokens(args.get_tokens()),
        );
    }

    let list = execute_single(&mut args[0], env)?;

    if let ExprKind::List(ref lst) = list.kind {
        if lst.is_empty() {
            Ok(ExprKind::List(vec![]).into())
        } else {
            Ok(ExprKind::List(lst[1..].to_vec()).into())
        }
    } else {
        Err(
            SpressoError::from(RuntimeError::from("rest: argument must be a list"))
                .maybe_with_tokens(list.get_tokens()),
        )
    }
}

/// Check if a list is empty
/// # Usage
/// `(empty? list)`
pub fn is_empty(args: &mut [Expr], env: &mut Env) -> Result<Expr, SpressoError> {
    if args.len() != 1 {
        return Err(
            SpressoError::from(RuntimeError::from("empty? needs a list argument"))
                .maybe_with_tokens(args.get_tokens()),
        );
    }

    let list = execute_single(&mut args[0], env)?;

    if let ExprKind::List(ref lst) = list.kind {
        Ok(ExprKind::Atom(Atom::Bool(lst.is_empty())).into())
    } else {
        Err(
            SpressoError::from(RuntimeError::from("empty?: argument must be a list"))
                .maybe_with_tokens(list.get_tokens()),
        )
    }
}

/// Reduce (fold) a list using a binary function
/// # Usage
/// `(reduce list initial_value lambda)`
/// The lambda should take two arguments: accumulator and current element
pub fn reduce(args: &mut [Expr], env: &mut Env) -> Result<Expr, SpressoError> {
    if args.len() != 3 {
        return Err(SpressoError::from(RuntimeError::from(
            "reduce needs a list, initial value, and a lambda",
        ))
        .maybe_with_tokens(args.get_tokens()));
    }

    let list = execute_single(&mut args[0], env)?;
    let initial = execute_single(&mut args[1], env)?;
    let lambda = execute_single(&mut args[2], env)?;

    if let ExprKind::Lambda(lambda) = lambda.kind {
        if let ExprKind::List(lst) = list.kind {
            // fold over the list using the lambda
            lst.into_iter().try_fold(initial, |acc, elem| {
                functions::execute_lambda(&lambda, &mut [acc, elem], env)
            })
        } else {
            Err(
                SpressoError::from(RuntimeError::from("reduce: first argument must be a list"))
                    .maybe_with_tokens(list.get_tokens()),
            )
        }
    } else {
        Err(SpressoError::from(RuntimeError::from(
            "reduce: third argument must be a lambda",
        ))
        .maybe_with_tokens(lambda.get_tokens()))
    }
}

/// Filter a list using a predicate function
/// # Usage
/// `(filter list lambda)`
/// The lambda should take one argument and return a boolean
pub fn filter(args: &mut [Expr], env: &mut Env) -> Result<Expr, SpressoError> {
    if args.len() != 2 {
        return Err(SpressoError::from(RuntimeError::from(
            "filter needs a list and a predicate lambda",
        ))
        .maybe_with_tokens(args.get_tokens()));
    }

    let list = execute_single(&mut args[0], env)?;
    let lambda = execute_single(&mut args[1], env)?;

    if let ExprKind::Lambda(lambda) = lambda.kind {
        if let ExprKind::List(lst) = list.kind {
            // filter the list using the predicate lambda
            let filtered: Result<Vec<Expr>, SpressoError> = lst
                .into_iter()
                .map(|elem| {
                    let mut lambda_args = [elem];
                    let result = functions::execute_lambda(&lambda, &mut lambda_args, env)?;
                    if let ExprKind::Atom(Atom::Bool(keep)) = result.kind {
                        let [expr] = lambda_args;
                        Ok((keep, expr))
                    } else {
                        Err(SpressoError::from(RuntimeError::from(
                            "filter: predicate must return a boolean",
                        ))
                        .maybe_with_tokens(result.get_tokens()))
                    }
                })
                .filter_map(|r: Result<(bool, Expr), SpressoError>| match r {
                    Ok((true, expr)) => Some(Ok(expr)),
                    Ok((false, _)) => None,
                    Err(e) => Some(Err(e)),
                })
                .collect();

            Ok(ExprKind::List(filtered?).into())
        } else {
            Err(
                SpressoError::from(RuntimeError::from("filter: first argument must be a list"))
                    .maybe_with_tokens(list.get_tokens()),
            )
        }
    } else {
        Err(SpressoError::from(RuntimeError::from(
            "filter: second argument must be a lambda",
        ))
        .maybe_with_tokens(lambda.get_tokens()))
    }
}

pub fn join(args: &mut [Expr], env: &mut Env) -> Result<Expr, SpressoError> {
    if args.len() != 1 {
        return Err(
            SpressoError::from(RuntimeError::from("join: needs a single list"))
                .maybe_with_tokens(args.get_tokens()),
        );
    }

    let list = execute_single(&mut args[0], env)?;
    if let ExprKind::List(lst) = list.kind {
        let extracted_list = lst
            .into_iter()
            .map(|x| {
                if let ExprKind::Atom(Atom::String(str)) = x.kind {
                    Ok(str)
                } else {
                    Err(
                        SpressoError::from(RuntimeError::from("join: invalid items in list"))
                            .maybe_with_tokens(x.get_tokens()),
                    )
                }
            })
            .collect::<Result<Vec<_>, SpressoError>>()?;

        Ok(ExprKind::Atom(Atom::String(extracted_list.join(""))).into())
    } else {
        Err(
            SpressoError::from(RuntimeError::from("join: argument must be a list"))
                .maybe_with_tokens(list.get_tokens()),
        )
    }
}
