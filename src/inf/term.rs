use console::{style, Style};
use terminal_size::terminal_size;

const TITLE_SPLITTER: &str = ">>";

pub trait Display {
    fn display(&self, _reporter: &mut Term) {}
    fn to_string(&self) -> String {
        String::new()
    }
}

pub struct Term {
    _offset: usize,
}

impl Term {
    pub fn new() -> Self {
        Self { _offset: 0 }
    }

    pub fn print_fmt<'a, T>(&self, lines: &[T])
    where
        T: 'a + ToOwned + ToString,
    {
        print_fmt(lines, self._offset);
    }

    pub fn print<'a, T>(&self, msg: T)
    where
        T: 'a + ToOwned + ToString,
    {
        print(msg, self._offset, None, false)
    }

    pub fn printnl<'a, T>(&self, msg: T)
    where
        T: 'a + ToOwned + ToString,
    {
        print(msg, self._offset, None, true)
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
                style(&pair.0).bold().white(),
                " ".repeat(max - pair.0.len()),
            );
            print(&pair.1, max + 3 + self._offset, None, false);
            println!();
        });
    }

    pub fn bold<'a, T>(&self, msg: T)
    where
        T: 'a + ToOwned + ToString,
    {
        print(msg, self._offset, Some(Style::new().white().bold()), false)
    }

    pub fn boldnl<'a, T>(&self, msg: T)
    where
        T: 'a + ToOwned + ToString,
    {
        print(msg, self._offset, Some(Style::new().white().bold()), true)
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

    pub fn err<'a, T>(&mut self, msg: T)
    where
        T: 'a + ToOwned + ToString,
    {
        self.with_title("ERROR".to_string(), msg.to_string());
    }

    pub fn with_title<'a, T>(&mut self, title: T, msg: T)
    where
        T: 'a + ToOwned + ToString,
    {
        self.bold(format!("{}:\n", title.to_string()));
        self.step_right();
        self.printnl(msg);
        self.step_left();
        println!()
    }
}

pub fn print_fmt<'a, T>(lines: &[T], offset: usize)
where
    T: 'a + ToOwned + ToString,
{
    let max = lines
        .iter()
        .map(|s| {
            let s = s.to_string();
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
        let line = line.to_string();
        let mut columns = line.split(TITLE_SPLITTER).collect::<Vec<&str>>();
        if columns.len() < 2 {
            print(line, offset, None, true);
        } else {
            print!("{}", " ".repeat(offset));
            let first = columns.remove(0).trim();
            print!(
                "{}{} - ",
                style(first).bold().white(),
                " ".repeat(max - first.len()),
            );
            print(
                columns.join(TITLE_SPLITTER).trim(),
                offset + max + 3,
                None,
                false,
            );
        }
        println!();
    });
}

pub fn print<'a, T>(msg: T, offset: usize, style: Option<Style>, nl: bool)
where
    T: 'a + ToOwned + ToString,
{
    let msg = msg.to_string();
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
            if cursor == 0 && !nl {
                "".to_string()
            } else {
                " ".repeat(offset)
            },
            if let Some(style) = style.as_ref() {
                style.apply_to(chunk).to_string()
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
