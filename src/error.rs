use crate::scanner::{Token, TokenType};
use std::fmt;

#[derive(Debug)]
pub struct ParserError {
    pub message: String,
    pub line: usize,
    pub token: Token,
}

impl ParserError {
    pub fn new(message: &str, line: usize, token: Token) -> Self {
        Self {
            message: message.to_string(),
            line,
            token,
        }
    }

    pub fn report(&self) {
        if self.token.token_type == TokenType::EOF {
            println!("LINE {}: {}", self.line, self.message);
        } else {
            println!(
                "LINE {}: '{}': {}",
                self.line, self.token.lexeme, self.message
            );
        }
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.token.token_type == TokenType::EOF {
            write!(f, "LINE {}: {}", self.line, self.message)
        } else {
            write!(
                f,
                "LINE {}: '{}': {}",
                self.line, self.token.lexeme, self.message
            )
        }
    }
}
