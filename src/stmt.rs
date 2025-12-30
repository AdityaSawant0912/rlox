use crate::{
    expr::Expr,
    token::Token
};

pub enum Stmt {
    Block {
        statements : Vec<Box<Stmt>>,
    },
    Expression {
        expression : Expr,
    },
    Print {
        expression : Expr,
    },
    Var {
        name : Token,
        initializer : Expr,
    },
}