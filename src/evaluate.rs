use crate::expression::ExpressionToken;

pub fn evaluate_postfix(expressions: &[ExpressionToken]) -> Result<f64, String> {
    let mut stack: Vec<f64> = Vec::new();

    for token in expressions {
        match token {
            ExpressionToken::Operand(x) => match x.parse::<f64>() {
                Ok(x) => stack.push(x),
                Err(_) => return Err("Invalid operand".to_string()),
            },
            ExpressionToken::Operator(op) => {
                if stack.len() < 2 {
                    return Err("Not enough operand".to_string());
                }

                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();

                match op.compute(left, right) {
                    Ok(x) => stack.push(x),
                    Err(e) => return Err(e),
                }
            }
            x => return Err(format!("Invalid token: {}", x)),
        }
    }

    if stack.len() != 1 {
        return Err("Invalid expressions".to_string());
    }

    Ok(stack[0])
}
