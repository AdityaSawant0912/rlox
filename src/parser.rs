use crate::{
    error::{self},
    error_type::LoxError,
    expr::Expr,
    stmt::Stmt,
    token::{LiteralType, Token},
    token_type::TokenType,
};

pub struct Parser {
    pub tokens: Vec<Token>,
    current: usize,
    pub had_error: bool,
}

impl Default for Parser {
    fn default() -> Self {
        Self {
            tokens: Vec::new(),
            current: 0,
            had_error: false,
        }
    }
}

impl Parser {
    pub fn new(token: Vec<Token>) -> Self {
        Self {
            tokens: token,
            current: 0,
            had_error: false,
        }
    }

    fn is_at_end(&self) -> bool {
        self.tokens[self.current]._type == TokenType::Eof
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        return self.previous();
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn check(&self, _type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        return self.peek()._type == _type;
    }

    fn _match(&mut self, types: Vec<TokenType>) -> bool {
        for t in types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }
        return false;
    }

    fn error(&mut self, token: Token, message: &str) -> LoxError {
        self.had_error = true;
        error::token_error(token, message);
        return LoxError::ParseError;
    }

    fn consume(&mut self, _type: TokenType, message: &str) -> Result<Token, LoxError> {
        if self.check(_type) {
            return Ok(self.advance());
        }
        return Err(self.error(self.peek().clone(), message));
    }

    fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if self.previous()._type == TokenType::Semicolon {
                return;
            }
            match self.peek()._type {
                TokenType::Class => return,
                TokenType::Fun => return,
                TokenType::Var => return,
                TokenType::For => return,
                TokenType::If => return,
                TokenType::While => return,
                TokenType::Print => return,
                TokenType::Return => return,
                _ => {
                    self.advance();
                }
            }
        }
    }

    fn primary(&mut self) -> Result<Expr, LoxError> {
        if self._match(Vec::from([TokenType::False])) {
            return Ok(Expr::Literal {
                value: LiteralType::Boolean(false),
            });
        }
        if self._match(Vec::from([TokenType::True])) {
            return Ok(Expr::Literal {
                value: LiteralType::Boolean(true),
            });
        }
        if self._match(Vec::from([TokenType::Nil])) {
            return Ok(Expr::Literal {
                value: LiteralType::None,
            });
        }
        if self._match(Vec::from([TokenType::Number, TokenType::String])) {
            return Ok(Expr::Literal {
                value: self.previous().literal.clone(),
            });
        }
        if self._match(Vec::from([TokenType::Var])) {
            return Ok(Expr::Variable {
                name: self.previous(),
            });
        }
        if self._match(Vec::from([TokenType::LeftParen])) {
            match self.expression() {
                Ok(expr) => {
                    match self.consume(TokenType::RightParen, "Expect ')' after expression.") {
                        Ok(_token) => {}
                        Err(_e) => {}
                    }
                    return Ok(Expr::Grouping {
                        expression: Box::new(expr),
                    });
                }
                Err(e) => return Err(e),
            }
        }
        if self._match(Vec::from([TokenType::Identifier])) {
            return Ok(Expr::Variable {
                name: self.previous(),
            });
        }
        return Err(self.error(self.peek().clone(), "Expected expression."));
    }

    fn finish_call(&mut self, callee: &Expr) -> Result<Expr, LoxError> {
        let mut arguments: Vec<Box<Expr>> = Vec::new();
        if !self.check(TokenType::RightParen) {
            arguments.push(Box::new(self.expression()?));
            while self._match(Vec::from([TokenType::Comma])) {
                if arguments.len() >= 225 {
                    return Err(self.error(self.peek().clone(), "Can't have more than 255 arguments"));
                }
                arguments.push(Box::new(self.expression()?));
            }
        }
        let paren: Token = self.consume(TokenType::RightParen, "Expected ')' after arguments.")?;

        return Ok(Expr::Call { callee: Box::new(callee.clone()), paren, arguments })
    }

    fn call(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.primary()?;
        loop {
            if self._match(Vec::from([TokenType::LeftParen])) {
                expr = self.finish_call(&expr)?;
            } else if self._match(Vec::from([TokenType::Dot])) {
                let name: Token = self.consume(TokenType::Identifier, "Expected property name after '.'.")?;
                expr = Expr::Get { object: Box::new(expr), name }
            } else {
                break;
            }
        }
        return Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, LoxError> {
        if self._match(Vec::from([TokenType::Bang, TokenType::Minus])) {
            let operator = self.previous();
            match self.unary() {
                Ok(right) => {
                    return Ok(Expr::Unary {
                        operator: operator,
                        right: Box::new(right),
                    });
                }
                Err(e) => return Err(e),
            }
        }
        self.call()
    }

    fn factor(&mut self) -> Result<Expr, LoxError> {
        match self.unary() {
            Ok(mut expr) => {
                while self._match(Vec::from([TokenType::Slash, TokenType::Star])) {
                    let operator = self.previous();
                    match self.unary() {
                        Ok(right) => {
                            expr = Expr::Binary {
                                left: Box::new(expr),
                                operator: operator,
                                right: Box::new(right),
                            }
                        }
                        Err(e) => return Err(e),
                    }
                }
                return Ok(expr);
            }
            Err(e) => return Err(e),
        }
    }

    fn term(&mut self) -> Result<Expr, LoxError> {
        match self.factor() {
            Ok(mut expr) => {
                while self._match(Vec::from([TokenType::Minus, TokenType::Plus])) {
                    let operator = self.previous();
                    match self.factor() {
                        Ok(right) => {
                            expr = Expr::Binary {
                                left: Box::new(expr),
                                operator: operator,
                                right: Box::new(right),
                            }
                        }
                        Err(e) => return Err(e),
                    }
                }
                return Ok(expr);
            }
            Err(e) => return Err(e),
        }
    }

    fn comparison(&mut self) -> Result<Expr, LoxError> {
        match self.term() {
            Ok(mut expr) => {
                while self._match(Vec::from([
                    TokenType::Greater,
                    TokenType::GreaterEqual,
                    TokenType::Less,
                    TokenType::LessEqual,
                ])) {
                    let operator = self.previous();
                    match self.term() {
                        Ok(right) => {
                            expr = Expr::Binary {
                                left: Box::new(expr),
                                operator: operator,
                                right: Box::new(right),
                            }
                        }
                        Err(e) => return Err(e),
                    }
                }
                return Ok(expr);
            }
            Err(e) => return Err(e),
        }
    }

    fn equality(&mut self) -> Result<Expr, LoxError> {
        match self.comparison() {
            Ok(mut expr) => {
                while self._match(Vec::from([TokenType::BangEqual, TokenType::EqualEqual])) {
                    let operator = self.previous();
                    match self.comparison() {
                        Ok(right) => {
                            expr = Expr::Binary {
                                left: Box::new(expr),
                                operator: operator,
                                right: Box::new(right),
                            }
                        }
                        Err(e) => return Err(e),
                    }
                }
                return Ok(expr);
            }
            Err(e) => return Err(e),
        }
    }

    fn and(&mut self) -> Result<Expr, LoxError> {
        let expr: Expr = self.equality()?;
        if self._match(Vec::from([TokenType::And])) {
            let operator: Token = self.previous();
            let right: Expr = self.equality()?;
            return Ok(Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }
        return Ok(expr);
    }

    fn or(&mut self) -> Result<Expr, LoxError> {
        let expr: Expr = self.and()?;
        if self._match(Vec::from([TokenType::Or])) {
            let operator: Token = self.previous();
            let right: Expr = self.and()?;
            return Ok(Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }
        return Ok(expr);
    }

    fn assignment(&mut self) -> Result<Expr, LoxError> {
        let expr: Expr = self.or()?;
        if self._match(Vec::from([TokenType::Equal])) {
            let equals: Token = self.previous();
            let value: Expr = self.assignment()?;
            if let Expr::Variable { name } = expr {
                return Ok(Expr::Assign {
                    name,
                    value: Box::new(value),
                });
            } else if let Expr::Get { object, name } = expr {
                return Ok(Expr::Set { object, name, value: Box::new(value) })
            }
            return Err(self.error(equals, "Invalid assignment target."));
        }
        return Ok(expr);
    }

    fn expression(&mut self) -> Result<Expr, LoxError> {
        self.assignment()
    }

    fn for_statement(&mut self) -> Result<Stmt, LoxError> {
        self.consume(TokenType::LeftParen, "Expected '(' after 'for'.")?;
        let initializer: Stmt;
        if self._match(Vec::from([TokenType::Semicolon])) {
            initializer = Stmt::Expression { expression: Expr::Literal { value: LiteralType::None } }
        } else if self._match(Vec::from([TokenType::Var])) {
            initializer = self.var_declaration()?;
        } else {
            initializer = self.expression_statement()?;
        }
        let mut condition:Expr;
        if !self.check(TokenType::Semicolon) {
            condition = self.expression()?;
        } else {
            condition = Expr::Literal { value: LiteralType::None };
        }
        
        self.consume(TokenType::Semicolon, "Expected ';' after loop condition.")?;

        let increment:Expr;
        if !self.check(TokenType::RightParen) {
            increment = self.expression()?;
        } else {
            increment = Expr::Literal { value: LiteralType::None };
        }

        self.consume(TokenType::RightParen, "Expected ')' after for clauses.")?;

        let mut body = self.statement()?;

        if increment != (Expr::Literal { value: LiteralType::None }){
            body = Stmt::Block { statements: Vec::from([
                Box::new(body),
                Box::new(Stmt::Expression { expression: increment })
            ]) }
        }

        if condition == (Expr::Literal { value: LiteralType::None }) {
            condition = Expr::Literal { value: LiteralType::Boolean(true) }
        }

        body = Stmt::While { condition, body: Box::new(body) };

        if initializer != (Stmt::Expression { expression: Expr::Literal { value: LiteralType::None } }) {
            body = Stmt::Block { statements: Vec::from([
                Box::new(initializer),
                Box::new(body)
            ]) }
        }

        return Ok(body)
    }

    fn if_statement(&mut self) -> Result<Stmt, LoxError> {
        self.consume(TokenType::LeftParen, "Expected '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expected ')' after if condition.")?;
        let then_branch = self.statement()?;
        let mut else_branch: Option<Box<Stmt>> = None;
        if self._match(Vec::from([TokenType::Else])) {
            else_branch = Some(Box::new(self.statement()?));
        }
        return Ok(Stmt::If {
            condition,
            then_branch: Box::new(then_branch),
            else_branch,
        });
    }

    fn print_statement(&mut self) -> Result<Stmt, LoxError> {
        match self.expression() {
            Ok(expr) => match self.consume(TokenType::Semicolon, "Expect ';' after value.") {
                Ok(_token) => return Ok(Stmt::Print { expression: expr }),
                Err(e) => return Err(e),
            },
            Err(e) => return Err(e),
        }
    }

    fn return_statement(&mut self) -> Result<Stmt, LoxError> {
        let keyword = self.previous();
        let value: Expr;
        if !self.check(TokenType::Semicolon) {
            value = self.expression()?;
        } else {
            value = Expr::Literal { value: LiteralType::None };
        }

        self.consume(TokenType::Semicolon, "Expected ';' after return value.")?;
        return Ok(Stmt::Return { keyword, value })
    }

    fn while_statement(&mut self) -> Result<Stmt, LoxError> {
        self.consume(TokenType::LeftParen, "Expected '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expected ')' after if condition.")?;
        let body = self.statement()?;
        return Ok(Stmt::While {
            condition,
            body: Box::new(body),
        });
    }

    fn expression_statement(&mut self) -> Result<Stmt, LoxError> {
        match self.expression() {
            Ok(expr) => match self.consume(TokenType::Semicolon, "Expect ';' after expression.") {
                Ok(_token) => return Ok(Stmt::Expression { expression: expr }),
                Err(e) => return Err(e),
            },
            Err(e) => return Err(e),
        }
    }

    fn block(&mut self) -> Result<Vec<Box<Stmt>>, LoxError> {
        let mut statements: Vec<Box<Stmt>> = Vec::new();
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            if let Some(stmt) = self.declaration() {
                statements.push(Box::new(stmt));
            }
        }
        self.consume(TokenType::RightBrace, "Expected '}' after block")?;
        return Ok(statements);
    }

    fn statement(&mut self) -> Result<Stmt, LoxError> {
        if self._match(Vec::from([TokenType::For])) {
            return self.for_statement();
        }
        if self._match(Vec::from([TokenType::If])) {
            return self.if_statement();
        }
        if self._match(Vec::from([TokenType::Print])) {
            return self.print_statement();
        }
        if self._match(Vec::from([TokenType::Return])) {
            return self.return_statement();
        }
        if self._match(Vec::from([TokenType::While])) {
            return self.while_statement();
        }
        if self._match(Vec::from([TokenType::LeftBrace])) {
            return Ok(Stmt::Block {
                statements: self.block()?,
            });
        }
        return self.expression_statement();
    }

    fn class_declaration(&mut self) -> Result<Stmt, LoxError> {
        let name = self.consume(TokenType::Identifier, &format!("Expect class name."))?;
        self.consume(
            TokenType::LeftBrace,
            &format!("Expect '{{' before class body."),
        )?;
        
        let mut methods: Vec<Box<Stmt>> = Vec::new();

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            match self.function("method") {
                Ok(method) => methods.push(Box::new(method)),
                Err(e) =>  return Err(e)
            }
        }
        
        self.consume(
            TokenType::RightBrace,
            &format!("Expect '}}' after class body."),
        )?;

        return Ok(Stmt::Class { name, methods })
    }

    fn function(&mut self, kind: &str) -> Result<Stmt, LoxError> {
        let name = self.consume(TokenType::Identifier, &format!("Expect {} name.", kind))?;
        self.consume(
            TokenType::LeftParen,
            &format!("Expect '(' after {} name.", kind),
        )?;
        let mut parameters: Vec<Token> = Vec::new();
        if !self.check(TokenType::RightParen) {
            loop {
                if parameters.len() >= 225 {
                    self.error(self.peek().clone(), "Can't have more than 255 parameters.");
                }
                parameters.push(self.consume(TokenType::Identifier, "Expected parameter name.")?);

                if !self._match(Vec::from([TokenType::Comma])) {
                    break;
                } 
            }
        }
        self.consume(
            TokenType::RightParen,
            &format!("Expect ')' after {} name.", kind),
        )?;

        self.consume(
            TokenType::LeftBrace,
            &format!("Expect '{{' before {} body.", kind),
        )?;
        let body = self.block()?;
        return Ok(Stmt::Function { name, params: parameters, body })
    }

    fn var_declaration(&mut self) -> Result<Stmt, LoxError> {
        let name = self.consume(TokenType::Identifier, "Expect variable name.")?;
        let mut initializer: Expr = Expr::Literal {
            value: LiteralType::None,
        };
        if self._match(Vec::from([TokenType::Equal])) {
            initializer = self.expression()?;
        }
        self.consume(
            TokenType::Semicolon,
            "Expected ';' after variable declaration",
        )?;
        return Ok(Stmt::Var { name, initializer });
    }

    fn declaration(&mut self) -> Option<Stmt> {
        if self._match(Vec::from([TokenType::Class])) {
            match self.class_declaration() {
                Ok(stmt) => return Some(stmt),
                Err(_e) => {
                    self.synchronize();
                    return None;
                }
            }
        }
        if self._match(Vec::from([TokenType::Fun])) {
            match self.function("function") {
                Ok(stmt) => return Some(stmt),
                Err(_e) => {
                    self.synchronize();
                    return None;
                }
            }
        }
        if self._match(Vec::from([TokenType::Var])) {
            match self.var_declaration() {
                Ok(stmt) => return Some(stmt),
                Err(_e) => {
                    self.synchronize();
                    return None;
                }
            }
        }
        match self.statement() {
            Ok(stmt) => return Some(stmt),
            Err(_e) => {
                self.synchronize();
                return None;
            }
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, LoxError> {
        let mut statements: Vec<Stmt> = Vec::new();
        while !self.is_at_end() {
            match self.declaration() {
                Some(stmt) => statements.push(stmt),
                None => continue,
            }
        }
        if self.had_error {
            return Err(LoxError::ParseError);
        }
        return Ok(statements);
    }
}
