use std::io::{self, Write};
use std::fmt;

#[derive(Debug, Clone)]
enum Expr {
    Atom(Atom),
    List(List),
}

#[derive(Debug, Clone)]
enum Atom {
    Symbol(String),
    Number(Number),
}

#[derive(Debug, Clone)]
enum Number {
    Int(i64),
    Float(f64),
}

#[derive(Debug, Clone)]
enum List {
    Symbol(Vec<String>),
    Number(Vec<Number>),
    Expr(Vec<Expr>),
}

#[derive(Debug, Clone)]
struct SyntaxError {
    err: String,
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Syntax Error: {}", self.err)
    }
}

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
        let repr = read_from_tokens(&mut tokenized_input);
        println!("{:?}", repr);
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
           let mut ast: Vec<Expr> = Vec::new();
           while tokens[0] != ")" {
               let inner_ast = match read_from_tokens(tokens) {
                   Ok(res) => res,
                   Err(e) => return Err(e),
               };
               ast.push(inner_ast);
           }
           tokens.remove(0);
           return Ok(Expr::List(List::Expr(ast)));
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
