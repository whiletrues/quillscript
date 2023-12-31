use std::str::Chars;

use crate::token::{Token, TokenKind};


pub struct Lexer<'a> {
    source: &'a String,
    chars: Chars<'a>
}

impl<'a> Lexer<'a> {

    pub fn new(source: &'a String) -> Self {
        Self {
            source,
            chars: source.chars()
        }
    }

    fn read_next_king(&mut self) -> TokenKind {
        while let Some(char) = self.chars.next() {
            return match char {
                '(' => TokenKind::LeftParen,
                ')' => TokenKind::RightParen,
                '{' => TokenKind::LeftBrace,
                '}' => TokenKind::RightBrace,
                ',' => TokenKind::Comma,
                '.' => TokenKind::Dot,
                '-' => TokenKind::Minus,
                '+' => TokenKind::Plus,
                ';' => TokenKind::Semicolon,
                '*' => TokenKind::Star,
                _ => todo!()
            }
        }
        TokenKind::Eof
    }

    fn read_next_token(&mut self) -> Token {
        let start = self.offset();
        let kind = self.read_next_king();
        let end = self.offset();
        return Token { start, kind, end, value }
    }

    fn offset(&self) -> usize {
        self.source.len() - self.chars.as_str().len()
    }

    // Return the next char witthout modify the original iterator
    fn peek(&self) -> Option<char> {
        self.chars.clone().next()
    }

    pub fn scan(&mut self) -> Vec<Token> {

        let mut tokens: Vec<Token> = Vec::new();

        let mut line = 1;

        let mut char_indices = self.chars.peekable();

        while let Some((position, char)) = char_indices.next() {
            let Kind = match char {
                '(' => TokenKind::LeftParen,
                ')' => TokenKind::RightParen,
                '{' => TokenKind::LeftBrace,
                '}' => TokenKind::RightBrace,
                ',' => TokenKind::Comma,
                '.' => TokenKind::Dot,
                '-' => TokenKind::Minus,
                '+' => TokenKind::Plus,
                ';' => TokenKind::Semicolon,
                '*' => TokenKind::Star,
                '!' => match char_indices.next_if_eq(&(position + 1, '=')) {
                    Some(_) => TokenKind::BangEqual,
                    None => TokenKind::Bang,
                },
                '=' => match char_indices.next_if_eq(&(position + 1, '=')) {
                    Some(_) => TokenKind::EqualEqual,
                    None => TokenKind::Equal,
                },
                '<' => match char_indices.next_if_eq(&(position + 1, '=')) {
                    Some(_) => TokenKind::LessEqual,
                    None => TokenKind::Less,
                },
                '>' => match char_indices.next_if_eq(&(position + 1, '=')) {
                    Some(_) => TokenKind::GreaterEqual,
                    None => TokenKind::Greater,
                },
                // TODO : parse comment
                '/' => match char_indices.next_if_eq(&(position + 1, '/')) {
                    Some(_) => TokenKind::Slash,
                    None => TokenKind::Slash,
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
                        '"' => TokenKind::String(string),
                        _ => TokenKind::Invalid(
                            line,
                            internal_position as i32,
                            "unterminated string".to_string(),
                        ),
                    }
                }
                ' ' => continue,
                '\n' => {
                    line += 1;
                    TokenKind::Line
                }
                _ => {
                    if self.is_digit(char) {
                        let mut number = String::new();

                        number.push(char);

                        while let Some((_, num)) =
                            char_indices.next_if(|(_pos, c)| self.is_digit(*c) || *c == '.')
                        {
                            number.push(num);
                        }

                        TokenKind::Number(number.parse::<f64>().unwrap())
                    } else if self.is_alpha(char) {
                        let mut identifier = String::new();

                        identifier.push(char);

                        while let Some((_, c)) =
                            char_indices.next_if(|(_pos, c)| self.is_alpha_numeric(*c))
                        {
                            identifier.push(c);
                        }

                        match identifier.as_str() {
                            "true" => TokenKind::True,
                            "false" => TokenKind::False,
                            "and" => TokenKind::And,
                            "or" => TokenKind::Or,
                            "if" => TokenKind::If,
                            "else" => TokenKind::Else,
                            "for" => TokenKind::For,
                            "while" => TokenKind::While,
                            "print" => TokenKind::Print,
                            "var" => TokenKind::Var,
                            "func" => TokenKind::Func,
                            "return" => TokenKind::Return,
                            "class" => TokenKind::Class,
                            "super" => TokenKind::Super,
                            "nil" => TokenKind::Nil,
                            _ => TokenKind::Identifier,
                        }
                    } else {
                        TokenKind::Invalid(line, position as i32, "Unrecognized Kind".to_string())
                    }
                }
            };
            self.toke.push(Kind);
        }

        return &self.Kinds;
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
    use super::{Lexer, Kind};

    #[test]
    fn test_number() {
        let source = String::from("1283293");
        let mut Lexer = Lexer::new(source);

        let Kinds: &Vec<Kind> = Lexer.scan();

        let number_Kind = Kinds.first().unwrap();

        assert_eq!(number_Kind, &Kind::Number(1283293 as f64))
    }

    #[test]
    fn test_floating_number() {
        let source = String::from("1283.293");
        let mut Lexer = Lexer::new(source);

        let Kinds = Lexer.scan();

        let number_Kind = Kinds.first().unwrap();

        assert_eq!(number_Kind, &Kind::Number(1283.293 as f64))
    }

    #[test]
    fn test_unterminated_string() {
        let source = String::from("\"the unterminated string ");
        let mut Lexer = Lexer::new(source);

        let Kinds = Lexer.scan();

        let number_Kind = Kinds.first().unwrap();

        assert_eq!(
            number_Kind,
            &Kind::Invalid(1, 24, "unterminated string".to_string())
        )
    }

    #[test]
    fn test_string() {
        let source = String::from("\"a string\"");
        let mut Lexer = Lexer::new(source);

        let Kinds = Lexer.scan();

        let number_Kind = Kinds.first().unwrap();

        assert_eq!(number_Kind, &Kind::String("a string".to_string()))
    }
}
