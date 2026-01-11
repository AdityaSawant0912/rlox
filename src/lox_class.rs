use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{error_type::LoxError, interpreter::Interpreter, lox_callable::LoxCallable, lox_function::LoxFunction, lox_instance::LoxInstance, token::LiteralType};

#[derive(Clone)]
pub struct LoxClass {
    pub name: String,
    methods: HashMap<String, LoxFunction>
}

impl LoxClass {
    pub fn new(name: &str, methods: HashMap<String, LoxFunction>) -> Self {
        return Self {
            name: name.to_string(),
            methods
        }
    }

    pub fn find_method(&self, name: &str) -> Option<&LoxFunction> {
        return self.methods.get(name);
    }

}

impl LoxCallable for LoxClass {
    fn call(&self, _: &mut Interpreter, _: Vec<LiteralType>) 
        -> Result<LiteralType, LoxError> {
        let instance = Rc::new(RefCell::new(LoxInstance::new(RefCell::new(Rc::new(self.clone())))));
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