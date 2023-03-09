use crate::token::TokenType;

use super::token::Token;
use super::expr::Expr;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
        }
    }

    /// expression -> equality ;
    fn expression(&mut self) -> Expr {
        self.equality()
    }

    /// equality -> comparison ( (!= | ==) comparison )* ;
    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        use TokenType::{ BangEqual, EqualEqual };
        while self.match_type(BangEqual) || self.match_type(EqualEqual) {
            let operator = self.previous().clone();
            let right = Box::new(self.comparison());
            expr = Expr::Binary { left: Box::new(expr), operator, right };
        }
        expr
    }

    /// comparison -> term ( (> | >= | < | <=) term )* ;
    fn comparison(&mut self) -> Expr {
        Expr::Literal { value: Token::new(, lexeme, literal, line) }
    }

    /// term -> factor ( (- | +) factor )* ;
    fn term() {}

    /// factor -> unary ( (/ | *) unary )* ;
    fn factor() {}

    /// unary -> (! | -) unary | primary ;
    fn unary() {}

    /// primary -> Number | String | true | false | nil | "(" expr ")"
    fn primary() {}

    fn match_type(&mut self, ty: TokenType) -> bool {
        if self.check(ty) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn check(&self, ty: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().type_ == ty
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn is_at_end(&self) -> bool {
        self.peek().type_ == TokenType::Eof
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

}

