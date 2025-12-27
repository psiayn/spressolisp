use crate::{
    ast::{Atom, Expr, ExprKind},
    env::Env,
    errors::{SpressoError, SyntaxError},
    eval::{execute_single, extract_num},
};

pub fn lt(args: &mut [Expr], env: &mut Env) -> Result<Expr, SpressoError> {
    if args.len() != 2 {
        return Err(SpressoError::from(SyntaxError::from(
            "Tried to call < with more than 2 args",
        )));
    }

    // execute the statements and get the results
    let mut first = execute_single(&mut args[0], env)?;
    let mut second = execute_single(&mut args[1], env)?;
    // override by trying to extract num
    let first = extract_num(&mut first, env)?;
    let second = extract_num(&mut second, env)?;
    // return result
    Ok(ExprKind::Atom(Atom::Bool(first < second)).into())
}

pub fn lteq(args: &mut [Expr], env: &mut Env) -> Result<Expr, SpressoError> {
    if args.len() != 2 {
        return Err(SpressoError::from(SyntaxError::from(
            "Tried to call <= with more than 2 args",
        )));
    }

    // execute the statements and get the results
    let mut first = execute_single(&mut args[0], env)?;
    let mut second = execute_single(&mut args[1], env)?;
    // override by trying to extract num
    let first = extract_num(&mut first, env)?;
    let second = extract_num(&mut second, env)?;
    // return result
    Ok(ExprKind::Atom(Atom::Bool(first <= second)).into())
}

pub fn gt(args: &mut [Expr], env: &mut Env) -> Result<Expr, SpressoError> {
    if args.len() != 2 {
        return Err(SpressoError::from(SyntaxError::from(
            "Tried to call > with more than 2 args",
        )));
    }

    // execute the statements and get the results
    let mut first = execute_single(&mut args[0], env)?;
    let mut second = execute_single(&mut args[1], env)?;
    // override by trying to extract num
    let first = extract_num(&mut first, env)?;
    let second = extract_num(&mut second, env)?;
    // return result
    Ok(ExprKind::Atom(Atom::Bool(first > second)).into())
}

pub fn gteq(args: &mut [Expr], env: &mut Env) -> Result<Expr, SpressoError> {
    if args.len() != 2 {
        return Err(SpressoError::from(SyntaxError::from(
            "Tried to call >= with more than 2 args",
        )));
    }

    // execute the statements and get the results
    let mut first = execute_single(&mut args[0], env)?;
    let mut second = execute_single(&mut args[1], env)?;
    // override by trying to extract num
    let first = extract_num(&mut first, env)?;
    let second = extract_num(&mut second, env)?;
    // return result
    Ok(ExprKind::Atom(Atom::Bool(first >= second)).into())
}

pub fn eq(args: &mut [Expr], env: &mut Env) -> Result<Expr, SpressoError> {
    if args.len() != 2 {
        return Err(SpressoError::from(SyntaxError::from(
            "Tried to call == with more than 2 args",
        )));
    }

    // execute the statements and get the results
    let first = execute_single(&mut args[0], env)?;
    let second = execute_single(&mut args[1], env)?;
    // return result
    Ok(ExprKind::Atom(Atom::Bool(first == second)).into())
}

pub fn neq(args: &mut [Expr], env: &mut Env) -> Result<Expr, SpressoError> {
    if args.len() != 2 {
        return Err(SpressoError::from(SyntaxError::from(
            "Tried to call != with more than 2 args",
        )));
    }

    // execute the statements and get the results
    let first = execute_single(&mut args[0], env)?;
    let second = execute_single(&mut args[1], env)?;
    // return result
    Ok(ExprKind::Atom(Atom::Bool(first != second)).into())
}
