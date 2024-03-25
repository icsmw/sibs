pub mod chars;
pub mod error;
mod extentions;
pub mod ids;
pub mod map;
#[cfg(test)]
pub mod tests;
pub mod words;

use crate::{
    elements::{ElTarget, Element},
    error::LinkedErr,
    executors::{import::Import, Executor},
    inf::context::Context,
};
pub use error::E;
use extentions::*;
use map::{Fragment, Map};
use std::{cell::RefCell, fs, future::Future, path::PathBuf, pin::Pin, rc::Rc};

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
    _map: Rc<RefCell<Map>>,
    _offset: usize,
    _recent: usize,
}

impl Reader {
    pub fn bound(content: String, cx: &Context) -> Self {
        cx.map.borrow_mut().set_content(&content);
        Self {
            content,
            pos: 0,
            chars: &[],
            _offset: 0,
            _map: cx.get_map_ref(),
            _recent: 0,
        }
    }
    #[cfg(test)]
    pub fn unbound(content: String) -> Self {
        let map = Map::new_wrapped(&content);
        Self {
            content,
            pos: 0,
            chars: &[],
            _offset: 0,
            _map: map,
            _recent: 0,
        }
    }
    pub fn inherit(content: String, map: Rc<RefCell<Map>>, offset: usize) -> Self {
        Self {
            content,
            pos: 0,
            chars: &[],
            _offset: offset,
            _map: map,
            _recent: 0,
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
        self._map.borrow_mut().add(from + self._offset, len);
    }
    pub fn token(&self) -> Result<Token, E> {
        let content = self._map.borrow().content.to_string();
        self._map
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
                    bound: Reader::inherit(value, self._map.clone(), from),
                }
            })
            .ok_or(E::FailGetToken)
    }
    pub fn open_token(&mut self) -> impl Fn(&mut Reader) -> usize {
        self.move_to().any();
        let from = self.pos + self._offset;
        move |reader: &mut Reader| {
            reader
                ._map
                .borrow_mut()
                .add(from, (reader.pos + reader._offset) - from)
        }
    }
    pub fn pin(&mut self) -> impl Fn(&mut Reader) {
        let from = self.pos;
        let restore_map = self._map.borrow().pin();
        move |reader: &mut Reader| {
            reader.pos = from;
            restore_map(&mut reader._map.borrow_mut());
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
        self._map.borrow().get_fragment(token)
    }
    #[cfg(test)]
    pub fn recent(&mut self) -> &str {
        if self.pos == 0 {
            ""
        } else {
            let readed = &self.content[self._recent..self.pos];
            self._recent = self.pos;
            readed
        }
    }
}

pub type ReadFileResult = Result<Vec<Element>, LinkedErr<E>>;

pub fn read_file<'a>(cx: &'a mut Context) -> Pin<Box<dyn Future<Output = ReadFileResult> + 'a>> {
    Box::pin(async move {
        if !cx.scenario.filename.exists() {
            Err(E::FileNotExists(
                cx.scenario.filename.to_string_lossy().to_string(),
            ))?
        }
        let mut reader = Reader::bound(fs::read_to_string(&cx.scenario.filename)?, cx);
        let mut imports: Vec<PathBuf> = vec![];
        let mut elements: Vec<Element> = vec![];
        while let Some(Element::Function(func, md)) =
            Element::include(&mut reader, &[ElTarget::Function])?
        {
            if Import::get_name() != func.name {
                Err(E::OnlyImportFunctionAllowedOnRoot.by_reader(&reader))?;
            }
            let path_to_import = if func.args.len() == 1 {
                Import::get(PathBuf::from(func.args[0].to_string()), cx)?
            } else {
                return Err(E::ImportFunctionInvalidArgs.by_reader(&reader))?;
            };
            imports.push(path_to_import);
            if reader.move_to().char(&[&chars::SEMICOLON]).is_none() {
                Err(E::MissedSemicolon.by_reader(&reader))?;
            }
            elements.push(Element::Function(func, md));
        }
        for import_path in imports.iter_mut() {
            let mut cx = Context::from_filename(import_path)?;
            elements.append(&mut read_file(&mut cx).await?);
        }
        while let Some(el) = Element::include(&mut reader, &[ElTarget::Component])? {
            elements.push(el);
        }
        Ok(elements)
    })
}
