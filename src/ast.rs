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

fn pretty_ast(ast: Expr, level: usize) {
    match ast {
        Expr::List(list) => {
            println!("{}List", "\t".repeat(level));
            list.into_iter().for_each(|token| pretty_ast(token, level + 1));
        },
        Expr::Atom(token) => println!("{}{:?}", "\t".repeat(level) , token),
    };
}

pub fn print_ast(ast: Expr) {
    pretty_ast(ast, 0);
}
