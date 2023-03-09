use crate::token::{Token, TokenType};
use crate::expr::Expr;
use thiserror::Error;

pub type PResult<T> = Result<T, ParseError>;

pub fn expression(tokens: &[Token]) -> PResult<(Expr, &[Token])> {
    equality(tokens)
}

/// equality -> comparison ( ( != | == ) comparison )* ;
fn equality(tokens: &[Token]) -> PResult<(Expr, &[Token])> {
    let (mut expr, mut tokens) = comparison(tokens)?;

    use TokenType::{ BangEqual, EqualEqual };
    loop { match tokens[0] {
        Token{type_: BangEqual, ..} | Token{type_: EqualEqual, ..} => {
            let operator = tokens[0].clone();
            let (right, tks) = comparison(&tokens[1..])?;
            tokens = tks;
            expr = Expr::Binary {
                left: Box::new(expr), operator, right: Box::new(right)};
        }
        _ => break,
    }}
    Ok((expr, tokens))
}

/// comparison -> term ( (> | >= | < | <=) term )* ;
fn comparison(tokens: &[Token]) -> PResult<(Expr, &[Token])> {
    let (mut expr, mut tokens) = term(tokens)?;

    use TokenType::{Less, LessEqual, Greater, GreaterEqual};
    loop { match tokens[0] {
        Token{type_: Greater,..} | Token{type_: GreaterEqual,..} |
            Token{type_: Less,..} | Token{type_: LessEqual,..} => {
            let operator = tokens[0].clone();
            let (right, tks) = term(&tokens[1..])?;
            tokens = tks;
            expr = Expr::Binary {
                left: Box::new(expr), operator, right: Box::new(right)};
        }
        _ => break,
    }}
    Ok((expr, tokens))
}

/// term -> factor ( (- | +) factor )* ;
fn term(tokens: &[Token]) -> PResult<(Expr, &[Token])> {
    let (mut expr, mut tokens) = factor(tokens)?;

    use TokenType::{Minus, Plus};
    loop { match tokens[0] {
        Token{type_: Minus,..} | Token{type_: Plus,..} => {
            let operator = tokens[0].clone();
            let (right, tks) = factor(&tokens[1..])?;
            tokens = tks;
            expr = Expr::Binary {
                left: Box::new(expr), operator, right: Box::new(right) };
        }
        _ => break,
    }}
    Ok((expr, tokens))
}

/// factor -> unary ( (/ | *) unary )*;
fn factor(tokens: &[Token]) -> PResult<(Expr, &[Token])> {
    let (mut expr, mut tokens) = unary(tokens)?;

    use TokenType::{Star, Slash};
    loop { match tokens[0] {
        Token{type_: Star,..} | Token{type_: Slash,..} => {
            let operator = tokens[0].clone();
            let (right, tks) = unary(&tokens[1..])?;
            tokens = tks;
            expr = Expr::Binary {
                left: Box::new(expr), operator, right: Box::new(right) };
        }
        _ => break,
    }}
    Ok((expr, tokens))
}

/// unary -> (! | -) unary | primary;
fn unary(tokens: &[Token]) -> PResult<(Expr, &[Token])> {
    use TokenType::{Bang, Minus};
    match tokens[0] {
        Token{type_: Bang,..} | Token{type_: Minus,..} => {
            let operator = tokens[0].clone();
            let (right, tokens) = unary(&tokens[1..])?;
            let expr = Expr::Unary { operator, right: Box::new(right) };
            return Ok((expr, tokens));
        }
        _ => (),
    }
    primary(tokens)
}

/// primary -> NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")"
fn primary(tokens: &[Token]) -> PResult<(Expr, &[Token])> {
    use TokenType::{False, True, Nil, Number, String_, LeftParen, RightParen, Eof};
    let token = &tokens[0];
    match token {
        Token{type_: False,..} | Token{type_: True,..} | Token{type_: Nil,..} |
            Token{type_: Number,..} | Token{type_: String_,..} =>
            Ok((Expr::Literal{value: token.clone()}, &tokens[1..])),
        Token{type_: LeftParen,..} => {
            let (expr, tokens) = expression(&tokens[1..])?;
            match &tokens[0] {
                Token{type_: RightParen,..} =>
                    Ok((Expr::Grouping{expression: Box::new(expr)}, &tokens[1..])),
                tk if tk.type_ == Eof => Err(ParseError::new(
                    tk.line, tk.clone(), "Expect a ')' token at end.".into())),
                tk => Err(ParseError::new(
                    tk.line, tk.clone(), "Expect a ')' token here.".into())),
            }
        }
        // Token{type_: Eof,..} => TODO
        _ => Err(ParseError::new(
            token.line, token.clone(), "Cannot parse this token.".into())),
    }
}

#[derive(Error, Debug)]
#[error("[line {line:}][token {token}] {message}")]
pub struct ParseError {
    line: usize,
    token: Token,
    message: String,
}

impl ParseError {
    pub fn new(line: usize, token: Token, message: String) -> Self {
        Self { line, token, message }
    }
}

