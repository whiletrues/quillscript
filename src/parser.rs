use core::panic;

use crate::{
    expression::{BinaryOperator, Expression, UnaryOperator, LogicalOperator},
    token::Token,
};

#[derive(PartialEq, PartialOrd)]
pub enum Precedence {
    None,
    Assign,
    Or,
    And,
    Equality,
    Comparison,
    Term,
    Factor,
    Unary,
    Call
}

pub struct Parser {
    pub tokens: Vec<Token>,
    pub position: usize,
}

impl Parser {
    pub fn next(&mut self) -> &Token {
        let token = self.tokens.get(self.position).unwrap_or(&Token::Eof);
        self.position = self.position + 1;
        return token;
    }

    pub fn peek(&mut self) -> &Token {
        return self.tokens.get(self.position).unwrap_or(&Token::Eof);
    }
}

pub fn parse(parser: &mut Parser) -> Expression {
    return parse_expr(parser, Precedence::None);
}

fn parse_expr(parser: &mut Parser, precedence: Precedence) -> Expression {

    let mut expr: Option<Expression> = None;

    if is_prefix(parser) {
        expr = parse_prefix(parser)
    }

    while parser.peek() != &Token::Eof  {
        if precedence >= get_precedence(parser.peek()) {
            break;
        }

        if is_infix(parser) {
            expr = Some(parse_infix(parser, expr.unwrap()))
        }
    }

    return match expr {
        Some(expr) => expr,
        None => panic!("wtf"),
    };
}

fn is_infix(parser: &mut Parser) -> bool {
    match parser.peek() {
        Token::Plus
        | Token::Minus
        | Token::Slash
        | Token::Star
        | Token::Bang
        | Token::BangEqual
        | Token::Equal
        | Token::EqualEqual
        | Token::Greater
        | Token::GreaterEqual
        | Token::Less
        | Token::LessEqual
        | Token::And
        | Token::Or => true,
        _ => false
    }
}

fn parse_infix(parser: &mut Parser, left: Expression) -> Expression {
    match parser.peek() {
        Token::Plus
        | Token::Minus
        | Token::Slash
        | Token::Star
        | Token::Bang
        | Token::BangEqual
        | Token::EqualEqual
        | Token::Greater
        | Token::GreaterEqual
        | Token::Less
        | Token::LessEqual => parse_binary(parser, left),
        Token::Equal => parse_assignment(parser, left),
        Token::And | Token::Or => parse_logical(parser, left),
        _ => panic!("unknow infix token")
    }
}

fn parse_logical(parser: &mut Parser, left: Expression) -> Expression {
    let precedence = get_precedence(parser.peek());
    let operator = get_logical_operator(parser);
    let right = parse_expr(parser, precedence);
    return Expression::Logical(Box::new(left), operator, Box::new(right));
}

fn parse_assignment(parser: &mut Parser, left: Expression) -> Expression {

    if parser.next() != &Token::Equal {
        panic!("expect equal for assignment")
    }

    let right = parse_expr(parser, Precedence::None);
    match left {
        Expression::Variable(identifier) => Expression::Assign(identifier, Box::new(right)),
        _ => panic!("unknow type a assignment")
    }
}

fn is_prefix(parser: &mut Parser) -> bool {
    match parser.peek() {
        Token::String(_)
        | Token::Number(_)
        | Token::True
        | Token::False  
        | Token::Minus
        | Token::Bang
        | Token::Identifier(_)
        | Token::LeftParen => true,
        _ => false,    
    }
}

fn parse_prefix(parser: &mut Parser) -> Option<Expression> {
    match parser.peek() {
        Token::String(_)
        | Token::Number(_)
        | Token::True
        | Token::Identifier(_)
        | Token::False => Some(parse_primary(parser)),
        | Token::Minus | Token::Bang => Some(parse_unary(parser)),
        | Token::LeftParen => Some(parse_grouping(parser)),
        _ => None,
    }
}

fn parse_grouping(parser: &mut Parser) -> Expression {
    let grouping = match parser.next() {
        Token::LeftParen => {
            let expr = parse_expr(parser, Precedence::None);
            if parser.next().clone() != Token::RightParen {
                panic!()
            }
            expr
        }
        _ => panic!(""),
    };

    return Expression::Grouping(Box::new(grouping));
}

fn parse_binary(parser: &mut Parser, left: Expression) -> Expression {
    let precedence = get_precedence(parser.peek());
    let operator = get_binary_operator(parser);
    let right = parse_expr(parser, precedence);
    Expression::Binary(Box::new(left), operator, Box::new(right))
}

fn parse_unary(parser: &mut Parser) -> Expression {
    let operator = get_unary_operator(parser);
    let right = parse_expr(parser, Precedence::Unary);
    Expression::Unary(operator, Box::new(right))
}

fn parse_primary(parser: &mut Parser) -> Expression {
    return match parser.next() {
        Token::Number(number) => Expression::Number(number.clone()),
        Token::String(string) => Expression::String(string.clone()),
        Token::True => Expression::Boolean(true),
        Token::False => Expression::Boolean(false),
        Token::Identifier(identifier) => Expression::Variable(identifier.clone()),
        _ => panic!(),
    };
}

fn get_precedence(token: &Token) -> Precedence {
    return match token {
        Token::Equal => Precedence::Assign,

        Token::EqualEqual => Precedence::Equality,
        Token::BangEqual => Precedence::Equality,

        Token::Less => Precedence::Comparison,
        Token::LessEqual => Precedence::Comparison,
        Token::Greater => Precedence::Comparison,
        Token::GreaterEqual => Precedence::Comparison,

        Token::Minus => Precedence::Term,
        Token::Plus => Precedence::Term,

        Token::Star => Precedence::Factor,
        Token::Slash => Precedence::Factor,

        Token::Bang => Precedence::Unary,
        Token::LeftParen => Precedence::Call,

        Token::And => Precedence::And,
        Token::Or => Precedence::Or,
        _ => Precedence::None,
    };
}

fn get_logical_operator(parser: &mut Parser) -> LogicalOperator {
    return match parser.next() {
        Token::And => LogicalOperator::And,
        Token::Or => LogicalOperator::Or,
        _ => todo!(),
    }
}

fn get_unary_operator(parser: &mut Parser) -> UnaryOperator {
    return match parser.next() {
        Token::Minus => UnaryOperator::Minus,
        Token::Bang => UnaryOperator::Bang,
        _ => todo!()
    }
}

fn get_binary_operator(parser: &mut Parser) -> BinaryOperator {
    return match parser.next() {
        Token::BangEqual => BinaryOperator::BangEqual,
        Token::Less => BinaryOperator::Less,
        Token::LessEqual => BinaryOperator::LessEqual,
        Token::Greater => BinaryOperator::Greater,
        Token::GreaterEqual => BinaryOperator::GreaterEqual,
        Token::Minus => BinaryOperator::Minus,
        Token::Plus => BinaryOperator::Plus,
        Token::Star => BinaryOperator::Star,
        Token::Slash => BinaryOperator::Slash,
        Token::EqualEqual => BinaryOperator::EqualEqual,
        _ => todo!(),
    };
}

#[cfg(test)]
mod tests {
    use super::{Parser, Token, parse};

    #[test]
    pub fn test_parse_grouping() {
        let tokens: Vec<Token> = vec![
            Token::LeftParen,
            Token::Number(10.3),
            Token::Plus,
            Token::Number(23.0),
            Token::RightParen,
        ];

        let mut parser = Parser {
            tokens: tokens.clone(),
            position: 0
        };

        let expr = parse(&mut parser);
    }
}
