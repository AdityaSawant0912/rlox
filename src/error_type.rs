use std::fmt::Display;

use crate::token::LiteralType;

pub enum LoxError {
    // FileNotFound,
    RuntimeError,
    ParseError,
    Return(LiteralType)
}


impl Display for LoxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoxError::RuntimeError => write!(f, "Runtime error occurred"),
            LoxError::ParseError => write!(f, "Parse error occurred"),
            _ => write!(f, "Some error occurred")
            // Add other variants as needed
        }
    }
}