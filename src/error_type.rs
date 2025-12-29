use std::fmt::Display;

#[derive(Debug)]
pub enum LoxError {
    // FileNotFound,
    RuntimeError,
    ParseError
}


impl Display for LoxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // {:?} tells Rust to use the Debug implementation
        write!(f, "{:?}", self)
    }
}
