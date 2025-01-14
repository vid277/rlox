use crate::expr::Expr;
use crate::{expr::LiteralValue, scanner::Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn expression(&mut self) -> Expr {
        return self.equality();
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.matches(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison();

            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }

        expr
    }
    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while self.matches(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term();

            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator.clone(),
                right: Box::new(right),
            };
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.matches(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor();

            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.matches(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary();

            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        if self.matches(vec![TokenType::Minus, TokenType::Bang]) {
            let operator = self.previous();
            let right = self.unary();

            return Expr::Unary {
                operator,
                right: Box::new(right),
            };
        }

        self.primary();
    }

    fn primary(&mut self) -> Expr {
        if self.matches(vec![TokenType::False]) {
            return Expr::Literal {
                value: LiteralValue::False,
            };
        }

        if self.matches(vec![TokenType::True]) {
            return Expr::Literal {
                value: LiteralValue::True,
            };
        }

        if self.matches(vec![TokenType::Nil]) {
            return Expr::Literal {
                value: LiteralValue::Nil,
            };
        }

        if self.matches(vec![TokenType::Number]) {
            return Expr::Literal {
                value: LiteralValue::Number(self.previous().literal.unwrap_number()),
            };
        }

        if self.matches(vec![TokenType::String]) {
            return Expr::Literal {
                value: LiteralValue::StringValue(self.previous().literal.unwrap_string()),
            };
        }

        if self.matches(vec![TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expect ')' after expression.");
            return Expr::Grouping {
                expression: Box::new(expr),
            };
        }

        panic!("Expect expression.")
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Token {
        todo!()
    }

    fn matches(&mut self, token_types: Vec<TokenType>) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        return false;
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        return self.peek().token_type == token_type;
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        return self.previous().clone();
    }

    fn is_at_end(&self) -> bool {
        return self.peek().token_type == TokenType::EOF;
    }

    fn peek(&self) -> Token {
        return self.tokens[self.current].clone();
    }

    fn previous(&self) -> Token {
        return self.tokens[self.current - 1].clone();
    }
}
