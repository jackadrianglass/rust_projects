use std::{iter::Peekable, str::Chars};

//----------------------------------------------------------------------
// Token
//----------------------------------------------------------------------
#[derive(Debug, PartialEq)]
pub enum Type {
    // Single-character tokens
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens
    Bang,
    BangEq,
    Eq,
    EqEq,
    Gt,
    GtEq,
    Lt,
    LtEq,

    // Literals
    Identifier { name: String },
    String { value: String },
    Number { value: i64 },

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Invalid { value: String },
    Eof,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    kind: Type,
    line: i32,
}

impl Token {
    pub fn new(kind: Type, line: i32) -> Self {
        Self { kind, line }
    }
}

//----------------------------------------------------------------------
// Lexer
//----------------------------------------------------------------------

pub fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut line = 0;
    let mut iter = input.chars().peekable();

    loop {
        consume_whitespace(&mut iter, &mut line);
        consume_comments(&mut iter);

        if let Some(_) = iter.peek() {
            if let Some(token) = match_single(&mut iter, line) {
                tokens.push(token);
            } else if let Some(token) = match_single_double(&mut iter, line) {
                tokens.push(token);
            } else if let Some(token) = match_number_literal(&mut iter, line) {
                tokens.push(token);
            } else if let Some(token) = match_string_literal(&mut iter, line) {
                tokens.push(token);
            } else if let Some(token) = match_identifier_or_keyword(&mut iter, line) {
                tokens.push(token);
            } else {
                if let Some(ch) = iter.next() {
                    tokens.push(Token::new(
                        Type::Invalid {
                            value: ch.to_string(),
                        },
                        line,
                    ));
                } else {
                    panic!("Theoretically impossible token consumption");
                }
            }
        } else {
            tokens.push(Token::new(Type::Eof, line));
            break;
        }
    }
    tokens
}

fn consume_whitespace(iter: &mut Peekable<Chars>, line: &mut i32) {
    while let Some(ch) = iter.peek() {
        match ch {
            '\n' => {
                let _ = iter.next();
                *line += 1;
            }
            ' ' | '\t' | '\r' => {
                let _ = iter.next();
            }
            _ => {
                break;
            }
        }
    }
}

fn consume_comments(iter: &mut Peekable<Chars>) {
    // Make a copy as there is no way to peek more than one character
    // without consuming characters
    let mut copy = iter.clone();
    if let Some('/') = copy.next() {
        if let Some('/') = copy.next() {
            while let Some(next) = iter.peek() {
                if *next == '\n' {
                    break;
                }
                let _ = iter.next();
            }
        }
    }
}

fn match_single(iter: &mut Peekable<Chars>, line: i32) -> Option<Token> {
    if let Some(ch) = iter.peek() {
        let result = match ch {
            '(' => Some(Token::new(Type::LParen, line)),
            ')' => Some(Token::new(Type::RParen, line)),
            '{' => Some(Token::new(Type::LBrace, line)),
            '}' => Some(Token::new(Type::RBrace, line)),
            ',' => Some(Token::new(Type::Comma, line)),
            '.' => Some(Token::new(Type::Dot, line)),
            '-' => Some(Token::new(Type::Minus, line)),
            '+' => Some(Token::new(Type::Plus, line)),
            ';' => Some(Token::new(Type::Semicolon, line)),
            '/' => Some(Token::new(Type::Slash, line)),
            '*' => Some(Token::new(Type::Star, line)),
            _ => None,
        };
        if let Some(_) = result {
            let _ = iter.next();
        };
        result
    } else {
        None
    }
}

fn match_single_double(iter: &mut Peekable<Chars>, line: i32) -> Option<Token> {
    if let Some(ch) = iter.peek() {
        match ch {
            '!' => {
                let _ = iter.next();
                if let Some('=') = iter.peek() {
                    let _ = iter.next();
                    Some(Token::new(Type::BangEq, line))
                } else {
                    Some(Token::new(Type::Bang, line))
                }
            }
            '=' => {
                let _ = iter.next();
                if let Some('=') = iter.peek() {
                    let _ = iter.next();
                    Some(Token::new(Type::EqEq, line))
                } else {
                    Some(Token::new(Type::Eq, line))
                }
            }
            '>' => {
                let _ = iter.next();
                if let Some('=') = iter.peek() {
                    let _ = iter.next();
                    Some(Token::new(Type::GtEq, line))
                } else {
                    Some(Token::new(Type::Gt, line))
                }
            }
            '<' => {
                let _ = iter.next();
                if let Some('=') = iter.peek() {
                    let _ = iter.next();
                    Some(Token::new(Type::LtEq, line))
                } else {
                    Some(Token::new(Type::Lt, line))
                }
            }
            _ => None,
        }
    } else {
        None
    }
}

