use std::{iter::Peekable, str::Chars};
use super::token::{DELIMIERS, Token, KEYWORDS};

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable()
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(ch) = self.chars.next() {
            if DELIMIERS.contains(&ch) {
                return Some(Token::Delimiter(ch));
            } else if ch == '\n' {
                return Some(Token::EOL);
            } else if ch == '+' || ch == '-' || ch == '*' || ch == '/' {
                return Some(Token::Operator(ch.to_string()));
            } else if ch == '=' { // Handle == and =
                if let Some(next_char) = self.chars.peek() {
                    if next_char == &'=' {
                        self.chars.next();
                        return Some(Token::Operator("==".to_string()));
                    } else if next_char == &' ' {
                        self.chars.next();
                        return Some(Token::Assignment);
                    } else {
                        return Some(Token::Invalid);
                    }
                }
            } else if ch == '>' || ch == '<' || ch == '!' { // Handle comparison <, >, >=, <=, !=
                if let Some(next_char) = self.chars.peek() {
                    if next_char == &' ' && ch != '!' {
                        self.chars.next();
                        return Some(Token::Operator(ch.to_string()));
                    } else if next_char == &'=' {
                        self.chars.next();
                        return Some(Token::Operator(format!("{}=", ch)));
                    } else {
                        return Some(Token::Invalid);
                    }
                }
            } else if ch == '&' || ch == '|' { // Handle AND and OR
                if let Some(next_char) = self.chars.peek() {
                    if next_char == &'&' && ch == '&' {
                        self.chars.next();
                        return Some(Token::Operator("&&".to_string()));
                    } else if next_char == &'|' && ch == '|' {
                        self.chars.next();
                        return Some(Token::Operator("||".to_string()));
                    } else {
                        return Some(Token::Invalid);
                    }
                }
            } else if !ch.is_whitespace() {
                let mut buf = String::from(ch);
                loop {
                    if let Some(next_char) = self.chars.peek() {
                        if !next_char.is_whitespace() {
                            buf.push(*next_char);
                            self.chars.next();
                        } else {
                            break;
                        }
                    }
                }
                if KEYWORDS.contains(&buf.as_str()) {
                    return Some(Token::Keyword(buf));
                }
                if buf.chars().all(|c| c.is_numeric()) {
                    // Todo: Move number scanning outside
                    if let Ok(num) = buf.parse::<i32>() {
                        return Some(Token::Number(num));
                    }
                }
                return Some(Token::Identifier(buf));
            }
        }
        None
    }
}