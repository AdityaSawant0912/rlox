use crate::{error_type::LoxError, interpreter::interpret, stmt::Stmt, token::literal_stringify};

pub fn execute(stmt: Stmt) -> Result<(), LoxError> {
    match stmt {
        Stmt::Expression { expression } => {
            interpret(expression)?;
            return Ok(());
        }
        Stmt::Print { expression } => match interpret(expression) {
            Ok(value) => {
                println!("{}", literal_stringify(value));
                return Ok(());
            }
            Err(e) => {
                return Err(e);
            }
        },
        Stmt::Var { name, initializer } => {
            Ok(())
        }
    }
}
