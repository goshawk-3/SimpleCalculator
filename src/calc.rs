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

