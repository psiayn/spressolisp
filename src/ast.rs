use std::fmt;

use crate::env::EnvType;

#[derive(Clone)]
pub enum Expr {
    Atom(Atom),
    List(Vec<Expr>),
    Func(fn(Vec<Expr>, &mut EnvType) -> Result<Expr, RuntimeError>),
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

impl std::ops::Add<Number> for Number {
    type Output = Number;
    fn add(self, rhs: Number) -> Self::Output {
        match rhs {
            Number::Float(num) => match self {
                Number::Float(lhs) => Number::Float(lhs + num),
                Number::Int(lhs) => Number::Float(lhs as f64 + num),
            },
            Number::Int(num) => match self {
                Number::Float(lhs) => Number::Float(lhs + num as f64),
                Number::Int(lhs) => Number::Int(lhs + num),
            },
        }
    }
}

impl std::ops::Mul<Number> for Number {
    type Output = Number;
    fn mul(self, rhs: Number) -> Self::Output {
        match rhs {
            Number::Float(num) => match self {
                Number::Float(lhs) => Number::Float(lhs * num),
                Number::Int(lhs) => Number::Float(lhs as f64 * num),
            },
            Number::Int(num) => match self {
                Number::Float(lhs) => Number::Float(lhs * num as f64),
                Number::Int(lhs) => Number::Int(lhs * num),
            },
        }
    }
}

impl std::ops::Sub<Number> for Number {
    type Output = Number;
    fn sub(self, rhs: Number) -> Self::Output {
        match rhs {
            Number::Float(num) => match self {
                Number::Float(lhs) => Number::Float(lhs - num),
                Number::Int(lhs) => Number::Float(lhs as f64 - num),
            },
            Number::Int(num) => match self {
                Number::Float(lhs) => Number::Float(lhs - num as f64),
                Number::Int(lhs) => Number::Int(lhs - num),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct RuntimeError {
    pub err: String,
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Runtime Error: {}", self.err)
    }
}

fn pretty_ast(ast: &Expr, level: usize, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    // do stuff
    fn type_name_of<T>(_: T) -> &'static str {
        std::any::type_name::<T>()
    }

    match ast {
        Expr::List(list) => {
            write!(f, "{}List\n", "\t".repeat(level)).unwrap();
            list.into_iter().map(|token| pretty_ast(token, level + 1, f)).collect()
        },
        Expr::Atom(token) => write!(f, "{}{}\n", "\t".repeat(level), token),
        Expr::Func(func) => write!(f, "{}{}\n", "\t".repeat(level), type_name_of(func)),
    }
}
