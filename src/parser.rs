use crate::{
    error,
    error_type::LoxError,
    expr::Expr,
    stmt::Stmt,
    token::{LiteralType, Token},
    token_type::TokenType,
};

pub struct Parser {
    pub tokens: Vec<Token>,
    current: usize,
}

impl Default for Parser {
    fn default() -> Self {
        Self {
            tokens: Vec::new(),
            current: 0,
        }
    }
}

impl Parser {
    pub fn new(token: Vec<Token>) -> Self {
        Self {
            tokens: token,
            current: 0,
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

    fn error(&self, token: Token, message: &str) -> LoxError {
        error::token_error(token, message);
        return LoxError::ParseError;
    }

    fn consume(&mut self, _type: TokenType, message: &str) -> Result<Token, LoxError> {
        if self.check(_type) {
            return Ok(self.advance());
        }
        Err(self.error(self.peek().clone(), message))
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
            return Ok(Expr::Variable { name: self.previous() });
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

        Err(self.error(self.peek().clone(), "Expected expression."))
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
        self.primary()
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
                Ok(expr)
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
                Ok(expr)
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
                Ok(expr)
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
                Ok(expr)
            }
            Err(e) => return Err(e),
        }
    }

    fn expression(&mut self) -> Result<Expr, LoxError> {
        self.equality()
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
    fn expression_statement(&mut self) -> Result<Stmt, LoxError> {
        match self.expression() {
            Ok(expr) => match self.consume(TokenType::Semicolon, "Expect ';' after expression.") {
                Ok(_token) => return Ok(Stmt::Expression { expression: expr }),
                Err(e) => return Err(e),
            },
            Err(e) => return Err(e),
        }
    }

    fn statement(&mut self) -> Result<Stmt, LoxError> {
        if self._match(Vec::from([TokenType::Print])) {
            return self.print_statement();
        }
        return self.expression_statement();
    }

    fn var_declaration(&mut self) -> Result<Stmt, LoxError> {
        let name = self.consume(TokenType::Identifier, "Expect variable name.")?;
        let mut initializer: Expr = Expr::Literal { value: LiteralType::None };
        if self._match(Vec::from([TokenType::Equal])) {
            initializer = self.expression()?;
        }
        self.consume(TokenType::Semicolon, "Expected ';' after variable declaration")?;
        return Ok( Stmt::Var { name, initializer })
    }

    fn declaration(&mut self) -> Option<Stmt> {
        if self._match(Vec::from([TokenType::Var])) {
            match self.var_declaration() {
                Ok(stmt) => return Some(stmt),
                Err(e) => {
                    self.synchronize();
                    return None;
                }
            }
        }
        match self.statement() {
            Ok(stmt) => return Some(stmt),
            Err(e) => {
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
                None => continue
            }
            
        }
        return Ok(statements);
    }
}
