mod error;
mod location;

use diagnostics::LinkedErr;
use std::{fmt, path::PathBuf};
use uuid::Uuid;

pub(crate) use asttree::*;
pub(crate) use lexer::*;
pub(crate) use parser::*;
pub(crate) use semantic::*;

pub(crate) use error::*;
pub(crate) use location::*;

fn find_node<'a>(nodes: Vec<&'a LinkedNode>, src: &Uuid, pos: usize) -> Option<&'a LinkedNode> {
    let Some(found) = nodes.iter().find(|n| n.located(src, pos)) else {
        return None;
    };
    Some(find_node(found.childs(), src, pos).unwrap_or(&found))
}

#[derive(Debug)]
pub enum Error {
    Parsing(LinkedErr<ParserError>),
    Semantic(LinkedErr<SemanticError>),
}
pub struct Driver {
    parser: Option<Parser>,
    scx: Option<SemanticCx>,
    anchor: Option<Anchor>,
    err: Option<Error>,
    path: PathBuf,
}

impl Driver {
    pub fn new<P: Into<PathBuf>>(path: P) -> Self {
        Self {
            parser: None,
            scx: None,
            anchor: None,
            err: None,
            path: path.into(),
        }
    }

    pub fn read(&mut self) -> Result<(), E> {
        let mut parser = Parser::new(&self.path, true)?;
        let anchor = match Anchor::read(&mut parser) {
            Ok(Some(anchor)) => anchor,
            Ok(None) => {
                self.parser = Some(parser);
                return Ok(());
            }
            Err(err) => {
                self.parser = Some(parser);
                self.err = Some(Error::Parsing(err));
                return Ok(());
            }
        };
        self.parser = Some(parser);
        let mut scx = SemanticCx::new(true);
        functions::register(&mut scx.fns.efns)?;
        if let Err(err) = anchor.initialize(&mut scx) {
            self.scx = Some(scx);
            self.err = Some(Error::Semantic(err));
            self.anchor = Some(anchor);
            return Ok(());
        }
        if let Err(err) = anchor.infer_type(&mut scx) {
            self.scx = Some(scx);
            self.err = Some(Error::Semantic(err));
            self.anchor = Some(anchor);
            return Ok(());
        }
        if let Err(err) = anchor.finalize(&mut scx) {
            self.scx = Some(scx);
            self.err = Some(Error::Semantic(err));
        }
        self.anchor = Some(anchor);
        Ok(())
    }

    pub fn is_valid(&self) -> bool {
        if self.err.is_some() || self.anchor.is_none() {
            return false;
        }
        if let Some(parser) = self.parser.as_ref() {
            if !parser.errs.borrow().is_empty() {
                return false;
            }
        }
        if let Some(scx) = self.scx.as_ref() {
            if !scx.errs.is_empty() {
                return false;
            }
        }
        true
    }

    pub fn iter_from(&self, pos: usize, src: Option<Uuid>) -> Option<LocationIterator<'_>> {
        let (Some(anchor), Some(parser)) = (self.anchor.as_ref(), self.parser.as_ref()) else {
            return None;
        };
        Some(LocationIterator::new(
            anchor,
            src.unwrap_or(anchor.uuid),
            pos,
            parser,
        ))
    }

    pub fn locate(&self, pos: usize, src: Option<Uuid>) -> Option<&LinkedNode> {
        let Some(anchor) = self.anchor.as_ref() else {
            return None;
        };
        find_node(anchor.childs(), &src.unwrap_or(anchor.uuid), pos)
    }
}

#[test]
fn test() {
    use std::env::current_dir;
    let mut locator = Driver::new(current_dir().unwrap().join("../tests/playground/test.sibs"));
    locator.read().unwrap();
    if let Some(mut iterator) = locator.iter_from(127, None) {
        while let Some(fragment) = iterator.next() {
            println!("{}", fragment.to_string());
        }
    }
    println!("{:?}", locator.err);
}
