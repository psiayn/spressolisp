mod conditional;
mod functions;
mod lists;
mod logical;
mod loops;
mod macros;
mod number;
mod relational;
mod strings;
mod types;

use std::io;

pub use conditional::*;
pub use functions::*;
pub use lists::*;
pub use logical::*;
pub use loops::*;
pub use macros::*;
pub use number::*;
pub use relational::*;
pub use strings::*;
pub use types::*;

use crate::{
    ast::{Atom, Expr, ExprKind},
    env::Env,
    errors::{RuntimeError, SpressoError},
    TokenGiver, TokenHoarder,
};

pub fn execute(exprs: &mut Vec<Expr>, env: &mut Env) -> Result<Expr, SpressoError> {
    let (first, rest) = exprs.split_at_mut(1);
    let first_arg = &mut first[0];

    match first_arg.kind {
        ExprKind::Func(func) => func(rest.to_vec(), env),
        ExprKind::List(ref mut list) => {
            // println!("Execute: List => {:?}", list);
            let res = execute(list, env)?;
            let mut evaluated = vec![res];
            evaluated.extend_from_slice(rest);
            execute(&mut evaluated, env)
        }
        ExprKind::Atom(Atom::Symbol(ref symbol)) => {
            // println!("Execute: Atom: Symbol => {:?}", symbol);
            let value = env
                .get_symbol(symbol.as_str())
                .maybe_with_tokens(first_arg.get_tokens());

            exprs[0] = value?;
            execute(exprs, env)
        }
        // ExprKind::Atom(Atom::String(string)) => {
        //     println!("Execute: Atom: String => {:?}", string);
        //     Ok(ExprKind::Atom(Atom::String(string)).into())
        // }
        // ExprKind::Atom(Atom::Number(number)) => {
        //     println!("Execute: Atom: Number => {:?}", number);
        //     Ok(ExprKind::Atom(Atom::Number(number)).into())
        // }
        // ExprKind::Atom(Atom::Bool(bool_val)) => {
        //     println!("Execute: Atom: Bool => {:?}", bool_val);
        //     Ok(ExprKind::Atom(Atom::Bool(bool_val)).into())
        // }
        // ExprKind::Atom(Atom::Unit) => {
        //     println!("Execute: Atom: Unit");
        //     Ok(ExprKind::Atom(Atom::Unit).into())
        // }
        ExprKind::Lambda(ref lambda) => execute_lambda(lambda, rest.to_vec(), env),
        ExprKind::Macro(ref macro_def) => {
            // Expand the macro with unevaluated arguments
            let expanded = expand_macro(macro_def, rest.to_vec(), env)?;
            // Then evaluate the expanded form
            execute_single(expanded, env)
        }
        _ => Err(SpressoError::from(RuntimeError::from(format!(
            "this is not something I can execute: {}",
            first_arg
        )))
        .maybe_with_tokens(first_arg.get_tokens())),
    }
}

pub fn execute_single(expr: Expr, env: &mut Env) -> Result<Expr, SpressoError> {
    let res = match expr.kind {
        ExprKind::Func(func) => func(vec![], env),
        ExprKind::Atom(Atom::Symbol(ref symbol)) => env
            .get_symbol(symbol.as_str())
            .maybe_with_tokens(expr.get_tokens()),
        ExprKind::List(mut exprs) => execute(&mut exprs, env),
        ExprKind::Lambda(lambda) => execute_lambda(&lambda, vec![], env),
        ExprKind::Macro(macro_def) => {
            // Expand the macro with no arguments
            let expanded = expand_macro(&macro_def, vec![], env)?;
            // Then evaluate the expanded form
            execute_single(expanded, env)
        }
        ExprKind::Atom(_) => Ok(expr),
    };

    env.cleanup();

    res
}

pub fn define(args: Vec<Expr>, env: &mut Env) -> Result<Expr, SpressoError> {
    if args.len() != 2 {
        return Err(SpressoError::from(RuntimeError::from(
            "define needs a variable name and a value to assign to it.",
        ))
        .maybe_with_tokens(args.get_tokens()));
    }

    let variable_name = args[0].clone();
    let result = execute_single(args[1].clone(), env)?.maybe_with_tokens(args.get_tokens());
    env.insert(variable_name.to_string().trim(), result.clone());
    Ok(result)
}

pub fn print(args: Vec<Expr>, env: &mut Env) -> Result<Expr, SpressoError> {
    for arg in args {
        let result = execute_single(arg, env)?;
        println!("{}", result);
    }
    Ok(Expr::from(ExprKind::Atom(Atom::Unit)))
}

pub fn input(args: Vec<Expr>, env: &mut Env) -> Result<Expr, SpressoError> {
    if !args.is_empty() {
        print(args, env)?;
    }
    let mut buffer = String::new();
    if let Err(err) = io::stdin().read_line(&mut buffer) {
        return Err(SpressoError::from(RuntimeError::from(format!("{}", err))));
    }
    buffer = buffer.trim().to_string();
    Ok(Expr::from(ExprKind::Atom(Atom::String(buffer))))
}

pub fn spresso_list(args: Vec<Expr>, env: &mut Env) -> Result<Expr, SpressoError> {
    if args.is_empty() {
        return Ok(ExprKind::List(vec![]).into());
    }
    if args.len() == 1 {
        match &args[0].kind {
            ExprKind::List(list) => Ok({
                let mut res: Vec<Expr> = Vec::new();
                for i in list {
                    let item_res = execute_single(i.clone(), env)?;
                    res.push(item_res);
                }
                ExprKind::List(res).into()
            }),
            _ => Err(SpressoError::from(RuntimeError::from(format!(
                "list needs a list. got: {}",
                args[0]
            )))
            .maybe_with_tokens(args.get_tokens())),
        }
    } else {
        Err(SpressoError::from(RuntimeError::from(format!(
            "list takes only one list argument. got: {:?}",
            args
        )))
        .maybe_with_tokens(args.get_tokens()))
    }
}
