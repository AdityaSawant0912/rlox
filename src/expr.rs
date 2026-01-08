use crate::{
    token::{LiteralType, Token}
};

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    Assign {
        name : Token,
        value : Box<Expr>,
    },
    Binary {
        left : Box<Expr>,
        operator : Token,
        right : Box<Expr>,
    },
    Call {
        callee : Box<Expr>,
        paren : Token,
        arguments : Vec<Box<Expr>>,
    },
    Grouping {
        expression : Box<Expr>,
    },
    Literal {
        value : LiteralType,
    },
    Logical {
        left : Box<Expr>,
        operator : Token,
        right : Box<Expr>,
    },
    Unary {
        operator : Token,
        right : Box<Expr>,
    },
    Variable {
        name : Token,
    },
}