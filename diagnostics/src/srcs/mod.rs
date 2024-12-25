use console::Style;
use std::{collections::HashMap, fs, io, path::PathBuf};
use uuid::Uuid;

const REPORT_LN_AROUND: usize = 6;

pub enum CodeSource {
    Content(String),
    File(PathBuf),
}

impl CodeSource {
    pub fn content(&self) -> Result<String, io::Error> {
        Ok(match self {
            Self::Content(c) => c.to_owned(),
            Self::File(filename) => fs::read_to_string(filename)?,
        })
    }
}

pub struct CodeSources {
    pub sources: HashMap<Uuid, CodeSource>,
}

impl CodeSources {
    pub fn err<S: AsRef<str>>(
        self,
        scr_uuid: &Uuid,
        msg: S,
        from: usize,
        to: usize,
    ) -> Result<String, io::Error> {
        let Some(src) = self.sources.get(scr_uuid) else {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Fail to get content of {scr_uuid}"),
            ));
        };
        let src = src.content()?;
        let num_rate = src.split('\n').count().to_string().len() + 1;
        let from_ln = &src[0..from]
            .split('\n')
            .last()
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
            return Ok(format!("{}\n", msg.as_ref()));
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
                            msg.as_ref()
                        )
                    } else if error_last_ln != i {
                        format!("{}{filler}{} {ln}", i + 1, style.apply_to(">"))
                    } else {
                        format!(
                            "{}{filler}{} {ln}\n{}\n",
                            i + 1,
                            style.apply_to(">"),
                            msg.as_ref()
                        )
                    }
                } else {
                    format!("{}{filler}│ {ln}", i + 1)
                }
            })
            .collect::<Vec<String>>();
        Ok(format!(
            "file: {}\n{}",
            "no file",
            report[(error_first_ln.min(if error_first_ln >= REPORT_LN_AROUND {
                error_first_ln - REPORT_LN_AROUND
            } else {
                0
            }))..report.len().min(error_last_ln + REPORT_LN_AROUND)]
                .join("\n")
        ))
    }
}

#[test]
fn test() -> Result<(), io::Error> {
    let mut sources = HashMap::new();
    let uuid = Uuid::new_v4();
    sources.insert(
        uuid,
        CodeSource::Content(
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
    let msg = srcs.err(&uuid, String::from("test"), 3, 3 + 4)?;
    println!("{msg}");
    Ok(())
}
