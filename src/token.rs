use std::{fmt::{Debug, Display}};
use crate::token_type;

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralType {
    Number(f64),
    String(String),
    Boolean(bool),
    None,
}

impl Display for LiteralType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LiteralType::Number(n) => write!(f, "{}", n),
            LiteralType::String(s) => write!(f, "{}", s),
            LiteralType::Boolean(b) => write!(f, "{}", b),
            LiteralType::None => write!(f, "None")
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token{
    pub _type: token_type::TokenType,
    pub lexeme: String,
    pub literal: LiteralType,
    pub line: usize,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

pub fn literal_stringify(value: LiteralType) -> String {
    match value {
        LiteralType::None => return "nil".to_string(),
        LiteralType::String(s) => return s,
        LiteralType:: Boolean(b) => return format!("{b}"),
        LiteralType:: Number(n) => return format!("{n}")
    }
}

// impl Token {
//     pub fn to_string(&self) -> String {
//         return format!("{:?} {} {:?}", self._type, self.lexeme, self.literal)
//     }
// }