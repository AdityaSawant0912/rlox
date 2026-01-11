use std::{cell::RefCell, rc::Rc};

use crate::lox_class::LoxClass;


#[derive(Clone)]
pub struct LoxInstance {
    klass: RefCell<Rc<LoxClass>>
}

impl LoxInstance {
    pub fn new(klass: RefCell<Rc<LoxClass>>) -> Self {
        return Self {
            klass
        }
    }
}



impl std::fmt::Display for LoxInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "<instance of {}>", self.klass.borrow_mut().name)
        
    }
}