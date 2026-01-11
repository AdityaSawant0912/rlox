use std::{cell::RefCell, rc::Rc};

use crate::{error_type::LoxError, interpreter::Interpreter, lox_callable::LoxCallable, lox_instance::LoxInstance, token::LiteralType};

#[derive(Clone)]
pub struct LoxClass {
    pub name: String
}

impl LoxClass {
    pub fn new(name: &str) -> Self {
        return Self {
            name: name.to_string()
        }
    }
}

impl LoxCallable for LoxClass {
    fn call(&self, _: &mut Interpreter, _: Vec<LiteralType>) 
        -> Result<LiteralType, LoxError> {
        let instance = LoxInstance::new(RefCell::new(Rc::new(self.clone())));
        return Ok(LiteralType::Instance(instance))
    }

    fn arity(&self) -> usize {
        return 0
    }
}


impl std::fmt::Display for LoxClass {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "<class {}>", self.name)
        
    }
}