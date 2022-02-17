use std::fmt;

use crate::env::Env;
use crate::errors::{NumericError, SpressoError};
use crate::Token;

pub type FuncType = fn(Vec<Expr>, &mut Env) -> Result<Expr, SpressoError>;

#[derive(Clone)]
pub struct Expr {
    pub kind: ExprKind,
    tokens: Option<Vec<Token>>,
}

impl Expr {
    pub fn new(kind: ExprKind) -> Self {
        Self { kind, tokens: None }
    }
}

impl From<ExprKind> for Expr {
    fn from(kind: ExprKind) -> Self {
        Expr::new(kind)
    }
}

#[derive(Clone)]
pub enum ExprKind {
    Atom(Atom),
    List(Vec<Expr>),
    Func(FuncType),
    Lambda(Lambda),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        print_expr(self, 0, f)
    }
}

#[derive(Debug, Clone)]
pub enum Atom {
    Symbol(String),
    Number(Number),
    Bool(bool),
    String(String),
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Atom::Symbol(string) => write!(f, "{}", string),
            Atom::Number(num) => write!(f, "{}", num),
            Atom::Bool(boolean) => write!(f, "{}", boolean),
            Atom::String(string) => write!(f, "\"{}\"", string),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
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
    type Output = Result<Number, SpressoError>;
    fn add(self, rhs: Number) -> Self::Output {
        match rhs {
            Number::Float(num) => match self {
                Number::Float(lhs) => Ok(Number::Float(lhs + num)),
                Number::Int(lhs) => Ok(Number::Float(lhs as f64 + num)),
            },
            Number::Int(num) => match self {
                Number::Float(lhs) => Ok(Number::Float(lhs + num as f64)),
                Number::Int(lhs) => Ok(Number::Int(lhs + num)),
            },
        }
    }
}

impl std::ops::Mul<Number> for Number {
    type Output = Result<Number, SpressoError>;
    fn mul(self, rhs: Number) -> Self::Output {
        match rhs {
            Number::Float(num) => match self {
                Number::Float(lhs) => Ok(Number::Float(lhs * num)),
                Number::Int(lhs) => Ok(Number::Float(lhs as f64 * num)),
            },
            Number::Int(num) => match self {
                Number::Float(lhs) => Ok(Number::Float(lhs * num as f64)),
                Number::Int(lhs) => Ok(Number::Int(lhs * num)),
            },
        }
    }
}

impl std::ops::Sub<Number> for Number {
    type Output = Result<Number, SpressoError>;
    fn sub(self, rhs: Number) -> Self::Output {
        match rhs {
            Number::Float(num) => match self {
                Number::Float(lhs) => Ok(Number::Float(lhs - num)),
                Number::Int(lhs) => Ok(Number::Float(lhs as f64 - num)),
            },
            Number::Int(num) => match self {
                Number::Float(lhs) => Ok(Number::Float(lhs - num as f64)),
                Number::Int(lhs) => Ok(Number::Int(lhs - num)),
            },
        }
    }
}

impl std::ops::Div<Number> for Number {
    type Output = Result<Number, SpressoError>;
    fn div(self, rhs: Number) -> Self::Output {
        match rhs {
            Number::Float(num) => {
                if num == 0.0 || num == -0.0 {
                    return Err(NumericError {
                        err: "Division By Zero".to_string(),
                    }
                    .into());
                }
                match self {
                    Number::Float(lhs) => Ok(Number::Float(lhs / num)),
                    Number::Int(lhs) => Ok(Number::Float(lhs as f64 / num)),
                }
            }
            Number::Int(num) => {
                if num == 0 || num == -0 {
                    return Err(NumericError {
                        err: "Division By Zero".to_string(),
                    }
                    .into());
                }
                match self {
                    Number::Float(lhs) => Ok(Number::Float(lhs / num as f64)),
                    Number::Int(lhs) => Ok(Number::Int(lhs / num)),
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct Lambda {
    pub params: Vec<String>,
    pub body: Vec<Expr>,
}

impl fmt::Display for Lambda {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "λ: [{}] -> ...", self.params.join(", "))
    }
}

fn pretty_ast(ast: &Expr, level: usize, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match &ast.kind {
        ExprKind::List(list) => {
            write!(f, "{}List\n", "\t".repeat(level)).unwrap();
            list.into_iter()
                .map(|token| pretty_ast(&token, level + 1, f))
                .collect()
        }
        ExprKind::Atom(token) => write!(f, "{}{}\n", "\t".repeat(level), token),
        ExprKind::Func(..) => write!(f, "{}{}\n", "\t".repeat(level), "built-in function"),
        ExprKind::Lambda(lambda) => write!(f, "{}{}\n", "\t".repeat(level), lambda),
    }
}

fn print_expr(ast: &Expr, level: usize, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match &ast.kind {
        ExprKind::List(list) => {
            write!(f, "[ ").unwrap();
            let hmm = list
                .into_iter()
                .map(|token| print_expr(&token, level + 1, f))
                .collect::<fmt::Result>();
            write!(f, "] ").unwrap();
            hmm
        }
        ExprKind::Atom(token) => write!(f, "{} ", token),
        ExprKind::Func(..) => write!(f, "{} ", "built-in function"),
        ExprKind::Lambda(lambda) => write!(f, "{} ", lambda),
    }
}
