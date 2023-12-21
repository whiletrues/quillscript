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

                    let string = char_indices
                        .by_ref()
                        .take_while(|(_pos, c)| {
                            last_matched = *c;
                            *c != '"'
                        })
                        .map(|(_pos, c)| c)
                        .collect();
                    match last_matched {
                        '"' => Token::String(string),
                        _ => Token::Invalid("unterminated string".to_string()),
                    }
                }
                ' ' => Token::Space,
                '\n' => Token::Line,
                _ => Token::Invalid("Unknow token".to_string()),
            };
            self.tokens.push(token);
        }

        return &self.tokens;
    }
}
