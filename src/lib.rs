pub mod ast;
pub mod env;
pub mod errors;
pub mod eval;

use std::collections::VecDeque;
use std::iter::Peekable;
use std::rc::Rc;
use std::str::Chars;

use crate::ast::{Atom, Expr, Number};
use crate::env::Env;
use crate::errors::{RuntimeError, SpressoError, SyntaxError};
use crate::eval::execute;

pub fn evaluate_expression(input: String, env: &mut Env) -> Result<Expr, SpressoError> {
    let mut tokenized_input: VecDeque<Token> = tokenize(input);
    let ast = parse(&mut tokenized_input)?;
    match ast {
        Expr::List(mut exprs) => execute(&mut exprs, env),
        _ => Err(SpressoError::Runtime(RuntimeError::from(format!(
            "Hmm I can't execute something that is not a list: {}",
            ast
        )))),
    }
}

struct Token {
    text: String,
    line_num: usize,
    col_num_start: usize,
    col_num_end: usize,
    program_text: Rc<String>,
}

fn tokenize(input: String) -> VecDeque<Token> {
    let program_text = Rc::new(input);
    let mut tokens = VecDeque::new();

    let mut line_num = 1;
    let mut col_num = 1;

    let char_processor = |c: char,
                          chars: &mut Peekable<Chars>,
                          line_num: &mut usize,
                          col_num: &mut usize|
     -> Option<String> {
        let mut new_token = String::from(c);
        match c {
            '(' => Some(new_token),
            ')' => Some(new_token),
            '0'..='9' | '.' => {
                loop {
                    // TODO: ensure only one "." in number
                    match chars.peek() {
                        Some('0'..='9') | Some('.') => new_token.push(chars.next().unwrap()),
                        Some(_) => break,
                        None => break,
                    }
                }

                Some(new_token)
            }
            ' ' => {
                *col_num += 1;
                None
            }
            '\n' => {
                *line_num += 1;
                *col_num = 1;
                None
            }
            '"' => {
                loop {
                    match chars.peek() {
                        Some('"') => {
                            new_token.push(chars.next().unwrap());
                            break;
                        }
                        Some(_) => new_token.push(chars.next().unwrap()),
                        None => break,
                    }
                }

                Some(new_token)
            }
            _ => {
                loop {
                    match chars.peek() {
                        Some(' ' | '\n' | '(' | ')') => break,
                        Some(_) => new_token.push(chars.next().unwrap()),
                        None => break,
                    }
                }

                Some(new_token)
            }
        }
    };

    let program_text = Rc::clone(&program_text);
    let mut chars = program_text.chars().peekable();

    while let Some(c) = chars.next() {
        let col_num_start = col_num;
        if let Some(new_token) = char_processor(c, &mut chars, &mut line_num, &mut col_num) {
            let col_num_end = col_num + new_token.len();
            tokens.push_back(Token {
                text: new_token,
                line_num,
                col_num_start,
                col_num_end,
                program_text: Rc::clone(&program_text),
            })
        }
    }

    tokens
}

fn parse(tokens: &mut VecDeque<Token>) -> Result<Expr, SyntaxError> {
    let token = match tokens.pop_front() {
        Some(token) => token,
        // no tokens (vec was empty)
        None => return Err(SyntaxError::from("Unexpected EOF".to_string())),
    };

    match token.text.as_str() {
        "(" => {
            // collect everything before ")"
            let mut ast: Vec<Expr> = Vec::new();
            while !tokens.is_empty() && tokens[0].text != ")" {
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
        _ => Ok(Expr::Atom(parse_atom(token.text)?)),
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
        }
        '"' => {
            let str_without_quotes = match token.strip_suffix("\"") {
                Some(s) => s,
                None => return Err(SyntaxError::from("String not closed")),
            };

            // we know it already start with " so belieb in unwrap
            let str_without_quotes = str_without_quotes.strip_prefix("\"").unwrap();

            Ok(Atom::String(str_without_quotes.to_string()))
        }
        _ => Ok(Atom::Symbol(token)),
    }
}
