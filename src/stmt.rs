use crate::{
    expr::Expr,
    token::Token
};

#[derive(Debug, Clone)]pub enum Stmt {
    Block {
        statements : Vec<Box<Stmt>>,
    },
    Expression {
        expression : Expr,
    },
    If {
        condition : Expr,
        then_branch : Box<Stmt>,
        else_branch : Option<Box<Stmt>>,
    },
    Print {
        expression : Expr,
    },
    While {
        condition : Expr,
        body : Box<Stmt>,
    },
    Var {
        name : Token,
        initializer : Expr,
    },
}