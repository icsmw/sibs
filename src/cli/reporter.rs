use terminal_size::terminal_size;

use ansi_term::{Color, Style};

const TITLE_SPLITTER: &str = ">>";

pub trait Display {
    fn display(&self, _reporter: &mut Reporter) {}
    fn to_string(&self) -> String {
        String::new()
    }
}

pub struct Reporter {
    _offset: usize,
}

impl Reporter {
    pub fn new() -> Self {
        Self { _offset: 0 }
    }

    pub fn pair(&self, key: &str, value: &str) {
        println!(
            "{}{}: {value}",
            " ".repeat(self._offset),
            Color::White.bold().paint(key)
        );
    }

    pub fn print_fmt(&self, lines: &[&str]) {
        print_fmt(lines, self._offset);
    }

    pub fn print(&self, msg: &str) {
        print(msg, self._offset, None)
    }

    pub fn pairs(&self, pairs: Vec<(String, String)>) {
        let max = pairs
            .iter()
            .map(|pair| pair.0.len())
            .max()
            .unwrap_or_default();
        pairs.iter().for_each(|pair| {
            print!(
                "{}{}{} - ",
                self.offset(),
                Color::White.bold().paint(&pair.0),
                " ".repeat(max - pair.0.len()),
            );
            print(&pair.1, max + 3 + self._offset, None);
            println!();
        });
    }

    pub fn bold(&self, msg: &str) {
        print(msg, self._offset, Some(Color::White.bold()))
    }

    pub fn step_left(&mut self) {
        if self._offset > 0 {
            self._offset -= 4;
        }
    }

    pub fn step_right(&mut self) {
        self._offset += 4;
    }

    pub fn offset(&self) -> String {
        " ".repeat(self._offset).to_string()
    }
}

pub fn print_fmt(lines: &[&str], offset: usize) {
    let max = lines
        .iter()
        .map(|s| {
            let columns = s.split(TITLE_SPLITTER).collect::<Vec<&str>>();
            if columns.len() < 2 {
                0
            } else {
                columns.first().map(|s| s.len()).unwrap_or_default()
            }
        })
        .max()
        .unwrap_or_default();
    lines.iter().for_each(|line| {
        let mut columns = line.split(TITLE_SPLITTER).collect::<Vec<&str>>();
        print!("{}", " ".repeat(offset));
        if columns.len() < 2 {
            print(line, offset, Some(Color::White.normal()));
        } else {
            let first = columns.remove(0).trim();
            print!(
                "{}{} - ",
                Color::White.bold().paint(first),
                " ".repeat(max - first.len()),
            );
            print(columns.join(TITLE_SPLITTER).trim(), offset + max + 3, None);
        }
        println!();
    });
}

pub fn print(msg: &str, offset: usize, style: Option<Style>) {
    if msg.is_empty() {
        return;
    }
    let mut width = term_width();
    if width <= offset {
        width = offset * 2;
    }
    let mut cursor: usize = 0;
    loop {
        let mut next = cursor + (width - offset - 1);
        if next > msg.len() - 1 {
            next = msg.len() - 1;
        }
        let mut chunk = &msg[cursor..=next];
        if cursor > 0 {
            chunk = chunk.trim();
        }
        print!(
            "{}{}",
            if cursor == 0 {
                "".to_string()
            } else {
                " ".repeat(offset)
            },
            if let Some(style) = style.as_ref() {
                style.paint(chunk).to_string()
            } else {
                chunk.to_string()
            }
        );
        if next == msg.len() - 1 {
            break;
        } else {
            println!();
            cursor = next;
        }
    }
}

fn term_width() -> usize {
    terminal_size().map(|(w, _)| w.0).unwrap_or(250) as usize
}
