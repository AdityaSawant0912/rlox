use crate::{
    expr::Expr,
    token::Token
};

#[derive(Clone, PartialEq)]
pub enum Stmt {
    Block {
        statements : Vec<Box<Stmt>>,
    },
    Expression {
        expression : Expr,
    },
    Function {
        name : Token,
        params : Vec<Token>,
        body : Vec<Box<Stmt>>,
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