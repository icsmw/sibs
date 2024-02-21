use crate::reader::E;
use console::Style;
use regex::Regex;
use std::collections::HashMap;

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
#[derive(Debug, Clone)]
pub struct Map {
    //              <id,    (from,  len  )>
    pub map: HashMap<usize, (usize, usize)>,
    pub content: String,
    index: usize,
}

impl Map {
    pub fn new(content: String) -> Self {
        Self {
            map: HashMap::new(),
            content,
            index: 0,
        }
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
    pub fn extend(&mut self) {
        if self.index == 0 {
            return;
        }
        let index = self.index - 1;
        self.map.entry(index).and_modify(|(_from, len)| {
            *len += 1;
        });
    }
    pub fn print_until(&self, token: &usize) -> Result<(), E> {
        let (from, len) = self.map.get(token).ok_or(E::TokenNotFound(*token))?;
        let num_rate = self.content.split('\n').count().to_string().len() + 1;
        let mut pos: usize = 0;
        let mut lns: usize = 0;
        let from_ln = &self.content[0..*from]
            .split('\n')
            .last()
            .map(|s| s.len())
            .unwrap_or(0);
        let report = self
            .content
            .split('\n')
            .enumerate()
            .map(|(i, ln)| {
                let filler = " ".repeat(num_rate - (i + 1).to_string().len());
                pos += ln.len();
                if pos >= *from && pos < (from + len) {
                    lns += 1;
                    if lns == 1 {
                        format!(
                            "{}{filler}│ {ln}\n{}{}",
                            i + 1,
                            " ".repeat(
                                *from_ln + filler.len() + (i + 1).to_string().len() + "| ".len()
                            ),
                            Style::new()
                                .red()
                                .bold()
                                .apply_to("^".repeat(ln.len() - *from_ln - 1))
                        )
                    } else {
                        format!("{}{filler}| {ln}", i + 1)
                    }
                } else {
                    format!("{}{filler}│ {ln}", i + 1)
                }
            })
            .collect::<Vec<String>>()
            .join("\n");
        println!("\n\n\n\n{report}\n{:?}\n\n\n", self.get_fragment(token));
        Ok(())
    }
}
