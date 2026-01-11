use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    error::token_error,
    error_type::LoxError,
    lox_class::LoxClass,
    token::{LiteralType, Token},
};

#[derive(Clone)]
pub struct LoxInstance {
    klass: RefCell<Rc<LoxClass>>,
    fields: HashMap<String, LiteralType>,
}

impl LoxInstance {
    pub fn new(klass: RefCell<Rc<LoxClass>>) -> Self {
        return Self {
            klass,
            fields: HashMap::new(),
        };
    }

    pub fn get(instance: Rc<RefCell<LoxInstance>>, name: Token) -> Result<LiteralType, LoxError> {
        let borrowed = instance.borrow();

        if borrowed.fields.contains_key(&name.lexeme) {
            return Ok(borrowed.fields.get(&name.lexeme).unwrap().clone());
        }

        let klass = borrowed.klass.borrow();
        let method = klass.find_method(&name.lexeme);

        if let Some(method) = method {
            // drop(borrowed); // Release borrow before calling bind
            return Ok(method.bind(Rc::clone(&instance)));
        }

        token_error(
            name.clone(),
            &format!("Undefined property '{}'.", name.lexeme),
        );
        Err(LoxError::RuntimeError)
    }

    pub fn set(&mut self, name: Token, value: LiteralType) {
        self.fields.insert(name.lexeme, value);
    }
}

impl std::fmt::Display for LoxInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "<instance of {}>", self.klass.borrow_mut().name);
    }
}
