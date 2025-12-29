use crate::{
    error::token_error,
    error_type::LoxError,
    expr::Expr,
    token::{LiteralType, Token},
    token_type::TokenType,
};

fn evaluate(expression: Expr) -> Result<LiteralType, LoxError> {
    interpret(expression)
}

fn check_number_operand(operator: Token, operand: &LiteralType) -> Result<(), LoxError> {
    if let LiteralType::Number(_x) = operand {
        return Ok(());
    }
    token_error(operator, "Operand must be a number.");
    Err(LoxError::RuntimeError)
}

fn check_number_operands(
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

fn is_truthy(expr: LiteralType) -> LiteralType {
    match expr {
        LiteralType::None => return LiteralType::Boolean(false),
        LiteralType::Boolean(x) => return LiteralType::Boolean(x),
        _ => return LiteralType::Boolean(true),
    }
}

fn not(expr: LiteralType) -> LiteralType {
    match expr {
        LiteralType::Boolean(x) => return LiteralType::Boolean(!x),
        _ => return LiteralType::Boolean(false), // This case should not hit
    }
}

pub fn interpret(expr: Expr) -> Result<LiteralType, LoxError> {
    match expr {
        Expr::Binary {
            left,
            operator,
            right,
        } => {
            let evaluated_left: LiteralType = evaluate(*left)?;
            let evaluated_right: LiteralType = evaluate(*right)?;
            match operator._type {
                TokenType::Greater => {
                    match check_number_operands(operator, &evaluated_left, &evaluated_right) {
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
                    match check_number_operands(operator, &evaluated_left, &evaluated_right) {
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
                    match check_number_operands(operator, &evaluated_left, &evaluated_right) {
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
                    match check_number_operands(operator, &evaluated_left, &evaluated_right) {
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
                    match check_number_operands(operator, &evaluated_left, &evaluated_right) {
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
                    match check_number_operands(operator, &evaluated_left, &evaluated_right) {
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
                    match check_number_operands(operator, &evaluated_left, &evaluated_right) {
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
                    return Err(LoxError::RuntimeError)
                }
            }
        }
        Expr::Grouping { expression } => evaluate(*expression),
        Expr::Literal { value } => return Ok(value),
        Expr::Unary { operator, right } => {
            let evaluated_right: LiteralType = evaluate(*right)?;
            match operator._type {
                TokenType::Bang => return Ok(not(is_truthy(evaluated_right))),
                TokenType::Minus => {
                    match check_number_operand(operator, &evaluated_right) {
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
    }
}
