pub const KEYWORDS: [&str; 5] = ["let", "if", "for", "while", "fn"];
pub const DELIMIERS: [char; 8] = ['{', '}', '[', ']', '(', ')', '"', '\''];

#[derive(Debug, PartialEq)]
pub enum Token {
    Invalid,
    EOL,
    Assignment,
    Keyword(String),
    Operator(String),
    Number(i32),
    Identifier(String),
    Delimiter(char)
}