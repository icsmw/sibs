pub mod chars;
pub mod error;
mod extentions;
pub mod sources;

#[cfg(test)]
pub mod tests;
pub mod words;

use crate::{
    elements::{ElTarget, Element},
    error::LinkedErr,
    executors::{import::Import, Executor},
    inf::{
        context::Context,
        map::{Fragment, Map as MapTrait},
    },
};
pub use error::E;
use extentions::*;
pub use sources::*;
use std::{cell::RefCell, future::Future, path::PathBuf, pin::Pin, rc::Rc};

pub trait Reading<T> {
    fn read(reader: &mut Reader) -> Result<Option<T>, LinkedErr<E>>;
}

#[derive(Debug)]
pub struct Token {
    pub content: String,
    pub id: usize,
    pub from: usize,
    pub len: usize,
    pub bound: Reader,
}

#[derive(Debug)]
pub struct Reader {
    pub content: String,
    pos: usize,
    chars: &'static [&'static char],
    map: Rc<RefCell<Map>>,
    offset: usize,
    #[cfg(test)]
    recent: usize,
}

impl Reader {
    pub fn new(map: Rc<RefCell<Map>>) -> Self {
        let content = map.borrow().content.clone();
        Self {
            content,
            pos: 0,
            chars: &[],
            offset: 0,
            map,
            #[cfg(test)]
            recent: 0,
        }
    }
    fn inherit(&self, content: String, offset: usize) -> Self {
        Self {
            content,
            pos: 0,
            chars: &[],
            offset,
            map: self.map.clone(),
            #[cfg(test)]
            recent: 0,
        }
    }
    pub fn move_to(&mut self) -> MoveTo<'_> {
        MoveTo::new(self)
    }
    pub fn until(&mut self) -> Until<'_> {
        Until::new(self)
    }
    pub fn group(&mut self) -> Group<'_> {
        Group::new(self)
    }
    pub fn next(&self) -> Next<'_> {
        Next::new(self)
    }
    pub fn rest(&self) -> &str {
        &self.content[self.pos..]
    }
    // pub fn around(&self, offset: usize) -> &str {
    //     &self.content[if self.pos > offset {
    //         self.pos - offset
    //     } else {
    //         0
    //     }..]
    // }
    pub fn trim(&mut self) {
        let content = &self.content[self.pos..];
        for (pos, char) in content.chars().enumerate() {
            if !char.is_whitespace() {
                self.pos += pos;
                return;
            }
        }
    }
    pub(self) fn index(&mut self, from: usize, len: usize) {
        self.map.borrow_mut().add(from + self.offset, len);
    }
    pub fn token(&self) -> Result<Token, E> {
        let content = self.map.borrow().content.to_string();
        self.map
            .borrow_mut()
            .last()
            .map(|(id, (from, len))| {
                let value = if len == 0 {
                    String::new()
                } else {
                    content[from..(from + len)].to_string()
                };
                Token {
                    content: value.to_string(),
                    id,
                    from,
                    len,
                    bound: self.inherit(value, from),
                }
            })
            .ok_or(E::FailGetToken)
    }
    pub fn open_token(&mut self) -> impl Fn(&mut Reader) -> usize {
        self.move_to().any();
        let from = self.pos + self.offset;
        move |reader: &mut Reader| {
            reader
                .map
                .borrow_mut()
                .add(from, (reader.pos + reader.offset) - from)
        }
    }
    pub fn pin(&mut self) -> impl Fn(&mut Reader) {
        let from = self.pos;
        let restore_map = self.map.borrow().pin();
        move |reader: &mut Reader| {
            reader.pos = from;
            restore_map(&mut reader.map.borrow_mut());
        }
    }
    pub fn done(&self) -> bool {
        self.pos == self.content.len()
    }
    pub fn is_empty(&self) -> bool {
        self.rest().trim().is_empty()
    }
    pub fn is_ascii_alphabetic_and_alphanumeric(content: &str, exceptions: &[&char]) -> bool {
        for char in content.chars() {
            if !char.is_ascii_alphanumeric()
                && !char.is_ascii_alphabetic()
                && !exceptions.contains(&&char)
            {
                return false;
            }
        }
        true
    }
    pub fn get_fragment(&self, token: &usize) -> Result<Fragment, E> {
        Ok(self.map.borrow().get_fragment(token)?)
    }
    #[cfg(test)]
    pub fn recent(&mut self) -> &str {
        if self.pos == 0 {
            ""
        } else {
            let readed = &self.content[self.recent..self.pos];
            self.recent = self.pos;
            readed
        }
    }
}

pub type ReadFileResult = Result<Vec<Element>, LinkedErr<E>>;

pub fn read_file<'a>(
    cx: &'a mut Context,
    filename: PathBuf,
    import: bool,
) -> Pin<Box<dyn Future<Output = ReadFileResult> + 'a>> {
    Box::pin(async move {
        let mut reader = cx.reader().from_file(&filename)?;
        let mut elements: Vec<Element> = vec![];
        while let Some(el) =
            Element::include(&mut reader, &[ElTarget::Function, ElTarget::Component])?
        {
            if let Element::Function(func, _) = &el {
                if Import::get_name() != func.name {
                    Err(E::OnlyImportFunctionAllowedOnRoot.by_reader(&reader))?;
                }
                let path = if func.args.len() == 1 {
                    Import::get(PathBuf::from(func.args[0].to_string()), cx)?
                } else {
                    return Err(E::ImportFunctionInvalidArgs.by_reader(&reader))?;
                };
                if import {
                    elements.append(&mut read_file(cx, path.to_owned(), true).await?);
                }
                if reader.move_to().char(&[&chars::SEMICOLON]).is_none() {
                    Err(E::MissedSemicolon.by_reader(&reader))?;
                }
            }
            elements.push(el);
        }
        Ok(elements)
    })
}

pub fn read_string<'a>(
    cx: &'a mut Context,
    content: &'a str,
) -> Pin<Box<dyn Future<Output = ReadFileResult> + 'a>> {
    Box::pin(async move {
        let mut reader = cx.reader().from_str(content)?;
        let mut elements: Vec<Element> = vec![];
        while let Some(el) =
            Element::include(&mut reader, &[ElTarget::Function, ElTarget::Component])?
        {
            if let Element::Function(func, _) = &el {
                if Import::get_name() != func.name {
                    Err(E::OnlyImportFunctionAllowedOnRoot.by_reader(&reader))?;
                }
                if func.args.len() != 1 {
                    return Err(E::ImportFunctionInvalidArgs.by_reader(&reader))?;
                };
                if reader.move_to().char(&[&chars::SEMICOLON]).is_none() {
                    Err(E::MissedSemicolon.by_reader(&reader))?;
                }
            }
            elements.push(el);
        }
        Ok(elements)
    })
}
