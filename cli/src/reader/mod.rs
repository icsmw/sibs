pub mod chars;
pub mod error;
mod extentions;
pub mod sources;
#[cfg(test)]
pub mod tests;
pub mod variables;
pub mod words;

use crate::{
    elements::{ElTarget, Element},
    error::LinkedErr,
    functions::load,
    inf::{
        map::{Fragment, Mapping},
        Journal,
    },
};
pub use error::E;
use extentions::*;
pub use sources::*;
use std::{cell::RefCell, future::Future, path::PathBuf, pin::Pin, rc::Rc};
pub use variables::*;

pub trait TryDissect<T> {
    fn try_dissect(reader: &mut Reader) -> Result<Option<T>, LinkedErr<E>>;
}

pub trait Dissect<T, O: TryDissect<T>> {
    fn dissect(reader: &mut Reader) -> Result<Option<T>, LinkedErr<E>> {
        let restore = reader.pin();
        Ok(O::try_dissect(reader)?.or_else(|| {
            restore(reader);
            None
        }))
    }
}

#[derive(Debug)]
pub struct Token {
    pub content: String,
    pub id: usize,
    #[allow(unused)]
    pub from: usize,
    #[allow(unused)]
    pub len: usize,
    pub bound: Reader,
}

#[derive(Debug)]
pub struct Reader {
    pub content: String,
    pub variables: Variables,
    pos: usize,
    chars: &'static [&'static char],
    map: Rc<RefCell<Map>>,
    offset: usize,
    #[cfg(test)]
    recent: usize,
}

impl Reader {
    pub fn read_file<'a>(
        filename: &'a PathBuf,
        import: bool,
        src: Option<&'a mut Sources>,
        journal: &Journal,
    ) -> Pin<Box<dyn Future<Output = ReadFileResult> + 'a>> {
        let journal = journal.clone();
        Box::pin(async move {
            let mut inner = Sources::new(&journal);
            read_file(
                &filename
                    .parent()
                    .ok_or(E::NoCurrentWorkingFolder(filename.to_owned()))?
                    .to_owned(),
                if let Some(src) = src { src } else { &mut inner },
                filename,
                import,
            )
            .await
        })
    }
    #[cfg(test)]
    pub fn read_string<'a>(
        content: &'a str,
        journal: &Journal,
    ) -> Pin<Box<dyn Future<Output = ReadFileResult> + 'a>> {
        let journal = journal.clone();
        Box::pin(async move {
            let mut src = Sources::new(&journal);
            let mut reader = Reader::unbound(&mut src, content)?;
            let mut elements: Vec<Element> = Vec::new();
            while let Some(el) =
                Element::include(&mut reader, &[ElTarget::Function, ElTarget::Component])?
            {
                if let Element::Function(func, _) = &el {
                    if load::NAME != func.name {
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
    #[cfg(test)]
    pub fn unbound(src: &mut Sources, content: &str) -> Result<Self, E> {
        let map = src.add_from_str(content)?;
        Ok(Self {
            content: content.to_owned(),
            variables: Variables::default(),
            pos: 0,
            chars: &[],
            offset: 0,
            map,
            #[cfg(test)]
            recent: 0,
        })
    }
    fn bound(src: &mut Sources, filename: &PathBuf) -> Result<Self, E> {
        let map = src.add_from_file(filename)?;
        let content = map.borrow().content.clone();
        Ok(Self {
            content,
            variables: Variables::default(),
            pos: 0,
            chars: &[],
            offset: 0,
            map,
            #[cfg(test)]
            recent: 0,
        })
    }
    fn inherit(&self, content: String, offset: usize) -> Self {
        Self {
            content,
            variables: Variables::default(),
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
    pub fn trim(&mut self) {
        let content = &self.content[self.pos..];
        for (pos, char) in content.chars().enumerate() {
            if !char.is_whitespace() {
                self.pos += pos;
                return;
            }
        }
    }
    pub(self) fn index(&mut self, el: Option<ElTarget>, from: usize, len: usize) {
        self.map.borrow_mut().add(el, from + self.offset, len);
    }
    pub fn token(&self) -> Result<Token, E> {
        let content = self.map.borrow().content.to_string();
        self.map
            .borrow_mut()
            .last()
            .map(|(id, fr)| {
                let value = if fr.len() == 0 {
                    String::new()
                } else {
                    content[fr.from()..fr.to()].to_string()
                };
                Token {
                    content: value.to_string(),
                    id,
                    from: fr.from(),
                    len: fr.len(),
                    bound: self.inherit(value, fr.from()),
                }
            })
            .ok_or(E::FailGetToken)
    }
    pub fn open_token(&mut self, el: ElTarget) -> impl Fn(&mut Reader) -> usize {
        self.move_to().any();
        let from = self.pos + self.offset;
        move |reader: &mut Reader| {
            reader
                .map
                .borrow_mut()
                .add(Some(el), from, (reader.pos + reader.offset) - from)
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

fn read_file<'a>(
    cwd: &'a PathBuf,
    src: &'a mut Sources,
    filename: &'a PathBuf,
    import: bool,
) -> Pin<Box<dyn Future<Output = ReadFileResult> + 'a>> {
    Box::pin(async move {
        let mut reader = Reader::bound(src, filename)?;
        let mut elements: Vec<Element> = Vec::new();
        while let Some(el) =
            Element::include(&mut reader, &[ElTarget::Function, ElTarget::Component])?
        {
            if let Element::Function(func, _) = &el {
                if load::NAME != func.name {
                    Err(E::OnlyImportFunctionAllowedOnRoot.by_reader(&reader))?;
                }
                let path = if func.args.len() == 1 {
                    load::get(PathBuf::from(func.args[0].to_string()), cwd.to_path_buf())?
                } else {
                    return Err(E::ImportFunctionInvalidArgs.by_reader(&reader))?;
                };
                if import {
                    elements.append(&mut read_file(cwd, src, &path, true).await?);
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
