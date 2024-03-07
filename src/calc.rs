use crate::{Error, OperandType};
use std::collections::VecDeque;
use std::fmt::Display;
use std::ops::{Add, Div, Mul, Sub};

type Stack = VecDeque<OperandType>;

#[derive(PartialEq)]
pub(crate) enum Token {
    Add,
    Sub,
    Mul,
    Div,
    Lparen,
    Rparen,
    Operand(OperandType),
}

impl Token {
    pub fn precedence(&self) -> u32 {
        match *self {
            Self::Lparen | Self::Rparen => 0,
            Self::Add | Self::Sub => 1,
            Self::Mul | Self::Div => 2,
            Self::Operand(_) => panic!("invalid expr"),
        }
    }

    pub fn from_char(c: char) -> Option<Token> {
        Some(match c {
            '+' => Self::Add,
            '-' => Self::Sub,
            '*' => Self::Mul,
            '/' => Self::Div,
            '(' => Self::Lparen,
            ')' => Self::Rparen,
            _ => return None,
        })
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Token::Add => "+".to_owned(),
            Token::Sub => "-".to_owned(),
            Token::Mul => "*".to_owned(),
            Token::Div => "/".to_owned(),
            Token::Lparen => "(".to_owned(),
            Token::Rparen => ")".to_owned(),
            Token::Operand(operand) => operand.to_string(),
        };
        write!(f, "{}", str)
    }
}

/// Evaluates a binary operator
fn eval_binary<F>(stack: &mut Stack, ops: F) -> Result<(), Error>
    where
        F: FnOnce(OperandType, OperandType) -> OperandType,
{
    let a = stack.pop_back().ok_or::<Error>(Error::InvalidExpr)?;
    let b = stack.pop_back().ok_or::<Error>(Error::InvalidExpr)?;
    stack.push_back(ops(b, a));
    Ok(())
}


/// Evaluates any of the supported operands or push a number to the stack.
pub(crate) fn eval(stack: &mut Stack, token: Token) -> Result<(), Error> {
    match token {
        Token::Add => eval_binary(stack, OperandType::add),
        Token::Sub => eval_binary(stack, OperandType::sub),
        Token::Mul => eval_binary(stack, OperandType::mul),
        Token::Div => {
            match stack.get(1) {
                None => return Err(Error::InvalidExpr),
                Some(v) => {
                    if *v == 0 {
                        return Err(Error::DivideByZero);
                    }
                }
            }
            eval_binary(stack, OperandType::div)
        }
        Token::Operand(n) => { stack.push_back(n); Ok(())},
        _ => Err(Error::InvalidToken(token.to_string())),
    }
}