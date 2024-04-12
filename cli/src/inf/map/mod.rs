mod error;
mod fragment;

use console::Style;
pub use error::E;
pub use fragment::*;
use std::{collections::HashMap, fmt::Display};

const REPORT_LN_AROUND: usize = 8;

pub trait Mapping {
    fn get_fragments(&self) -> &HashMap<usize, (usize, usize)>;
    fn get_content(&self) -> &str;
    fn contains(&self, token: &usize) -> bool {
        self.get_fragments().contains_key(token)
    }
    fn get_fragment(&self, token: &usize) -> Result<Fragment, E> {
        let content = self.get_content();
        let (from, len) = self
            .get_fragments()
            .get(token)
            .ok_or(E::TokenNotFound(*token))?;
        if content.len() < from + len {
            Err(E::TokenHasInvalidRange(
                *token,
                content.len(),
                *from,
                from + len,
            ))?;
        }
        Ok(Fragment::new(
            content[*from..(from + len)].to_string(),
            *from,
            *len,
        ))
    }
    fn report_err<'a, T>(&mut self, token: &usize, msg: T) -> Result<String, E>
    where
        T: 'a + ToOwned + ToString,
    {
        let report = self.report_gen(
            token,
            format!(
                "{} {}",
                Style::new().red().bold().apply_to("ERROR:"),
                Style::new().white().apply_to(msg.to_string())
            ),
            Some(Style::new().red().bold()),
        )?;
        println!("{report}",);
        Ok(report)
    }
    fn report_gen<'a, T>(
        &mut self,
        token: &usize,
        msg: T,
        style: Option<Style>,
    ) -> Result<String, E>
    where
        T: 'a + Display,
    {
        let (from, len) = self
            .get_fragments()
            .get(token)
            .ok_or(E::TokenNotFound(*token))?;
        let content = self.get_content();
        let num_rate = content.split('\n').count().to_string().len() + 1;
        let mut cursor: usize = 0;
        let from_ln = &content[0..*from]
            .split('\n')
            .last()
            .map(|s| s.len())
            .unwrap_or(0);
        let error_range = *from..(from + len);
        let error_lns = content
            .split('\n')
            .enumerate()
            .filter_map(|(i, ln)| {
                let range = cursor..=cursor + ln.len();
                cursor += ln.len() + 1;
                if range.contains(from)
                    || range.contains(&(from + len))
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
            return Ok(format!("{msg}\n",));
        }
        cursor = 0;
        let error_first_ln = *error_lns.first().unwrap_or(&0);
        let error_last_ln = *error_lns.last().unwrap_or(&0);
        let style = style.unwrap_or(Style::new().red().bold());
        let report = content
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
                            "{}{filler}│ {ln}\n{offset}{}\n{offset}{msg}\n",
                            i + 1,
                            style.apply_to("^".repeat(*len)),
                        )
                    } else if error_last_ln != i {
                        format!("{}{filler}{} {ln}", i + 1, style.apply_to(">"))
                    } else {
                        format!("{}{filler}{} {ln}\n{msg}\n", i + 1, style.apply_to(">"),)
                    }
                } else {
                    format!("{}{filler}│ {ln}", i + 1)
                }
            })
            .collect::<Vec<String>>();
        Ok(
            report[(error_first_ln - error_first_ln.min(REPORT_LN_AROUND))
                ..report.len().min(error_last_ln + REPORT_LN_AROUND)]
                .join("\n"),
        )
    }
}
