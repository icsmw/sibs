use crate::lexer::*;
use std::fmt;

#[enum_ids::enum_ids]
#[derive(Debug, PartialEq, Clone)]
pub enum Kind {
    If,
    Else,
    While,
    Loop,
    For,
    Return,
    Let,
    True,
    False,
    Identifier(String),
    Number(f64),
    String(String),
    InterpolatedString(Vec<StringPart>), // concatenated string
    Command(Vec<StringPart>),            // string with command
    SingleQuote,                         // '
    DoubleQuote,                         // "
    Tilde,                               // ~
    Backtick,                            // `
    Question,                            // ?
    Dollar,                              // $
    At,                                  // @
    Pound,                               // #
    Plus,                                // +
    Minus,                               // -
    Star,                                // *
    Slash,                               // /
    Percent,                             // %
    Equals,                              // =
    EqualEqual,                          // ==
    BangEqual,                           // !=
    Less,                                // <
    LessEqual,                           // <=
    Greater,                             // >
    GreaterEqual,                        // >=
    And,                                 // &&
    Or,                                  // ||
    VerticalBar,                         // |
    Bang,                                // !
    PlusEqual,                           // +=
    MinusEqual,                          // -=
    StarEqual,                           // *=
    SlashEqual,                          // /=
    LeftParen,                           // (
    RightParen,                          // )
    LeftBrace,                           // {
    RightBrace,                          // }
    LeftBracket,                         // [
    RightBracket,                        // ]
    Comma,                               // ,
    Dot,                                 // .
    DotDot,                              // ..
    Semicolon,                           // ;
    Colon,                               // :
    Arrow,                               // ->
    DoubleArrow,                         // =>
    Whitespace(String),                  // any whitespaces
    Comment(String),                     // //
    Meta(String),                        // ///
    LF,                                  // \n Line Feed
    CR,                                  // \r Carriage Return
    CRLF,                                // \r\n
    EOF,
    BOF,
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::If => "if".to_owned(),
                Self::Else => "else".to_owned(),
                Self::While => "while".to_owned(),
                Self::Loop => "loop".to_owned(),
                Self::For => "for".to_owned(),
                Self::Return => "return".to_owned(),
                Self::Let => "let".to_owned(),
                Self::True => "true".to_owned(),
                Self::False => "false".to_owned(),
                Self::Identifier(s) => s.clone(),
                Self::Number(n) => n.to_string(),
                Self::String(s) => format!("\"{s}\""),
                Self::Whitespace(s) => s.clone(),
                Self::InterpolatedString(s) => s
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(""),
                Self::Command(s) => s
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(""),
                Self::SingleQuote => "'".to_owned(),
                Self::DoubleQuote => "\"".to_owned(),
                Self::Tilde => "~".to_owned(),
                Self::Backtick => "`".to_owned(),
                Self::Question => "?".to_owned(),
                Self::Dollar => "$".to_owned(),
                Self::At => "@".to_owned(),
                Self::Pound => "#".to_owned(),
                Self::Plus => "+".to_owned(),
                Self::Minus => "-".to_owned(),
                Self::Star => "*".to_owned(),
                Self::Slash => "/".to_owned(),
                Self::Percent => "%".to_owned(),
                Self::Equals => "=".to_owned(),
                Self::EqualEqual => "==".to_owned(),
                Self::BangEqual => "!=".to_owned(),
                Self::Less => "<".to_owned(),
                Self::LessEqual => "<=".to_owned(),
                Self::Greater => ">".to_owned(),
                Self::GreaterEqual => ">=".to_owned(),
                Self::And => "&&".to_owned(),
                Self::Or => "||".to_owned(),
                Self::VerticalBar => "|".to_owned(),
                Self::Bang => "!".to_owned(),
                Self::PlusEqual => "+=".to_owned(),
                Self::MinusEqual => "-=".to_owned(),
                Self::StarEqual => "*=".to_owned(),
                Self::SlashEqual => "/=".to_owned(),
                Self::LeftParen => "(".to_owned(),
                Self::RightParen => ")".to_owned(),
                Self::LeftBrace => "{".to_owned(),
                Self::RightBrace => "}".to_owned(),
                Self::LeftBracket => "[".to_owned(),
                Self::RightBracket => "]".to_owned(),
                Self::Comma => ",".to_owned(),
                Self::Colon => ":".to_owned(),
                Self::Dot => ".".to_owned(),
                Self::DotDot => "..".to_owned(),
                Self::Semicolon => ";".to_owned(),
                Self::Arrow => "->".to_owned(),
                Self::DoubleArrow => "=>".to_owned(),
                Self::Comment(s) => format!("//{s}"),
                Self::Meta(s) => format!("///{s}"),
                Self::LF => "\n".to_string(),
                Self::CR => "\r".to_string(),
                Self::CRLF => "\r\n".to_owned(),
                Self::EOF | Self::BOF => String::new(),
            }
        )
    }
}

