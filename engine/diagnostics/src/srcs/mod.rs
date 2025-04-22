use console::Style;
use std::{
    collections::HashMap,
    fmt::Display,
    fs, io,
    path::{Path, PathBuf},
};
use uuid::Uuid;

use crate::LinkedErr;

const REPORT_LN_AROUND: usize = 6;

#[derive(Debug)]
pub enum CodeSource {
    Inline(String),
    File(PathBuf),
}

impl CodeSource {
    pub fn content(&self) -> Result<String, io::Error> {
        Ok(match self {
            Self::Inline(c) => c.to_owned(),
            Self::File(filename) => fs::read_to_string(filename)?,
        })
    }
    pub fn sig(&self) -> Option<String> {
        match self {
            Self::Inline(_) => None,
            Self::File(filename) => Some(filename.to_string_lossy().to_string()),
        }
    }
}

#[derive(Debug, Default)]
pub struct CodeSources {
    pub sources: HashMap<Uuid, CodeSource>,
}

impl CodeSources {
    pub fn bound<P: AsRef<Path>>(filename: P, uuid: &Uuid) -> Result<Self, io::Error> {
        let mut sources = HashMap::new();
        let filename = std::fs::canonicalize(filename)?;
        sources.insert(*uuid, CodeSource::File(filename));
        Ok(Self { sources })
    }
    pub fn unbound<S: AsRef<str>>(content: S, uuid: &Uuid) -> Self {
        let mut sources = HashMap::new();
        sources.insert(*uuid, CodeSource::Inline(content.as_ref().to_owned()));
        Self { sources }
    }
    pub fn add_file_src<P: AsRef<Path>>(
        &mut self,
        filename: P,
        uuid: &Uuid,
    ) -> Result<(), io::Error> {
        let filename = std::fs::canonicalize(filename)?;
        if self.sources.iter().any(|(_, cs)| {
            if let CodeSource::File(path) = cs {
                path == &filename
            } else {
                false
            }
        }) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!(
                    "File \"{}\" already has been attached",
                    filename.to_string_lossy()
                ),
            ));
        }
        self.sources.insert(*uuid, CodeSource::File(filename));
        Ok(())
    }
    pub fn add_inline_src<S: AsRef<str>>(&mut self, content: S, uuid: &Uuid) {
        self.sources
            .insert(*uuid, CodeSource::Inline(content.as_ref().to_owned()));
    }
    pub fn err<T: Display>(&self, err: &LinkedErr<T>) -> Result<String, io::Error> {
        let from = err.link.from;
        let to = err.link.to;
        let Some(code_src) = self.sources.get(&err.link.src) else {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Fail to get content of {}", err.link.src),
            ));
        };
        let src = code_src.content()?;
        let num_rate = src.split('\n').count().to_string().len() + 1;
        let from_ln = &src[0..from]
            .split('\n')
            .next_back()
            .map(|s| s.len())
            .unwrap_or(0);
        let error_range = from..to;
        let mut cursor: usize = 0;
        let error_lns = src
            .split('\n')
            .enumerate()
            .filter_map(|(i, ln)| {
                let range = cursor..=cursor + ln.len();
                cursor += ln.len() + 1;
                if range.contains(&from)
                    || range.contains(&to)
                    || error_range.contains(range.start())
                    || error_range.contains(range.end())
                {
                    Some(i)
                } else {
                    None
                }
            })
            .collect::<Vec<usize>>();
        if error_lns.is_empty() {
            return Ok(format!("{}\n", err.e));
        }
        cursor = 0;
        let error_first_ln = *error_lns.first().unwrap_or(&0);
        let error_last_ln = *error_lns.last().unwrap_or(&0);
        let style = Style::new().red().bold();
        let report = src
            .split('\n')
            .enumerate()
            .map(|(i, ln)| {
                cursor += ln.len() + 1;
                let filler = " ".repeat(num_rate - (i + 1).to_string().len());
                if error_lns.contains(&i) {
                    if error_lns.len() == 1 {
                        let offset = " ".repeat(
                            *from_ln + filler.len() + (i + 1).to_string().len() + "| ".len(),
                        );
                        format!(
                            "{}{filler}│ {ln}\n{offset}{}\n{offset}{}\n",
                            i + 1,
                            style.apply_to("^".repeat(to - from)),
                            err.e
                        )
                    } else if error_last_ln != i {
                        format!("{}{filler}{} {ln}", i + 1, style.apply_to(">"))
                    } else {
                        format!("{}{filler}{} {ln}\n{}\n", i + 1, style.apply_to(">"), err.e)
                    }
                } else {
                    format!("{}{filler}│ {ln}", i + 1)
                }
            })
            .collect::<Vec<String>>();
        Ok(format!(
            "{}{}",
            code_src
                .sig()
                .map(|filename| format!("file: {filename}\n"))
                .unwrap_or_default(),
            report[(error_first_ln.saturating_sub(REPORT_LN_AROUND))
                ..report.len().min(error_last_ln + REPORT_LN_AROUND)]
                .join("\n")
        ))
    }
}

#[test]
fn test_sl() -> Result<(), io::Error> {
    let mut sources = HashMap::new();
    let uuid = Uuid::new_v4();
    sources.insert(
        uuid,
        CodeSource::Inline(
            r#"fn test() {
    let a = 4 + 5;
    b - c;
    if c > 100 {
        exit;
    }
}
    "#
            .to_string(),
        ),
    );
    let srcs = CodeSources { sources };
    let msg = srcs.err(&LinkedErr {
        e: String::from("Test Error Messaging"),
        link: lexer::LinkedPosition::new(3, 3 + 4, &uuid),
    })?;
    println!("{msg}");
    Ok(())
}

#[test]
fn test_ml() -> Result<(), io::Error> {
    let mut sources = HashMap::new();
    let uuid = Uuid::new_v4();
    sources.insert(
        uuid,
        CodeSource::Inline(
            r#"fn test() {
    let a = 4 + 5;
    b - c;
    if c > 100 {
        exit;
    }
}
    "#
            .to_string(),
        ),
    );
    let srcs = CodeSources { sources };
    let msg = srcs.err(&LinkedErr {
        e: String::from("Test Error Messaging"),
        link: lexer::LinkedPosition::new(15, 50, &uuid),
    })?;
    println!("{msg}");
    Ok(())
}
