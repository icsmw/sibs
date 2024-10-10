use crate::elements::Meta;
use std::fmt;

impl fmt::Display for Meta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.inner
                .iter()
                .map(|v| format!("/// {v}"))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}
