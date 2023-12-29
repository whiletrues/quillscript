use crate::token::Token;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            tokens: Vec::new(),
        }
    }

    pub fn scan(&mut self) -> &Vec<Token> {
        let mut line = 1;

        let mut char_indices = self.source.char_indices().peekable();

        while let Some((position, char)) = char_indices.next() {
            let token = match char {
                '(' => Token::LeftParen,
                ')' => Token::RightParen,
                '{' => Token::LeftBrace,
                '}' => Token::RightBrace,
                ',' => Token::Comma,
                '.' => Token::Dot,
                '-' => Token::Minus,
                '+' => Token::Plus,
                ';' => Token::Semicolon,
                '*' => Token::Star,
                '!' => match char_indices.next_if_eq(&(position + 1, '=')) {
                    Some(_) => Token::BangEqual,
                    None => Token::Bang,
                },
                '=' => match char_indices.next_if_eq(&(position + 1, '=')) {
                    Some(_) => Token::EqualEqual,
                    None => Token::Equal,
                },
                '<' => match char_indices.next_if_eq(&(position + 1, '=')) {
                    Some(_) => Token::LessEqual,
                    None => Token::Less,
                },
                '>' => match char_indices.next_if_eq(&(position + 1, '=')) {
                    Some(_) => Token::GreaterEqual,
                    None => Token::Greater,
                },
                // TODO : parse comment
                '/' => match char_indices.next_if_eq(&(position + 1, '/')) {
                    Some(_) => Token::Slash,
                    None => Token::Slash,
                },
                '"' => {
                    let mut last_matched: char = '\0';
                    let mut internal_position: usize = 0;

                    let string = char_indices
                        .by_ref()
                        .take_while(|(pos, c)| {
                            internal_position = *pos;
                            last_matched = *c;
                            *c != '"'
                        })
                        .map(|(_pos, c)| c)
                        .collect();
                    match last_matched {
                        '"' => Token::String(string),
                        _ => Token::Invalid(
                            line,
                            internal_position as i32,
                            "unterminated string".to_string(),
                        ),
                    }
                }
                ' ' => Token::Space,
                '\n' => {
                    line += 1;
                    Token::Line
                }
                _ => {
                    if self.is_digit(char) {
                        let mut number = String::new();
                        
                        number.push(char);

                        while let Some((position, num)) = char_indices
                            .next_if(|(_pos, c)| self.is_digit(*c) || *c == '.') {                        
                                number.push(num);
                        }

                        Token::Number(number.parse::<f64>().unwrap())
                    } else if self.is_alpha(char) {
                        let mut identifier = String::new();

                        identifier.push(char);

                        let ident: String = char_indices
                            .by_ref()
                            .take_while(|(_pos, c)| self.is_alpha_numeric(*c))
                            .map(|(_pos, c)| c)
                            .collect();

                        identifier.push_str(&ident);

                        match identifier.as_str() {
                            "true" => Token::True,
                            "false" => Token::False,
                            "and" => Token::And,
                            "or" => Token::Or,
                            "if" => Token::If,
                            "else" => Token::Else,
                            "for" => Token::For,
                            "while" => Token::While,
                            "print" => Token::Print,
                            "var" => Token::Var,
                            "func" => Token::Func,
                            "return" => Token::Return,
                            "class" => Token::Class,
                            "super" => Token::Super,
                            "nil" => Token::Nil,
                            _ => Token::Identifier(identifier),
                        }
                    } else {
                        Token::Invalid(line, position as i32, "Unrecognized token".to_string())
                    }
                }
            };
            self.tokens.push(token);
        }

        return &self.tokens;
    }

    fn is_alpha_numeric(&self, char: char) -> bool {
        self.is_alpha(char) || self.is_digit(char)
    }

    fn is_digit(&self, char: char) -> bool {
        char >= '0' && char <= '9'
    }

    fn is_alpha(&self, char: char) -> bool {
        char >= 'a' && char <= 'z' || char >= 'A' && char <= 'Z' || char == '_'
    }
}

#[cfg(test)]
mod tests {
    use super::{Scanner, Token};

    #[test]
    fn test_number() {
        let source = String::from("1283293");
        let mut scanner = Scanner::new(source);

        let tokens: &Vec<Token> = scanner.scan();

        let number_token = tokens.first().unwrap();

        assert_eq!(number_token, &Token::Number(1283293 as f64))
    }

    #[test]
    fn test_floating_number() {
        let source = String::from("1283.293");
        let mut scanner = Scanner::new(source);

        let tokens = scanner.scan();

        let number_token = tokens.first().unwrap();

        assert_eq!(number_token, &Token::Number(1283.293 as f64))
    }

    #[test]
    fn test_unterminated_string() {
        let source = String::from("\"the unterminated string ");
        let mut scanner = Scanner::new(source);

        let tokens = scanner.scan();

        let number_token = tokens.first().unwrap();

        assert_eq!(
            number_token,
            &Token::Invalid(1, 24, "unterminated string".to_string())
        )
    }

    #[test]
    fn test_string() {
        let source = String::from("\"a string\"");
        let mut scanner = Scanner::new(source);

        let tokens = scanner.scan();

        let number_token = tokens.first().unwrap();

        assert_eq!(
            number_token,
            &Token::String("a string".to_string())
        )
    }

}
