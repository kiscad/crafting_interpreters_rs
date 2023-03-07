use std::fmt;

#[derive(Debug)]
pub enum TokenType {
    // Single-character tokens
    LeftParen, RightParen,
    LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus,
    Semicolon, Slash, Star,
    // One or two character tokens.
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,
    // Literals
    Identifier, String_, Number,
    // Keywords
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,
    Eof,
}

#[derive(Debug)]
pub enum Literal {
    Num(f64),
    Str(String),
    Null,
}

#[derive(Debug)]
pub struct Token {
    pub type_: TokenType,
    pub lexeme: String,
    pub literal: Literal,
    line: usize,
}

impl Token {
    pub fn new(type_: TokenType, lexeme: String, literal: Literal, line: usize) -> Self {
        Self {
            type_,
            lexeme,
            literal,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} {} {:?}", self.type_, self.lexeme, self.literal)
    }
}
