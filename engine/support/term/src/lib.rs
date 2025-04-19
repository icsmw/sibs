mod styled;

pub(crate) use console::Style;
pub(crate) use regex::{Captures, Regex};
pub(crate) use styled::*;
pub(crate) use terminal_size::terminal_size;

pub fn print<T: AsRef<str>>(content: T) {
    println!("{}", styled::apply(term_width(), content));
}

pub fn styled<T: AsRef<str>>(content: T) -> String {
    styled::apply(term_width(), content)
}

fn term_width() -> usize {
    terminal_size().map(|(w, _)| w.0).unwrap_or(250) as usize
}
