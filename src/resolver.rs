use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    error::token_error,
    expr::Expr,
    interpreter::Interpreter,
    stmt::Stmt,
    token::{LiteralType, Token},
};

pub struct Resolver {
    interpreter: Rc<RefCell<Interpreter>>,
    scopes: Vec<HashMap<String, bool>>,
    current_function: FunctionType,
    pub had_error: bool
}

#[derive(Clone, PartialEq)]
enum FunctionType {
    None, 
    Function
}

impl Resolver {
    pub fn new(interpreter: Rc<RefCell<Interpreter>>) -> Self {
        Self {
            interpreter,
            scopes: Vec::new(),
            current_function: FunctionType::None,
            had_error: false
        }
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: Token) {
        if self.scopes.is_empty() {
            return;
        }
        if self.scopes.last_mut().unwrap().contains_key(&name.lexeme) {
        token_error(name, "Already a variable with this name in this scope.");
        self.had_error = true;
        } else {
            self.scopes.last_mut().unwrap().insert(name.lexeme, false);
        }
    }

    fn define(&mut self, name: Token) {
        if self.scopes.is_empty() {
            return;
        }
        self.scopes.last_mut().unwrap().insert(name.lexeme, true);
    }

    fn resolve_local(&mut self, expr: Expr, name: Token) {
        for i in 0..self.scopes.len() {
            if self.scopes.get(i).unwrap().contains_key(&name.lexeme) {
                self.interpreter.borrow_mut().resolve(expr.clone(), self.scopes.len() - 1 - i);
            }
        }
    }

    fn resolve_function(&mut self, stmt: Stmt, _type: FunctionType) {
        match stmt {
            Stmt::Function {
                name: _,
                params,
                body,
            } => {
                let enclosing_function = self.current_function.clone();
                self.current_function = _type;

                self.begin_scope();
                for param in params {
                    self.declare(param.clone());
                    self.define(param);
                }
                self.resolve_stmts(body);
                self.end_scope();

                self.current_function = enclosing_function;
            }
            _ => {}
        }
    }

    fn resolve_expr(&mut self, expr: Expr) {
        match expr.clone() {
            Expr::Variable { name } => {
                if !self.scopes.is_empty()
                    && self.scopes.last_mut().unwrap().get(&name.lexeme) == Some(&false)
                {
                    token_error(
                        name.clone(),
                        "Can't read local variable in its own initializer.",
                    );
                    self.had_error = true;
                }
                self.resolve_local(expr, name);
            }
            Expr::Assign { name, value } => {
                self.resolve_expr(*value);
                self.resolve_local(expr, name);
            }
            Expr::Binary {
                left,
                operator: _,
                right,
            } => {
                self.resolve_expr(*left);
                self.resolve_expr(*right);
            }
            Expr::Call {
                callee,
                paren: _,
                arguments,
            } => {
                self.resolve_expr(*callee);
                for argument in arguments {
                    self.resolve_expr(*argument);
                }
            }
            Expr::Grouping { expression } => {
                self.resolve_expr(*expression);
            }
            Expr::Literal { value: _ } => {}
            Expr::Logical {
                left,
                operator: _,
                right,
            } => {
                self.resolve_expr(*left);
                self.resolve_expr(*right);
            }
            Expr::Unary { operator: _, right } => {
                self.resolve_expr(*right);
            }
        }
    }

    fn resolve_stmt(&mut self, stmt: Stmt) {
        match stmt.clone() {
            Stmt::Block { statements } => {
                self.begin_scope();
                self.resolve_stmts(statements);
                self.end_scope();
            }
            Stmt::Class { name, methods } => {
                self.declare(name.clone());
                self.define(name);
            }
            Stmt::Var { name, initializer } => {
                self.declare(name.clone());
                if let Expr::Literal {
                    value: LiteralType::None,
                } = initializer
                {
                    self.resolve_expr(initializer);
                }
                self.define(name)
            }
            Stmt::Function {
                name,
                params: _,
                body: _,
            } => {
                self.declare(name.clone());
                self.define(name);

                self.resolve_function(stmt, FunctionType::Function);
            }
            Stmt::Expression { expression } => {
                self.resolve_expr(expression);
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.resolve_expr(condition);
                self.resolve_stmt(*then_branch);
                if let Some(else_branch) = else_branch {
                    self.resolve_stmt(*else_branch);
                }
            }
            Stmt::Print { expression } => {
                self.resolve_expr(expression);
            }
            Stmt::Return { keyword, value } => {

                if self.current_function == FunctionType::None {
                    token_error(keyword, "Can't return from top-level code.");
                    self.had_error = true;
                }

                if let Expr::Literal {
                    value: LiteralType::None,
                } = value
                {
                    self.resolve_expr(value);
                }
            }
            Stmt::While { condition, body } => {
                self.resolve_expr(condition);
                self.resolve_stmt(*body);
            }
        }
    }

    pub fn resolve_stmts(&mut self, stmts: Vec<Box<Stmt>>) {
        for stmt in stmts {
            self.resolve_stmt(*stmt);
        }
    }
}
