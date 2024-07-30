mod styled;

use terminal_size::terminal_size;

pub fn print<T: AsRef<str>>(content: T) {
    println!("{}", styled::apply(term_width(), content));
}

pub fn styled<T: AsRef<str>>(content: T) -> String {
    styled::apply(term_width(), content)
}

fn term_width() -> usize {
    terminal_size().map(|(w, _)| w.0).unwrap_or(250) as usize
}
