use crate::{error_type::LoxError, interpreter::Interpreter, lox_callable::LoxCallable, token::LiteralType};

pub struct ClockNative;

impl LoxCallable for ClockNative {
    fn arity(&self) -> usize {
        0
    }
    
    fn call(&self, _interpreter: &mut Interpreter, _arguments: Vec<LiteralType>) 
        -> Result<LiteralType, LoxError> {
        use std::time::{SystemTime, UNIX_EPOCH};
        
        let duration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap();
        
        Ok(LiteralType::Number(duration.as_secs_f64()))
    }
}

impl std::fmt::Display for ClockNative {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<native fn>")
    }
}