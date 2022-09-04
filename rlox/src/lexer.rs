use std::{iter::Peekable, str::Chars};

//----------------------------------------------------------------------
// Token
//----------------------------------------------------------------------
#[derive(Debug, PartialEq, Clone)]
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
    Ge,
    Lt,
    Le,

    // Literals
    Identifier { name: String },
    String { value: String },
    Int { value: i64 },
    Float { value: f64 },

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

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: Type,
    pub line: i32,
}

impl Token {
    pub fn new(kind: Type, line: i32) -> Self {
        Self { kind, line }
    }
}

//----------------------------------------------------------------------
// Lexer
//----------------------------------------------------------------------

pub struct Lexer<'a> {
    iter: Peekable<Chars<'a>>,
    line: i32,
    done: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            iter: src.chars().peekable(),
            line: 0,
            done: false,
        }
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        // consume as much as possible
        loop {
            consume_whitespace(&mut self.iter, &mut self.line);
            if let None = consume_comments(&mut self.iter, &mut self.line) {
                break;
            }
        }

        if let Some(token) = match_single(&mut self.iter, self.line) {
            Some(token)
        } else if let Some(token) = match_single_double(&mut self.iter, self.line) {
            Some(token)
        } else if let Some(token) = match_number_literal(&mut self.iter, self.line) {
            Some(token)
        } else if let Some(token) = match_string_literal(&mut self.iter, self.line) {
            Some(token)
        } else if let Some(token) = match_identifier_or_keyword(&mut self.iter, self.line) {
            Some(token)
        } else if let Some(token) = match_invalid(&mut self.iter, self.line) {
            Some(token)
        } else {
            self.done = true;
            Some(Token::new(Type::Eof, self.line))
        }
    }
}

