use crate::calc::Token;
use crate::{Error, OperandType};

/// Try to convert an expression into Reverse Polish Notation
pub fn to_rpn(expr: &str) -> Result<Vec<Token>, Error> {
    let mut rpn: Vec<Token> = Vec::new();
    let mut stack: Vec<Token> = Vec::new();

    let mut value: String = String::new();
    let mut is_negative = true;
    for t in expr.chars().filter(|c| !c.is_whitespace()) {
        if t.is_numeric() {
            value.push(t);
            is_negative = false;
            continue;
        }

        if t == '-' && is_negative {
            value.push('-');
            is_negative = false;
            continue;
        }

        if !value.is_empty() {
            let token = value
                .parse::<OperandType>()
                .map(Token::Operand)
                .map_err(|_| Error::InvalidNumber)?;

            rpn.push(token);
            value.clear();
        }

        match Token::from_char(t) {
            Some(Token::Lparen) => {
                stack.push(Token::Lparen);
                is_negative = true;
            }
            Some(Token::Rparen) => {
                while let Some(token) = stack.pop() {
                    if token == Token::Lparen {
                        break;
                    }
                    rpn.push(token);
                }
            }
            Some(token) => {
                if rpn.is_empty() && value.is_empty() {
                    return Err(Error::InvalidExpr);
                }

                while let Some(qe) = stack.last() {
                    if token.precedence() <= qe.precedence() {
                        rpn.push(stack.pop().ok_or::<Error>(Error::InvalidExpr)?);
                    } else {
                        break;
                    }
                }
                stack.push(token);
                is_negative = true;
            }
            None => return Err(Error::InvalidToken(t.to_string())),
        };
    }

    if !value.is_empty() {
        let token = value
            .parse::<OperandType>()
            .map(Token::Operand)
            .map_err(|_| Error::InvalidNumber)?;

        rpn.push(token);
    }

    rpn.extend(stack);
    Ok(rpn)
}
