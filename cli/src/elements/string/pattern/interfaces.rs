use crate::elements::{Element, PatternString, TokenGetter};
use std::fmt;

impl fmt::Display for PatternString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\"{}\"",
            self.elements
                .iter()
                .map(|el| {
                    if let Element::SimpleString(el, _) = el {
                        el.to_string()
                    } else {
                        format!("{{{el}}}",)
                    }
                })
                .collect::<Vec<String>>()
                .join("")
        )
    }
}

impl TokenGetter for PatternString {
    fn token(&self) -> usize {
        self.token
    }
}