impl fmt::Display for KindId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::If => "if".to_owned(),
                Self::Else => "else".to_owned(),
                Self::While => "while".to_owned(),
                Self::Loop => "loop".to_owned(),
                Self::For => "for".to_owned(),
                Self::Return => "return".to_owned(),
                Self::Let => "let".to_owned(),
                Self::True => "true".to_owned(),
                Self::False => "false".to_owned(),
                Self::SingleQuote => "'".to_owned(),
                Self::DoubleQuote => "\"".to_owned(),
                Self::Tilde => "~".to_owned(),
                Self::Backtick => "`".to_owned(),
                Self::Question => "?".to_owned(),
                Self::Dollar => "$".to_owned(),
                Self::At => "@".to_owned(),
                Self::Pound => "#".to_owned(),
                Self::Plus => "+".to_owned(),
                Self::Minus => "-".to_owned(),
                Self::Star => "*".to_owned(),
                Self::Slash => "/".to_owned(),
                Self::Percent => "%".to_owned(),
                Self::Equals => "=".to_owned(),
                Self::EqualEqual => "==".to_owned(),
                Self::BangEqual => "!=".to_owned(),
                Self::Less => "<".to_owned(),
                Self::LessEqual => "<=".to_owned(),
                Self::Greater => ">".to_owned(),
                Self::GreaterEqual => ">=".to_owned(),
                Self::And => "&&".to_owned(),
                Self::Or => "||".to_owned(),
                Self::VerticalBar => "|".to_owned(),
                Self::Bang => "!".to_owned(),
                Self::PlusEqual => "+=".to_owned(),
                Self::MinusEqual => "-=".to_owned(),
                Self::StarEqual => "*=".to_owned(),
                Self::SlashEqual => "/=".to_owned(),
                Self::LeftParen => "(".to_owned(),
                Self::RightParen => ")".to_owned(),
                Self::LeftBrace => "{".to_owned(),
                Self::RightBrace => "}".to_owned(),
                Self::LeftBracket => "[".to_owned(),
                Self::RightBracket => "]".to_owned(),
                Self::Comma => ",".to_owned(),
                Self::Colon => ":".to_owned(),
                Self::Dot => ".".to_owned(),
                Self::DotDot => "..".to_owned(),
                Self::Semicolon => ";".to_owned(),
                Self::Arrow => "->".to_owned(),
                Self::DoubleArrow => "=>".to_owned(),
                Self::Comment => "//".to_owned(),
                Self::Meta => "///".to_owned(),
                Self::LF => '\n'.to_string(),
                Self::CR => '\r'.to_string(),
                Self::CRLF => "\r\n".to_owned(),
                Self::Identifier
                | Self::Number
                | Self::String
                | Self::InterpolatedString
                | Self::Command
                | Self::EOF
                | Self::BOF
                | Self::Whitespace => String::new(),
            }
        )
    }
}

