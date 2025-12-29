use crate::{ token::Token, token_type::TokenType};

pub fn token_error(token: Token, message:&str) {
    if token._type == TokenType::Eof {
        report(token.line, " at end", message);
    } else {
        report(token.line, &format!(" at '{}'", token.lexeme), message);
    }
}

pub fn error(line:usize, message:&str) {
    report(line, "", message)
}

pub fn report(line:usize, _where:&str, message:&str) {
    eprintln!("[line {}] Error {}: {}", line, _where, message);
}