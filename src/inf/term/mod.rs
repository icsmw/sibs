mod styled;

use std::fmt::Display;
use styled::*;

use crate::elements::Metadata;
use console::{style, Style};
use terminal_size::terminal_size;

const TITLE_SPLITTER: &str = ">>";

/*
impl term::Display for Component {
    fn display(&self, md: Metadata, term: &mut Term) {
        term.bold("COMPONENT:\n");
        term.right();
        term.boldnl(&self.name);
        term.left();
        term.bold("\nTASKS:\n");
        term.right();
        self.elements.iter().for_each(|el| {
            if let Element::Task(el, _) = el {
                el.display(term);
            }
        });
        term.left();
    }
}

impl term::Display for Command {
    fn display(&self, md: &Metadata, term: &mut Term) {
        term.printnl(&self.pattern);
    }
}

impl term::Display for Task {
    fn display(&self, md: &Metadata, term: &mut Term) {
        term.bold(format!("{}[{}]", term.offset(), self.name.value));
        println!();
        term.right();
        term.print(format!(
            "{}USAGE: {}{}{}",
            term.offset(),
            self.name.value,
            if self.declarations.is_empty() {
                ""
            } else {
                " "
            },
            self.declarations
                .iter()
                .map(term::Display::to_string)
                .collect::<Vec<String>>()
                .join(" ")
        ));
        println!();
        self.block.display(term);
        term.left();
    }
}

impl term::Display for Meta {
    fn display(&self, md: &Metadata, term: &mut Term) {
        term.print_fmt(&self.as_lines());
    }
}


impl term::Display for Arguments {
    fn display(&self, term: &mut Term) {
        term.print_fmt(
            &[
                exertion::Scenario::desc(),
                exertion::Help::desc(),
                exertion::Trace::desc(),
                exertion::Output::desc(),
                exertion::LogFile::desc(),
                exertion::Format::desc(),
                exertion::Version::desc(),
            ]
            .iter()
            .flat_map(|desc| {
                [
                    vec![format!("{}>>{}", desc.key.join(", "), desc.desc)],
                    desc.pairs
                        .iter()
                        .map(|(key, value)| format!("{}>>{}", key, value))
                        .collect::<Vec<String>>(),
                ]
                .concat()
            })
            .collect::<Vec<String>>(),
        );
    }
}

*/

pub fn print<'a, T>(content: &T)
where
    T: 'a + ToOwned + ToString + Display + ?Sized,
{
    println!("{}", styled::apply(term_width(), content));
}

fn term_width() -> usize {
    terminal_size().map(|(w, _)| w.0).unwrap_or(250) as usize
}
