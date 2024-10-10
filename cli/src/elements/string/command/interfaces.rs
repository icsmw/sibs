use crate::elements::{Command, Element, TokenGetter};
use std::fmt;

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "`{}`",
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

impl TokenGetter for Command {
    fn token(&self) -> usize {
        self.token
    }
}
