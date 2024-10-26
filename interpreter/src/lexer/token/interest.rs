use crate::lexer::*;

pub trait Interest {
    fn interest_in_char(&self, ch: &char) -> bool;
    fn interest_in_identifier(&self, ident: &str) -> bool;
}

impl Interest for KindId {
    fn interest_in_char(&self, ch: &char) -> bool {
        match self {
            Self::If
            | Self::Else
            | Self::While
            | Self::Loop
            | Self::For
            | Self::Return
            | Self::Let
            | Self::True
            | Self::False
            | Self::EOF
            | Self::BOF => false,
            Self::Whitespace => ch.is_whitespace(),
            Self::Identifier => ch.is_alphabetic(),
            Self::Number => ch.is_alphanumeric(),
            Self::String => &'"' == ch,
            Self::InterpolatedString => &'\'' == ch,
            Self::Command => &'`' == ch,
            Self::SingleQuote => &'\'' == ch,
            Self::DoubleQuote => &'"' == ch,
            Self::Tilde => &'~' == ch,
            Self::Backtick => &'`' == ch,
            Self::Question => &'?' == ch,
            Self::Dollar => &'$' == ch,
            Self::At => &'@' == ch,
            Self::Pound => &'#' == ch,
            Self::Plus => &'+' == ch,
            Self::Minus => &'-' == ch,
            Self::Star => &'*' == ch,
            Self::Slash => &'/' == ch,
            Self::Percent => &'%' == ch,
            Self::Equals => &'=' == ch,
            Self::EqualEqual => &'=' == ch,
            Self::BangEqual => &'!' == ch,
            Self::Less => &'<' == ch,
            Self::LessEqual => &'<' == ch,
            Self::Greater => &'>' == ch,
            Self::GreaterEqual => &'>' == ch,
            Self::And => &'&' == ch,
            Self::Or => &'|' == ch,
            Self::VerticalBar => &'|' == ch,
            Self::Bang => &'!' == ch,
            Self::PlusEqual => &'+' == ch,
            Self::MinusEqual => &'-' == ch,
            Self::StarEqual => &'*' == ch,
            Self::SlashEqual => &'/' == ch,
            Self::LeftParen => &'(' == ch,
            Self::RightParen => &')' == ch,
            Self::LeftBrace => &'{' == ch,
            Self::RightBrace => &'}' == ch,
            Self::LeftBracket => &'[' == ch,
            Self::RightBracket => &']' == ch,
            Self::Comma => &',' == ch,
            Self::Colon => &':' == ch,
            Self::Dot => &'.' == ch,
            Self::DotDot => &'.' == ch,
            Self::Semicolon => &';' == ch,
            Self::Arrow => &'-' == ch,
            Self::DoubleArrow => &'=' == ch,
            Self::Comment => &'/' == ch,
            Self::Meta => &'/' == ch,
            Self::LF => &'\n' == ch,
            Self::CR => &'\r' == ch,
            Self::CRLF => &'\r' == ch,
        }
    }
    fn interest_in_identifier(&self, ident: &str) -> bool {
        match self {
            Self::If => "if" == ident,
            Self::Else => "else" == ident,
            Self::While => "while" == ident,
            Self::Loop => "loop" == ident,
            Self::For => "for" == ident,
            Self::Return => "return" == ident,
            Self::Let => "let" == ident,
            Self::True => "true" == ident,
            Self::False => "false" == ident,
            Self::Identifier => true,
            Self::Number
            | Self::String
            | Self::InterpolatedString
            | Self::Command
            | Self::SingleQuote
            | Self::DoubleQuote
            | Self::Tilde
            | Self::Backtick
            | Self::Question
            | Self::Dollar
            | Self::At
            | Self::Pound
            | Self::Plus
            | Self::Minus
            | Self::Star
            | Self::Slash
            | Self::Percent
            | Self::Equals
            | Self::EqualEqual
            | Self::BangEqual
            | Self::Less
            | Self::LessEqual
            | Self::Greater
            | Self::GreaterEqual
            | Self::And
            | Self::Or
            | Self::VerticalBar
            | Self::Bang
            | Self::PlusEqual
            | Self::MinusEqual
            | Self::StarEqual
            | Self::SlashEqual
            | Self::LeftParen
            | Self::RightParen
            | Self::LeftBrace
            | Self::RightBrace
            | Self::LeftBracket
            | Self::RightBracket
            | Self::Comma
            | Self::Colon
            | Self::Dot
            | Self::DotDot
            | Self::Semicolon
            | Self::Arrow
            | Self::DoubleArrow
            | Self::Comment
            | Self::Meta
            | Self::LF
            | Self::CR
            | Self::CRLF
            | Self::EOF
            | Self::BOF
            | Self::Whitespace => false,
        }
    }
}
