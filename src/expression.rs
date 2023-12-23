use crate::token::Token;

pub enum Expression {
    Binary(Box<Expression>, Token, Box<Expression>),
    Grouping(Box<Expression>),

    // Literal
    String(String),
    Boolean(bool),
    Number(f64),

    Unary(Token, Box<Expression>),
    Variable(Token, Box<Expression>),
    Assign(Token, Box<Expression>),
    Logical(Box<Expression>, Token, Box<Expression>)
}