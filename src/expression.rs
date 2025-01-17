use std::fmt::Display;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
}

impl Operator {
    pub fn precedence(&self) -> i32 {
        match self {
            Operator::Plus | Operator::Minus => 1,
            Operator::Multiply | Operator::Divide => 2,
        }
    }

    pub fn compute(&self, left: f64, right: f64) -> Result<f64, String> {
        match self {
            Operator::Plus => Ok(left + right),
            Operator::Minus => Ok(left - right),
            Operator::Multiply => Ok(left * right),
            Operator::Divide => {
                if right == 0.0 {
                    return Err("Divide by 0".to_string());
                }
                Ok(left / right)
            }
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op = match self {
            Operator::Plus => "+",
            Operator::Minus => "-",
            Operator::Multiply => "*",
            Operator::Divide => "/",
        };

        write!(f, "{}", op)
    }
}

#[derive(PartialEq, Debug)]
pub enum ExpressionToken {
    Operand(String),
    Operator(Operator),
    OpenParen,
    CloseParen,
}

impl Display for ExpressionToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ex = match self {
            ExpressionToken::Operand(x) => x.clone(),
            ExpressionToken::Operator(op) => op.to_string(),
            ExpressionToken::OpenParen => "(".to_string(),
            ExpressionToken::CloseParen => ")".to_string(),
        };

        write!(f, "{}", ex)
    }
}

pub fn possible_next_expressions(expressions: &[ExpressionToken]) -> Vec<ExpressionToken> {
    let mut possible_next = Vec::new();

    if expressions.is_empty() {
        // If the expression is empty, the only valid start is an operand or open parenthesis
        possible_next.push(ExpressionToken::Operand("".to_string())); // Provide an empty string as a placeholder
        possible_next.push(ExpressionToken::OpenParen);
        return possible_next;
    }

    match expressions.last().unwrap() {
        ExpressionToken::Operand(_) => {
            // After an operand, an operator or close parenthesis is expected
            possible_next.push(ExpressionToken::Operator(Operator::Plus)); // Placeholder operator

            //Check if there are open parentheses to close
            let open_paren_count = expressions
                .iter()
                .filter(|&x| *x == ExpressionToken::OpenParen)
                .count();
            let close_paren_count = expressions
                .iter()
                .filter(|&x| *x == ExpressionToken::CloseParen)
                .count();

            if open_paren_count > close_paren_count {
                possible_next.push(ExpressionToken::CloseParen);
            }
        }
        ExpressionToken::Operator(_) | ExpressionToken::OpenParen => {
            // After an operator, an operand or open parenthesis is expected
            possible_next.push(ExpressionToken::Operand("".to_string())); // Placeholder operand
            possible_next.push(ExpressionToken::OpenParen);
        }
        ExpressionToken::CloseParen => {
            // After a close parenthesis, an operator or close parenthesis is expected
            possible_next.push(ExpressionToken::Operator(Operator::Plus)); // Placeholder operator
                                                                           //Check if there are open parentheses to close
            let open_paren_count = expressions
                .iter()
                .filter(|&x| *x == ExpressionToken::OpenParen)
                .count();
            let close_paren_count = expressions
                .iter()
                .filter(|&x| *x == ExpressionToken::CloseParen)
                .count();

            if open_paren_count > close_paren_count {
                possible_next.push(ExpressionToken::CloseParen);
            }
        }
    }

    possible_next
}

enum OperatorStackToken {
    Operator(Operator),
    OpenParen,
}

// Convert infix to postfix using  Shunting Yard Algorithm
pub fn infix_to_postfix(expressions: &[ExpressionToken]) -> Result<Vec<ExpressionToken>, String> {
    let mut output: Vec<ExpressionToken> = Vec::new();
    let mut operator_stack: Vec<OperatorStackToken> = Vec::new();

    for token in expressions {
        match token {
            ExpressionToken::Operand(x) => output.push(ExpressionToken::Operand(x.clone())),
            ExpressionToken::Operator(op) => {
                while let Some(top) = operator_stack.last() {
                    match top {
                        OperatorStackToken::Operator(top_op) => {
                            let top_precedence = top_op.precedence();
                            let current_precedence = op.precedence();
                            if top_precedence >= current_precedence {
                                output.push(ExpressionToken::Operator(*top_op));
                                operator_stack.pop();
                            } else {
                                break;
                            }
                        }
                        OperatorStackToken::OpenParen => break,
                    }
                }
                operator_stack.push(OperatorStackToken::Operator(*op));
            }
            ExpressionToken::OpenParen => operator_stack.push(OperatorStackToken::OpenParen),
            ExpressionToken::CloseParen => {
                let mut found_open = false;
                while let Some(op) = operator_stack.pop() {
                    match op {
                        OperatorStackToken::Operator(op) => {
                            output.push(ExpressionToken::Operator(op))
                        }
                        OperatorStackToken::OpenParen => {
                            found_open = true;
                            break;
                        }
                    }
                }

                if !found_open {
                    return Err("Unmatched Parentheses".to_string());
                }
            }
        }
    }

    while let Some(token) = operator_stack.pop() {
        match token {
            OperatorStackToken::Operator(op) => output.push(ExpressionToken::Operator(op)),
            OperatorStackToken::OpenParen => return Err("Unmatched Parentheses".to_string()),
        }
    }

    Ok(output)
}
