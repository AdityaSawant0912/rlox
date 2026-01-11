use std::{cell::RefCell, env, rc::Rc};

use crate::{environment::Environment, error_type::LoxError, interpreter::Interpreter, lox_callable::LoxCallable, lox_instance::LoxInstance, stmt::Stmt, token::LiteralType};

#[derive(Clone)]
pub struct LoxFunction {
    pub declaration: Stmt,
    pub closure: Rc<RefCell<Environment>>
}

impl LoxFunction {
    pub fn new(declaration:Stmt, closure: Rc<RefCell<Environment>>) -> Self {
        Self {
            declaration,
            closure
        }
    }

    pub fn bind(&self, instance: LoxInstance) -> LiteralType {
        let environment = Rc::new(RefCell::new(Environment::new(Some(Rc::clone(
            &self.closure,
        )))));
        environment.borrow_mut().define("this", &LiteralType::Instance(instance));

        return LiteralType::Callable(Rc::new(LoxFunction::new(self.declaration.clone(), environment)))
    }

}

impl LoxCallable for LoxFunction {
    fn call(&self, interpreter: &mut Interpreter, arguments:Vec<LiteralType>) -> Result<LiteralType, LoxError> {
        let environment = Rc::new(RefCell::new(Environment::new(Some(Rc::clone(
            &self.closure,
        )))));

        match &self.declaration {
            Stmt::Function { name: _, params, body } => {
                for (index, param) in params.iter().enumerate() {
                    environment.borrow_mut().define(&param.lexeme, arguments.get(index).unwrap());
                }
                match interpreter.execute_block(body.clone(), environment) {
                    Ok(()) => return Ok(LiteralType::None),
                    Err(e) => {
                        match e {
                            LoxError::Return(value) => return Ok(value),
                            _ => return Ok(LiteralType::None)
                        }
                    }
                }
            },
            _ => return Ok(LiteralType::None)
        }
    }

    fn arity(&self) -> usize {
        match &self.declaration {
            Stmt::Function { name: _, params, body: _ } => {
                return params.len()
            },
            _ => return 0
        }
    }
}

impl std::fmt::Display for LoxFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.declaration {
            Stmt::Function { name, params: _, body: _ } => {
                return write!(f, "<fn {}>", name.lexeme)
            },
            _ => return write!(f, "<fn unknown>")
        }
        
    }
}