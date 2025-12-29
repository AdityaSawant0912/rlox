use crate::{error_type::LoxError, interpreter::interpret, stmt::Stmt, token::literal_stringify};

pub fn execute(stmt: Stmt) -> Result<(), LoxError> {
    match stmt {
        Stmt::Print { expression } => {
            interpret(expression);
            return Ok(());
        }
        Stmt::Expression { expression } => match interpret(expression) {
            Ok(value) => {
                println!("{}", literal_stringify(value));
                return Ok(());
            }
            Err(e) => {
                return Err(e);
            }
        },
    }
}
