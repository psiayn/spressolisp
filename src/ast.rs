use std::fmt;

#[derive(Debug, Clone)]
pub enum Expr {
    Atom(Atom),
    List(Vec<Expr>),
}

#[derive(Debug, Clone)]
pub enum Atom {
    Symbol(String),
    Number(Number),
}

#[derive(Debug, Clone)]
pub enum Number {
    Int(i64),
    Float(f64),
}

#[derive(Debug, Clone)]
pub struct SyntaxError {
    pub err: String,
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Syntax Error: {}", self.err)
    }
}

pub fn print_ast(ast: Expr) {
    match ast {
        Expr::List(list) => list.into_iter().for_each(|token| print_ast(token)),
        Expr::Atom(token) => println!("{:?}", token),
    };
}
