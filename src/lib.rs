use std::collections::VecDeque;
use thiserror::Error;

mod calc;
mod yard;
type OperandType = i32;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("devide-by-zero")]
    DivideByZero,
    #[error("invalid token {}", .0)]
    InvalidToken(String),
    #[error("invalid expression")]
    InvalidExpr,
    #[error("invalid number")]
    InvalidNumber,
}

pub fn try_eval(expr: &str) -> Result<OperandType, Error> {
    let rpn = yard::to_rpn(expr)?;
    let mut stack = VecDeque::new();
    for token in rpn {
        calc::eval(&mut stack, token)?;
    }

    if stack.len() != 1 {
        return Err(Error::InvalidExpr);
    }

    Ok(stack[0])
}

