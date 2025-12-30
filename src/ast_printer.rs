use crate::{expr::Expr, token::LiteralType};

fn parenthesize(name: &str, exprs: Vec<&Box<Expr>>) -> String {
    let mut builder = String::new();
    builder = format!("{}({}", builder, name);
    for expr in exprs {
        builder = format!("{} ", builder);
        builder = format!("{}{}", builder, print_ast(&expr));
    }

    builder = format!("{})", builder);
    builder.to_string()
}

pub fn print_ast(expr: &Expr) -> String {
    match expr {
        Expr::Binary {
            left,
            operator,
            right,
        } => return parenthesize(&operator.lexeme, Vec::from([left, right])),
        Expr::Grouping { expression } => {
            return parenthesize(&"group", Vec::from([expression]));
        }
        Expr::Literal { value } => {
            if value.clone() == LiteralType::None {
                return "nil".to_string();
            } 
            value.to_string()
        }
        Expr::Unary { operator, right } => {
            return parenthesize(&operator.lexeme, Vec::from([right]));
        }
        Expr::Variable { name } => {
            return name.lexeme.to_string();
        }

    }
}
