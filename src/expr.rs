use core::panic;

use crate::token::Literal;

use super::token::{ Token, TokenType};

pub enum Expr {
    Literal { value: Token },
    Unary { operator: Token, right: Box<Expr> },
    Binary { left: Box<Expr>, operator: Token, right: Box<Expr> },
    Grouping { expression: Box<Expr> },
}

impl Expr {
    pub fn format_ast(&self) -> String {
        match self {
            Self::Literal { value } => {
                match value.type_ {
                    TokenType::Nil => String::from("nil"),
                    TokenType::True => String::from("true"),
                    TokenType::False => String::from("false"),
                    _ => match &value.literal {
                        Literal::Num(n) => format!("{:?}", n),
                        Literal::Str(s) => format!("{:?}", s),
                        Literal::Null => panic!(), 
                    }
                }
            }
            Self::Binary { left, operator, right } =>
                format!("({} {} {})", operator.lexeme, left.format_ast(), right.format_ast()),
            Self::Grouping { expression } =>
                format!("(group {})", expression.format_ast()),
            Self::Unary { operator, right } =>
                format!("({} {})", operator.lexeme, right.format_ast()),
        }
    }
}


#[test]
fn test_format_ast() {
    let tkn_minus = Token::new(TokenType::Minus, "-".into(), Literal::Null, 1);
    let tkn_star = Token::new(TokenType::Star, "*".into(), Literal::Null, 1);
    let tkn_num1 = Token::new(TokenType::Number, "123".into(), Literal::Num(123.0), 1);
    let tkn_num2 = Token::new(TokenType::Number, "45.67".into(), Literal::Num(45.67), 1);
    let expr = Expr::Binary {
        left: Box::new(Expr::Unary {
            operator: tkn_minus,
            right: Box::new(Expr::Literal { value: tkn_num1 }),
        }),
        operator: tkn_star,
        right: Box::new(Expr::Grouping { expression: Box::new(Expr::Literal { value: tkn_num2 }) })
    };
    let format_ast = expr.format_ast();
    assert_eq!(format_ast, String::from("(* (- 123.0) (group 45.67))"))
}
