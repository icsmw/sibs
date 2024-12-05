use std::ops::RangeInclusive;

use crate::*;

use proptest::prelude::*;

/// Generates a `BoxedStrategy` with random `Kind` instance,
///
/// # Returns
///
/// A `BoxedStrategy` with random`Kind` instance.
pub fn rnd_kind() -> BoxedStrategy<Kind> {
    prop::strategy::Union::new(KindId::as_vec().into_iter().map(kind)).boxed()
}

/// Generates a `BoxedStrategy` with random `Kind` instance, including specified Kind.
///
/// # Arguments
///
/// * `includes` - A vector of `KindId` instances to include from the generated strategies.
///
/// # Returns
///
/// A `BoxedStrategy` with random`Kind` instance.
pub fn rnd_kind_with(includes: Vec<KindId>) -> BoxedStrategy<Kind> {
    prop::strategy::Union::new(includes.into_iter().map(kind)).boxed()
}

/// Generates a `BoxedStrategy` with random `Kind` instance, including specified Keyword.
///
/// # Arguments
///
/// * `includes` - A vector of `KeywordId` instances to include from the generated strategies.
///
/// # Returns
///
/// A `BoxedStrategy` with random`Kind` instance.
pub fn rnd_keyword_with(includes: Vec<KeywordId>) -> BoxedStrategy<Kind> {
    prop::strategy::Union::new(includes.into_iter().map(keyword))
        .boxed()
        .prop_map(Kind::Keyword)
        .boxed()
}

/// Generates a `BoxedStrategy` with random `Kind` instance, excluding specified exceptions.
///
/// This function is used in property-based testing to create random sequences
/// of `Kind` tokens while omitting any kinds listed in the `exceptions`.
///
/// # Arguments
///
/// * `exceptions` - A vector of `KindId` instances to exclude from the generated strategies.
///
/// # Returns
///
/// A `BoxedStrategy` with random`Kind` instance.
pub fn rnd_kind_without(exceptions: Vec<KindId>) -> BoxedStrategy<Kind> {
    let includes = KindId::as_vec()
        .into_iter()
        .filter(|id| !exceptions.contains(id))
        .collect::<Vec<KindId>>();
    prop::strategy::Union::new(includes.into_iter().map(kind)).boxed()
}

/// Generates random string. Sensitive to SIBS_DEBUG_PROPTEST environment variable.
/// In case of SIBS_DEBUG_PROPTEST = true, it will generate a random ASCII string instead
/// completely random.
///
/// # Arguments
///
/// * `range` - The `RangeInclusive<usize>` to define min and max length of string.
///
/// # Returns
///
/// `BoxedStrategy<String>`
pub fn gen_string(range: RangeInclusive<usize>) -> BoxedStrategy<String> {
    if common::is_proptest_debug() {
        "[a-z][a-z0-9]*".prop_map(String::from).boxed()
    } else {
        proptest::collection::vec(any::<char>(), range)
            .prop_map(|chars| chars.into_iter().collect::<String>())
            .boxed()
    }
}

