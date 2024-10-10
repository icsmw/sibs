use crate::elements::{TokenGetter, VariableVariants};
use std::fmt;

impl TokenGetter for VariableVariants {
    fn token(&self) -> usize {
        self.token
    }
}

impl fmt::Display for VariableVariants {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.values
                .iter()
                .map(|v| v.as_string().expect("Value variant can be only String"))
                .collect::<Vec<String>>()
                .join(" | ")
        )
    }
}
