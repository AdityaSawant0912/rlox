use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::{
    error,
    error_type::LoxError,
    token::{LiteralType, Token},
};
#[derive(Clone)]
pub struct Environment {
    values: HashMap<String, LiteralType>,
    enclosing: Option<Rc<RefCell<Environment>>>,
}

// impl Default for Environment {
//     fn default() -> Self {
//         Self {
//             values: HashMap::new(),
//         }
//     }
// }

impl Environment {
    pub fn new(enclosing: Option<Rc<RefCell<Environment>>>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing,
        }
    }

    pub fn define(&mut self, name: &str, value: &LiteralType) {
        self.values.insert(name.to_string(), value.clone());
    }

    pub fn get(&mut self, name: &Token) -> Result<LiteralType, LoxError> {
        if let Some(literal) = self.values.get(&name.lexeme) {
            return Ok(literal.clone());
        }
        if let Some(env) = &self.enclosing {
            return env.borrow_mut().get(name);
        }
        error::token_error(
            name.clone(),
            &format!("Undefined variable '{}'.", name.lexeme),
        );
        Err(LoxError::RuntimeError)
    }

    pub fn assign(&mut self, name: &Token, value: &LiteralType) -> Result<(), LoxError> {
        if self.values.contains_key(&name.lexeme.to_string()) {
            self.values.insert(name.lexeme.to_string(), value.clone());
            return Ok(());
        }
        if let Some(env) = &mut self.enclosing {
            return env.borrow_mut().assign(name, value);
        }
        error::token_error(
            name.clone(),
            &format!("Undefined variable '{}'.", name.lexeme),
        );
        Err(LoxError::RuntimeError)
    }

    pub fn _dump(&self, depth: Option<usize>) {
        let d = depth.unwrap_or(0);
        println!("Env depth = {}", d);
        for (key, value) in &self.values {
            println!("{}: {}", key, value);
        }
        if let Some(env) = &self.enclosing {
            env.borrow_mut()._dump(Some(d + 1));
        }
    }
}
