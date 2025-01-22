use crate::{error::RuntimeError, expr::*, Token, TokenType};

fn visit_literal_expr(expr: &Expr) -> LiteralValue {
    match expr {
        Expr::Literal { value } => value.clone(),
        _ => panic!("Expected literal expression"),
    }
}

fn visit_grouping_expr(expr: &Expr) -> LiteralValue {
    match expr {
        Expr::Grouping { expression } => evaluate(expression),
        _ => panic!("Expected grouping expression"),
    }
}

fn evaluate(expr: &Expr) -> LiteralValue {
    match expr {
        Expr::Literal { value } => visit_literal_expr(expr),
        Expr::Grouping { expression } => visit_grouping_expr(expr),
        Expr::Unary { operator, right } => visit_unary_expr(expr),
        Expr::Binary {
            left,
            operator,
            right,
        } => visit_binary_expr(expr),
    }
}

fn visit_unary_expr(expr: &Expr) -> LiteralValue {
    match expr {
        Expr::Unary { operator, right } => {
            let right_val = evaluate(right);

            match operator.token_type {
                TokenType::Bang => match !is_truthy(right_val) {
                    true => LiteralValue::True,
                    false => LiteralValue::False,
                },
                TokenType::Minus => match right_val {
                    LiteralValue::Number(n) => LiteralValue::Number(-n),
                    _ => panic!("Expected number for unary minus"),
                },
                _ => panic!("Unhandled unary operator"),
            }
        }
        _ => panic!("Expected unary expression"),
    }
}

fn is_truthy(value: LiteralValue) -> bool {
    match value {
        LiteralValue::False => false,
        LiteralValue::Nil => false,
        _ => true,
    }
}

fn visit_binary_expr(expr: &Expr) -> LiteralValue {
    match expr {
        Expr::Binary {
            left,
            operator,
            right,
        } => {
            let left_val = evaluate(left);
            let right_val = evaluate(right);

            match operator.token_type {
                TokenType::Minus => match (left_val, right_val) {
                    (LiteralValue::Number(l), LiteralValue::Number(r)) => {
                        LiteralValue::Number(l - r)
                    }
                    _ => panic!("Operands must be numbers"),
                },
                TokenType::Slash => match (left_val, right_val) {
                    (LiteralValue::Number(l), LiteralValue::Number(r)) => {
                        LiteralValue::Number(l / r)
                    }
                    _ => panic!("Operands must be numbers"),
                },
                TokenType::Star => match (left_val, right_val) {
                    (LiteralValue::Number(l), LiteralValue::Number(r)) => {
                        LiteralValue::Number(l * r)
                    }
                    _ => panic!("Operands must be numbers"),
                },
                TokenType::Plus => match (left_val, right_val) {
                    (LiteralValue::Number(l), LiteralValue::Number(r)) => {
                        LiteralValue::Number(l + r)
                    }
                    (LiteralValue::StringValue(l), LiteralValue::StringValue(r)) => {
                        LiteralValue::StringValue(format!("{}{}", l, r))
                    }
                    _ => panic!("Operands must be two numbers or two strings"),
                },
                TokenType::Greater => match (left_val, right_val) {
                    (LiteralValue::Number(l), LiteralValue::Number(r)) => {
                        if l > r {
                            LiteralValue::True
                        } else {
                            LiteralValue::False
                        }
                    }
                    _ => panic!("Operands must be numbers"),
                },
                TokenType::GreaterEqual => match (left_val, right_val) {
                    (LiteralValue::Number(l), LiteralValue::Number(r)) => {
                        if l >= r {
                            LiteralValue::True
                        } else {
                            LiteralValue::False
                        }
                    }
                    _ => panic!("Operands must be numbers"),
                },
                TokenType::Less => match (left_val, right_val) {
                    (LiteralValue::Number(l), LiteralValue::Number(r)) => {
                        if l < r {
                            LiteralValue::True
                        } else {
                            LiteralValue::False
                        }
                    }
                    _ => panic!("Operands must be numbers"),
                },
                TokenType::LessEqual => match (left_val, right_val) {
                    (LiteralValue::Number(l), LiteralValue::Number(r)) => {
                        if l <= r {
                            LiteralValue::True
                        } else {
                            LiteralValue::False
                        }
                    }
                    _ => panic!("Operands must be numbers"),
                },
                TokenType::BangEqual => match (left_val, right_val) {
                    (LiteralValue::Number(l), LiteralValue::Number(r)) => {
                        if l != r {
                            LiteralValue::True
                        } else {
                            LiteralValue::False
                        }
                    }
                    _ => panic!("Operands must be numbers"),
                },
                TokenType::EqualEqual => match (left_val, right_val) {
                    (LiteralValue::Number(l), LiteralValue::Number(r)) => {
                        if l == r {
                            LiteralValue::True
                        } else {
                            LiteralValue::False
                        }
                    }
                    _ => panic!("Operands must be numbers"),
                },
                _ => panic!("Unhandled binary operator"),
            }
        }
        _ => panic!("Expected binary expression"),
    }
}

pub fn interpret(expr: &Expr) -> String {
    let result = evaluate(expr);
    stringify(result)
}

fn stringify(value: LiteralValue) -> String {
    match value {
        LiteralValue::Number(n) => n.to_string().replace(".0", ""),
        LiteralValue::StringValue(s) => s,
        LiteralValue::True => "true".to_string(),
        LiteralValue::False => "false".to_string(),
        LiteralValue::Nil => "nil".to_string(),
    }
}
