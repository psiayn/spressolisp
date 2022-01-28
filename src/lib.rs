pub mod ast;
pub mod env;
pub mod errors;
pub mod eval;

use std::collections::VecDeque;

use crate::ast::{Atom, Expr, Number};
use crate::env::Env;
use crate::errors::{RuntimeError, SpressoError, SyntaxError};
use crate::eval::execute;

pub fn evaluate_expression(input: String, env: &mut Env) -> Result<Expr, SpressoError> {
    let mut tokenized_input: VecDeque<String> = tokenize(input);
    let ast = parse(&mut tokenized_input)?;
    match ast {
        Expr::List(mut exprs) => execute(&mut exprs, env),
        _ => Err(SpressoError::Runtime(RuntimeError::from(format!(
            "Hmm I can't execute something that is not a list: {}",
            ast
        )))),
    }
}

fn tokenize(input: String) -> VecDeque<String> {
    let input: String = input.replace("(", " ( ").replace(")", " ) ");
    let res = input
        .split_whitespace()
        .map(|tok| tok.to_string())
        .collect();
    return res;
}

fn parse(tokens: &mut VecDeque<String>) -> Result<Expr, SyntaxError> {
    let token = match tokens.pop_front() {
        Some(token) => token,
        // no tokens (vec was empty)
        None => return Err(SyntaxError::from("Unexpected EOF".to_string())),
    };

    match token.as_str() {
        "(" => {
            // collect everything before ")"
            let mut ast: Vec<Expr> = Vec::new();
            while !tokens.is_empty() && tokens[0] != ")" {
                // recursively parse each of them
                let inner_ast = parse(tokens)?;
                ast.push(inner_ast);
            }

            // there should be a closing ")" after parsing everything inside
            if let None = tokens.pop_front() {
                return Err(SyntaxError::from("'(' not closed"));
            }

            return Ok(Expr::List(ast));
        }
        ")" => return Err(SyntaxError::from("Unexpected ')'")),
        _ => Ok(Expr::Atom(parse_atom(token)?)),
    }
}

fn parse_atom(token: String) -> Result<Atom, SyntaxError> {
    let first_char = match token.chars().next() {
        Some(char) => char,
        None => return Err(SyntaxError::from("Expected something, found nothing?")),
    };

    match first_char {
        '0'..='9' | '.' => {
            if let Ok(num) = token.parse::<i64>() {
                return Ok(Atom::Number(Number::Int(num)));
            }

            if let Ok(num) = token.parse::<f64>() {
                return Ok(Atom::Number(Number::Float(num)));
            }

            Err(SyntaxError::from("Symbols cannot start with a number"))
        },
        '"' => {
            let str_without_quotes = match token.strip_suffix("\"") {
                Some(s) => s,
                None => return Err(SyntaxError::from("String not closed")),
            };

            // we know it already start with " so belieb in unwrap
            let str_without_quotes = str_without_quotes.strip_prefix("\"").unwrap();

            Ok(Atom::String(str_without_quotes.to_string()))
        }
        _ => Ok(Atom::Symbol(token))
    }
}
