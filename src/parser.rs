use core::panic;

use crate::{
    expression::{BinaryOperator, Expression, UnaryOperator, LogicalOperator},
    Kind,
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
    pub Kinds: Vec<Kind>,
    pub position: usize,
}

impl Parser {
    pub fn next(&mut self) -> &Kind {
        let Kind = self.Kinds.get(self.position).unwrap_or(&Kind::Eof);
        self.position = self.position + 1;
        return Kind;
    }

    pub fn peek(&mut self) -> &Kind {
        return self.Kinds.get(self.position).unwrap_or(&Kind::Eof);
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

    while parser.peek() != &Kind::Eof  {
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
        Kind::Plus
        | Kind::Minus
        | Kind::Slash
        | Kind::Star
        | Kind::Bang
        | Kind::BangEqual
        | Kind::Equal
        | Kind::EqualEqual
        | Kind::Greater
        | Kind::GreaterEqual
        | Kind::Less
        | Kind::LessEqual
        | Kind::And
        | Kind::Or => true,
        _ => false
    }
}

fn parse_infix(parser: &mut Parser, left: Expression) -> Expression {
    match parser.peek() {
        Kind::Plus
        | Kind::Minus
        | Kind::Slash
        | Kind::Star
        | Kind::Bang
        | Kind::BangEqual
        | Kind::EqualEqual
        | Kind::Greater
        | Kind::GreaterEqual
        | Kind::Less
        | Kind::LessEqual => parse_binary(parser, left),
        Kind::Equal => parse_assignment(parser, left),
        Kind::And | Kind::Or => parse_logical(parser, left),
        _ => panic!("unknow infix Kind")
    }
}

fn parse_logical(parser: &mut Parser, left: Expression) -> Expression {
    let precedence = get_precedence(parser.peek());
    let operator = get_logical_operator(parser);
    let right = parse_expr(parser, precedence);
    return Expression::Logical(Box::new(left), operator, Box::new(right));
}

fn parse_assignment(parser: &mut Parser, left: Expression) -> Expression {

    if parser.next() != &Kind::Equal {
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
        Kind::String(_)
        | Kind::Number(_)
        | Kind::True
        | Kind::False  
        | Kind::Minus
        | Kind::Bang
        | Kind::Identifier(_)
        | Kind::LeftParen => true,
        _ => false,    
    }
}

fn parse_prefix(parser: &mut Parser) -> Option<Expression> {
    match parser.peek() {
        Kind::String(_)
        | Kind::Number(_)
        | Kind::True
        | Kind::Identifier(_)
        | Kind::False => Some(parse_primary(parser)),
        | Kind::Minus | Kind::Bang => Some(parse_unary(parser)),
        | Kind::LeftParen => Some(parse_grouping(parser)),
        _ => None,
    }
}

fn parse_grouping(parser: &mut Parser) -> Expression {
    let grouping = match parser.next() {
        Kind::LeftParen => {
            let expr = parse_expr(parser, Precedence::None);
            if parser.next().clone() != Kind::RightParen {
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
        Kind::Number(number) => Expression::Number(number.clone()),
        Kind::String(string) => Expression::String(string.clone()),
        Kind::True => Expression::Boolean(true),
        Kind::False => Expression::Boolean(false),
        Kind::Identifier(identifier) => Expression::Variable(identifier.clone()),
        _ => panic!(),
    };
}

fn get_precedence(Kind: &Kind) -> Precedence {
    return match Kind {
        Kind::Equal => Precedence::Assign,

        Kind::EqualEqual => Precedence::Equality,
        Kind::BangEqual => Precedence::Equality,

        Kind::Less => Precedence::Comparison,
        Kind::LessEqual => Precedence::Comparison,
        Kind::Greater => Precedence::Comparison,
        Kind::GreaterEqual => Precedence::Comparison,

        Kind::Minus => Precedence::Term,
        Kind::Plus => Precedence::Term,

        Kind::Star => Precedence::Factor,
        Kind::Slash => Precedence::Factor,

        Kind::Bang => Precedence::Unary,
        Kind::LeftParen => Precedence::Call,

        Kind::And => Precedence::And,
        Kind::Or => Precedence::Or,
        _ => Precedence::None,
    };
}

fn get_logical_operator(parser: &mut Parser) -> LogicalOperator {
    return match parser.next() {
        Kind::And => LogicalOperator::And,
        Kind::Or => LogicalOperator::Or,
        _ => todo!(),
    }
}

fn get_unary_operator(parser: &mut Parser) -> UnaryOperator {
    return match parser.next() {
        Kind::Minus => UnaryOperator::Minus,
        Kind::Bang => UnaryOperator::Bang,
        _ => todo!()
    }
}

fn get_binary_operator(parser: &mut Parser) -> BinaryOperator {
    return match parser.next() {
        Kind::BangEqual => BinaryOperator::BangEqual,
        Kind::Less => BinaryOperator::Less,
        Kind::LessEqual => BinaryOperator::LessEqual,
        Kind::Greater => BinaryOperator::Greater,
        Kind::GreaterEqual => BinaryOperator::GreaterEqual,
        Kind::Minus => BinaryOperator::Minus,
        Kind::Plus => BinaryOperator::Plus,
        Kind::Star => BinaryOperator::Star,
        Kind::Slash => BinaryOperator::Slash,
        Kind::EqualEqual => BinaryOperator::EqualEqual,
        _ => todo!(),
    };
}

#[cfg(test)]
mod tests {
    use super::{Parser, Kind, parse};

    #[test]
    pub fn test_parse_grouping() {
        let Kinds: Vec<Kind> = vec![
            Kind::LeftParen,
            Kind::Number(10.3),
            Kind::Plus,
            Kind::Number(23.0),
            Kind::RightParen,
        ];

        let mut parser = Parser {
            Kinds: Kinds.clone(),
            position: 0
        };

        let expr = parse(&mut parser);
    }
}
