use std::fmt;

use colored::Colorize;

use crate::{display_and_mark, Token, TokenGiver, TokenHoarder};

#[derive(Clone)]
pub struct SpressoError {
    pub detail: SpressoErrorType,
    tokens: Option<Vec<Token>>,
}

#[derive(Clone)]
pub enum SpressoErrorType {
    Runtime(RuntimeError),
    Syntax(SyntaxError),
    Numeric(NumericError),
}

impl SpressoError {
    pub fn new(detail: SpressoErrorType) -> Self {
        SpressoError {
            detail,
            tokens: None,
        }
    }

    /// Possibly unstable API.
    pub fn text(&self) -> &str {
        // TODO: consider using a trait?
        match &self.detail {
            SpressoErrorType::Runtime(err) => err.err.as_str(),
            SpressoErrorType::Syntax(err) => err.err.as_str(),
            SpressoErrorType::Numeric(err) => err.err.as_str(),
        }
    }

    /// Possibly unstable API.
    pub fn name(&self) -> &str {
        // TODO: consider using a trait?
        match &self.detail {
            SpressoErrorType::Runtime(..) => "Runtime Error",
            SpressoErrorType::Syntax(..) => "Syntax Error",
            SpressoErrorType::Numeric(..) => "Numeric Error",
        }
    }
}

impl TokenHoarder for SpressoError {
    fn with_token(mut self, token: Token) -> Self {
        match &mut self.tokens {
            Some(tokens) => tokens.push(token),
            None => self.tokens = Some(vec![token]),
        }
        self
    }
}

impl TokenGiver for SpressoError {
    fn get_tokens(&self) -> Option<Vec<Token>> {
        self.tokens.clone()
    }
}

impl fmt::Display for SpressoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}: {}", self.name().red(), self.text())?;

        if let Some(tokens) = &self.tokens {
            display_and_mark(f, tokens)?;
        }

        Ok(())
    }
}

impl From<RuntimeError> for SpressoError {
    fn from(err: RuntimeError) -> Self {
        SpressoError::new(SpressoErrorType::Runtime(err))
    }
}

impl From<SyntaxError> for SpressoError {
    fn from(err: SyntaxError) -> Self {
        SpressoError::new(SpressoErrorType::Syntax(err))
    }
}

impl From<NumericError> for SpressoError {
    fn from(err: NumericError) -> Self {
        SpressoError::new(SpressoErrorType::Numeric(err))
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
}

impl From<&str> for SyntaxError {
    fn from(message: &str) -> Self {
        SyntaxError {
            err: message.to_string(),
        }
    }
}

impl From<String> for SyntaxError {
    fn from(message: String) -> Self {
        SyntaxError { err: message }
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
