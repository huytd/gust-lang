use std::{iter::Peekable, str::Chars};

use super::token::Token;

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable(),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(&' ') = self.chars.peek() {
            self.chars.next();
        }
        // Process Single-char tokens
        if let Some(c) = self.chars.peek() {
            if let Some(token) = match c {
                '{' => Some(Token::LeftBracket),
                '}' => Some(Token::RightBracket),
                '(' => Some(Token::LeftParen),
                ')' => Some(Token::RightParen),
                '[' => Some(Token::LeftSquareBracket),
                ']' => Some(Token::RightSquareBracket),
                ',' => Some(Token::Comma),
                '.' => Some(Token::Dot),
                '-' => Some(Token::Minus),
                '+' => Some(Token::Plus),
                '/' => Some(Token::Slash),
                '*' => Some(Token::Star),
                ':' => Some(Token::Colon),
                '\n' => Some(Token::EOL),
                _ => None,
            } {
                self.chars.next();
                return Some(token);
            }
        }
        // Conditional tokens and others
        if let Some(c) = self.chars.next() {
            if let Some(c_next) = self.chars.peek() {
                if let Some(token) = match c {
                    '!' => {
                        if c_next == &'=' {
                            Some(Token::BangEqual)
                        } else if c_next.is_alphabetic() {
                            Some(Token::Bang)
                        } else {
                            None
                        }
                    }
                    '=' => {
                        if c_next == &'=' {
                            Some(Token::EqualEqual)
                        } else {
                            Some(Token::Equal)
                        }
                    }
                    '>' => {
                        if c_next == &'=' {
                            Some(Token::GreaterEqual)
                        } else {
                            Some(Token::Greater)
                        }
                    }
                    '<' => {
                        if c_next == &'=' {
                            Some(Token::LessEqual)
                        } else {
                            Some(Token::Less)
                        }
                    }
                    _ => {
                        if c.is_alphabetic() {
                            // Identifier
                        }
                        if c.is_numeric() {
                            // Number
                        }
                        None
                    }
                } {
                    self.chars.next();
                    return Some(token);
                }
            }
        }
        None
    }
}
