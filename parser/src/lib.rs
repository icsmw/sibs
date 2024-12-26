mod ast;
mod error;

pub use ast::*;
pub use error::E as ParserError;
pub(crate) use error::*;
mod conflict;
mod interest;
mod nodes;
mod paths;
mod read;

pub(crate) use conflict::*;
pub(crate) use interest::*;
pub use nodes::*;
pub(crate) use paths::*;
pub use read::*;

pub(crate) use asttree::*;
pub(crate) use diagnostics::*;
pub(crate) use lexer::*;
use std::{
    cell::RefCell,
    fmt::{self, Display},
    path::{Path, PathBuf},
    rc::Rc,
};
pub(crate) use uuid::Uuid;

#[derive(Debug)]
pub struct Parser {
    pub tokens: Vec<Token>,
    pub(crate) pos: usize,
    pub(crate) src: Uuid,
    pub(crate) filename: Option<PathBuf>,
    pub(crate) cwd: Option<PathBuf>,
    pub(crate) srcs: Rc<RefCell<CodeSources>>,
}

impl Parser {
    pub fn unbound<S: AsRef<str>>(tokens: Vec<Token>, src: &Uuid, content: S) -> Self {
        Self {
            tokens,
            pos: 0,
            src: *src,
            filename: None,
            cwd: None,
            srcs: Rc::new(RefCell::new(CodeSources::unbound(content, src))),
        }
    }
    pub fn new<P: AsRef<Path>>(filename: P) -> Result<Self, E> {
        let (filename, cwd, tokens, src) = BoundLexer::new(filename.as_ref())?.inner();
        Ok(Self {
            tokens,
            pos: 0,
            src,
            filename: Some(filename.clone()),
            srcs: Rc::new(RefCell::new(CodeSources::bound(filename, &src))),
            cwd: Some(cwd),
        })
    }
    pub fn new_child<P: AsRef<Path>>(&mut self, filename: P) -> Result<Self, E> {
        let (filename, cwd, tokens, src) = BoundLexer::new(filename.as_ref())?.inner();
        self.srcs.borrow_mut().add_file_src(&filename, &src);
        Ok(Self {
            tokens,
            pos: 0,
            src,
            filename: Some(filename.clone()),
            srcs: self.srcs.clone(),
            cwd: Some(cwd),
        })
    }

    pub fn report_err<T: Display>(&self, err: &LinkedErr<T>) -> Result<String, E> {
        self.srcs.borrow().err(err).map_err(E::IOError)
    }

    pub(crate) fn inherit(&self, tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            pos: 0,
            src: self.src,
            filename: self.filename.clone(),
            srcs: self.srcs.clone(),
            cwd: self.cwd.clone(),
        }
    }

    pub(crate) fn token(&mut self) -> Option<&Token> {
        while let Some(tk) = self.tokens.get(self.pos) {
            if !matches!(
                tk.id(),
                KindId::Whitespace
                    | KindId::BOF
                    | KindId::EOF
                    | KindId::LF
                    | KindId::CR
                    | KindId::CRLF
            ) {
                self.pos += 1;
                return Some(tk);
            }
            self.pos += 1;
        }
        None
    }

    pub(crate) fn current(&self) -> Option<&Token> {
        self.tokens.get(self.pos).or_else(|| self.tokens.last())
    }

    pub(crate) fn until_end(&self) -> Option<(&Token, &Token)> {
        if let (Some(from), Some(to)) = (
            self.tokens.get(self.pos).or_else(|| self.tokens.last()),
            self.tokens.last(),
        ) {
            Some((from, to))
        } else {
            None
        }
    }

    pub(crate) fn tokens(&mut self, nm: usize) -> Option<Vec<Token>> {
        let mut tokens = Vec::new();
        while let Some(tk) = self.token().cloned() {
            tokens.push(tk);
            if tokens.len() == nm {
                return Some(tokens);
            }
        }
        None
    }

    pub(crate) fn is_next(&mut self, kind: KindId) -> bool {
        let restore = self.pin();
        let tk = self.token().cloned();
        restore(self);
        if let Some(tk) = tk {
            return tk.id() == kind;
        }
        false
    }

    pub(crate) fn next(&mut self) -> Option<Token> {
        let restore = self.pin();
        let tk = self.token().cloned();
        restore(self);
        tk
    }

    pub(crate) fn pin(&mut self) -> impl Fn(&mut Parser) -> usize {
        let pos = self.pos;
        move |parser: &mut Parser| {
            let to_restore = parser.pos;
            parser.pos = pos;
            to_restore
        }
    }

    pub(crate) fn between(
        &mut self,
        left: KindId,
        right: KindId,
    ) -> Result<Option<(Parser, Token, Token)>, LinkedErr<E>> {
        let Some(open_tk) = self.token().cloned() else {
            return Ok(None);
        };
        if open_tk.id() != left {
            return Ok(None);
        }
        let mut tokens = Vec::new();
        let mut inside = 0;
        let close_tk = loop {
            let Some(tk) = self.token() else {
                break None;
            };
            if tk.id() == left {
                inside += 1;
                tokens.push(tk.clone());
                continue;
            }
            if tk.id() == right {
                if inside == 0 {
                    break Some(tk.to_owned());
                } else {
                    inside -= 1;
                    tokens.push(tk.clone());
                    continue;
                }
            }
            tokens.push(tk.clone());
        };
        let close_tk = close_tk.ok_or_else(|| LinkedErr::token(E::NoClosing(right), &open_tk))?;
        Ok(Some((self.inherit(tokens), open_tk, close_tk)))
    }

    pub(crate) fn is_done(&mut self) -> bool {
        let restore = self.pin();
        let is_done = self.token().is_none();
        restore(self);
        is_done
    }

    pub(crate) fn err_current(&self, err: E) -> LinkedErr<E> {
        LinkedErr {
            link: self
                .current()
                .map(|tk| tk.into())
                .unwrap_or(LinkedPosition::new(0, 0, &self.src)),
            e: err,
        }
    }
    pub(crate) fn err_until_end(&self, err: E) -> LinkedErr<E> {
        LinkedErr {
            link: self
                .until_end()
                .map(|tks| tks.into())
                .unwrap_or(LinkedPosition::new(0, 0, &self.src)),
            e: err,
        }
    }
}

impl fmt::Display for Parser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.tokens[self.pos..]
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join("")
        )
    }
}
