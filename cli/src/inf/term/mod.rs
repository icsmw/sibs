mod styled;

use std::fmt::Display;

use terminal_size::terminal_size;

pub fn print<'a, T>(content: &T)
where
    T: 'a + ToOwned + ToString + Display + ?Sized,
{
    println!("{}", styled::apply(term_width(), content));
}

pub fn styled<'a, T>(content: &T) -> String
where
    T: 'a + ToOwned + ToString + Display + ?Sized,
{
    styled::apply(term_width(), content)
}

fn term_width() -> usize {
    terminal_size().map(|(w, _)| w.0).unwrap_or(250) as usize
}
