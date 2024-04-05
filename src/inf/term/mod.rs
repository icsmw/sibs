mod styled;

use std::fmt::Display;

use terminal_size::terminal_size;

const TAB_SPACES: usize = 4;

pub fn print<'a, T>(content: &T)
where
    T: 'a + ToOwned + ToString + Display + ?Sized,
{
    println!("{}", styled::apply(term_width(), content));
}

pub fn tab(tab: usize) -> String {
    " ".repeat(TAB_SPACES * tab)
}

fn term_width() -> usize {
    terminal_size().map(|(w, _)| w.0).unwrap_or(250) as usize
}
