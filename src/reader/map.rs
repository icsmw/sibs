use crate::reader::E;
use console::Style;
use regex::Regex;
use std::{cell::RefCell, collections::HashMap, fmt::Display, rc::Rc};

#[derive(Debug)]
pub struct Fragment {
    pub content: String,
    pub lined: String,
    pub from: usize,
    pub len: usize,
    pub to: usize,
}

impl Fragment {
    pub fn new(content: String, from: usize, len: usize) -> Self {
        let lined = Regex::new(r"[\n\r]\s*")
            .expect("Regex [\\n\\r]\\s* should be constructed")
            .replace_all(&content, "")
            .to_string();
        Fragment {
            content,
            lined,
            from,
            len,
            to: from + len,
        }
    }
}

const REPORT_LN_AROUND: usize = 8;

#[derive(Debug, Clone)]
pub struct Map {
    //              <id,    (from,  len  )>
    pub map: HashMap<usize, (usize, usize)>,
    pub reports: Vec<String>,
    pub content: String,
    cursor: Option<usize>,
    index: usize,
}

impl Map {
    pub fn new_wrapped(content: &str) -> Rc<RefCell<Map>> {
        Rc::new(RefCell::new(Map::new(content)))
    }
    pub fn new(content: &str) -> Self {
        Self {
            map: HashMap::new(),
            reports: vec![],
            content: content.to_owned(),
            cursor: None,
            index: 0,
        }
    }
    pub fn pin(&self) -> impl Fn(&mut Map) {
        let last = self.index;
        move |map: &mut Map| {
            map.index = last;
            map.map.retain(|k, _| k <= &last);
        }
    }
    pub fn set_content(&mut self, content: &str) {
        content.clone_into(&mut self.content);
    }
    pub fn set_cursor(&mut self, token: usize) {
        self.cursor = Some(token);
    }
    pub fn last(&self) -> Option<(usize, (usize, usize))> {
        if self.index > 0 {
            let index = self.index - 1;
            self.map.get(&index).map(|coors| (index, *coors))
        } else {
            None
        }
    }
    pub fn add(&mut self, from: usize, len: usize) -> usize {
        self.map.insert(self.index, (from, len));
        self.index += 1;
        self.index - 1
    }
    pub fn get_fragment(&self, token: &usize) -> Result<Fragment, E> {
        let (from, len) = self.map.get(token).ok_or(E::TokenNotFound(*token))?;
        if self.content.len() < from + len {
            Err(E::TokenHasInvalidRange(
                *token,
                self.content.len(),
                *from,
                from + len,
            ))?;
        }
        Ok(Fragment::new(
            self.content[*from..(from + len)].to_string(),
            *from,
            *len,
        ))
    }
    pub fn gen_report<'a, T>(&mut self, token: &usize, msg: T) -> Result<(), E>
    where
        T: 'a + ToOwned + ToString,
    {
        let (from, len) = self.map.get(token).ok_or(E::TokenNotFound(*token))?;
        let num_rate = self.content.split('\n').count().to_string().len() + 1;
        let mut cursor: usize = 0;
        let from_ln = &self.content[0..*from]
            .split('\n')
            .last()
            .map(|s| s.len())
            .unwrap_or(0);
        let error_range = *from..(from + len);
        let error_lns = self
            .content
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
            self.reports.push(format!(
                "{} {}\n",
                Style::new().red().bold().apply_to("ERROR:"),
                Style::new().white().apply_to(msg.to_string())
            ));
            return Ok(());
        }
        cursor = 0;
        let error_first_ln = *error_lns.first().unwrap_or(&0);
        let error_last_ln = *error_lns.last().unwrap_or(&0);
        let report = self
            .content
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
                            "{}{filler}│ {ln}\n{offset}{}\n{offset}{} {}\n",
                            i + 1,
                            Style::new().red().bold().apply_to("^".repeat(*len)),
                            Style::new().red().bold().apply_to("ERROR:"),
                            Style::new().white().apply_to(msg.to_string())
                        )
                    } else if error_last_ln != i {
                        format!(
                            "{}{filler}{} {ln}",
                            i + 1,
                            Style::new().red().bold().apply_to(">")
                        )
                    } else {
                        format!(
                            "{}{filler}{} {ln}\n{} {}\n",
                            i + 1,
                            Style::new().red().bold().apply_to(">"),
                            Style::new().red().bold().apply_to("ERROR:"),
                            Style::new().white().apply_to(msg.to_string())
                        )
                    }
                } else {
                    format!("{}{filler}│ {ln}", i + 1)
                }
            })
            .collect::<Vec<String>>();
        self.reports.push(
            report[(error_first_ln - error_first_ln.min(REPORT_LN_AROUND))
                ..report.len().min(error_last_ln + REPORT_LN_AROUND)]
                .join("\n"),
        );
        Ok(())
    }
    pub fn assign_error<T>(&mut self, err: &T) -> Result<(), E>
    where
        T: std::error::Error + Display + ToString,
    {
        if !self.reports.is_empty() {
            Ok(())
        } else if let (true, Some(token)) = (self.reports.is_empty(), self.cursor) {
            self.gen_report(&token, err.to_string())
        } else {
            Ok(())
        }
    }
    pub fn post_reports(&self) {
        self.reports.iter().for_each(|report| {
            println!("\n{report}");
        });
    }
}
