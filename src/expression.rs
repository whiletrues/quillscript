
use core::fmt;

use crate::token::Token;

#[derive(Clone, PartialEq, Debug)]
pub enum Expression {
    Binary(Box<Expression>, BinaryOperator, Box<Expression>),
    Grouping(Box<Expression>),

    String(String),
    Boolean(bool),
    Number(f64),

    Unary(UnaryOperator, Box<Expression>),
    Variable(Token, Box<Expression>),
    Assign(Token, Box<Expression>),
    Logical(Box<Expression>, LogicalOperator, Box<Expression>),
}


#[derive(Clone, PartialEq, Debug)]
pub enum BinaryOperator {
    Slash,
    Star,
    Plus,
    Minus,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    BangEqual,
    EqualEqual,
}

#[derive(Clone, PartialEq, Debug)]
pub enum UnaryOperator {
    Bang,
    Minus
}

#[derive(Clone, PartialEq, Debug)]
pub enum LogicalOperator {
    And,
    Or,
}


impl fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BinaryOperator::Slash => write!(f, "/"),
            BinaryOperator::Star => write!(f, "*"),
            BinaryOperator::Plus => write!(f, "+"),
            BinaryOperator::Minus => write!(f, "-"),
            BinaryOperator::Greater => write!(f, ">"),
            BinaryOperator::GreaterEqual => write!(f, ">="),
            BinaryOperator::Less => write!(f, "<"),
            BinaryOperator::LessEqual => write!(f, "<="),
            BinaryOperator::BangEqual => write!(f, "=="),
            BinaryOperator::EqualEqual => write!(f, "==")   
        }
    }
}