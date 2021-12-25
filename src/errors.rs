use std::fmt;

#[derive(Debug, Clone)]
pub struct RuntimeError {
    pub err: String,
}

impl From<&str> for RuntimeError {
    fn from(message: &str) -> Self {
        RuntimeError { err: message.to_string() }
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
