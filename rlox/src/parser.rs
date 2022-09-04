#![allow(dead_code)]
use crate::grammar::*;
use crate::lexer::{Token, Type};

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }
    pub fn parse(&mut self) -> Result<Expr, ()> {
        Err(())
    }

    fn peek(&self) -> Option<Token> {
        let idx = self.current;
        if idx < self.tokens.len() {
            Some(self.tokens[idx].clone())
        } else {
            None
        }
    }

    fn next(&mut self) -> Option<Token> {
        let idx = self.current;
        if idx < self.tokens.len() {
            self.current += 1;
            Some(self.tokens[idx].clone())
        } else {
            None
        }
    }

    fn expression(&mut self) -> Result<Expr, ()> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, ()> {
        let mut expr = self.comparison()?;
        while let Some(Token {
            kind: Type::EqEq | Type::BangEq,
            line: _,
        }) = self.peek()
        {
            let op_token = self.next().unwrap();
            let op = match op_token.kind {
                Type::EqEq => BinaryOp::Eq,
                Type::BangEq => BinaryOp::Ne,
                _ => panic!(),
            };
            expr = Expr::BinaryExpr(Box::new(expr), op, Box::new(self.comparison()?));
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ()> {
        let mut term = self.term()?;
        while let Some(Token {
            kind: Type::Gt | Type::Ge | Type::Lt | Type::Le,
            line: _,
        }) = self.peek()
        {
            let op_token = self.next().unwrap();
            let op = match op_token.kind {
                Type::Gt => BinaryOp::Gt,
                Type::Ge => BinaryOp::Ge,
                Type::Lt => BinaryOp::Lt,
                Type::Le => BinaryOp::Le,
                _ => panic!(),
            };
            term = Expr::BinaryExpr(Box::new(term), op, Box::new(self.term()?));
        }
        Ok(term)
    }

    fn term(&mut self) -> Result<Expr, ()> {
        let mut factor = self.factor()?;
        while let Some(Token {
            kind: Type::Minus | Type::Plus,
            line: _,
        }) = self.peek()
        {
            let op_token = self.next().unwrap();
            let op = match op_token.kind {
                Type::Plus => BinaryOp::Plus,
                Type::Minus => BinaryOp::Minus,
                _ => panic!(),
            };
            factor = Expr::BinaryExpr(Box::new(factor), op, Box::new(self.factor()?));
        }
        Ok(factor)
    }

    fn factor(&mut self) -> Result<Expr, ()> {
        let mut unary = self.unary()?;
        while let Some(Token {
            kind: Type::Star | Type::Slash,
            line: _,
        }) = self.peek()
        {
            let op_token = self.next().unwrap();
            let op = match op_token.kind {
                Type::Star => BinaryOp::Times,
                Type::Slash => BinaryOp::Divide,
                _ => panic!(),
            };
            unary = Expr::BinaryExpr(Box::new(unary), op, Box::new(self.unary()?));
        }
        Ok(unary)
    }

    fn unary(&mut self) -> Result<Expr, ()> {
        if let Some(Token {
            kind: Type::Minus | Type::Bang,
            line: _,
        }) = self.peek()
        {
            let op_token = self.next().unwrap();
            let op = match op_token.kind {
                Type::Minus => UnaryOp::Negative,
                Type::Bang => UnaryOp::Not,
                _ => panic!(),
            };
            Ok(Expr::UnaryExpr(op, Box::new(self.unary()?)))
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expr, ()> {
        match self.next() {
            Some(token) => match token.kind {
                Type::Nil => Ok(Expr::Literal(Literal::Nil)),
                Type::True => Ok(Expr::Literal(Literal::True)),
                Type::False => Ok(Expr::Literal(Literal::False)),
                Type::String { value } => Ok(Expr::Literal(Literal::String(value))),
                Type::Float { value } => Ok(Expr::Literal(Literal::Float(value))),
                Type::Int { value } => Ok(Expr::Literal(Literal::Int(value))),
                _ => Err(()),
            },
            None => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    #[test]
    fn test_primary() {
        let tests = vec![
            ("nil", Ok(Expr::Literal(Literal::Nil))),
            (
                "\"something\"",
                Ok(Expr::Literal(Literal::String("something".to_string()))),
            ),
        ];
        for (input, expected) in tests.iter() {
            let tokens: Vec<Token> = Lexer::new(input).collect();
            let mut parser = Parser::new(tokens);
            assert_eq!(parser.primary(), *expected);
        }
    }

    #[test]
    fn test_unary() {
        let tests = vec![
            (
                "-1",
                Ok(Expr::UnaryExpr(
                    UnaryOp::Negative,
                    Box::new(Expr::Literal(Literal::Int(1))),
                )),
            ),
            (
                "!\"val\"",
                Ok(Expr::UnaryExpr(
                    UnaryOp::Not,
                    Box::new(Expr::Literal(Literal::String("val".to_string()))),
                )),
            ),
        ];
        for (input, expected) in tests.iter() {
            let tokens: Vec<Token> = Lexer::new(input).collect();
            let mut parser = Parser::new(tokens);
            assert_eq!(parser.unary(), *expected);
        }
    }

    #[test]
    fn test_factor() {
        let tests = vec![
            (
                "-1",
                Ok(Expr::UnaryExpr(
                    UnaryOp::Negative,
                    Box::new(Expr::Literal(Literal::Int(1))),
                )),
            ),
            (
                "1 * -1",
                Ok(Expr::BinaryExpr(
                    Box::new(Expr::Literal(Literal::Int(1))),
                    BinaryOp::Times,
                    Box::new(Expr::UnaryExpr(
                        UnaryOp::Negative,
                        Box::new(Expr::Literal(Literal::Int(1))),
                    )),
                )),
            ),
        ];
        for (input, expected) in tests.iter() {
            let tokens: Vec<Token> = Lexer::new(input).collect();
            let mut parser = Parser::new(tokens);
            assert_eq!(parser.factor(), *expected);
        }
    }

    #[test]
    fn test_term() {
        let tests = vec![
            (
                "8 * 8 + 1",
                Ok(Expr::BinaryExpr(
                    Box::new(Expr::BinaryExpr(
                        Box::new(Expr::Literal(Literal::Int(8))),
                        BinaryOp::Times,
                        Box::new(Expr::Literal(Literal::Int(8))),
                    )),
                    BinaryOp::Plus,
                    Box::new(Expr::Literal(Literal::Int(1)))
                )),
            ),
            (
                "16 - 10 / 5",
                Ok(Expr::BinaryExpr(
                    Box::new(Expr::Literal(Literal::Int(16))),
                    BinaryOp::Minus,
                    Box::new(Expr::BinaryExpr(
                        Box::new(Expr::Literal(Literal::Int(10))),
                        BinaryOp::Divide,
                        Box::new(Expr::Literal(Literal::Int(5))),
                    ))
                )),
            ),
        ];
        for (input, expected) in tests.iter() {
            let tokens: Vec<Token> = Lexer::new(input).collect();
            let mut parser = Parser::new(tokens);
            assert_eq!(parser.term(), *expected);
        }
    }

    #[test]
    fn test_comparison() {
        let tests = vec![
            (
                "8 * 8 > 1",
                Ok(Expr::BinaryExpr(
                    Box::new(Expr::BinaryExpr(
                        Box::new(Expr::Literal(Literal::Int(8))),
                        BinaryOp::Times,
                        Box::new(Expr::Literal(Literal::Int(8))),
                    )),
                    BinaryOp::Gt,
                    Box::new(Expr::Literal(Literal::Int(1)))
                )),
            ),
            (
                "16 <= 10 + 5",
                Ok(Expr::BinaryExpr(
                    Box::new(Expr::Literal(Literal::Int(16))),
                    BinaryOp::Le,
                    Box::new(Expr::BinaryExpr(
                        Box::new(Expr::Literal(Literal::Int(10))),
                        BinaryOp::Plus,
                        Box::new(Expr::Literal(Literal::Int(5))),
                    ))
                )),
            ),
        ];
        for (input, expected) in tests.iter() {
            let tokens: Vec<Token> = Lexer::new(input).collect();
            let mut parser = Parser::new(tokens);
            assert_eq!(parser.comparison(), *expected);
        }
    }

    #[test]
    fn test_equality() {
        let tests = vec![
            (
                "8 * 8 == 1",
                Ok(Expr::BinaryExpr(
                    Box::new(Expr::BinaryExpr(
                        Box::new(Expr::Literal(Literal::Int(8))),
                        BinaryOp::Times,
                        Box::new(Expr::Literal(Literal::Int(8))),
                    )),
                    BinaryOp::Eq,
                    Box::new(Expr::Literal(Literal::Int(1)))
                )),
            ),
            (
                "16 != 10 + 5",
                Ok(Expr::BinaryExpr(
                    Box::new(Expr::Literal(Literal::Int(16))),
                    BinaryOp::Ne,
                    Box::new(Expr::BinaryExpr(
                        Box::new(Expr::Literal(Literal::Int(10))),
                        BinaryOp::Plus,
                        Box::new(Expr::Literal(Literal::Int(5))),
                    ))
                )),
            ),
        ];
        for (input, expected) in tests.iter() {
            let tokens: Vec<Token> = Lexer::new(input).collect();
            let mut parser = Parser::new(tokens);
            assert_eq!(parser.equality(), *expected);
        }
    }
}
