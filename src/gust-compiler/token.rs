#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Invalid,
    EOL,

    // Single char tokens
    LeftBracket,
    RightBracket,
    LeftParen,
    RightParen,
    LeftSquareBracket,
    RightSquareBracket,
    Comma,
    Dot,
    Minus,
    Plus,
    Colon,
    Slash,
    Star,

    // Conditional tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Keywords
    And,
    Or,
    If,
    Else,
    Func,
    For,
    While,
    Let,
    Nil,
    Return,
    Print,
    True,
    False,

    // Others
    Identifier(&'a str),
    String(&'a str),
    Number(&'a str),
}
