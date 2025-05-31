mod completion;
mod error;
mod errors;
mod locator;

use std::{cell::Ref, fmt, io, path::PathBuf};
use uuid::Uuid;

pub(crate) use asttree::*;
pub(crate) use diagnostics::*;
pub(crate) use lexer::*;
pub(crate) use parser::*;
pub(crate) use runtime::{Fns, Ty, TyScope};
pub(crate) use semantic::*;

pub(crate) use completion::*;
pub(crate) use error::*;
pub(crate) use errors::*;
pub(crate) use locator::*;

pub use error::E as DriverError;

fn find_node<'a>(
    nodes: Vec<&'a LinkedNode>,
    src: &Uuid,
    token: &Ref<Token>,
) -> Option<&'a LinkedNode> {
    let Some((owner, ..)) = token.owner.as_ref() else {
        return None;
    };
    if let Some(found) = nodes.iter().find(|n| n.uuid() == owner) {
        Some(&found)
    } else {
        for node in nodes.iter() {
            if let Some(found) = find_node(node.childs(), src, token) {
                return Some(found);
            }
        }
        None
    }
}

fn get_ownership_tree<'a>(
    nodes: Vec<&'a LinkedNode>,
    src: &Uuid,
    pos: usize,
) -> Vec<&'a LinkedNode> {
    fn fill<'a>(
        list: &mut Vec<&'a LinkedNode>,
        nodes: Vec<&'a LinkedNode>,
        src: &Uuid,
        pos: usize,
    ) {
        list.extend(
            nodes
                .iter()
                .filter(|n| n.get_node().located(src, pos))
                .map(|n| *n)
                .collect::<Vec<&'a LinkedNode>>(),
        );
        for node in nodes.into_iter() {
            if !node.childs().is_empty() {
                fill(list, node.childs(), src, pos);
            }
        }
    }
    let mut collected = Vec::new();
    fill(&mut collected, nodes, src, pos);
    collected
}

pub enum CodeSrc {
    Path(PathBuf),
    Text(String),
}

impl fmt::Display for CodeSrc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Path(path) => path.to_string_lossy().to_string(),
                Self::Text(..) => String::from("text codebase"),
            }
        )
    }
}
pub struct Driver {
    parser: Option<Parser>,
    scx: Option<SemanticCx>,
    anchor: Option<Anchor>,
    errors: Vec<DrivingError>,
    src: CodeSrc,
    resilience: bool,
}

impl Driver {
    pub fn new<P: Into<PathBuf>>(path: P, resilience: bool) -> Self {
        Self {
            parser: None,
            scx: None,
            anchor: None,
            errors: Vec::new(),
            src: CodeSrc::Path(path.into()),
            resilience,
        }
    }
    pub fn unbound<S: ToString>(content: S, resilience: bool) -> Self {
        Self {
            parser: None,
            scx: None,
            anchor: None,
            errors: Vec::new(),
            src: CodeSrc::Text(content.to_string()),
            resilience,
        }
    }

    pub fn read(&mut self) -> Result<(), E> {
        let mut parser = match &self.src {
            CodeSrc::Path(path) => Parser::new(&path, self.resilience)?,
            CodeSrc::Text(content) => {
                let mut lx = lexer::Lexer::new(&content, 0);
                Parser::unbound(lx.read()?.tokens, &lx.uuid, &content, self.resilience)
            }
        };
        let anchor = match Anchor::read(&mut parser) {
            Ok(Some(anchor)) => anchor,
            Ok(None) => {
                self.parser = Some(parser);
                return if !self.resilience {
                    Err(E::FailExtractAnchorNodeFrom(self.src.to_string()))
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

    /// If src is `None` will return content of root file
    pub fn get_src_content(&self, src: Option<&Uuid>) -> Result<Option<String>, io::Error> {
        let Some(parser) = self.parser.as_ref() else {
            return Ok(None);
        };
        parser.get_src_content(src)
    }

    pub fn get_semantic_tokens(&self) -> Vec<LinkedSemanticToken> {
        self.anchor
            .as_ref()
            .map(|n| n.get_semantic_tokens(SemanticTokenContext::Ignored))
            .unwrap_or_default()
    }

    pub fn is_valid(&self) -> bool {
        !self.errors.is_empty() || self.anchor.is_none()
    }

    pub fn locator(&self, idx: usize, src: Option<Uuid>) -> Option<LocationIterator<'_>> {
        let (Some(anchor), Some(parser)) = (self.anchor.as_ref(), self.parser.as_ref()) else {
            return None;
        };
        Some(LocationIterator::new(
            anchor,
            src.unwrap_or(anchor.uuid),
            idx,
            parser,
        ))
    }

    pub fn completion(&self, pos: usize, src: Option<Uuid>) -> Option<Completion<'_>> {
        let (token, idx) = self.find_token(pos, src)?;
        Some(Completion::new(
            self.locator(idx, src)?,
            self.scx.as_ref()?,
            token.to_string()[..pos.saturating_sub(token.pos.from.abs)].to_owned(),
            pos,
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
        let (token, _idx) = self.find_token(pos, src)?;
        let Some(anchor) = self.anchor.as_ref() else {
            return None;
        };
        find_node(anchor.childs(), &src.unwrap_or(anchor.uuid), &token)
    }

    pub fn find_token(&self, pos: usize, _src: Option<Uuid>) -> Option<(Ref<Token>, usize)> {
        // TODO: consider SRC
        self.parser
            .as_ref()
            .map(|parser| parser.get_token_by_pos(pos))
            .flatten()
    }

    pub fn print_errs(&self) -> Result<(), E> {
        let Some(parser) = self.parser.as_ref() else {
            return Ok(());
        };
        for err in parser.errs.borrow().errors.iter() {
            println!("{}", parser.report_err(err)?);
        }
        for err in self.errors.iter() {
            println!(
                "{}",
                match err {
                    DrivingError::Parsing(err) => parser.report_err(err)?,
                    DrivingError::Semantic(err) => parser.report_err(err)?,
                }
            );
        }
        Ok(())
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
    if let Some(mut locator) = driver.locator(1, None) {
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
    let mut tokens = driver.get_semantic_tokens();
    tokens.sort_by(|a, b| a.position.from.abs.cmp(&b.position.from.abs));
    println!("{tokens:?}");
    let content = driver
        .get_src_content(None)
        .expect("Source isn't available")
        .expect("Source isn't found");
    println!("\n{}\n{content}\n{}\n", "=".repeat(50), "=".repeat(50));
    for tk in tokens.iter() {
        println!(
            "token: {}",
            tk.extract_by_relative(&content)
                .expect("Token extracted by (ln, col) coors")
        );
    }
}
