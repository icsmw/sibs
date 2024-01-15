use crate::{
    inf::reporter::{self, Reporter},
    reader::{
        chars,
        entry::{Reader, Reading},
        words, E,
    },
};
use std::fmt;

#[derive(Debug)]
pub struct Meta {
    pub inner: Vec<String>,
    pub index: usize,
}

impl Meta {
    pub fn as_string(&self) -> String {
        self.inner.join("\n")
    }
    pub fn as_lines(&self) -> Vec<&str> {
        self.inner.iter().map(|s| s.as_str()).collect()
    }
}

impl Reading<Meta> for Meta {
    fn read(reader: &mut Reader) -> Result<Option<Self>, E> {
        let mut inner: Vec<String> = vec![];
        while reader.move_to().word(&[&words::META]).is_some() {
            if let Some((line, _)) = reader.until().char(&[&chars::CARET]) {
                inner.push(line.trim().to_string());
            } else {
                Err(E::NoMetaContent)?
            }
        }
        if inner.is_empty() {
            Ok(None)
        } else {
            Ok(Some(Meta {
                inner,
                index: reader.token()?.id,
            }))
        }
    }
}

impl fmt::Display for Meta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.inner
                .iter()
                .map(|v| format!("/// {v}"))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

impl reporter::Display for Meta {
    fn display(&self, reporter: &mut Reporter) {
        reporter.print_fmt(&self.as_lines());
    }
}
