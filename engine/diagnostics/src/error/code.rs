use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ErrorSource {
    Parser,
    Semantic,
    Lint,
    Runtime,
    Driver,
}

impl fmt::Display for ErrorSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Driver => "DR",
                Self::Parser => "PA",
                Self::Runtime => "RT",
                Self::Semantic => "SE",
                Self::Lint => "LI",
            }
        )
    }
}

pub trait ErrorCode {
    fn code(&self) -> &'static str;
    fn src(&self) -> ErrorSource;
    fn formattable(&self) -> String {
        format!("{}-{}", self.src(), self.code())
    }
}
