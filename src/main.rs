use std::env;
use std::io::{BufRead, BufReader, Write, stdin};
use std::path::PathBuf;
use std::process;
use std::fs;
use anyhow::Error;
use std::fmt::{self, write};

static mut G_HAD_ERROR: bool = false;

fn main() {
    let args: Vec<_> = env::args().skip(1).collect();
    
    if args.len() > 1 {
        println!("UsafeL jlox [script]");
        process::exit(64);
    } else if args.len() == 1 {
        run_file(&args[0]).unwrap();
    } else {
        run_promt().unwrap();
    }
}

fn run_file(fname: &str) -> Result<(), Error>{
    let path = PathBuf::from(fname);
    let src = fs::read_to_string(path)?;
    run(src);

    unsafe { if G_HAD_ERROR {
        process::exit(65);
    }}
    Ok(())
}

fn run_promt() -> Result<(), Error>{
    let input = stdin();
    let mut reader = BufReader::new(input);

    loop {
        print!("> ");
        std::io::stdout().flush()?;
        let mut line = String::new();
        reader.read_line(&mut line)?;

        if line.is_empty() {
            break
        }

        run(line);

        unsafe { G_HAD_ERROR = false; }
    }
    Ok(())
}

fn run(src: String) {
    print!("{src}");
    let mut scanner = Scanner::new();
    let tokens = scanner.scan_tokens(src);

    for token in tokens {
        println!("{token:?}");
    }
}

struct Scanner {
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    fn new() -> Self {
        Self {
            start: 0,
            current: 0,
            line: 0,
        }
    }

    fn scan_tokens(&mut self, src: String) -> Vec<Token> {
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

#[derive(Debug)]
enum Literal {
    Num(f64),
    Str(String),
    None,
}

#[derive(Debug)]
struct Token {
    type_: TokenType,
    lexeme: String,
    literal: Literal,
    line: usize,
}

impl Token {
    fn new(type_: TokenType, lexeme: String, literal: Literal, line: usize) -> Self {
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

#[derive(Debug)]
enum TokenType {
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


fn error(line: usize, message: &str) {
    report(line, "", message);
}

fn report(line: usize, where_: &str, message: &str) {
    println!("[line {line}] Error {where_}: {message}");
    unsafe { G_HAD_ERROR = true };
}