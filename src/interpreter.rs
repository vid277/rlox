use crate::{error::RuntimeError, expr::*, TokenType};

fn evaluate(expr: &Expr) -> Result<LiteralValue, RuntimeError> {
    match expr {
        Expr::Literal { value } => Ok(value.clone()),
        Expr::Grouping { expression } => evaluate(expression),
        Expr::Unary { operator, right } => {
            let right_val = evaluate(right)?;
            match operator.token_type {
                TokenType::Bang => Ok(match !is_truthy(right_val) {
                    true => LiteralValue::True,
                    false => LiteralValue::False,
                }),
                TokenType::Minus => match right_val {
                    LiteralValue::Number(n) => Ok(LiteralValue::Number(-n)),
                    _ => Err(RuntimeError::new(
                        "Operand must be a number.",
                        operator.line_number,
                        operator.clone(),
                    )),
                },
                _ => Err(RuntimeError::new(
                    "Invalid unary operator.",
                    operator.line_number,
                    operator.clone(),
                )),
            }
        }
        Expr::Binary {
            left,
            operator,
            right,
        } => {
            let left_val = evaluate(left)?;
            let right_val = evaluate(right)?;

            match operator.token_type {
                TokenType::Minus => match (left_val, right_val) {
                    (LiteralValue::Number(l), LiteralValue::Number(r)) => {
                        Ok(LiteralValue::Number(l - r))
                    }
                    _ => Err(RuntimeError::new(
                        "Operands must be numbers.",
                        operator.line_number,
                        operator.clone(),
                    )),
                },
                TokenType::Slash => match (left_val, right_val) {
                    (LiteralValue::Number(l), LiteralValue::Number(r)) => {
                        Ok(LiteralValue::Number(l / r))
                    }
                    _ => Err(RuntimeError::new(
                        "Operands must be numbers.",
                        operator.line_number,
                        operator.clone(),
                    )),
                },
                TokenType::Star => match (left_val, right_val) {
                    (LiteralValue::Number(l), LiteralValue::Number(r)) => {
                        Ok(LiteralValue::Number(l * r))
                    }
                    _ => Err(RuntimeError::new(
                        "Operands must be numbers.",
                        operator.line_number,
                        operator.clone(),
                    )),
                },
                TokenType::Plus => match (left_val, right_val) {
                    (LiteralValue::Number(l), LiteralValue::Number(r)) => {
                        Ok(LiteralValue::Number(l + r))
                    }
                    (LiteralValue::StringValue(l), LiteralValue::StringValue(r)) => {
                        Ok(LiteralValue::StringValue(format!("{}{}", l, r)))
                    }
                    (LiteralValue::Number(l), LiteralValue::StringValue(r)) => {
                        Ok(LiteralValue::StringValue(format!("{}{}", l, r)))
                    }
                    (LiteralValue::StringValue(l), LiteralValue::Number(r)) => {
                        Ok(LiteralValue::StringValue(format!("{}{}", l, r)))
                    }
                    _ => Err(RuntimeError::new(
                        "Operands must be two numbers or two strings.",
                        operator.line_number,
                        operator.clone(),
                    )),
                },
                TokenType::Greater => match (left_val, right_val) {
                    (LiteralValue::Number(l), LiteralValue::Number(r)) => Ok(if l > r {
                        LiteralValue::True
                    } else {
                        LiteralValue::False
                    }),
                    (LiteralValue::StringValue(l), LiteralValue::StringValue(r)) => Ok(if l > r {
                        LiteralValue::True
                    } else {
                        LiteralValue::False
                    }),
                    _ => Err(RuntimeError::new(
                        "Operands must be numbers or strings.",
                        operator.line_number,
                        operator.clone(),
                    )),
                },
                TokenType::GreaterEqual => match (left_val, right_val) {
                    (LiteralValue::Number(l), LiteralValue::Number(r)) => Ok(if l >= r {
                        LiteralValue::True
                    } else {
                        LiteralValue::False
                    }),
                    (LiteralValue::StringValue(l), LiteralValue::StringValue(r)) => Ok(if l >= r {
                        LiteralValue::True
                    } else {
                        LiteralValue::False
                    }),
                    _ => Err(RuntimeError::new(
                        "Operands must be numbers or strings.",
                        operator.line_number,
                        operator.clone(),
                    )),
                },
                TokenType::Less => match (left_val, right_val) {
                    (LiteralValue::Number(l), LiteralValue::Number(r)) => Ok(if l < r {
                        LiteralValue::True
                    } else {
                        LiteralValue::False
                    }),
                    (LiteralValue::StringValue(l), LiteralValue::StringValue(r)) => Ok(if l < r {
                        LiteralValue::True
                    } else {
                        LiteralValue::False
                    }),
                    _ => Err(RuntimeError::new(
                        "Operands must be numbers or strings.",
                        operator.line_number,
                        operator.clone(),
                    )),
                },
                TokenType::LessEqual => match (left_val, right_val) {
                    (LiteralValue::Number(l), LiteralValue::Number(r)) => Ok(if l <= r {
                        LiteralValue::True
                    } else {
                        LiteralValue::False
                    }),
                    (LiteralValue::StringValue(l), LiteralValue::StringValue(r)) => Ok(if l <= r {
                        LiteralValue::True
                    } else {
                        LiteralValue::False
                    }),
                    _ => Err(RuntimeError::new(
                        "Operands must be numbers or strings.",
                        operator.line_number,
                        operator.clone(),
                    )),
                },
                TokenType::BangEqual => match (left_val, right_val) {
                    (LiteralValue::Number(l), LiteralValue::Number(r)) => Ok(if l != r {
                        LiteralValue::True
                    } else {
                        LiteralValue::False
                    }),
                    (LiteralValue::StringValue(l), LiteralValue::StringValue(r)) => Ok(if l != r {
                        LiteralValue::True
                    } else {
                        LiteralValue::False
                    }),
                    (LiteralValue::True, LiteralValue::False) => Ok(LiteralValue::True),
                    (LiteralValue::False, LiteralValue::True) => Ok(LiteralValue::True),
                    (LiteralValue::True, LiteralValue::True) => Ok(LiteralValue::False),
                    (LiteralValue::False, LiteralValue::False) => Ok(LiteralValue::False),

                    (LiteralValue::True, LiteralValue::Nil) => Ok(LiteralValue::True),
                    (LiteralValue::False, LiteralValue::Nil) => Ok(LiteralValue::True),
                    (LiteralValue::Nil, LiteralValue::True) => Ok(LiteralValue::True),
                    (LiteralValue::Nil, LiteralValue::False) => Ok(LiteralValue::True),

                    (LiteralValue::Nil, LiteralValue::Nil) => Ok(LiteralValue::False),

                    _ => Err(RuntimeError::new(
                        "Operands must be numbers, strings, or booleans.",
                        operator.line_number,
                        operator.clone(),
                    )),
                },
                TokenType::EqualEqual => match (left_val, right_val) {
                    (LiteralValue::Number(l), LiteralValue::Number(r)) => Ok(if l == r {
                        LiteralValue::True
                    } else {
                        LiteralValue::False
                    }),
                    (LiteralValue::StringValue(l), LiteralValue::StringValue(r)) => Ok(if l == r {
                        LiteralValue::True
                    } else {
                        LiteralValue::False
                    }),

                    (LiteralValue::True, LiteralValue::True) => Ok(LiteralValue::True),
                    (LiteralValue::False, LiteralValue::False) => Ok(LiteralValue::True),
                    (LiteralValue::True, LiteralValue::False) => Ok(LiteralValue::False),
                    (LiteralValue::False, LiteralValue::True) => Ok(LiteralValue::False),

                    (LiteralValue::True, LiteralValue::Nil) => Ok(LiteralValue::False),
                    (LiteralValue::False, LiteralValue::Nil) => Ok(LiteralValue::False),
                    (LiteralValue::Nil, LiteralValue::True) => Ok(LiteralValue::False),
                    (LiteralValue::Nil, LiteralValue::False) => Ok(LiteralValue::False),
                    
                    (LiteralValue::Nil, LiteralValue::Nil) => Ok(LiteralValue::True),
                    _ => Err(RuntimeError::new(
                        "Operands must be numbers, strings, or booleans.",
                        operator.line_number,
                        operator.clone(),
                    )),
                },
                _ => Err(RuntimeError::new(
                    "Invalid binary operator.",
                    operator.line_number,
                    operator.clone(),
                )),
            }
        }
    }
}

fn is_truthy(value: LiteralValue) -> bool {
    match value {
        LiteralValue::False => false,
        LiteralValue::Nil => false,
        _ => true,
    }
}

pub fn interpret(expr: &Expr) -> Result<String, RuntimeError> {
    let result = evaluate(expr)?;
    Ok(stringify(result))
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
