use std::fmt;

#[enum_ids::enum_ids]
#[derive(Debug, PartialEq, Clone)]
pub enum Keyword {
    /// The `if` keyword.
    If,
    /// The `else` keyword.
    Else,
    /// The `while` keyword.
    While,
    /// The `loop` keyword.
    Loop,
    /// The `for` keyword.
    For,
    /// The `each` keyword.
    Each,
    /// The `in` keyword.
    In,
    /// The `return` keyword.
    Return,
    /// The `break` keyword.
    Break,
    /// The `let` keyword.
    Let,
    /// The `Join` keyword.
    Join,
    /// The `oneof` keyword.
    OneOf,
    /// The boolean literal `true`.
    True,
    /// The boolean literal `false`.
    False,
    /// The `str` type-keyword.
    Str,
    /// The `bool` type-keyword.
    Bool,
    /// The `num` type-keyword.
    Num,
    /// The `Vec` type-keyword.
    Vec,
}

impl TryFrom<String> for Keyword {
    type Error = ();
    fn try_from(value: String) -> Result<Self, Self::Error> {
        KeywordId::as_vec()
            .into_iter()
            .find(|kw| Into::<Keyword>::into(kw).to_string() == value)
            .map(|kw| Into::<Keyword>::into(&kw))
            .ok_or(())
    }
}

impl Keyword {
    pub fn length(&self) -> usize {
        self.to_string().len()
    }
}

impl fmt::Display for Keyword {
    /// Formats the `Keyword` variant into its corresponding string representation.
    ///
    /// This implementation is used to convert a `Keyword` into a human-readable string,
    /// which is useful for debugging and error messages.
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
                Self::Each => "each".to_owned(),
                Self::Return => "return".to_owned(),
                Self::Break => "break".to_owned(),
                Self::Let => "let".to_owned(),
                Self::In => "in".to_owned(),
                Self::OneOf => "oneof".to_owned(),
                Self::Join => "join".to_owned(),
                Self::True => "true".to_owned(),
                Self::False => "false".to_owned(),
                Self::Str => "str".to_owned(),
                Self::Bool => "bool".to_owned(),
                Self::Num => "num".to_owned(),
                Self::Vec => "Vec".to_owned(),
            }
        )
    }
}

impl From<&KeywordId> for Keyword {
    fn from(value: &KeywordId) -> Self {
        match value {
            KeywordId::If => Keyword::If,
            KeywordId::Else => Keyword::Else,
            KeywordId::While => Keyword::While,
            KeywordId::Loop => Keyword::Loop,
            KeywordId::For => Keyword::For,
            KeywordId::Each => Keyword::Each,
            KeywordId::Return => Keyword::Return,
            KeywordId::Break => Keyword::Break,
            KeywordId::Let => Keyword::Let,
            KeywordId::In => Keyword::In,
            KeywordId::OneOf => Keyword::OneOf,
            KeywordId::Join => Keyword::Join,
            KeywordId::True => Keyword::True,
            KeywordId::False => Keyword::False,
            KeywordId::Str => Keyword::Str,
            KeywordId::Bool => Keyword::Bool,
            KeywordId::Num => Keyword::Num,
            KeywordId::Vec => Keyword::Vec,
        }
    }
}