/// Generates strategies for creating `Kind` instances based on the given `KindId`.
///
/// This function maps a `KindId` to one or more `BoxedStrategy<Kind>` instances,
/// which are used in property-based testing to generate random `Kind` tokens.
///
/// # Arguments
///
/// * `id` - The `KindId` for which to generate strategies.
///
/// # Returns
///
/// `BoxedStrategy<Kind>` instance corresponding to the provided `KindId`.
pub fn kind(id: KindId) -> BoxedStrategy<Kind> {
    match id {
        KindId::Keyword => prop::strategy::Union::new(
            KeywordId::as_vec()
                .into_iter()
                .map(|kw| Just(Into::<Keyword>::into(&kw)))
                .collect::<Vec<Just<Keyword>>>(),
        )
        .boxed()
        .prop_map(Kind::Keyword)
        .boxed(),
        KindId::Question
        | KindId::SingleQuote
        | KindId::DoubleQuote
        | KindId::Tilde
        | KindId::Backtick
        | KindId::Dollar
        | KindId::At
        | KindId::Pound
        | KindId::Plus
        | KindId::Minus
        | KindId::Star
        | KindId::Slash
        | KindId::Percent
        | KindId::Equals
        | KindId::EqualEqual
        | KindId::BangEqual
        | KindId::Less
        | KindId::LessEqual
        | KindId::Greater
        | KindId::GreaterEqual
        | KindId::And
        | KindId::Or
        | KindId::VerticalBar
        | KindId::Bang
        | KindId::PlusEqual
        | KindId::MinusEqual
        | KindId::StarEqual
        | KindId::SlashEqual
        | KindId::LeftParen
        | KindId::RightParen
        | KindId::LeftBrace
        | KindId::RightBrace
        | KindId::LeftBracket
        | KindId::RightBracket
        | KindId::Comma
        | KindId::Colon
        | KindId::Dot
        | KindId::DotDot
        | KindId::Semicolon
        | KindId::Arrow
        | KindId::DoubleArrow
        | KindId::BOF
        | KindId::EOF
        | KindId::LF
        | KindId::CR
        | KindId::CRLF
        | KindId::Whitespace => {
            Just(id.try_into().expect("Failed to convert KindId to Kind")).boxed()
        }
        KindId::Identifier => "[a-z][a-z0-9]*"
            .prop_map(String::from)
            .prop_filter("conflicts", |s| {
                if let Some(ch) = s.chars().next() {
                    if ch.is_numeric() {
                        return false;
                    }
                }
                !KeywordId::as_vec()
                    .into_iter()
                    .map(|kw| Into::<Keyword>::into(&kw).to_string())
                    .any(|kw| &kw == s)
            })
            .prop_map(Kind::Identifier)
            .boxed(),
        KindId::Number => proptest::num::f64::NORMAL
            .prop_filter("Finite f64", |x| x.is_finite())
            .prop_map(Kind::Number)
            .boxed(),
        KindId::String => gen_string(RangeInclusive::new(0, 200))
            .prop_map(|str| {
                Kind::String(
                    str.chars()
                        .map(|ch| {
                            if ch == '"' {
                                "\\\"".to_string()
                            } else if ch == '\\' {
                                "_".to_string()
                            } else {
                                ch.to_string()
                            }
                        })
                        .collect::<String>(),
                )
            })
            .boxed(),
        KindId::InterpolatedString => (1..10, prop_oneof![Just(0u8), Just(1u8)])
            .prop_flat_map(|(count, first)| {
                let mut parts: Vec<BoxedStrategy<StringPart>> = vec![Just(StringPart::Open(
                    Token::by_pos(Kind::SingleQuote, &Uuid::new_v4(), 0, 0),
                ))
                .boxed()];
                let mut variant = first;
                for _ in 0..count {
                    parts.push(StringPart::arbitrary_with((
                        variant,
                        KindId::SingleQuote.try_into().expect("SingleQuote as char"),
                    )));
                    variant = if variant == 0 { 1 } else { 0 };
                }
                parts.push(
                    Just(StringPart::Close(Token::by_pos(
                        Kind::SingleQuote,
                        &Uuid::new_v4(),
                        0,
                        0,
                    )))
                    .boxed(),
                );
                parts.prop_map(Kind::InterpolatedString).boxed()
            })
            .boxed(),
        KindId::Command => (1..10, prop_oneof![Just(0u8), Just(1u8)])
            .prop_flat_map(|(count, first)| {
                let mut parts: Vec<BoxedStrategy<StringPart>> = vec![Just(StringPart::Open(
                    Token::by_pos(Kind::Backtick, &Uuid::new_v4(), 0, 0),
                ))
                .boxed()];
                let mut variant = first;
                for _ in 0..count {
                    parts.push(StringPart::arbitrary_with((
                        variant,
                        KindId::Backtick.try_into().expect("Backtick as char"),
                    )));
                    variant = if variant == 0 { 1 } else { 0 };
                }
                parts.push(
                    Just(StringPart::Close(Token::by_pos(
                        Kind::Backtick,
                        &Uuid::new_v4(),
                        0,
                        0,
                    )))
                    .boxed(),
                );
                parts.prop_map(Kind::Command).boxed()
            })
            .boxed(),
        KindId::Comment => gen_string(RangeInclusive::new(0, 200))
            .prop_map(|str| {
                Kind::Comment(format!(
                    "comment:{}",
                    str.chars()
                        .map(|ch| {
                            if ch == '\n' || ch == '\r' || ch == '\\' {
                                "_".to_string()
                            } else {
                                ch.to_string()
                            }
                        })
                        .collect::<String>()
                ))
            })
            .boxed(),
        KindId::Meta => gen_string(RangeInclusive::new(0, 200))
            .prop_map(|str| {
                Kind::Meta(format!(
                    "meta:{}",
                    str.chars()
                        .map(|ch| {
                            if ch == '\n' || ch == '\r' || ch == '\\' {
                                "_".to_string()
                            } else {
                                ch.to_string()
                            }
                        })
                        .collect::<String>()
                ))
            })
            .boxed(),
    }
}

