use std::{cell::RefCell, fmt::Display, hash::Hash, rc::Rc};
use crate::{lox_callable::LoxCallable, lox_class::LoxClass, lox_instance::LoxInstance, token_type};

pub enum LiteralType {
    Number(f64),
    String(String),
    Boolean(bool),
    None,
    Callable(Rc<dyn LoxCallable>),
    Instance(Rc<RefCell<LoxInstance>>),
    Class(LoxClass)
}

impl Eq for LiteralType {}

impl Hash for LiteralType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            LiteralType::Number(n) => {
                // Hash the bit representation of the float
                n.to_bits().hash(state);
            }
            LiteralType::String(s) => s.hash(state),
            LiteralType::Boolean(b) => b.hash(state),
            LiteralType::None => 0.hash(state),
            LiteralType::Callable(_) => {
                // Functions can't really be hashed meaningfully
                // You could hash a unique ID or just hash a constant
                "callable".hash(state);
            }
            LiteralType::Instance(_) => {
                "instance".hash(state);
            }
            LiteralType::Class(_) => {
                "class".hash(state);
            }
        }
    }
}

impl Clone for LiteralType {
    fn clone(&self) -> Self {
        match self {
            Self::None => Self::None,
            Self::Number(n) => Self::Number(*n),
            Self::String(s) => Self::String(s.clone()),
            Self::Boolean(b) => Self::Boolean(*b),
            Self::Callable(c) => Self::Callable(Rc::clone(c)), // Cheap clone!
            Self::Instance(i) => Self::Instance(i.clone()), // Cheap clone!
            Self::Class(c) => Self::Class(c.clone()), // Cheap clone!
        }
    }
}

impl PartialEq for LiteralType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::None, Self::None) => true,
            (Self::Number(a), Self::Number(b)) => a == b,
            (Self::String(a), Self::String(b)) => a == b,
            (Self::Boolean(a), Self::Boolean(b)) => a == b,
            (Self::Callable(_), Self::Callable(_)) => false, // Functions not comparable
            (Self::Instance(_), Self::Instance(_)) => false, // Classes not comparable
            (Self::Class(_), Self::Class(_)) => false, // Classes not comparable
            _ => false,
        }
    }
}

impl Display for LiteralType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LiteralType::Number(n) => write!(f, "{}", n),
            LiteralType::String(s) => write!(f, "{}", s),
            LiteralType::Boolean(b) => write!(f, "{}", b),
            LiteralType::None => write!(f, "None"),
            LiteralType::Callable(_) => write!(f, "<fn>"),
            LiteralType::Instance(i) => write!(f, "{}", i.borrow()),
            LiteralType::Class(c) => write!(f, "{}", c)
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Token{
    pub _type: token_type::TokenType,
    pub lexeme: String,
    pub literal: LiteralType,
    pub line: usize,
}

// impl Display for Token {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         Debug::fmt(self, f)
//     }
// }

pub fn literal_stringify(value: LiteralType) -> String {
    match value {
        LiteralType::None => return "nil".to_string(),
        LiteralType::String(s) => return s,
        LiteralType:: Boolean(b) => return format!("{b}"),
        LiteralType:: Number(n) => return format!("{n}"),
        LiteralType:: Callable(_c) => return format!("<fn>"),
        LiteralType:: Instance(i) => return format!("{}", i.borrow()),
        LiteralType:: Class(i) => return format!("{i}"),
    }
}

// impl Token {
//     pub fn to_string(&self) -> String {
//         return format!("{:?} {} {:?}", self._type, self.lexeme, self.literal)
//     }
// }