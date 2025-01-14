use std::collections::HashMap;
use std::sync::LazyLock;

static KEYWORDS: LazyLock<HashMap<String, TokenType>> = LazyLock::new(|| {
    HashMap::from([
        ("and".to_string(), TokenType::And),
        ("class".to_string(), TokenType::Class),
        ("else".to_string(), TokenType::Else),
        ("false".to_string(), TokenType::False),
        ("for".to_string(), TokenType::For),
        ("fun".to_string(), TokenType::Fun),
        ("if".to_string(), TokenType::If),
        ("nil".to_string(), TokenType::Nil),
        ("or".to_string(), TokenType::Or),
        ("print".to_string(), TokenType::Print),
        ("return".to_string(), TokenType::Return),
        ("super".to_string(), TokenType::Super),
        ("this".to_string(), TokenType::This),
        ("true".to_string(), TokenType::True),
        ("var".to_string(), TokenType::Var),
        ("while".to_string(), TokenType::While),
    ])
});

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(self: &mut Self) -> Result<Vec<Token>, String> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token {
            token_type: TokenType::EOF,
            lexeme: "".to_string(),
            literal: None,
            line_number: self.line,
        });

        Ok(self.tokens.clone())
    }

    fn scan_token(self: &mut Self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenType::BangEqual);
                } else {
                    self.add_token(TokenType::Bang);
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenType::EqualEqual);
                } else {
                    self.add_token(TokenType::Equal);
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenType::LessEqual);
                } else {
                    self.add_token(TokenType::Less);
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenType::GreaterEqual);
                } else {
                    self.add_token(TokenType::Greater);
                }
            }
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            '"' => {
                self.string();
            }
            ' ' | '\r' | '\t' => {}
            '\n' => {
                self.line += 1;
            }
            _ => {
                if self.is_digit(c) {
                    self.number();
                } else if self.is_alpha(c) {
                    self.identifier();
                } else {
                    println!("LINE {}: Unexpected character.", self.line);
                }
            }
        }
    }

    fn is_digit(self: &Self, c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn identifier(self: &mut Self) {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let text = self.source[self.start..self.current].to_string();
        let token_type = KEYWORDS.get(&text).unwrap_or(&TokenType::Identifier);

        self.add_token(token_type.clone());
    }

    fn is_alpha(self: &Self, c: char) -> bool {
        return c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z' || c == '_';
    }

    fn is_alpha_numeric(self: &Self, c: char) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }

    fn number(self: &mut Self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();
            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        let double_substring = self.source[self.start..self.current].to_string();
        self.add_token_literal(
            TokenType::Number,
            Some(LiteralValue::FloatValue(
                double_substring.parse::<f64>().unwrap(),
            )),
        );
    }

    fn string(self: &mut Self) {
        while (self.peek() != '"') && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            println!("LINE {}: Unterminated string.", self.line);
            return;
        }

        self.advance();
        let value_string = self.source[self.start + 1..self.current - 1].to_string();

        self.add_token_literal(
            TokenType::String,
            Some(LiteralValue::StringValue(value_string)),
        );
    }

    fn peek(self: &Self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap()
    }

    fn peek_next(self: &Self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source.chars().nth(self.current + 1).unwrap()
    }

    fn match_char(self: &mut Self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }

        self.current += 1;

        return true;
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        c
    }

    fn is_at_end(self: &Self) -> bool {
        self.current >= self.source.len()
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_literal(token_type, None);
    }

    fn add_token_literal(&mut self, token_type: TokenType, literal: Option<LiteralValue>) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens.push(Token {
            token_type,
            lexeme: text,
            literal,
            line_number: self.line,
        });
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Single-char
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two char
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String,
    Number,

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF,
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub enum LiteralValue {
    IntValue(i64),
    FloatValue(f64),
    StringValue(String),
    IdentifierValue(String),
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<LiteralValue>,
    pub line_number: usize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<LiteralValue>,
        line_number: usize,
    ) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line_number,
        }
    }

    pub fn to_string(self: &Self) -> String {
        format!("{} {} {:?}", self.token_type, self.lexeme, self.literal)
    }
}