/// Generates strategies for creating `Keyword` instances based on the given `KindId`.
///
/// This function maps a `KeywordId` to one or more `BoxedStrategy<Keyword>` instances,
/// which are used in property-based testing to generate random `Keyword` tokens.
///
/// # Arguments
///
/// * `id` - The `KeywordId` for which to generate strategies.
///
/// # Returns
///
/// `BoxedStrategy<Keyword>` instance corresponding to the provided `KeywordId`.
pub fn keyword(id: KeywordId) -> BoxedStrategy<Keyword> {
    match id {
        KeywordId::If => Just(Keyword::If).boxed(),
        KeywordId::Else => Just(Keyword::Else).boxed(),
        KeywordId::While => Just(Keyword::While).boxed(),
        KeywordId::Loop => Just(Keyword::Loop).boxed(),
        KeywordId::For => Just(Keyword::For).boxed(),
        KeywordId::Each => Just(Keyword::Each).boxed(),
        KeywordId::Return => Just(Keyword::Return).boxed(),
        KeywordId::Break => Just(Keyword::Break).boxed(),
        KeywordId::Let => Just(Keyword::Let).boxed(),
        KeywordId::In => Just(Keyword::In).boxed(),
        KeywordId::OneOf => Just(Keyword::OneOf).boxed(),
        KeywordId::Fn => Just(Keyword::Fn).boxed(),
        KeywordId::Include => Just(Keyword::Include).boxed(),
        KeywordId::Mod => Just(Keyword::Mod).boxed(),
        KeywordId::Join => Just(Keyword::Join).boxed(),
        KeywordId::True => Just(Keyword::True).boxed(),
        KeywordId::False => Just(Keyword::False).boxed(),
        KeywordId::Str => Just(Keyword::Str).boxed(),
        KeywordId::Bool => Just(Keyword::Bool).boxed(),
        KeywordId::Num => Just(Keyword::Num).boxed(),
        KeywordId::Vec => Just(Keyword::Vec).boxed(),
        KeywordId::Private => Just(Keyword::Private).boxed(),
        KeywordId::Task => Just(Keyword::Task).boxed(),
        KeywordId::Component => Just(Keyword::Component).boxed(),
        KeywordId::Error => Just(Keyword::Error).boxed(),
    }
}

/// Adds bound Kinds. For example Comment and Meta needs to be around LF
pub fn add_bound_kinds(knd: Kind) -> Vec<Kind> {
    match &knd {
        Kind::Comment(..) | Kind::Meta(..) => {
            vec![Kind::LF, knd, Kind::LF]
        }
        _ => vec![knd],
    }
}
