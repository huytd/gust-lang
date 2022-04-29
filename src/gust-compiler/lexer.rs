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
            let (_, c_next) = self.chars.peek().unwrap_or(&(0, '\0'));
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
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{Lexer, Token};
    #[test]
    fn lexer_variable_declaration_test() {
        let lexer = Lexer::new("let x = 10");
        let actual = lexer.collect::<Vec<Token>>();
        assert!(actual == vec![
            Token::Let,
            Token::Identifier("x"),
            Token::Equal,
            Token::Number("10")
        ])
    }

    #[test]
    fn lexer_variable_declaration_multiline_test() {
        let lexer = Lexer::new(r#"let x = 10
            let y = x
        "#);
        let actual = lexer.collect::<Vec<Token>>();
        assert!(actual == vec![
            Token::Let,
            Token::Identifier("x"),
            Token::Equal,
            Token::Number("10"),
            Token::EOL,
            Token::Let,
            Token::Identifier("y"),
            Token::Equal,
            Token::Identifier("x"),
            Token::EOL
        ])
    }

    #[test]
    fn lexer_if_statement_test() {
        let lexer = Lexer::new(r#"if a != b {
            print(a)
        }"#);
        let actual = lexer.collect::<Vec<Token>>();
        assert!(actual == vec![
            Token::If,
            Token::Identifier("a"),
            Token::BangEqual,
            Token::Identifier("b"),
            Token::LeftBracket,
            Token::EOL,
            Token::Print,
            Token::LeftParen,
            Token::Identifier("a"),
            Token::RightParen,
            Token::EOL,
            Token::RightBracket
        ])
    }

    #[test]
    fn lexer_if_statement_with_else_test() {
        let lexer = Lexer::new(r#"if a != b {
            print(a)
        } else {
            print(b)
        }"#);
        let actual = lexer.collect::<Vec<Token>>();
        assert!(actual == vec![
            Token::If,
            Token::Identifier("a"),
            Token::BangEqual,
            Token::Identifier("b"),
            Token::LeftBracket,
            Token::EOL,
            Token::Print,
            Token::LeftParen,
            Token::Identifier("a"),
            Token::RightParen,
            Token::EOL,
            Token::RightBracket,
            Token::Else,
            Token::LeftBracket,
            Token::EOL,
            Token::Print,
            Token::LeftParen,
            Token::Identifier("b"),
            Token::RightParen,
            Token::EOL,
            Token::RightBracket
        ])
    }

    #[test]
    fn lexer_if_statement_with_multiple_conditions_test() {
        let lexer = Lexer::new(r#"if a != b && c == 10 || d == x {
            print(a)
        }"#);
        let actual = lexer.collect::<Vec<Token>>();
        assert!(actual == vec![
            Token::If,
            Token::Identifier("a"),
            Token::BangEqual,
            Token::Identifier("b"),
            Token::And,
            Token::Identifier("c"),
            Token::EqualEqual,
            Token::Number("10"),
            Token::Or,
            Token::Identifier("d"),
            Token::EqualEqual,
            Token::Identifier("x"),
            Token::LeftBracket,
            Token::EOL,
            Token::Print,
            Token::LeftParen,
            Token::Identifier("a"),
            Token::RightParen,
            Token::EOL,
            Token::RightBracket
        ])
    }

    #[test]
    fn lexer_mathematic_expression_test() {
        let lexer = Lexer::new(r#"5 + a * 10_000 / 4.5 - c"#);
        let actual = lexer.collect::<Vec<Token>>();
        assert!(actual == vec![
            Token::Number("5"),
            Token::Plus,
            Token::Identifier("a"),
            Token::Star,
            Token::Number("10_000"),
            Token::Slash,
            Token::Number("4.5"),
            Token::Minus,
            Token::Identifier("c")
        ])
    }

    #[test]
    fn lexer_number_underdash_test() {
        let lexer = Lexer::new(r#"1_000_000"#);
        let actual = lexer.collect::<Vec<Token>>();
        assert!(actual == vec![
            Token::Number("1_000_000"),
        ])
    }

    #[test]
    fn lexer_decimal_number_test() {
        let lexer = Lexer::new(r#"3.14159265359"#);
        let actual = lexer.collect::<Vec<Token>>();
        assert!(actual == vec![
            Token::Number("3.14159265359"),
        ])
    }

    #[test]
    fn lexer_function_test() {
        let lexer = Lexer::new(r#"fn sum(a, b) {
            return a + b
        }"#);
        let actual = lexer.collect::<Vec<Token>>();
        assert!(actual == vec![
            Token::Func,
            Token::Identifier("sum"),
            Token::LeftParen,
            Token::Identifier("a"),
            Token::Comma,
            Token::Identifier("b"),
            Token::RightParen,
            Token::LeftBracket,
            Token::EOL,
            Token::Return,
            Token::Identifier("a"),
            Token::Plus,
            Token::Identifier("b"),
            Token::EOL,
            Token::RightBracket
        ])
    }
}
