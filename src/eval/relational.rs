use crate::{
    ast::{Atom, Expr, ExprKind},
    env::Env,
    errors::{SpressoError, SyntaxError},
    eval::{execute_single, extract_num},
};

pub fn lt(args: Vec<Expr>, env: &mut Env) -> Result<Expr, SpressoError> {
    if args.len() != 2 {
        return Err(SpressoError::from(SyntaxError::from(
            "Tried to call < with more than 2 args",
        )));
    }

    // execute the statements and get the results
    let first = execute_single(args[0].clone(), env)?;
    let second = execute_single(args[1].clone(), env)?;
    // override by trying to extract num
    let first = extract_num(first, env)?;
    let second = extract_num(second, env)?;
    // return result
    Ok(ExprKind::Atom(Atom::Bool(first < second)).into())
}

pub fn lteq(args: Vec<Expr>, env: &mut Env) -> Result<Expr, SpressoError> {
    if args.len() != 2 {
        return Err(SpressoError::from(SyntaxError::from(
            "Tried to call <= with more than 2 args",
        )));
    }

    // execute the statements and get the results
    let first = execute_single(args[0].clone(), env)?;
    let second = execute_single(args[1].clone(), env)?;
    // override by trying to extract num
    let first = extract_num(first, env)?;
    let second = extract_num(second, env)?;
    // return result
    Ok(ExprKind::Atom(Atom::Bool(first <= second)).into())
}

pub fn gt(args: Vec<Expr>, env: &mut Env) -> Result<Expr, SpressoError> {
    if args.len() != 2 {
        return Err(SpressoError::from(SyntaxError::from(
            "Tried to call > with more than 2 args",
        )));
    }

    // execute the statements and get the results
    let first = execute_single(args[0].clone(), env)?;
    let second = execute_single(args[1].clone(), env)?;
    // override by trying to extract num
    let first = extract_num(first, env)?;
    let second = extract_num(second, env)?;
    // return result
    Ok(ExprKind::Atom(Atom::Bool(first > second)).into())
}

pub fn gteq(args: Vec<Expr>, env: &mut Env) -> Result<Expr, SpressoError> {
    if args.len() != 2 {
        return Err(SpressoError::from(SyntaxError::from(
            "Tried to call >= with more than 2 args",
        )));
    }

    // execute the statements and get the results
    let first = execute_single(args[0].clone(), env)?;
    let second = execute_single(args[1].clone(), env)?;
    // override by trying to extract num
    let first = extract_num(first, env)?;
    let second = extract_num(second, env)?;
    // return result
    Ok(ExprKind::Atom(Atom::Bool(first >= second)).into())
}

pub fn eq(args: Vec<Expr>, env: &mut Env) -> Result<Expr, SpressoError> {
    if args.len() != 2 {
        return Err(SpressoError::from(SyntaxError::from(
            "Tried to call == with more than 2 args",
        )));
    }

    // execute the statements and get the results
    let first = execute_single(args[0].clone(), env)?;
    let second = execute_single(args[1].clone(), env)?;
    // override by trying to extract num
    let first = extract_num(first, env)?;
    let second = extract_num(second, env)?;
    // return result
    Ok(ExprKind::Atom(Atom::Bool(first == second)).into())
}

pub fn neq(args: Vec<Expr>, env: &mut Env) -> Result<Expr, SpressoError> {
    if args.len() != 2 {
        return Err(SpressoError::from(SyntaxError::from(
            "Tried to call != with more than 2 args",
        )));
    }

    // execute the statements and get the results
    let first = execute_single(args[0].clone(), env)?;
    let second = execute_single(args[1].clone(), env)?;
    // override by trying to extract num
    let first = extract_num(first, env)?;
    let second = extract_num(second, env)?;
    // return result
    Ok(ExprKind::Atom(Atom::Bool(first != second)).into())
}
