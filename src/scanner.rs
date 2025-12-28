use crate::{
    error,
    token::{LiteralType, Token},
    token_type::{TokenType, create_keywords},
};

pub struct Scanner {
    pub source: String,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Default for Scanner {
    fn default() -> Self {
        Self {
            source: String::new(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source: source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()
        }
        self.tokens.push(Token {
            _type: TokenType::Eof,
            lexeme: String::new(),
            literal: LiteralType::None,
            line: self.line,
        });
        return self.tokens.clone();
    }

    fn is_at_end(&self) -> bool {
        return self.current >= self.source.len();
    }

    fn is_digit(&mut self, c: char) -> bool {
        return c >= '0' && c <= '9';
    }

    fn is_alpha(&mut self, c: char) -> bool {
        return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_';
    }

    fn is_alpha_numeric(&mut self, c: char) -> bool {
        return self.is_alpha(c) || self.is_digit(c);
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap(); // unwrap causes panic if some(None); But we never advance without checking so okay. 
        self.current += 1;
        return c
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        return self.source.chars().nth(self.current).unwrap();
    }
    fn peek_next(&mut self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        return self.source.chars().nth(self.current + 1).unwrap();
    }

    fn r#match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }
        self.current += 1;
        return false;
    }

    // fn add_token(&mut self, _type: TokenType) { // Rust doesn't support function overloading.
    //     self.add_token(_type, LiteralType::None);
    // }

    fn add_token(&mut self, _type: TokenType, literal: LiteralType) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token {
            _type: _type,
            lexeme: text.to_string(),
            literal: literal,
            line: self.line,
        });
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1
            }
            self.advance();
        }

        if self.is_at_end() {
            error::error(self.line, "Unterminated String");
            return;
        }

        self.advance(); // Close "
        let value = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token(TokenType::String, LiteralType::String(value));
    }

    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }
        let next = self.peek_next();
        if self.peek() == '.' && self.is_digit(next) {
            self.advance();
            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        let value = self.source[self.start..self.current]
            .to_string()
            .parse()
            .unwrap();
        self.add_token(TokenType::Number, LiteralType::Number(value));
    }

    fn identifier(&mut self) {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }
        let keywords = create_keywords();
        let text = self.source[self.start..self.current].to_string();
        let token_type = keywords.get(text.as_str());
        let _type;
        match token_type {
            Some(t) => {_type = t.clone()},
            None => _type = TokenType::Identifier
        }

        self.add_token(_type, LiteralType::None);
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen, LiteralType::None),
            ')' => self.add_token(TokenType::RightParen, LiteralType::None),
            '{' => self.add_token(TokenType::LeftBrace, LiteralType::None),
            '}' => self.add_token(TokenType::RightBrace, LiteralType::None),
            ',' => self.add_token(TokenType::Comma, LiteralType::None),
            '.' => self.add_token(TokenType::Dot, LiteralType::None),
            '-' => self.add_token(TokenType::Minus, LiteralType::None),
            '+' => self.add_token(TokenType::Plus, LiteralType::None),
            ';' => self.add_token(TokenType::Semicolon, LiteralType::None),
            '*' => self.add_token(TokenType::Star, LiteralType::None),

            '!' => {
                if self.r#match('=') {
                    self.add_token(TokenType::BangEqual, LiteralType::None)
                } else {
                    self.add_token(TokenType::Bang, LiteralType::None)
                }
            }
            '=' => {
                if self.r#match('=') {
                    self.add_token(TokenType::EqualEqual, LiteralType::None)
                } else {
                    self.add_token(TokenType::Equal, LiteralType::None)
                }
            }
            '<' => {
                if self.r#match('=') {
                    self.add_token(TokenType::LessEqual, LiteralType::None)
                } else {
                    self.add_token(TokenType::Less, LiteralType::None)
                }
            }
            '>' => {
                if self.r#match('=') {
                    self.add_token(TokenType::GreaterEqual, LiteralType::None)
                } else {
                    self.add_token(TokenType::Greater, LiteralType::None)
                }
            }
            '/' => {
                if self.r#match('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, LiteralType::None)
                }
            }

            ' ' => {}
            '\r' => {}
            '\t' => {}
            '\n' => self.line += 1,
            '"' => self.string(),

            _ => {
                if self.is_digit(c) {
                    self.number()
                } else if self.is_alpha(c) {
                    self.identifier()
                } else {
                    error::error(self.line, "Unexpected character. {}")
                }
            }
        }
    }
}
