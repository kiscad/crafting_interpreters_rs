use super::token::{Literal, Token, TokenType};
use super::utils;

pub struct Scanner {
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new() -> Self {
        Self {
            start: 0,
            current: 0,
            line: 0,
        }
    }

    pub fn scan_tokens(&mut self, src: &str) -> Vec<Token> {
        let mut tokens = vec![];
        let chars: Vec<char> = src.chars().collect();

        while self.current < chars.len() {
            self.start = self.current;
            if let Some(tkn) = self.scan_token(&chars) {
                tokens.push(tkn);
            }
        }
        tokens.push(Token::new(TokenType::Eof, String::from(""), Literal::Null, 0));
        tokens
    }

    fn scan_token(&mut self, chars: &[char]) -> Option<Token> {
        let c = chars[self.current];
        self.current += 1;

        use TokenType::*;
        let new_single_char_token = |t| Some(
            Token::new(t, format!("{}", chars[self.start]),
            Literal::Null, self.line)
        );
        let new_two_char_token = |t| Some(
            Token::new(t,
                chars[self.start..self.start+2].iter().collect(),
                Literal::Null, self.line)
        );
        let skip_comment = |mut cur| loop { match chars.get(cur) {
                Some('\n') | None => break cur, // end of comment or file
                _ => cur += 1,
        }};

        match c {
            '(' => new_single_char_token(LeftParen),
            ')' => new_single_char_token(RightParen),
            '{' => new_single_char_token(LeftBrace),
            '}' => new_single_char_token(RightBrace),
            ',' => new_single_char_token(Comma),
            '.' => new_single_char_token(Dot),
            '-' => new_single_char_token(Minus),
            '+' => new_single_char_token(Plus),
            ';' => new_single_char_token(Semicolon),
            '*' => new_single_char_token(Star),
            '!' | '=' | '<' | '>' => {
                if chars[self.current] == '=' { match c {
                    '!' => { self.current += 1; new_two_char_token(BangEqual) },
                    '=' => { self.current += 1; new_two_char_token(EqualEqual) },
                    '<' => { self.current += 1; new_two_char_token(LessEqual) },
                    '>' => { self.current += 1; new_two_char_token(GreaterEqual) },
                    _ => unreachable!(),
                }} else { match c {
                    '!' => new_single_char_token(Bang),
                    '=' => new_single_char_token(Equal),
                    '<' => new_single_char_token(Less),
                    '>' => new_single_char_token(Greater),
                    _ => unreachable!(),
                }}
            }
            '/' => {
                if chars[self.current] == '/' {
                    self.current += 1;
                    self.current = skip_comment(self.current);
                    None
                } else {
                    new_single_char_token(Slash)
                }
            }
            ' ' | '\t' | '\r' => None,
            '\n' => { self.line += 1; None }
            '"' => self.new_string_literal_token(&chars),
            '0'..='9' => self.new_number_literal_token_(&chars),
            '_' | 'a'..='z' | 'A'..='Z' => self.new_identifier_or_keyword_token(&chars),
            _ => { utils::error(self.line, "Unexpected character."); None }
        }
    }

    fn new_string_literal_token(&mut self, chars: &[char]) -> Option<Token> {
        loop { match chars.get(self.current) {
            Some('"') => {
                self.current += 1;
                let lexeme: String = chars[self.start..self.current].iter().collect();
                let string = chars[self.start + 1 .. self.current - 1].iter().collect();
                break Some(Token::new(TokenType::String_,
                        lexeme,
                        Literal::Str(string),
                        self.line))
            },
            None => {
                utils::error(self.line, "Unterminated string.");
                break None
            }
            Some('\n') => {
                self.line += 1;
                self.current += 1;
            }
            _ => self.current += 1,
        }}
    }

    fn new_number_literal_token_(&mut self, chars: &[char]) -> Option<Token> {
        loop { match chars.get(self.current) {
            Some(&d) if d >= '0' && d <= '9' => self.current += 1,
            Some('.') => match chars.get(self.current + 1) {
                Some(&d) if d >= '0' && d <= '9' => self.current += 1,
                _ => {
                    utils::error(self.line, "Invalid literal number");
                    break None
                }
            }
            _ => {
                let lexeme: String =chars[self.start..self.current].iter().collect();
                let num: f64 = lexeme.parse().unwrap();
                let token = Token::new(TokenType::Number, lexeme, Literal::Num(num), self.line);
                break Some(token)
            }
        }}
    }

    fn new_identifier_or_keyword_token(&mut self, chars: &[char]) -> Option<Token> {
        loop {
            if let Some(&c) = chars.get(self.current) {
                match c {
                    '_' | 'a'..='z' | 'A'..='Z' | '0'..='9' => self.current += 1,
                    _ => {
                        let lexeme: String = chars[self.start..self.current].iter().collect();
                        let type_ = Scanner::eval_identifier_token_type(&lexeme);
                        let tkn = Token::new(
                            type_,
                            lexeme,
                            Literal::Null,
                            self.line);
                        break Some(tkn)
                    }
                }
            } else {
                let tkn = Token::new(
                    TokenType::Identifier,
                    chars[self.start..self.current].iter().collect(),
                    Literal::Null,
                    self.line);
                break Some(tkn)
            }
        }
    }

    fn eval_identifier_token_type(lexeme: &str) -> TokenType {
        use TokenType::*;
        match lexeme {
            "and" => And,
            "class" => Class,
            "else" => Else,
            "false" => False,
            "for" => For,
            "fun" => Fun,
            "if" => If,
            "nil" => Nil,
            "or" => Or,
            "print" => Print,
            "return" => Return,
            "super" => Super,
            "this" => This,
            "true" => True,
            "var" => Var,
            "while" => While,
            _ => Identifier
        }
    }
}
