mod scanner;
pub mod token;

use scanner::Scanner;

fn main() {
    
    let input = String::from("if 4554 {");

    let mut scanner = Scanner::new(input.clone());

    let tokens = scanner.scan();

    for token in tokens {
        print!("token {}", token);
    }
}
