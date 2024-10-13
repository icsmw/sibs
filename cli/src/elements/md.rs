use crate::{
    elements::{Element, Meta},
    inf::{Formation, FormationCursor},
};
use std::fmt;

#[cfg(test)]
use crate::elements::Comment;

#[derive(Debug, Clone, Default)]
pub struct Metadata {
    pub comments: Vec<Element>,
    pub meta: Vec<Element>,
    pub ppm: Option<Box<Element>>,
    pub tolerance: bool,
    pub inverting: bool,
    pub token: usize,
}

impl Metadata {
    pub fn empty() -> Self {
        Metadata {
            comments: Vec::new(),
            meta: Vec::new(),
            ppm: None,
            tolerance: false,
            inverting: false,
            token: 0,
        }
    }
    #[cfg(test)]
    pub fn comments(&self) -> Vec<&Comment> {
        self.comments
            .iter()
            .filter_map(|el| {
                if let Element::Comment(comment) = el {
                    Some(comment)
                } else {
                    None
                }
            })
            .collect::<Vec<&Comment>>()
    }
    pub fn meta(&self) -> Vec<&Meta> {
        self.meta
            .iter()
            .filter_map(|el| {
                if let Element::Meta(md) = el {
                    Some(md)
                } else {
                    None
                }
            })
            .collect::<Vec<&Meta>>()
    }
    pub fn meta_as_lines(&self) -> Vec<&str> {
        self.meta().iter().flat_map(|el| el.as_lines()).collect()
    }
    pub fn set_ppm(&mut self, el: Element) -> &mut Self {
        self.ppm = Some(Box::new(el));
        self
    }
    pub fn set_token(&mut self, token: usize) -> &mut Self {
        self.token = token;
        self
    }
    pub fn get_token(&self) -> usize {
        self.token
    }
}

impl fmt::Display for Metadata {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}{}{}",
            self.comments
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join("\n"),
            if self.comments.is_empty() { "" } else { "\n" },
            self.meta
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join("\n"),
            if self.meta.is_empty() { "" } else { "\n" },
        )
    }
}

impl Formation for Metadata {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        format!(
            "{}{}{}{}",
            self.comments
                .iter()
                .map(|c| c.format(cursor))
                .collect::<Vec<String>>()
                .join("\n"),
            if self.comments.is_empty() { "" } else { "\n" },
            self.meta
                .iter()
                .map(|c| c.format(cursor))
                .collect::<Vec<String>>()
                .join("\n"),
            if self.meta.is_empty() { "" } else { "\n" },
        )
    }
}
