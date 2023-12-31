use core::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,

    pub start: usize,

    pub end: usize,

    pub value: TokenValue
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenValue {
    None,
    Number(f64),
    String(String)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {

    LeftParen,
    RightParen, 
    LeftBrace, 
    RightBrace,
    Comma, 
    Dot,  
    Semicolon, 
 
    Plus,
    Minus, 
    Slash, 
    Star,
    Bang, 
    BangEqual,
    Equal, 
    EqualEqual,
    Greater,
    GreaterEqual,
    Less, 
    LessEqual,

    Identifier, 
    String, 
    Number,

    Class,
    Else, 
    False, 
    Func, 
    For, 
    If,
    And, 
    Nil, 
    Or,
    Print, 
    Return, 
    Super, 
    This, 
    True, 
    Var, 
    While,
    Eof,

    Space,
    Line,
    Invalid
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TokenKind::LeftParen => write!(f, "("),
            TokenKind::RightParen => write!(f, ")"),
            TokenKind::LeftBrace => write!(f, "{{"),
            TokenKind::RightBrace => write!(f, "}}"),
            TokenKind::Comma => write!(f, ","),
            TokenKind::Dot => write!(f, "."),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Semicolon => write!(f, ";"),
            TokenKind::Star => write!(f, "*"),
            TokenKind::Slash => write!(f, "/"),
            TokenKind::Bang => write!(f, "!"),
            TokenKind::BangEqual => write!(f, "!="),
            TokenKind::Equal => write!(f, "="),
            TokenKind::EqualEqual => write!(f, "=="),
            TokenKind::Greater =>write!(f, ">"),
            TokenKind::GreaterEqual =>write!(f, ">="),
            TokenKind::Less =>write!(f, "<"),
            TokenKind::LessEqual =>write!(f, "<="),
            TokenKind::Identifier =>write!(f, "identifier"),
            TokenKind::String =>write!(f, "string"),
            TokenKind::Number =>write!(f, "number"),
            TokenKind::Class => write!(f, "class"),
            TokenKind::Else =>write!(f, "else"),
            TokenKind::False =>write!(f, "false"),
            TokenKind::Func =>write!(f, "func"),
            TokenKind::For =>write!(f, "for"),
            TokenKind::If =>write!(f, "if"),
            TokenKind::And => write!(f, "and"),
            TokenKind::Nil =>write!(f, "nil"),
            TokenKind::Or =>write!(f, "or"),
            TokenKind::Print =>write!(f, "print"),
            TokenKind::Return =>write!(f, "return"),
            TokenKind::Super =>write!(f, "super"),
            TokenKind::This =>write!(f, "this"),
            TokenKind::True =>write!(f, "true"),
            TokenKind::Var =>write!(f, "var"),
            TokenKind::While =>write!(f, "while"),
            TokenKind::Eof =>write!(f, "eof"),
            TokenKind::Invalid =>write!(f, "invalid"),
            TokenKind::Space => write!(f, "space"),
            TokenKind::Line => write!(f, "line"),
        }
    }
}
