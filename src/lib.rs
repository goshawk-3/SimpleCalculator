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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_eval() {
        let table_tests = [
            ("(4 + 8) / (1 + 3)", 3),
            ("3211 - 22 * 3 + 999", 4144),
            ("(6 + 2 * 5) / (-2 + 3 * 2)", 4),
            ("6343 * 3234", 20513262),
            ("-2 * 4", -8),
            ("-2 *      4   + 3", -5),
        ];

        for test in table_tests.iter() {
            assert_eq!(try_eval(test.0).unwrap(), test.1);
        }
    }

    #[test]
    fn test_malformed_expr() {
        assert_eq!(try_eval("*32"), Err(Error::InvalidExpr));
        assert_eq!(try_eval("12**32"), Err(Error::InvalidExpr));
        assert_eq!(try_eval("3*2-"), Err(Error::InvalidExpr));
        assert_eq!(try_eval("10/0"), Err(Error::DivideByZero));
        assert_eq!(try_eval("10/(1-1)"), Err(Error::DivideByZero));
        assert_eq!(
            try_eval("12-2*1-z32"),
            Err(Error::InvalidToken("z".to_string()))
        );
    }
}
