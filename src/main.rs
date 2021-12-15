mod ast;
mod env;
mod eval;

use std::io::{self, Write};

use crate::{ast::{Expr, Atom, Number, SyntaxError}, env::standard_env};
use crate::eval::execute;

fn main() {
    loop {
        let mut inp = String::new();
        print!("spresso> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut inp).unwrap();
        let input = inp.trim().to_string();
        if input == ".quit" {
            break;
        }
        let mut tokenized_input: Vec<String> = tokenize(input);
        let mut global_env = standard_env();
        match read_from_tokens(&mut tokenized_input) {
            Ok(ast) => {
                println!("{}", ast);
                match ast {
                    Expr::List(mut exprs) => match execute(&mut exprs, &mut global_env) {
                        Ok(e) => println!("{}", e),
                        Err(e) => println!("{}", e),
                    },
                    _ => println!("{}", SyntaxError{err: "Hmm I can't execute something that is not a list".to_string()}),
                }
            },
            Err(e) => println!("{}", e),
        };
    }
    println!("goodbye!");
}

fn tokenize(input: String) -> Vec<String> {
    let input: String = input.replace("(", " ( ").replace(")", " ) ");
    let res = input
        .split_whitespace()
        .map(|tok| tok.to_string())
        .collect();
    return res;
}

fn read_from_tokens(tokens: &mut Vec<String>) -> Result<Expr, SyntaxError> {
    if tokens.len() == 0 {
        return Err(SyntaxError { err: "Unexpected EOF".to_string() });
    }
    let token = tokens.remove(0);
    match token.as_str() {
        "(" => {
           if tokens.len() == 0 {
               return Err(SyntaxError { err: "'(' not closed".to_string() });
           }
           let mut ast: Vec<Expr> = Vec::new();
           while tokens[0] != ")" {
               let inner_ast = match read_from_tokens(tokens) {
                   Ok(res) => res,
                   Err(e) => return Err(e),
               };
               ast.push(inner_ast);
               if tokens.len() == 0 {
                   return Err(SyntaxError { err: "'(' not closed".to_string() });
               }
           }
           tokens.remove(0);
           return Ok(Expr::List(ast));
        },
        ")" => return Err(SyntaxError { err: "Unexpected ')'".to_string() }),
        _ => Ok(Expr::Atom(atom(token))),
    }
}

fn atom(token: String) -> Atom {
    match token.parse::<i64>() {
        Ok(num) => return Atom::Number(Number::Int(num)),
        Err(_) => match token.parse::<f64>() {
            Ok(num) => return Atom::Number(Number::Float(num)),
            Err(_) => Atom::Symbol(token),
        },
    }
}
