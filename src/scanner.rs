use super::token::{Literal, Token, TokenType};
use super::utils::error;

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

    pub fn scan_tokens(&mut self, src: String) -> Vec<Token> {
        let mut tokens = vec![];
        let chars: Vec<char> = src.chars().collect();

        while self.current < chars.len() {
            self.start = self.current;
            if let Some(tkn) = self.scan_token(&chars) {
                tokens.push(tkn);
            }
        }
        tokens.push(Token::new(TokenType::Eof, String::from(""), Literal::None, 0));
        tokens
    }

    fn scan_token(&mut self, chars: &[char]) -> Option<Token> {
        let c = chars[self.current];
        self.current += 1;

        use TokenType::*;
        let new_single_char_token = |t| Some(
            Token::new(t, format!("{}", chars[self.start]),
            Literal::None, self.line)
        );
        let new_two_char_token = |t| Some(
            Token::new(t,
                (&chars[self.start..self.start+2]).iter().collect(),
                Literal::None, self.line)
        );
        let skip_comment = |mut cur| loop { match chars.get(cur) {
                Some('\n') | None => break cur, // end of comment or file
                _ => cur += 1,
        }};
        let new_string_literal_token = |mut line, start, mut cur| {
            loop { match chars.get(cur) {
                Some('"') => {
                    cur += 1;
                    let lexeme: String = (&chars[start..cur]).iter().collect();
                    let string = (&chars[start + 1 .. cur - 1]).iter().collect();
                    break (line, cur,
                        Some(Token::new(String_,
                            lexeme,
                            Literal::Str(string),
                            line)))
                },
                None => { error(line, "Unterminated string."); break (line, cur, None) }
                Some('\n') => { line += 1; cur += 1; },
                _ => cur += 1,
            }}
        };
        let new_number_literal_token = |line, start, mut cur| {
            loop { match chars.get(cur) {
                Some(&d) if d >= '0' && d <= '9' => cur += 1,
                Some('.') => match chars.get(cur + 1) {
                    Some(&d) if d >= '0' && d <= '9' => cur += 1,
                    _ => { error(line, "Invalid literal number"); break (cur + 1, None) }
                }
                _ => {
                    let lexeme: String = (&chars[start..cur]).iter().collect();
                    let num: f64 = lexeme.parse().unwrap();
                    let token = Token::new(Number, lexeme, Literal::Num(num), line);
                    break (cur, Some(token))
                }
            }}
        };
        let new_identifier_or_keyword_token = |line, start, mut cur| loop {
            if let Some(&c) = chars.get(cur) {
                match c {
                    '_' | 'a'..='z' | 'A'..='Z' | '0'..='9' => cur += 1,
                    _ => {
                        let lexeme: String = (&chars[start..cur]).iter().collect();
                        let type_ = Scanner::eval_identifier_token_type(&lexeme);
                        let tkn = Token::new(
                            type_,
                            lexeme,
                            Literal::None,
                            line);
                        break (cur, Some(tkn))
                    }
                }
            } else {
                let tkn = Token::new(
                    Identifier,
                    (&chars[start..cur]).iter().collect(),
                    Literal::None,
                    line);
                break (cur, Some(tkn))
            }
        };

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
            '"' => {
                let (line, cur, tkn) = new_string_literal_token(self.line, self.start, self.current);
                self.line = line;
                self.current = cur;
                tkn
            }
            '0'..='9' => {
                let (cur, tkn) = new_number_literal_token(self.line, self.start, self.current);
                self.current = cur;
                tkn
            }
            '_' | 'a'..='z' | 'A'..='Z' => {
                let (cur, tkn) = new_identifier_or_keyword_token(self.line, self.start, self.current);
                self.current = cur;
                tkn
            }
            _ => { error(self.line, "Unexpected character."); None }
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
