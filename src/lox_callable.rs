use crate::{error_type::LoxError, interpreter::{self, Interpreter}, token::LiteralType};


pub trait LoxCallable {
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<LiteralType>) 
        -> Result<LiteralType, LoxError>;
    fn arity(&self) -> usize;
}