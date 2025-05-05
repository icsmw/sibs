mod error;
mod errors;
mod locator;

use std::{cell::Ref, fmt, path::PathBuf};
use uuid::Uuid;

pub(crate) use asttree::*;
pub(crate) use diagnostics::*;
pub(crate) use lexer::*;
pub(crate) use parser::*;
pub(crate) use semantic::*;

pub(crate) use error::*;
pub(crate) use errors::*;
pub(crate) use locator::*;

pub use error::E as DriverError;

fn find_node<'a>(nodes: Vec<&'a LinkedNode>, src: &Uuid, pos: usize) -> Option<&'a LinkedNode> {
    let Some(found) = nodes.iter().find(|n| n.located(src, pos)) else {
        return None;
    };
    Some(find_node(found.childs(), src, pos).unwrap_or(&found))
}

pub struct Driver {
    parser: Option<Parser>,
    scx: Option<SemanticCx>,
    anchor: Option<Anchor>,
    errors: Vec<DrivingError>,
    path: PathBuf,
    resilience: bool,
}

impl Driver {
    pub fn new<P: Into<PathBuf>>(path: P, resilience: bool) -> Self {
        Self {
            parser: None,
            scx: None,
            anchor: None,
            errors: Vec::new(),
            path: path.into(),
            resilience,
        }
    }

    pub fn read(&mut self) -> Result<(), E> {
        let mut parser = Parser::new(&self.path, self.resilience)?;
        let anchor = match Anchor::read(&mut parser) {
            Ok(Some(anchor)) => anchor,
            Ok(None) => {
                self.parser = Some(parser);
                return if !self.resilience {
                    Err(E::FailExtractAnchorNodeFrom(
                        self.path.to_string_lossy().to_string(),
                    ))
                } else {
                    Ok(())
                };
            }
            Err(err) => {
                self.parser = Some(parser);
                return if !self.resilience {
                    Err(err.into())
                } else {
                    self.errors.push(DrivingError::Parsing(err));
                    Ok(())
                };
            }
        };
        self.errors.extend(
            parser
                .errs
                .borrow_mut()
                .extract()
                .into_iter()
                .map(|err| DrivingError::Parsing(err)),
        );
        parser.bind()?;
        self.parser = Some(parser);
        let mut scx = SemanticCx::new(self.resilience);
        functions::register(&mut scx.fns.efns)?;
        if let Err(err) = anchor.initialize(&mut scx) {
            if !self.resilience {
                return Err(err.into());
            }
            self.errors.push(DrivingError::Semantic(err));
        }
        if let Err(err) = anchor.infer_type(&mut scx) {
            if !self.resilience {
                return Err(err.into());
            }
            self.errors.push(DrivingError::Semantic(err));
        }
        if let Err(err) = anchor.finalize(&mut scx) {
            if !self.resilience {
                return Err(err.into());
            }
            self.errors.push(DrivingError::Semantic(err));
        }
        self.errors.extend(
            scx.errs
                .extract()
                .into_iter()
                .map(|err| DrivingError::Semantic(err)),
        );
        self.scx = Some(scx);
        self.anchor = Some(anchor);
        Ok(())
    }

    pub fn is_valid(&self) -> bool {
        !self.errors.is_empty() || self.anchor.is_none()
    }

    pub fn locator(&self, pos: usize, src: Option<Uuid>) -> Option<LocationIterator<'_>> {
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

    pub fn errors(&self) -> Option<ErrorsIterator<'_>> {
        let (Some(anchor), Some(parser)) = (self.anchor.as_ref(), self.parser.as_ref()) else {
            return None;
        };
        Some(ErrorsIterator::new(
            self.errors.iter().collect(),
            anchor,
            parser,
        ))
    }

    pub fn find_node(&self, pos: usize, src: Option<Uuid>) -> Option<&LinkedNode> {
        let Some(anchor) = self.anchor.as_ref() else {
            return None;
        };
        find_node(anchor.childs(), &src.unwrap_or(anchor.uuid), pos)
    }

    pub fn find_token(&self, pos: usize, _src: Option<Uuid>) -> Option<Ref<Token>> {
        // TODO: consider SRC
        self.parser
            .as_ref()
            .map(|parser| parser.get_token_by_pos(pos))
            .flatten()
    }
}

#[test]
fn test() {
    use std::env::current_dir;
    let mut driver = Driver::new(
        current_dir().unwrap().join("../tests/playground/test.sibs"),
        true,
    );
    driver.read().unwrap();
    if let Some(mut locator) = driver.locator(127, None) {
        while let Some(fragment) = locator.next_token() {
            println!("{}", fragment.to_string());
        }
    }
    if let Some(mut locator) = driver.locator(153, None) {
        while let Some(fragment) = locator.next_node() {
            println!("{}", fragment.to_string());
        }
    }

    if let Some(mut errors) = driver.errors() {
        while let Some(error) = errors.next() {
            println!("{:?}", error.err);
        }
    }
}
