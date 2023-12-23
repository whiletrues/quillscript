mod scanner;
pub mod token;
mod parser;
mod expression;

use parser::Parser;
use scanner::Scanner;

fn main() {
    
    let input = String::from("2 + 4");

    let mut scanner = Scanner::new(input.clone());

    let tokens = scanner.scan();

    let mut parser = Parser {
        token_iterator: tokens.iter().peekable() 
    };

    let expression = parser.parse();
}
