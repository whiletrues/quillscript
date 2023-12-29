mod scanner;
pub mod token;
mod parser;
mod expression;

use parser::{Parser, parse};
use scanner::Scanner;

use crate::expression::Expression;

fn main() {
    
    let input = String::from("(2*3)+(4*2)");

    let mut scanner = Scanner::new(input);

    let tokens = scanner.scan();

    let mut parser = Parser {
        tokens: tokens.clone(),
        position: 0
    };

    let expression = parse(&mut parser);
    printAst(expression);
}


fn printAst(expression: Expression) {
    match expression {
        Expression::Grouping(expr) => {
            print!("(");
            printAst(*expr);
            print!(")");
        },
        Expression::Binary(left, operator, right) => {
            printAst(*left);
            print!("{}", operator);
            printAst(*right);
        },
        Expression::Number(num) => {
            print!("{}",num)
        },
        Expression::Boolean(boolean) => print!("{}", boolean),
        Expression::Unary(_,right) => {
            print!("unary");
            printAst(*right);
        }
        _ => println!("other")
    }
}
