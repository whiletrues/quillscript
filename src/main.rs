mod expression;
mod parser;
mod scanner;
pub mod token;

use std::io;

use parser::{parse, Parser};
use scanner::Scanner;

use crate::expression::Expression;

fn main() {
    println!("Quillscript REPL (type 'exit' to stop):");

    loop {
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                input = input.trim().to_string(); // Remove whitespace and convert to String
                if input.to_lowercase().trim() == "exit" {
                    break; // Break the loop if the user types 'exit'
                }
                
                io::stdin().read_line(&mut input).unwrap();

                let mut scanner = Scanner::new(input);

                let tokens = scanner.scan();

                let mut parser = Parser {
                    tokens: tokens.clone(),
                    position: 0,
                };

                let expression = parse(&mut parser);
                print_ast(expression);
            }
            Err(error) => {
                println!("Error reading input: {}", error);
                break; // Break the loop on error
            }
        }
    }
}

fn print_ast(expression: Expression) {
    match expression {
        Expression::Grouping(expr) => {
            println!("Grouping (");
            print_ast(*expr);
            println!(")");
        }
        Expression::Binary(left, operator, right) => {
            print_ast(*left);
            println!(" operator {}", operator);
            print_ast(*right);
        }
        Expression::Variable(var) => println!("Variable ( name {})", var),
        Expression::Number(num) => {
            println!("{}", num)
        }
        Expression::Boolean(boolean) => println!("{}", boolean),
        Expression::Logical(left, operator, right) => {
            print_ast(*left);
            println!(" operator {}", operator);
            print_ast(*right);
        }
        Expression::Unary(operator, right) => {
            println!("operator {}", operator);
            print_ast(*right);
        },
        Expression::Assign(variable, right) => {
            println!("assign {}", variable);
            print_ast(*right);
        },
        _ => println!("other"),
    }
}
