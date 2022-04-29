use std::{
    iter::{Enumerate, Peekable},
    str::Chars,
};

use super::token::Token;

pub struct Lexer<'a> {
    chars: Peekable<Enumerate<Chars<'a>>>,
    source: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().enumerate().peekable(),
            source: input,
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((_, ' ')) = self.chars.peek() {
            self.chars.next();
        }
        // Process Single-char tokens
        if let Some((_, c)) = self.chars.peek() {
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
        if let Some((start, c)) = self.chars.next() {
            if let Some((_, c_next)) = self.chars.peek() {
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
                    '&' => {
                        if c_next == &'&' {
                            Some(Token::And)
                        } else {
                            None
                        }
                    }
                    '|' => {
                        if c_next == &'|' {
                            Some(Token::Or)
                        } else {
                            None
                        }
                    }
                    _ => {
                        if c.is_alphabetic() {
                            let mut end = start;
                            while let Some((next_end, c_next)) = self.chars.peek() {
                                if c_next.is_alphanumeric() || c_next == &'_' {
                                    end = *next_end;
                                    self.chars.next();
                                } else {
                                    break;
                                }
                            }
                            let word = &self.source[start..=end];
                            match word {
                                "if" => return Some(Token::If),
                                "else" => return Some(Token::Else),
                                "fn" => return Some(Token::Func),
                                "for" => return Some(Token::For),
                                "while" => return Some(Token::While),
                                "let" => return Some(Token::Let),
                                "return" => return Some(Token::Return),
                                "nil" => return Some(Token::Nil),
                                "true" => return Some(Token::True),
                                "false" => return Some(Token::False),
                                "print" => return Some(Token::Print),
                                _ => return Some(Token::Identifier(word)),
                            }
                        }
                        if c.is_numeric() {
                            let mut end = start;
                            while let Some((next_end, c_next)) = self.chars.peek() {
                                if c_next.is_digit(10) || c_next == &'_' || c_next == &'.' {
                                    end = *next_end;
                                    self.chars.next();
                                } else {
                                    break;
                                }
                            }
                            return Some(Token::Number(&self.source[start..=end]));
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
