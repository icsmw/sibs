mod completion;
mod error;
mod errors;
mod location;
mod locator;
mod map;
mod signature;

use interpreter::{DiagnosticError, InterContext, Script, ScriptError, ScriptOptions};
use std::{fmt, io, path::PathBuf};
use tracing::{debug, warn};
use uuid::Uuid;

pub(crate) use asttree::*;
pub(crate) use diagnostics::*;
pub(crate) use lexer::*;
pub(crate) use location::*;
pub(crate) use map::*;
#[cfg(test)]
pub(crate) use parser::*;
pub(crate) use runtime::{Fns, Ty, TyScope};
pub(crate) use semantic::*;

pub use completion::*;
pub use signature::*;

pub(crate) use error::*;
pub(crate) use errors::*;
pub(crate) use locator::*;

pub use error::E as DriverError;

fn find_node<'a>(
    nodes: Vec<&'a LinkedNode>,
    _src: &Uuid,
    token: &'a Token,
) -> Option<&'a LinkedNode> {
    let (owner, ..) = token.owner.as_ref()?;
    if let Some(found) = nodes.iter().find(|n| n.uuid() == owner) {
        Some(found)
    } else {
        for node in nodes.iter() {
            if let Some(found) = find_node(node.childs(), _src, token) {
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
                .copied()
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
    ctx: InterContext,
    src: CodeSrc,
    resilience: bool,
}

impl Driver {
    pub fn new<P: Into<PathBuf>>(path: P, resilience: bool) -> Self {
        Self {
            ctx: InterContext::default(),
            src: CodeSrc::Path(path.into()),
            resilience,
        }
    }
    pub fn unbound<S: ToString>(content: S, resilience: bool) -> Self {
        Self {
            ctx: InterContext::default(),
            src: CodeSrc::Text(content.to_string()),
            resilience,
        }
    }

    pub fn read(&mut self) -> Result<(), E> {
        self.ctx = InterContext::default();
        let options = ScriptOptions {
            resilience: self.resilience,
        };
        let result = match &self.src {
            CodeSrc::Path(path) => Script::from_file(path, options, &mut self.ctx),
            CodeSrc::Text(content) => Script::from_text(content, options, &mut self.ctx),
        };
        match result {
            Err(ScriptError::NotExecutable) if self.resilience => Ok(()),
            Err(ScriptError::FailExtractAnchorNodeFrom(_)) if self.resilience => Ok(()),
            result => result.map_err(E::from),
        }
    }

    /// If src is `None` will return content of root file
    pub fn get_src_content(&self, src: Option<&Uuid>) -> Result<Option<String>, CodeSourceError> {
        let Some(diagnostics) = self.ctx.get_diagnostics() else {
            return Ok(None);
        };
        let Some(src) = src.or_else(|| diagnostics.get_token(0).map(|token| &token.src)) else {
            return Ok(None);
        };
        diagnostics.sources().get_content(src)
    }

    pub fn get_semantic_tokens(&self) -> Vec<LinkedSemanticToken> {
        self.ctx
            .get_anchor()
            .map(|n| n.get_semantic_tokens(SemanticTokenContext::Ignored))
            .unwrap_or_default()
    }

    pub fn is_valid(&self) -> bool {
        self.ctx.get_anchor().is_none()
            || self
                .ctx
                .get_diagnostics()
                .is_some_and(|d| !d.errors().is_empty())
    }

    pub fn locator(&self, idx: usize, src: Option<Uuid>) -> Option<LocationIterator<'_>> {
        let anchor = self.ctx.get_anchor()?.extract::<Anchor>()?;
        self.ctx.get_diagnostics()?;
        Some(LocationIterator::new(
            src.unwrap_or(anchor.uuid),
            idx,
            &self.ctx,
        ))
    }

    pub fn signature(&self, pos: usize, src: Option<Uuid>) -> Option<Signature> {
        let anchor = self.ctx.get_anchor()?;
        let Some(node) = self.find_node(pos, src) else {
            debug!("Fail to find token for pos: {pos} (src {src:?})");
            return None;
        };
        Signature::from_node(
            anchor.extract::<Anchor>()?,
            node,
            self.ctx.get_semantic_cx(),
            pos,
        )
    }

    pub fn completion(&self, pos: usize, src: Option<Uuid>) -> Option<Completion<'_>> {
        let Some((token, idx)) = self.find_token(pos, src) else {
            debug!("Fail to find token for pos: {pos} (src {src:?})");
            return None;
        };
        Some(Completion::new(
            self.locator(idx, src)?,
            self.ctx.get_semantic_cx()?,
            token.to_string()[..pos.saturating_sub(token.pos.from.abs)].to_owned(),
            pos,
        ))
    }

    pub fn errors(&self) -> Option<ErrorsIterator<'_>> {
        let diagnostics = self.ctx.get_diagnostics()?;
        Some(ErrorsIterator::new(diagnostics.errors(), &self.ctx))
    }

    pub fn find_node(&self, pos: usize, src: Option<Uuid>) -> Option<&LinkedNode> {
        let (token, _idx) = self.find_token(pos, src)?;
        let anchor = self.ctx.get_anchor()?;
        find_node(anchor.childs(), &src.unwrap_or(*anchor.uuid()), token)
    }

    pub fn find_token(&self, pos: usize, _src: Option<Uuid>) -> Option<(&Token, usize)> {
        // TODO: consider SRC
        self.ctx.get_diagnostics()?.get_token_by_pos(pos)
    }

    pub fn print_errs(&self) -> Result<(), E> {
        let Some(diagnostics) = self.ctx.get_diagnostics() else {
            return Ok(());
        };
        let mut dest = io::stdout().lock();
        for err in diagnostics.errors() {
            diagnostics.err(err, &mut dest)?;
            io::Write::write_all(&mut dest, b"\n")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;
