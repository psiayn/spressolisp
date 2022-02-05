use std::fmt;

use colored::Colorize;

use crate::{display_and_mark, Token};

#[derive(Clone)]
pub enum SpressoError {
    Runtime(RuntimeError),
    Syntax(SyntaxError),
    Numeric(NumericError),
}

impl fmt::Display for SpressoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SpressoError::Runtime(err) => write!(f, "{}", err),
            SpressoError::Syntax(err) => write!(f, "{}", err),
            SpressoError::Numeric(err) => write!(f, "{}", err),
        }
    }
}

impl From<RuntimeError> for SpressoError {
    fn from(err: RuntimeError) -> Self {
        SpressoError::Runtime(err)
    }
}

impl From<SyntaxError> for SpressoError {
    fn from(err: SyntaxError) -> Self {
        SpressoError::Syntax(err)
    }
}

impl From<NumericError> for SpressoError {
    fn from(err: NumericError) -> Self {
        SpressoError::Numeric(err)
    }
}

#[derive(Debug, Clone)]
pub struct RuntimeError {
    pub err: String,
}

impl From<&str> for RuntimeError {
    fn from(message: &str) -> Self {
        RuntimeError {
            err: message.to_string(),
        }
    }
}

impl From<String> for RuntimeError {
    fn from(message: String) -> Self {
        RuntimeError { err: message }
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Runtime Error: {}", self.err)
    }
}

#[derive(Clone)]
pub struct SyntaxError {
    pub err: String,
    tokens: Option<Vec<Token>>,
}

impl From<&str> for SyntaxError {
    fn from(message: &str) -> Self {
        SyntaxError {
            err: message.to_string(),
            tokens: None,
        }
    }
}

impl From<String> for SyntaxError {
    fn from(message: String) -> Self {
        SyntaxError {
            err: message,
            tokens: None,
        }
    }
}

impl SyntaxError {
    pub fn with_tokens(mut self, tokens: Vec<Token>) -> Self {
        self.tokens = Some(tokens);
        self
    }
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}\n", "Syntax Error".red(), self.err)?;

        if let Some(tokens) = &self.tokens {
            display_and_mark(f, tokens)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct NumericError {
    pub err: String,
}

impl From<&str> for NumericError {
    fn from(message: &str) -> Self {
        NumericError {
            err: message.to_string(),
        }
    }
}

impl From<String> for NumericError {
    fn from(message: String) -> Self {
        NumericError { err: message }
    }
}

impl fmt::Display for NumericError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Numeric Error: {}", self.err)
    }
}
