use crate::inf::term::styled::Styled;
use console::Style;
use regex::{Captures, Regex};

pub struct Color {
    reg: Regex,
}
impl Color {
    pub fn new(_width: usize) -> Self {
        Self {
            reg: Regex::new(r"\[color:(?<color>[\w]*)\](?<inner>.*)\[/color\]")
                .expect("Regex for Styled::Color"),
        }
    }
    pub fn color(str: &str) -> Style {
        match str.to_ascii_lowercase().as_str() {
            "black" => Style::new().black(),
            "red" => Style::new().red(),
            "green" => Style::new().green(),
            "yellow" => Style::new().yellow(),
            "blue" => Style::new().blue(),
            "magenta" => Style::new().magenta(),
            "cyan" => Style::new().cyan(),
            "white" => Style::new().white(),
            _ => Style::new().white(),
        }
    }
}

impl Styled for Color {
    fn apply(&mut self, str: &str) -> String {
        self.reg
            .replace_all(str, |caps: &Captures| {
                Color::color(&caps["color"])
                    .apply_to(&caps["inner"])
                    .to_string()
            })
            .to_string()
    }
}

#[test]
fn test() {
    let mut bold = Color::new(40);
    assert_eq!(
        bold.apply("_[b]_[/b]_"),
        String::from("_\u{1b}[1m_\u{1b}[0m_")
    );
    assert_eq!(
        bold.apply("[b]_[/b]_"),
        String::from("\u{1b}[1m_\u{1b}[0m_")
    );
    assert_eq!(
        bold.apply("_[b]_[/b]"),
        String::from("_\u{1b}[1m_\u{1b}[0m")
    );
    assert_eq!(bold.apply("[b]_[/b]"), String::from("\u{1b}[1m_\u{1b}[0m"));
    assert_eq!(bold.apply("[b][/b]"), String::from("\u{1b}[1m\u{1b}[0m"));
}
