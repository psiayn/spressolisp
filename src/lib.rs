pub mod ast;
pub mod env;
pub mod eval;
pub mod errors;

use crate::ast::{Atom, Expr, Number};
use crate::env::Env;
use crate::errors::RuntimeError;
use crate::eval::execute;

pub fn evaluate_expression(input: String, env: &mut Env) -> Result<Expr, RuntimeError> {
    let mut tokenized_input: Vec<String> = tokenize(input);
    match parse(&mut tokenized_input) {
        Ok(ast) => {
            // println!("{}", ast);
            match ast {
                Expr::List(mut exprs) => execute(&mut exprs, env),
                _ => Err(RuntimeError::from(format!(
                    "Hmm I can't execute something that is not a list: {}",
                    ast
                ))),
            }
        }
        Err(e) => Err(e),
    }
}

fn tokenize(input: String) -> Vec<String> {
    let input: String = input.replace("(", " ( ").replace(")", " ) ");
    let res = input
        .split_whitespace()
        .map(|tok| tok.to_string())
        .collect();
    return res;
}

fn parse(tokens: &mut Vec<String>) -> Result<Expr, RuntimeError> {
    if tokens.len() == 0 {
        return Err(RuntimeError::from("Unexpected EOF".to_string()));
    }
    let token = tokens.remove(0);
    match token.as_str() {
        "(" => {
            if tokens.len() == 0 {
                return Err(RuntimeError::from("'(' not closed"));
            }
            let mut ast: Vec<Expr> = Vec::new();
            while tokens[0] != ")" {
                let inner_ast = match parse(tokens) {
                    Ok(res) => res,
                    Err(e) => return Err(e),
                };
                ast.push(inner_ast);
                if tokens.len() == 0 {
                    return Err(RuntimeError::from("'(' not closed"));
                }
            }
            tokens.remove(0);
            return Ok(Expr::List(ast));
        }
        ")" => return Err(RuntimeError::from("Unexpected ')'")),
        _ => Ok(Expr::Atom(parse_int(token))),
    }
}

fn parse_int(token: String) -> Atom {
    match token.parse::<i64>() {
        Ok(num) => return Atom::Number(Number::Int(num)),
        Err(_) => match token.parse::<f64>() {
            Ok(num) => return Atom::Number(Number::Float(num)),
            Err(_) => Atom::Symbol(token),
        },
    }
}