fn match_invalid(iter: &mut Peekable<Chars>, line: i32) -> Option<Token> {
    if let Some(ch) = iter.next() {
        Some(Token::new(
            Type::Invalid {
                value: ch.to_string(),
            },
            line,
        ))
    } else {
        None
    }
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

fn consume_inline_comment(iter: &mut Peekable<Chars>) {
    while let Some(next) = iter.next() {
        if next == '\n' {
            break;
        }
    }
}

fn consume_multiline_comment(iter: &mut Peekable<Chars>, line: &mut i32) {
    // increment iterator to start search for end of comment block
    let _ = iter.next();
    let _ = iter.next();

    let mut hit_star = false;
    while let Some(next) = iter.next() {
        match next {
            '*' => hit_star = true,
            '/' => {
                if hit_star {
                    break;
                }
            }
            '\n' => {
                hit_star = false;
                *line += 1;
            }
            _ => hit_star = false,
        };
    }
}

fn consume_comments(iter: &mut Peekable<Chars>, line: &mut i32) -> Option<()> {
    // Make a copy as there is no way to peek more than one character
    // without consuming characters
    let mut copy = iter.clone();
    if let Some('/') = copy.next() {
        let next = copy.next();

        if let Some('/') = next {
            consume_inline_comment(iter);
            *line += 1;
            return Some(());
        } else if let Some('*') = next {
            consume_multiline_comment(iter, line);
            return Some(());
        }
    }
    None
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
                    Some(Token::new(Type::Ge, line))
                } else {
                    Some(Token::new(Type::Gt, line))
                }
            }
            '<' => {
                let _ = iter.next();
                if let Some('=') = iter.peek() {
                    let _ = iter.next();
                    Some(Token::new(Type::Le, line))
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

fn match_number_literal(iter: &mut Peekable<Chars>, line: i32) -> Option<Token> {
    if let Some('0'..='9') = iter.peek() {
        // Can find a match
    } else {
        return None;
    }

    let mut value = String::new();
    let mut is_valid = true;
    let mut is_floating_point = false;

    while let Some(ch) = iter.peek() {
        match ch {
            '0'..='9' => value.push(iter.next().unwrap()),
            '.' => {
                if is_floating_point {
                    // can't have two periods in a number eg 2..4 or 2.3.4
                    is_valid = false;
                } else {
                    is_floating_point = true;
                }
                value.push(iter.next().unwrap())
            }
            'a'..='z' | 'A'..='Z' => {
                is_valid = false;
                value.push(iter.next().unwrap())
            }
            _ => break,
        };
    }

    if value.is_empty() {
        None
    } else if !is_valid {
        Some(Token::new(Type::Invalid { value }, line))
    } else if is_floating_point {
        Some(Token::new(
            Type::Float {
                value: value.parse().unwrap(),
            },
            line,
        ))
    } else {
        Some(Token::new(
            Type::Int {
                value: value.parse().unwrap(),
            },
            line,
        ))
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

fn match_identifier_like(iter: &mut Peekable<Chars>) -> Option<(bool, String)> {
    let mut value = String::new();

    while let Some(ch) = iter.peek() {
        match ch {
            'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => value.push(iter.next().unwrap()),
            _ => break,
        };
    }

    if let Some(first) = value.chars().nth(0) {
        let valid = !('0'..='9').contains(&first);
        Some((valid, value))
    } else {
        None
    }
}

fn match_identifier_or_keyword(iter: &mut Peekable<Chars>, line: i32) -> Option<Token> {
    if let Some((is_valid, value)) = match_identifier_like(iter) {
        if !is_valid {
            return Some(Token::new(Type::Invalid { value }, line));
        }
        let token = match value.as_str() {
            "and" => Token::new(Type::And, line),
            "class" => Token::new(Type::Class, line),
            "else" => Token::new(Type::Else, line),
            "false" => Token::new(Type::False, line),
            "fun" => Token::new(Type::Fun, line),
            "for" => Token::new(Type::For, line),
            "if" => Token::new(Type::If, line),
            "nil" => Token::new(Type::Nil, line),
            "or" => Token::new(Type::Or, line),
            "print" => Token::new(Type::Print, line),
            "return" => Token::new(Type::Return, line),
            "super" => Token::new(Type::Super, line),
            "this" => Token::new(Type::This, line),
            "true" => Token::new(Type::True, line),
            "var" => Token::new(Type::Var, line),
            "while" => Token::new(Type::While, line),
            _ => Token::new(Type::Identifier { name: value }, line),
        };
        Some(token)
    } else {
        None
    }
}

//----------------------------------------------------------------------
// Tests
//----------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::vec;

    #[test]
    fn test_consume_comment() {
        let input = "//some random nonsense\na";
        let mut iter = input.chars().peekable();
        let mut line = 0;

        consume_comments(&mut iter, &mut line);
        assert_eq!(Some(&'a'), iter.peek());
        assert_eq!(line, 1);

        let input = "/some random nonsense\na";
        let mut iter = input.chars().peekable();
        let mut line = 0;
        consume_comments(&mut iter, &mut line);
        assert_eq!(Some(&'/'), iter.peek());
        assert_eq!(line, 0);

        let input = "//some random nonsense";
        let mut iter = input.chars().peekable();
        let mut line = 0;
        consume_comments(&mut iter, &mut line);
        assert_eq!(None, iter.peek());
        assert_eq!(line, 1);
    }

    #[test]
    fn test_consume_comment_multiline() {
        let input = "/*some random nonsense*/";
        let mut iter = input.chars().peekable();
        let mut line = 0;

        consume_comments(&mut iter, &mut line);
        assert_eq!(None, iter.peek());
        assert_eq!(line, 0);

        let input = "/*some random nonsense*//*other rando nonsense*/";
        let mut iter = input.chars().peekable();
        let mut line = 0;

        consume_comments(&mut iter, &mut line);
        consume_comments(&mut iter, &mut line);
        assert_eq!(None, iter.peek());
        assert_eq!(line, 0);
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
    fn test_match_int_literal() {
        let input = "123";
        let mut iter = input.chars().peekable();
        assert_eq!(
            Some(Token::new(Type::Int { value: 123 }, 0)),
            match_number_literal(&mut iter, 0)
        );

        let input = "123.123";
        let mut iter = input.chars().peekable();
        assert_eq!(
            Some(Token::new(Type::Float { value: 123.123 }, 0)),
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

    #[test]
    fn test_match_float_literal() {
        let input = "1.23";
        let mut iter = input.chars().peekable();
        assert_eq!(
            Some(Token::new(Type::Float { value: 1.23 }, 0)),
            match_number_literal(&mut iter, 0)
        );

        let input = "1.2.3";
        let mut iter = input.chars().peekable();
        assert_eq!(
            Some(Token::new(
                Type::Invalid {
                    value: "1.2.3".to_owned()
                },
                0
            )),
            match_number_literal(&mut iter, 0)
        );
    }

    #[test]
    fn test_match_identifier_or_keyword() {
        let input = "andy";
        let mut iter = input.chars().peekable();
        assert_eq!(
            Some(Token::new(
                Type::Identifier {
                    name: "andy".to_string()
                },
                0
            )),
            match_identifier_or_keyword(&mut iter, 0)
        );

        let input = "and";
        let mut iter = input.chars().peekable();
        assert_eq!(
            Some(Token::new(Type::And, 0)),
            match_identifier_or_keyword(&mut iter, 0)
        );

        let input = "fun ";
        let mut iter = input.chars().peekable();
        assert_eq!(
            Some(Token::new(Type::Fun, 0)),
            match_identifier_or_keyword(&mut iter, 0)
        );

        let input = "01tony";
        let mut iter = input.chars().peekable();
        assert_eq!(
            Some(Token::new(
                Type::Invalid {
                    value: "01tony".to_string()
                },
                0
            )),
            match_identifier_or_keyword(&mut iter, 0)
        );

        let input = "";
        let mut iter = input.chars().peekable();
        assert_eq!(None, match_identifier_or_keyword(&mut iter, 0));
    }

    #[test]
    fn test_lex() {
        let input = "//some comment\nfun func(a, b) {return a + b; }";
        let expected = vec![
            Token::new(Type::Fun, 1),
            Token::new(
                Type::Identifier {
                    name: "func".to_string(),
                },
                1,
            ),
            Token::new(Type::LParen, 1),
            Token::new(
                Type::Identifier {
                    name: "a".to_string(),
                },
                1,
            ),
            Token::new(Type::Comma, 1),
            Token::new(
                Type::Identifier {
                    name: "b".to_string(),
                },
                1,
            ),
            Token::new(Type::RParen, 1),
            Token::new(Type::LBrace, 1),
            Token::new(Type::Return, 1),
            Token::new(
                Type::Identifier {
                    name: "a".to_string(),
                },
                1,
            ),
            Token::new(Type::Plus, 1),
            Token::new(
                Type::Identifier {
                    name: "b".to_string(),
                },
                1,
            ),
            Token::new(Type::Semicolon, 1),
            Token::new(Type::RBrace, 1),
            Token::new(Type::Eof, 1),
        ];

        let result: Vec<Token> = Lexer::new(input).collect();
        assert_eq!(result, expected);
    }
}
