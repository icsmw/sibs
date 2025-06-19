use std::fmt;

pub enum ErrorSource {
    Parser,
    Semantic,
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