// todo do floating point literals too
fn match_number_literal(iter: &mut Peekable<Chars>, line: i32) -> Option<Token> {
    let mut value = String::new();
    let mut is_valid = true;

    while let Some(ch) = iter.peek() {
        match ch {
            '0'..='9' => {
                if let Some(ch) = iter.next() {
                    value.push(ch)
                } else {
                    panic!("wtf")
                }
            }
            'a'..='z' | 'A'..='Z' => {
                if let Some(ch) = iter.next() {
                    is_valid = false;
                    value.push(ch)
                } else {
                    panic!("wtf")
                }
            }
            _ => break,
        };
    }

    if value.is_empty() {
        None
    } else if is_valid {
        Some(Token::new(
            Type::Number {
                value: value.parse().unwrap(),
            },
            line,
        ))
    } else {
        Some(Token::new(Type::Invalid { value }, line))
    }
}

fn match_string_literal(iter: &mut Peekable<Chars>, line: i32) -> Option<Token> {
    match iter.peek() {
        Some('"') => {}
        _ => {
            return None;
        }
    };

    let mut value = String::new();
    let _ = iter.next();
    while let Some(ch) = iter.next() {
        if ch == '"' {
            break;
        }
        value.push(ch);
    }
    Some(Token::new(Type::String { value }, line))
}

fn match_identifier_or_keyword(_iter: &mut Peekable<Chars>, _line: i32) -> Option<Token> {
    None
}

//----------------------------------------------------------------------
// Tests
//----------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consume_comment() {
        let input = "//some random nonsense\na";
        let mut iter = input.chars().peekable();
        consume_comments(&mut iter);
        assert_eq!(Some(&'\n'), iter.peek());

        let input = "/some random nonsense\na";
        let mut iter = input.chars().peekable();
        consume_comments(&mut iter);
        assert_eq!(Some(&'/'), iter.peek());

        let input = "//some random nonsense";
        let mut iter = input.chars().peekable();
        consume_comments(&mut iter);
        assert_eq!(None, iter.peek());
    }

    #[test]
    fn test_consume_whitespace() {
        let input = "\r\n    a";
        let mut iter = input.chars().peekable();
        let mut line = 0;
        consume_whitespace(&mut iter, &mut line);
        assert_eq!(Some(&'a'), iter.peek());
        assert_eq!(1, line);
    }

    #[test]
    fn test_match_single() {
        let input = "(";
        let mut iter = input.chars().peekable();
        assert_eq!(
            Some(Token::new(Type::LParen, 0)),
            match_single(&mut iter, 0)
        );

        let input = ")";
        let mut iter = input.chars().peekable();
        assert_eq!(
            Some(Token::new(Type::RParen, 0)),
            match_single(&mut iter, 0)
        );

        let input = "@";
        let mut iter = input.chars().peekable();
        assert_eq!(None, match_single(&mut iter, 0));
    }

    #[test]
    fn test_match_single_double() {
        let input = "==";
        let mut iter = input.chars().peekable();
        assert_eq!(
            Some(Token::new(Type::EqEq, 1)),
            match_single_double(&mut iter, 1)
        );
        assert_eq!(None, iter.peek());

        let input = "!@";
        let mut iter = input.chars().peekable();
        assert_eq!(
            Some(Token::new(Type::Bang, 1)),
            match_single_double(&mut iter, 1)
        );
        assert_eq!(Some(&'@'), iter.peek());
    }

    #[test]
    fn test_match_number_literal() {
        let input = "123";
        let mut iter = input.chars().peekable();
        assert_eq!(
            Some(Token::new(Type::Number { value: 123 }, 0)),
            match_number_literal(&mut iter, 0)
        );

        let input = "123abc";
        let mut iter = input.chars().peekable();
        assert_eq!(
            Some(Token::new(
                Type::Invalid {
                    value: "123abc".to_owned()
                },
                0
            )),
            match_number_literal(&mut iter, 0)
        );
    }
}
