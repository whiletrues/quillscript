use std::iter::Peekable;

use crate::{expression::Expression, token::Token};

#[derive(PartialEq, PartialOrd)]
pub enum Precedence {
    None,
    Equality,
    Comparison,
    Term,
    Factor,
    Unary,
    Primary,
}

pub struct Parser<'a> {
    pub token_iterator: Peekable<std::slice::Iter<'a, Token>>,
}

impl<'a> Parser<'a> {
    pub fn parse(&mut self) -> Expression {
        return self.expression();
    }
    
    fn expression(&mut self) -> Expression {
        let mut expr = self.comparison();

        let current = self.token_iterator.peek().unwrap();

        if current == &&Token::BangEqual || current == &&Token::EqualEqual {
            self.token_iterator.next();

            let operator = self.token_iterator.nth_back(1).unwrap();
            let right = self.comparison();
            expr = Expression::Binary(Box::new(expr), operator.clone(), Box::new(right))
        }

        return expr;
    }

    fn comparison(&mut self) -> Expression {
        let mut expr = self.term();

        let current = self.token_iterator.peek().unwrap();

        if current == &&Token::Greater
            || current == &&Token::GreaterEqual
            || current == &&Token::Less
            || current == &&Token::LessEqual
        {
            self.token_iterator.next();

            let operator = self.token_iterator.nth_back(1).unwrap();
            let right = self.term();
            expr = Expression::Binary(Box::new(expr), operator.clone(), Box::new(right))
        }

        return expr;
    }

    fn term(&mut self) -> Expression {
        let mut expr = self.factor();

        let current: &&Token = self.token_iterator.peek().unwrap();

        if current == &&Token::Minus || current == &&Token::Plus {
            self.token_iterator.next();

            let operator = self.token_iterator.nth_back(1).unwrap();
            let right = self.factor();
            expr = Expression::Binary(Box::new(expr), operator.clone(), Box::new(right))
        }

        return expr;
    }

    fn factor(&mut self) -> Expression {
        let mut expr = self.unary();

        let current: &&Token = self.token_iterator.peek().unwrap();

        if current == &&Token::Slash || current == &&Token::Star {
            self.token_iterator.next();

            let operator = self.token_iterator.nth_back(1).unwrap();
            let right = self.unary();
            expr = Expression::Binary(Box::new(expr), operator.clone(), Box::new(right))
        }

        return expr;
    }

    fn unary(&mut self) -> Expression {
        let current: &&Token = self.token_iterator.peek().unwrap();

        if current == &&Token::Bang || current == &&Token::Minus {
            self.token_iterator.next();

            let operator = self.token_iterator.nth_back(1).unwrap();
            let right = self.unary();
            return Expression::Unary(operator.clone(), Box::new(right));
        }

        return self.primary();
    }

    fn primary(&mut self) -> Expression {
        let current = self.token_iterator.peek().unwrap();

        let expr = match current.clone() {
            Token::Number(n) => Expression::Number(*n),
            Token::String(str) => Expression::String(str.clone()),
            _ => todo!(),
        };
        
        self.token_iterator.next();

        return expr;
    }
}
