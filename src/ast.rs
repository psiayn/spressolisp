use std::fmt;

#[derive(Debug, Clone)]
pub enum Expr {
    Atom(Atom),
    List(Vec<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        pretty_ast(self, 0, f)
    }
}

#[derive(Debug, Clone)]
pub enum Atom {
    Symbol(String),
    Number(Number),
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Atom::Symbol(string) => write!(f, "{}", string),
            Atom::Number(num) => write!(f, "{}", num),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Number {
    Int(i64),
    Float(f64),
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Number::Int(num) => write!(f, "{}", num),
            Number::Float(num) => write!(f, "{}", num),
        }
    }
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

fn pretty_ast(ast: &Expr, level: usize, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match ast {
        Expr::List(list) => {
            write!(f, "{}List\n", "\t".repeat(level)).unwrap();
            list.into_iter().map(|token| pretty_ast(token, level + 1, f)).collect()
        },
        Expr::Atom(token) => write!(f, "{}{}\n", "\t".repeat(level) , token),
    }
}
