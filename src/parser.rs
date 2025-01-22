use crate::error::RuntimeError;
use crate::expr::{Expr, LiteralValue as ExprLiteralValue};
use crate::scanner::{LiteralValue as ScannerLiteralValue, Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn expression(&mut self) -> Result<Expr, RuntimeError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, RuntimeError> {
        let mut expr = self.comparison()?;

        while self.matches(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, RuntimeError> {
        let mut expr = self.term()?;

        while self.matches(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, RuntimeError> {
        let mut expr = self.factor()?;

        while self.matches(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, RuntimeError> {
        let mut expr = self.unary()?;

        while self.matches(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, RuntimeError> {
        if self.matches(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;

            return Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            });
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, RuntimeError> {
        if self.matches(&[TokenType::False]) {
            return Ok(Expr::Literal {
                value: ExprLiteralValue::False,
            });
        }

        if self.matches(&[TokenType::True]) {
            return Ok(Expr::Literal {
                value: ExprLiteralValue::True,
            });
        }

        if self.matches(&[TokenType::Nil]) {
            return Ok(Expr::Literal {
                value: ExprLiteralValue::Nil,
            });
        }

        if self.matches(&[TokenType::Number]) {
            if let Some(ScannerLiteralValue::FloatValue(n)) = &self.previous().literal {
                return Ok(Expr::Literal {
                    value: ExprLiteralValue::Number(*n),
                });
            }
        }

        if self.matches(&[TokenType::String]) {
            if let Some(ScannerLiteralValue::StringValue(s)) = &self.previous().literal {
                return Ok(Expr::Literal {
                    value: ExprLiteralValue::StringValue(s.clone()),
                });
            }
        }

        if self.matches(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression")?;
            return Ok(Expr::Grouping {
                expression: Box::new(expr),
            });
        }
        Err(RuntimeError::new(
            &format!("Expected expression, but found {}", self.peek().token_type),
            self.peek().line_number,
            self.peek().clone(),
        ))
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<&Token, RuntimeError> {
        if self.check(token_type) {
            return Ok(self.advance());
        }

        Err(RuntimeError::new(
            &format!("{}, but found {}", message, self.peek().token_type),
            self.peek().line_number,
            self.peek().clone(),
        ))
    }

    fn matches(&mut self, token_types: &[TokenType]) -> bool {
        for &token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        return self.peek().token_type == token_type;
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        return self.peek().token_type == TokenType::EOF;
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn synchronize(&mut self) {
        self.advance();

        while (self.is_at_end()) {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            match self.peek().token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => (),
            }

            self.advance();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Scanner;

    use super::*;

    #[test]
    fn test_parse_expression() {
        let tokens = Scanner::new("(1 + 2) * 3").scan_tokens().unwrap();
        let mut parser = Parser::new(tokens);
        let expr = parser.expression();
        match expr {
            Ok(expr) => println!("Successfully parsed: {:?}", expr),
            Err(err) => println!("Parser error: {:?}", err.to_string()),
        }
    }
}
