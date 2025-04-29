use crate::*;
use std::fmt;

/// Represents the various kinds of tokens that can be identified by the lexer.
///
/// This enumeration includes all possible token types that the lexer can produce,
/// which are then used by the parser for further syntactic analysis.
#[allow(clippy::upper_case_acronyms)]
#[enum_ids::enum_ids]
#[derive(Debug, PartialEq, Clone)]
pub enum Kind {
    /// The keyword.
    Keyword(Keyword),
    /// An identifier consisting of a string.
    Identifier(String),
    /// A numeric literal represented as a floating-point number.
    Number(f64),
    /// A string literal. Used in strings and commands
    Literal(String),
    /// A string literal.
    String(String),
    /// An interpolated string, represented as a vector of `StringPart`.
    InterpolatedString(Vec<StringPart>),
    /// A command string, represented as a vector of `StringPart`.
    Command(Vec<StringPart>),
    /// A single quote character (`'`).
    SingleQuote,
    /// A double quote character (`"`).
    DoubleQuote,
    /// A tilde character (`~`).
    Tilde,
    /// A backtick character (`` ` ``).
    Backtick,
    /// A question mark character (`?`).
    Question,
    /// A dollar sign character (`$`).
    Dollar,
    /// An at symbol character (`@`).
    At,
    /// A pound/hash character (`#`).
    Pound,
    /// A plus sign character (`+`).
    Plus,
    /// A minus sign character (`-`).
    Minus,
    /// An asterisk character (`*`).
    Star,
    /// A slash character (`/`).
    Slash,
    /// A percent sign character (`%`).
    Percent,
    /// An equals sign character (`=`).
    Equals,
    /// A double equals sign (`==`).
    EqualEqual,
    /// A not equals sign (`!=`).
    BangEqual,
    /// A less-than sign (`<`).
    Less,
    /// A less-than or equal to sign (`<=`).
    LessEqual,
    /// A greater-than sign (`>`).
    Greater,
    /// A greater-than or equal to sign (`>=`).
    GreaterEqual,
    /// A logical AND operator (`&&`).
    And,
    /// A logical OR operator (`||`).
    Or,
    /// A vertical bar character (`|`).
    VerticalBar,
    /// A bang/exclamation mark character (`!`).
    Bang,
    /// A plus equals sign (`+=`).
    PlusEqual,
    /// A minus equals sign (`-=`).
    MinusEqual,
    /// An asterisk equals sign (`*=`).
    StarEqual,
    /// A slash equals sign (`/=`).
    SlashEqual,
    /// A left parenthesis character (`(`).
    LeftParen,
    /// A right parenthesis character (`)`).
    RightParen,
    /// A left brace character (`{`).
    LeftBrace,
    /// A right brace character (`}`).
    RightBrace,
    /// A left bracket character (`[`).
    LeftBracket,
    /// A right bracket character (`]`).
    RightBracket,
    /// A comma character (`,`).
    Comma,
    /// A dot character (`.`).
    Dot,
    /// A double dot (`..`).
    DotDot,
    /// A semicolon character (`;`).
    Semicolon,
    /// A colon character (`:`).
    Colon,
    /// An arrow (`->`).
    Arrow,
    /// A double arrow (`=>`).
    DoubleArrow,
    /// A whitespace sequence.
    Whitespace(String),
    /// A single-line comment starting with `//`.
    Comment(String),
    /// A meta comment starting with `///`.
    Meta(String),
    /// A line feed character (`\n`).
    LF,
    /// A carriage return character (`\r`).
    CR,
    /// A carriage return followed by a line feed (`\r\n`).
    CRLF,
    /// Represents the end of the file.
    EOF,
    /// Represents the beginning of the file.
    BOF,
}

impl fmt::Display for Kind {
    /// Formats the `Kind` variant into its corresponding string representation.
    ///
    /// This implementation is used to convert a `Kind` into a human-readable string,
    /// which is useful for debugging and error messages.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Keyword(s) => s.to_string(),
                Self::Identifier(s) => s.clone(),
                Self::Number(n) => n.to_string(),
                Self::Literal(n) => n.to_string(),
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

/// Represents the unique identifier for each `Kind` variant.
///
/// This is typically used internally for more efficient comparisons and mappings.
impl fmt::Display for KindId {
    /// Formats the `KindId` variant into its corresponding string representation.
    ///
    /// Similar to the `Display` implementation for `Kind`, this converts a `KindId`
    /// into a human-readable string.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
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
                | Self::Keyword
                | Self::Number
                | Self::String
                | Self::Literal
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
    /// Attempts to convert a `KindId` into its corresponding `Kind` variant.
    ///
    /// # Errors
    ///
    /// Returns an error of type `E::CannotConvertToKind` if the `KindId` does not
    /// have a corresponding `Kind` variant.
    fn try_from(id: KindId) -> Result<Self, Self::Error> {
        match id {
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
            | KindId::Keyword
            | KindId::Number
            | KindId::String
            | KindId::Literal
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
    /// Attempts to convert a `KindId` into its corresponding `char`.
    ///
    /// # Errors
    ///
    /// Returns an error of type `E::CannotConverToChar` if the `KindId` does not
    /// correspond to a single character.
    fn try_from(id: KindId) -> Result<Self, Self::Error> {
        match id {
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
            _ => Err(E::CannotConverToChar(id)),
        }
    }
}
