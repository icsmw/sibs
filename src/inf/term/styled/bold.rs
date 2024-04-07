use crate::inf::term::styled::Styled;
use console::Style;
use regex::{Captures, Regex};

pub struct Bold {
    reg: Regex,
}
impl Bold {
    pub fn new(_width: usize) -> Self {
        Self {
            reg: Regex::new(r"\[b\](.*?)\[/b\]").expect("Regex for Styled::Bold"),
        }
    }
}

impl Styled for Bold {
    fn apply(&mut self, str: &str) -> String {
        self.reg
            .replace_all(str, |caps: &Captures| {
                Style::new()
                    .bold()
                    .apply_to(
                        caps.get(1)
                            .expect("Styled::Bold: inner content extracted")
                            .as_str(),
                    )
                    .to_string()
            })
            .to_string()
    }
}

#[test]
fn test() {
    let mut bold = Bold::new(40);
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
    assert_eq!(
        bold.apply("[b]_[/b][b]_[/b]"),
        String::from("\u{1b}[1m_\u{1b}[0m\u{1b}[1m_\u{1b}[0m")
    );
}
