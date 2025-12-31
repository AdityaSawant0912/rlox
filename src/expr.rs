use crate::{
    token::{LiteralType, Token}
};

#[derive(Debug, Clone)]pub enum Expr {
    Assign {
        name : Token,
        value : Box<Expr>,
    },
    Binary {
        left : Box<Expr>,
        operator : Token,
        right : Box<Expr>,
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