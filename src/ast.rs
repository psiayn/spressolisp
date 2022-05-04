use std::fmt;

use crate::env::Env;
use crate::errors::{NumericError, SpressoError};
use crate::{Token, TokenGiver, TokenHoarder};

pub type FuncType = fn(Vec<Expr>, &mut Env) -> Result<Expr, SpressoError>;

#[derive(Clone, Debug)]
pub struct Expr {
    pub kind: ExprKind,
    tokens: Option<Vec<Token>>,
}

impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
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

impl TokenHoarder for Expr {
    fn with_token(mut self, token: Token) -> Self {
        if let Some(tokens) = &mut self.tokens {
            tokens.push(token);
        } else {
            self.tokens = Some(vec![token]);
        }
        self
    }
}

impl TokenGiver for Expr {
    fn get_tokens(&self) -> Option<Vec<Token>> {
        match &self.kind {
            ExprKind::List(exprs) => exprs.get_tokens(),
            _ => self.tokens.clone(),
        }
    }
}

impl TokenGiver for Vec<Expr> {
    fn get_tokens(&self) -> Option<Vec<Token>> {
        let mut tokens = Vec::new();

        for expr in self {
            if let Some(expr_tokens) = expr.get_tokens() {
                tokens.extend(expr_tokens);
            }
        }

        Some(tokens)
    }
}

#[derive(Clone)]
pub enum ExprKind {
    Atom(Atom),
    List(Vec<Expr>),
    Func(FuncType),
    Lambda(Lambda),
}

impl fmt::Debug for ExprKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Atom(arg0) => f.debug_tuple("Atom").field(arg0).finish(),
            Self::List(arg0) => f.debug_tuple("List").field(arg0).finish(),
            Self::Func(_) => f.debug_tuple("Func").finish(),
            Self::Lambda(arg0) => f.debug_tuple("Lambda").field(arg0).finish(),
        }
    }
}

impl PartialEq for ExprKind {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ExprKind::Atom(l0), ExprKind::Atom(r0)) => l0 == r0,
            (ExprKind::List(l0), ExprKind::List(r0)) => l0 == r0,
            (ExprKind::Func(l0), ExprKind::Func(r0)) => (*l0 as usize) == (*r0 as usize),
            (ExprKind::Lambda(l0), ExprKind::Lambda(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        print_expr(self, 0, f)
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum Atom {
    Symbol(String),
    Number(Number),
    Bool(bool),
    String(String),
    Unit,
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Atom::Symbol(string) => write!(f, "{}", string),
            Atom::Number(num) => write!(f, "{}", num),
            Atom::Bool(boolean) => write!(f, "{}", boolean),
            Atom::String(string) => write!(f, "\"{}\"", string),
            Atom::Unit => write!(f, "()"),
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
                if num == 0 {
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

#[derive(Clone, Debug)]
pub struct Lambda {
    pub params: Vec<String>,
    pub body: Vec<Expr>,
    param_tokens: Vec<Token>,
}

impl PartialEq for Lambda {
    fn eq(&self, other: &Self) -> bool {
        self.params == other.params && self.body == other.body
    }
}

impl Lambda {
    pub fn new(params: Vec<String>, body: Vec<Expr>) -> Self {
        Self {
            params,
            body,
            param_tokens: Vec::new(),
        }
    }
}

/// Note: Lambda itself should only store the tokens of its parameters
/// Tokens of the body are stored inside the body itself.
impl TokenHoarder for Lambda {
    fn with_token(mut self, token: Token) -> Self {
        self.param_tokens.push(token);
        self
    }
}

/// Note: Lambda itself only stores the tokens of its parameters.
/// Tokens of the body can be retrieved by `lambda.body.get_tokens()`.
impl TokenGiver for Lambda {
    fn get_tokens(&self) -> Option<Vec<Token>> {
        Some(self.param_tokens.clone())
    }
}

impl fmt::Display for Lambda {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "λ: [{}] -> ...", self.params.join(", "))
    }
}
#[allow(dead_code)]
fn pretty_ast(ast: &Expr, level: usize, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match &ast.kind {
        ExprKind::List(list) => {
            writeln!(f, "{}List", "\t".repeat(level)).unwrap();
            list.iter().try_for_each(|token| pretty_ast(token, level + 1, f))
        }
        ExprKind::Atom(token) => writeln!(f, "{}{}", "\t".repeat(level), token),
        ExprKind::Func(..) => writeln!(f, "{}built-in function", "\t".repeat(level)),
        ExprKind::Lambda(lambda) => writeln!(f, "{}{}", "\t".repeat(level), lambda),
    }
}

fn print_expr(ast: &Expr, level: usize, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match &ast.kind {
        ExprKind::List(list) => {
            write!(f, "[ ").unwrap();
            let hmm = list.iter().try_for_each(|token| print_expr(token, level + 1, f));
            write!(f, "] ").unwrap();
            hmm
        }
        ExprKind::Atom(token) => write!(f, "{} ", token),
        ExprKind::Func(..) => write!(f, "built-in function "),
        ExprKind::Lambda(lambda) => write!(f, "{} ", lambda),
    }
}