impl TryFrom<KindId> for Kind {
    type Error = E;
    fn try_from(id: KindId) -> Result<Self, Self::Error> {
        match id {
            KindId::If => Ok(Kind::If),
            KindId::Else => Ok(Kind::Else),
            KindId::While => Ok(Kind::While),
            KindId::Loop => Ok(Kind::Loop),
            KindId::For => Ok(Kind::For),
            KindId::Return => Ok(Kind::Return),
            KindId::Let => Ok(Kind::Let),
            KindId::True => Ok(Kind::True),
            KindId::False => Ok(Kind::False),
            KindId::SingleQuote => Ok(Kind::SingleQuote),
            KindId::DoubleQuote => Ok(Kind::DoubleQuote),
            KindId::Tilde => Ok(Kind::Tilde),
            KindId::Backtick => Ok(Kind::Backtick),
            KindId::Question => Ok(Kind::Question),
            KindId::Dollar => Ok(Kind::Dollar),
            KindId::At => Ok(Kind::At),
            KindId::Pound => Ok(Kind::Pound),
            KindId::Plus => Ok(Kind::Plus),
            KindId::Minus => Ok(Kind::Minus),
            KindId::Star => Ok(Kind::Star),
            KindId::Slash => Ok(Kind::Slash),
            KindId::Percent => Ok(Kind::Percent),
            KindId::Equals => Ok(Kind::Equals),
            KindId::EqualEqual => Ok(Kind::EqualEqual),
            KindId::BangEqual => Ok(Kind::BangEqual),
            KindId::Less => Ok(Kind::Less),
            KindId::LessEqual => Ok(Kind::LessEqual),
            KindId::Greater => Ok(Kind::Greater),
            KindId::GreaterEqual => Ok(Kind::GreaterEqual),
            KindId::And => Ok(Kind::And),
            KindId::Or => Ok(Kind::Or),
            KindId::VerticalBar => Ok(Kind::VerticalBar),
            KindId::Bang => Ok(Kind::Bang),
            KindId::PlusEqual => Ok(Kind::PlusEqual),
            KindId::MinusEqual => Ok(Kind::MinusEqual),
            KindId::StarEqual => Ok(Kind::StarEqual),
            KindId::SlashEqual => Ok(Kind::SlashEqual),
            KindId::LeftParen => Ok(Kind::LeftParen),
            KindId::RightParen => Ok(Kind::RightParen),
            KindId::LeftBrace => Ok(Kind::LeftBrace),
            KindId::RightBrace => Ok(Kind::RightBrace),
            KindId::LeftBracket => Ok(Kind::LeftBracket),
            KindId::RightBracket => Ok(Kind::RightBracket),
            KindId::Comma => Ok(Kind::Comma),
            KindId::Colon => Ok(Kind::Colon),
            KindId::Dot => Ok(Kind::Dot),
            KindId::DotDot => Ok(Kind::DotDot),
            KindId::Semicolon => Ok(Kind::Semicolon),
            KindId::Arrow => Ok(Kind::Arrow),
            KindId::DoubleArrow => Ok(Kind::DoubleArrow),
            KindId::LF => Ok(Kind::LF),
            KindId::CR => Ok(Kind::CR),
            KindId::CRLF => Ok(Kind::CRLF),
            KindId::EOF => Ok(Kind::EOF),
            KindId::BOF => Ok(Kind::BOF),
            KindId::Identifier
            | KindId::Number
            | KindId::String
            | KindId::Whitespace
            | KindId::InterpolatedString
            | KindId::Command
            | KindId::Comment
            | KindId::Meta => Err(E::CannotConvertToKind(id)),
        }
    }
}

impl TryFrom<KindId> for char {
    type Error = E;
    fn try_from(id: KindId) -> Result<Self, Self::Error> {
        match id {
            KindId::If
            | KindId::Else
            | KindId::While
            | KindId::Loop
            | KindId::For
            | KindId::Return
            | KindId::Let
            | KindId::True
            | KindId::False
            | KindId::Identifier
            | KindId::Number
            | KindId::String
            | KindId::InterpolatedString
            | KindId::Command
            | KindId::EOF
            | KindId::EqualEqual
            | KindId::BangEqual
            | KindId::LessEqual
            | KindId::GreaterEqual
            | KindId::And
            | KindId::Or
            | KindId::PlusEqual
            | KindId::MinusEqual
            | KindId::StarEqual
            | KindId::SlashEqual
            | KindId::DotDot
            | KindId::Arrow
            | KindId::DoubleArrow
            | KindId::Comment
            | KindId::Meta
            | KindId::BOF
            | KindId::CRLF
            | KindId::Whitespace => Err(E::CannotConverToChar(id)),
            KindId::SingleQuote => Ok('\''),
            KindId::DoubleQuote => Ok('"'),
            KindId::Tilde => Ok('~'),
            KindId::Backtick => Ok('`'),
            KindId::Question => Ok('?'),
            KindId::Dollar => Ok('$'),
            KindId::At => Ok('@'),
            KindId::Pound => Ok('#'),
            KindId::Plus => Ok('+'),
            KindId::Minus => Ok('-'),
            KindId::Star => Ok('*'),
            KindId::Slash => Ok('/'),
            KindId::Percent => Ok('%'),
            KindId::Equals => Ok('='),
            KindId::Less => Ok('<'),
            KindId::Greater => Ok('>'),
            KindId::VerticalBar => Ok('|'),
            KindId::Bang => Ok('!'),
            KindId::LeftParen => Ok('('),
            KindId::RightParen => Ok(')'),
            KindId::LeftBrace => Ok('{'),
            KindId::RightBrace => Ok('}'),
            KindId::LeftBracket => Ok('['),
            KindId::RightBracket => Ok(']'),
            KindId::Comma => Ok(','),
            KindId::Colon => Ok(':'),
            KindId::Dot => Ok('.'),
            KindId::Semicolon => Ok(';'),
            KindId::LF => Ok('\n'),
            KindId::CR => Ok('\r'),
        }
    }
}
