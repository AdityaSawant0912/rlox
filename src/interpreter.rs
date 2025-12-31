use crate::{
    environment::{self, Environment},
    error::token_error,
    error_type::LoxError,
    expr::Expr,
    stmt::Stmt,
    token::{LiteralType, Token, literal_stringify},
    token_type::TokenType,
};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Interpreter {
    pub environment: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Rc::new(RefCell::new(Environment::new(None))),
        }
    }

    fn evaluate(&mut self, expression: Expr) -> Result<LiteralType, LoxError> {
        self.interpret(expression)
    }

    fn check_number_operand(&self, operator: Token, operand: &LiteralType) -> Result<(), LoxError> {
        if let LiteralType::Number(_x) = operand {
            return Ok(());
        }
        token_error(operator, "Operand must be a number.");
        Err(LoxError::RuntimeError)
    }

    fn check_number_operands(
        &self,
        operator: Token,
        left: &LiteralType,
        right: &LiteralType,
    ) -> Result<(), LoxError> {
        if let LiteralType::Number(_x) = left
            && let LiteralType::Number(_y) = right
        {
            return Ok(());
        }
        token_error(operator, "Operands must be a number.");
        Err(LoxError::RuntimeError)
    }

    fn is_truthy(&self, expr: &LiteralType) -> LiteralType {
        match expr {
            LiteralType::None => return LiteralType::Boolean(false),
            LiteralType::Boolean(x) => return LiteralType::Boolean(*x),
            _ => return LiteralType::Boolean(true),
        }
    }

    fn not(&self, expr: LiteralType) -> LiteralType {
        match expr {
            LiteralType::Boolean(x) => return LiteralType::Boolean(!x),
            _ => return LiteralType::Boolean(false), // This case should not hit
        }
    }

    pub fn interpret(&mut self, expr: Expr) -> Result<LiteralType, LoxError> {
        match expr {
            Expr::Assign { name, value } => {
                let evaluated_value: LiteralType = self.evaluate(*value)?;
                self.environment
                    .borrow_mut()
                    .assign(&name, &evaluated_value)?;
                return Ok(evaluated_value);
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let evaluated_left: LiteralType = self.evaluate(*left)?;
                let evaluated_right: LiteralType = self.evaluate(*right)?;
                match operator._type {
                    TokenType::Greater => {
                        match self.check_number_operands(
                            operator,
                            &evaluated_left,
                            &evaluated_right,
                        ) {
                            Ok(()) => {
                                if let LiteralType::Number(l) = evaluated_left
                                    && let LiteralType::Number(r) = evaluated_right
                                {
                                    return Ok(LiteralType::Boolean(l > r));
                                } else {
                                    return Err(LoxError::RuntimeError);
                                }
                            }
                            Err(e) => Err(e),
                        }
                    }
                    TokenType::GreaterEqual => {
                        match self.check_number_operands(
                            operator,
                            &evaluated_left,
                            &evaluated_right,
                        ) {
                            Ok(()) => {
                                if let LiteralType::Number(l) = evaluated_left
                                    && let LiteralType::Number(r) = evaluated_right
                                {
                                    return Ok(LiteralType::Boolean(l >= r));
                                } else {
                                    return Err(LoxError::RuntimeError);
                                }
                            }
                            Err(e) => Err(e),
                        }
                    }
                    TokenType::Less => {
                        match self.check_number_operands(
                            operator,
                            &evaluated_left,
                            &evaluated_right,
                        ) {
                            Ok(()) => {
                                if let LiteralType::Number(l) = evaluated_left
                                    && let LiteralType::Number(r) = evaluated_right
                                {
                                    return Ok(LiteralType::Boolean(l < r));
                                } else {
                                    return Err(LoxError::RuntimeError);
                                }
                            }
                            Err(e) => Err(e),
                        }
                    }
                    TokenType::LessEqual => {
                        match self.check_number_operands(
                            operator,
                            &evaluated_left,
                            &evaluated_right,
                        ) {
                            Ok(()) => {
                                if let LiteralType::Number(l) = evaluated_left
                                    && let LiteralType::Number(r) = evaluated_right
                                {
                                    return Ok(LiteralType::Boolean(l <= r));
                                } else {
                                    return Err(LoxError::RuntimeError);
                                }
                            }
                            Err(e) => Err(e),
                        }
                    }
                    TokenType::Minus => {
                        match self.check_number_operands(
                            operator,
                            &evaluated_left,
                            &evaluated_right,
                        ) {
                            Ok(()) => {
                                if let LiteralType::Number(l) = evaluated_left
                                    && let LiteralType::Number(r) = evaluated_right
                                {
                                    return Ok(LiteralType::Number(l - r));
                                } else {
                                    return Err(LoxError::RuntimeError);
                                }
                            }
                            Err(e) => Err(e),
                        }
                    }
                    TokenType::Slash => {
                        match self.check_number_operands(
                            operator,
                            &evaluated_left,
                            &evaluated_right,
                        ) {
                            Ok(()) => {
                                if let LiteralType::Number(l) = evaluated_left
                                    && let LiteralType::Number(r) = evaluated_right
                                {
                                    return Ok(LiteralType::Number(l / r));
                                } else {
                                    return Err(LoxError::RuntimeError);
                                }
                            }
                            Err(e) => Err(e),
                        }
                    }
                    TokenType::Star => {
                        match self.check_number_operands(
                            operator,
                            &evaluated_left,
                            &evaluated_right,
                        ) {
                            Ok(()) => {
                                if let LiteralType::Number(l) = evaluated_left
                                    && let LiteralType::Number(r) = evaluated_right
                                {
                                    return Ok(LiteralType::Number(l * r));
                                } else {
                                    return Err(LoxError::RuntimeError);
                                }
                            }
                            Err(e) => Err(e),
                        }
                    }
                    TokenType::Plus => {
                        if let LiteralType::Number(l) = evaluated_left
                            && let LiteralType::Number(r) = evaluated_right
                        {
                            return Ok(LiteralType::Number(l + r));
                        }
                        if let LiteralType::String(l) = evaluated_left
                            && let LiteralType::String(r) = evaluated_right
                        {
                            return Ok(LiteralType::String(format!("{l}{r}")));
                        }
                        token_error(operator, "Operands must be two numbers or two strings.");
                        return Err(LoxError::RuntimeError);
                    }
                    _ => {
                        token_error(operator, "Invalid token.");
                        return Err(LoxError::RuntimeError);
                    }
                }
            }
            Expr::Grouping { expression } => self.evaluate(*expression),
            Expr::Literal { value } => return Ok(value),
            Expr::Logical {
                left,
                operator,
                right,
            } => {
                let left: LiteralType = self.interpret(*left)?;
                if operator._type == TokenType::Or {
                    if let LiteralType::Boolean(true) = self.is_truthy(&left.clone()) {
                        return Ok(left);
                    }
                } else {
                    if let LiteralType::Boolean(false) = self.is_truthy(&left.clone()) {
                        return Ok(left);
                    }
                }
                return self.interpret(*right);
            }
            Expr::Unary { operator, right } => {
                let evaluated_right: LiteralType = self.evaluate(*right)?;
                match operator._type {
                    TokenType::Bang => return Ok(self.not(self.is_truthy(&evaluated_right))),
                    TokenType::Minus => {
                        match self.check_number_operand(operator, &evaluated_right) {
                            Ok(()) => {
                                match evaluated_right {
                                    LiteralType::Number(x) => return Ok(LiteralType::Number(-x)),
                                    _ => return Err(LoxError::RuntimeError), // Unreachable
                                }
                            }
                            Err(e) => return Err(e),
                        }
                    }
                    _ => return Ok(LiteralType::None),
                }
            }
            Expr::Variable { name } => return self.environment.borrow_mut().get(&name),
        }
    }

    fn execute_block(&mut self, statements: Vec<Box<Stmt>>) -> Result<(), LoxError> {
        let new_env = Rc::new(RefCell::new(Environment::new(Some(Rc::clone(
            &self.environment,
        )))));

        // Swap it in, keeping the previous
        let previous = std::mem::replace(&mut self.environment, new_env);

        let result = (|| {
            for statement in statements {
                self.execute(*statement)?;
            }
            Ok(())
        })();

        self.environment = previous;
        return result;
    }

    pub fn execute(&mut self, stmt: Stmt) -> Result<(), LoxError> {
        match stmt {
            Stmt::Block { statements } => {
                return self.execute_block(statements);
            }
            Stmt::Expression { expression } => {
                self.interpret(expression)?;
                return Ok(());
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let evaluated_condition = self.interpret(condition)?;
                if let LiteralType::Boolean(true) = self.is_truthy(&evaluated_condition) {
                    self.execute(*then_branch)?;
                } else if let Some(else_branch) = else_branch {
                    self.execute(*else_branch)?;
                }
                Ok(())
            }
            Stmt::Print { expression } => match self.interpret(expression) {
                Ok(value) => {
                    // self.environment.borrow_mut().dump(None);
                    println!("{}", literal_stringify(value));
                    return Ok(());
                }
                Err(e) => {
                    return Err(e);
                }
            },
            Stmt::While { condition, body } => {
                let mut evaluated_condition = self.interpret(condition.clone())?;
                evaluated_condition = self.interpret(condition.clone())?;
                while let LiteralType::Boolean(true) = self.is_truthy(&evaluated_condition) {
                    self.execute(*body.clone())?;
                    evaluated_condition = self.interpret(condition.clone())?;
                }
                return Ok(());
            }
            Stmt::Var { name, initializer } => {
                let value = self.interpret(initializer)?;
                self.environment.borrow_mut().define(&name.lexeme, &value);
                return Ok(());
            }
        }
    }
}
