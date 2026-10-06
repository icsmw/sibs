mod error;
mod errors;

use console::Style;
use lexer::TokenStore;
use std::{fmt::Display, io};

use crate::*;
pub use error::*;
pub use errors::*;

const REPORT_LN_AROUND: usize = 6;

#[derive(Debug)]
pub struct Diagnostics<E: Display + ErrorCode> {
    sources: CodeSources,
    tokens: TokenStore,
    errors: Errors<E>,
}

impl<E: Display + ErrorCode> Diagnostics<E> {
    pub fn new(sources: CodeSources, tokens: TokenStore, errors: Errors<E>) -> Self {
        Self {
            sources,
            tokens,
            errors,
        }
    }
    pub fn transform<O: Display + ErrorCode>(
        self,
        map: impl FnMut(LinkedErr<E>) -> LinkedErr<O>,
    ) -> Diagnostics<O> {
        let Diagnostics {
            sources,
            tokens,
            errors,
        } = self;
        Diagnostics {
            sources,
            tokens,
            errors: errors.transform(map),
        }
    }
    pub fn tokens(&self) -> &TokenStore {
        &self.tokens
    }

    pub fn push_err(&mut self, err: LinkedErr<E>) {
        self.errors.push(err);
    }
    pub fn errors(&self) -> &[LinkedErr<E>] {
        self.errors.slice()
    }
    pub fn sources(&self) -> &CodeSources {
        &self.sources
    }
    pub fn err<T: Display + ErrorCode>(
        &self,
        err: &LinkedErr<T>,
        dest: &mut impl io::Write,
    ) -> Result<(), DiagnosticsError> {
        let from = err.link.from;
        let to = err.link.to;
        let Some(code_src) = self.sources.get_source(&err.link.src) else {
            return Err(DiagnosticsError::NotFound(err.link.src));
        };
        let src = code_src.content()?;
        let num_rate = src.split('\n').count().to_string().len() + 1;
        let from_ln = &src[0..from.abs]
            .split('\n')
            .next_back()
            .map(|s| s.len())
            .unwrap_or(0);
        let error_range = from.abs..to.abs;
        let mut cursor: usize = 0;
        let error_lns = src
            .split('\n')
            .enumerate()
            .filter_map(|(i, ln)| {
                let range = cursor..=cursor + ln.len();
                cursor += ln.len() + 1;
                if range.contains(&from.abs)
                    || range.contains(&to.abs)
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
            return writeln!(dest, "{}", err.e).map_err(|e| e.into());
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
                            style.apply_to("^".repeat(to.abs - from.abs)),
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
        write!(
            dest,
            "{}{}",
            code_src
                .sig()
                .map(|filename| format!("file: {filename}\n"))
                .unwrap_or_default(),
            report[(error_first_ln.saturating_sub(REPORT_LN_AROUND))
                ..report.len().min(error_last_ln + REPORT_LN_AROUND)]
                .join("\n")
        )
        .map_err(|e| e.into())
    }
}

#[cfg(test)]
mod tests;
