use crate::scanner::Token;

pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: LiteralValue,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}

#[derive(Debug, Clone)]
pub enum LiteralValue {
    Number(f64),
    StringValue(String),
    True,
    False,
    Nil,
}

impl Expr {
    pub fn to_string(&self) -> String {
        match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => format!(
                "({} {} {})",
                operator.lexeme,
                left.to_string(),
                right.to_string()
            ),
            Expr::Grouping { expression } => format!("(group {})", expression.to_string()),
            Expr::Literal { value } => format!("{}", value.to_string()),
            Expr::Unary { operator, right } => {
                format!("({} {})", operator.lexeme, right.to_string())
            }
        }
    }

    pub fn print(&self) {
        println!("{}", self.to_string());
    }
}

impl LiteralValue {
    fn to_string(&self) -> String {
        match self {
            LiteralValue::Number(val) => format!("{}", val),
            LiteralValue::StringValue(val) => val.clone(),
            LiteralValue::True => "true".to_string(),
            LiteralValue::False => "false".to_string(),
            LiteralValue::Nil => "nil".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scanner::TokenType::*;

    #[test]
    fn print_pretty_ast() {
        let minus_token = Token::new(Minus, "-".to_string(), None, 1);
        let star_token = Token::new(Star, "*".to_string(), None, 1);

        let number = Expr::Literal {
            value: LiteralValue::Number(123.0),
        };

        let group = Expr::Grouping {
            expression: Box::new(Expr::Literal {
                value: LiteralValue::Number(45.67),
            }),
        };

        let ast = Expr::Binary {
            left: Box::new(Expr::Unary {
                operator: minus_token,
                right: Box::new(number),
            }),
            operator: star_token,
            right: Box::new(group),
        };

        ast.print();
    }
}
