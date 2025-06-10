mod ast;
mod error;

pub use error::E as ParserError;
use error::*;
mod conflict;
mod interest;
mod nodes;
mod paths;
mod read;

use conflict::*;
use interest::*;
pub use nodes::*;
use paths::*;
pub use read::*;

use asttree::*;
use diagnostics::*;
use lexer::*;
use std::{
    cell::{Cell, Ref, RefCell},
    collections::HashMap,
    fmt::{self, Display},
    io,
    path::{Path, PathBuf},
    rc::Rc,
};
use tracing::warn;
use uuid::Uuid;

#[derive(Debug)]
pub struct Parser {
    pub tokens: Rc<RefCell<Vec<Token>>>,
    src: Uuid,
    filename: Option<PathBuf>,
    cwd: Option<PathBuf>,
    srcs: Rc<RefCell<CodeSources>>,
    pub errs: Rc<RefCell<Errors<E>>>,
    bindings: Rc<RefCell<HashMap<Uuid, (usize, usize)>>>,
    end: usize,
    pos: Cell<usize>,
    resilience: bool,
}

impl Parser {
    pub fn unbound<S: AsRef<str>>(
        tokens: Vec<Token>,
        src: &Uuid,
        content: S,
        resilience: bool,
    ) -> Self {
        let end = tokens.len().saturating_sub(1);
        Self {
            tokens: Rc::new(RefCell::new(tokens)),
            pos: Cell::new(0),
            src: *src,
            filename: None,
            cwd: None,
            srcs: Rc::new(RefCell::new(CodeSources::unbound(content, src))),
            errs: Rc::new(RefCell::new(Errors::default())),
            bindings: Rc::new(RefCell::new(HashMap::new())),
            end,
            resilience,
        }
    }
    pub fn new<P: AsRef<Path>>(filename: P, resilience: bool) -> Result<Self, E> {
        let (filename, cwd, tokens, src) = BoundLexer::new(filename.as_ref())?.inner();
        let end = tokens.len().saturating_sub(1);
        Ok(Self {
            tokens: Rc::new(RefCell::new(tokens)),
            pos: Cell::new(0),
            src,
            filename: Some(filename.clone()),
            srcs: Rc::new(RefCell::new(CodeSources::bound(filename, &src)?)),
            errs: Rc::new(RefCell::new(Errors::default())),
            bindings: Rc::new(RefCell::new(HashMap::new())),
            cwd: Some(cwd),
            end,
            resilience,
        })
    }
    pub fn new_child<P: AsRef<Path>>(&self, filename: P) -> Result<Self, E> {
        let (filename, cwd, tokens, src) = BoundLexer::new(filename.as_ref())?.inner();
        self.srcs.borrow_mut().add_file_src(&filename, &src)?;
        let end = tokens.len().saturating_sub(1);
        Ok(Self {
            tokens: Rc::new(RefCell::new(tokens)),
            pos: Cell::new(0),
            src,
            filename: Some(filename.clone()),
            srcs: self.srcs.clone(),
            errs: self.errs.clone(),
            cwd: Some(cwd),
            bindings: Rc::new(RefCell::new(HashMap::new())),
            end,
            resilience: self.resilience,
        })
    }

    pub fn is_resilience(&self) -> bool {
        self.resilience
    }

    pub fn from_node<N: GetFilename>(&self, node: &N) -> Result<Parser, E> {
        let mut filename = node.get_filename()?;
        if filename.is_relative() {
            filename = self.cwd.as_ref().ok_or(E::NoParentPath)?.join(filename);
        }
        if !filename.exists() {
            return Err(E::FileNotFound(filename.to_string_lossy().to_string()));
        }
        self.new_child(filename)
    }

    pub fn from_file<P: AsRef<Path>>(&self, filename: P) -> Result<Parser, E> {
        let mut filename = filename.as_ref().to_path_buf();
        if filename.is_relative() {
            filename = self.cwd.as_ref().ok_or(E::NoParentPath)?.join(filename);
        }
        if !filename.exists() {
            return Err(E::FileNotFound(filename.to_string_lossy().to_string()));
        }
        self.new_child(filename)
    }

    pub fn report_err<T: Display>(&self, err: &LinkedErr<T>) -> Result<String, E> {
        self.srcs.borrow().err(err).map_err(E::IOError)
    }

    pub fn get_err_report(&self) -> Option<LinkedErr<E>> {
        let Some(err) = self.errs.borrow_mut().first() else {
            return None;
        };
        Some(err)
    }

    pub fn get_token(&self, idx: isize) -> Option<Ref<Token>> {
        if idx < 0 {
            return None;
        }
        let tokens_ref = self.tokens.borrow();
        if (idx as usize) < tokens_ref.len() {
            Some(Ref::map(tokens_ref, |tokens| &tokens[idx as usize]))
        } else {
            None
        }
    }

    pub fn get_token_by_pos(&self, pos: usize) -> Option<(Ref<Token>, usize)> {
        let tokens_ref = self.tokens.borrow();
        let index = tokens_ref.iter().position(|tk| tk.pos.is_in(pos))?;
        Some((Ref::map(tokens_ref, |vec| &vec[index]), index))
    }

    pub fn len(&self) -> usize {
        self.tokens.borrow().len()
    }

    pub fn pos(&self) -> usize {
        self.pos.get()
    }

    pub fn set_pos(&self, pos: usize) {
        self.pos.set(pos);
    }

    /// During parsing might be would be created phantom nodes (which could be for example in
    /// conflict with others). At final point of parsing such nodes doesn't exist and we have
    /// to consider only real nodes. That's why we are expecting `Vec<Uuid>` with a list of
    /// accepted nodes.
    pub fn bind(&self, nodes: Vec<Uuid>) -> Result<(), E> {
        let mut tokens = self.tokens.try_borrow_mut()?;
        self.bindings
            .try_borrow()?
            .iter()
            .for_each(|(owner, (from, to))| {
                if !nodes.contains(owner) {
                    return;
                }
                tokens[*from..*to].iter_mut().for_each(|tk| {
                    let _ = tk.set_owner(owner, to.saturating_sub(*from));
                })
            });
        self.bindings.try_borrow_mut()?.clear();
        Ok(())
    }

    pub fn get_src_content(&self, src: Option<&Uuid>) -> Result<Option<String>, io::Error> {
        self.srcs.borrow().get_content(src.unwrap_or(&self.src))
    }

    fn add_binding(&self, from: usize, to: usize, uuid: &Uuid) {
        let mut bindings = self.bindings.borrow_mut();
        if bindings.contains_key(uuid) {
            warn!("Attempt to bind node {uuid} multiple times");
            return;
        }
        bindings.insert(*uuid, (from, to));
    }

    fn inherit(&self, from: usize, to: usize) -> Self {
        Self {
            tokens: self.tokens.clone(),
            pos: Cell::new(from),
            src: self.src,
            filename: self.filename.clone(),
            srcs: self.srcs.clone(),
            errs: self.errs.clone(),
            bindings: self.bindings.clone(),
            cwd: self.cwd.clone(),
            end: to.min(self.tokens.borrow().len() - 1),
            resilience: self.resilience,
        }
    }

    fn next_token_pos(&self) -> Option<usize> {
        let mut pos = self.pos();
        while let Some(tk) = self.tokens.borrow().get(pos) {
            if pos > self.end {
                return None;
            }
            if !matches!(
                tk.id(),
                KindId::Whitespace
                    | KindId::BOF
                    | KindId::EOF
                    | KindId::LF
                    | KindId::CR
                    | KindId::CRLF
            ) {
                return Some(pos);
            }
            pos += 1;
        }
        None
    }

    fn token(&self) -> Option<Ref<Token>> {
        let pos = self.next_token_pos()?;
        self.pos.set(pos + 1);
        let tokens_ref = self.tokens.borrow();
        Some(Ref::map(tokens_ref, |vec| &vec[pos]))
    }

    fn current(&self) -> Option<Ref<Token>> {
        let tokens_ref = self.tokens.borrow();
        let index = self.pos();
        let token_ref = tokens_ref.get(index).or_else(|| tokens_ref.get(self.end))?;
        let idx = tokens_ref
            .iter()
            .position(|tk| std::ptr::eq(tk, token_ref))?;
        Some(Ref::map(tokens_ref, move |vec| &vec[idx]))
    }

    fn until_end(&self) -> Option<(Ref<Token>, Ref<Token>)> {
        let tokens_ref = self.tokens.borrow();
        let pos = self.pos().min(self.end);
        let from_index = tokens_ref
            .get(pos)
            .or_else(|| tokens_ref.get(self.end))
            .and_then(|tk| tokens_ref.iter().position(|x| std::ptr::eq(x, tk)))?;

        let (from_ref, to_ref) = Ref::map_split(tokens_ref, move |vec| {
            let from = &vec[from_index];
            let to = &vec[self.end];
            (from, to)
        });

        Some((from_ref, to_ref))
    }

    fn tokens(&self, nm: usize) -> Option<Vec<Ref<Token>>> {
        let mut tokens = Vec::new();
        while let Some(tk) = self.token() {
            tokens.push(tk);
            if tokens.len() == nm {
                return Some(tokens);
            }
        }
        None
    }

    fn is_next(&self, kind: KindId) -> bool {
        let restore = self.pin();
        let tk = self.token();
        restore(self);
        if let Some(tk) = tk {
            return tk.id() == kind;
        }
        false
    }

    fn next(&self) -> Option<Ref<Token>> {
        let tokens = self.tokens.borrow();
        let pos = self.next_token_pos()?;
        Some(Ref::map(tokens, |tokens| &tokens[pos]))
    }

    fn pin(&self) -> impl Fn(&Parser) -> usize {
        let pos = self.pos();
        move |parser: &Parser| {
            let to_restore = parser.pos();
            parser.pos.set(pos);
            to_restore
        }
    }

    fn between(
        &self,
        left: KindId,
        right: KindId,
    ) -> Result<Option<(Parser, Ref<Token>, Ref<Token>)>, LinkedErr<E>> {
        let Some(from_tk) = self.token() else {
            return Ok(None);
        };
        if from_tk.id() != left {
            return Ok(None);
        }
        let from_idx = self.pos();
        let mut to_idx = self.pos();
        let mut to_tk = None;
        let mut inside = 0;
        loop {
            let Some(tk) = self.token() else {
                break;
            };
            if tk.id() == left {
                inside += 1;
                continue;
            }
            if tk.id() == right {
                if inside == 0 {
                    to_idx = self.pos().saturating_sub(2);
                    to_tk = Some(tk);
                    break;
                } else {
                    inside -= 1;
                    continue;
                }
            }
        }
        let Some(to_tk) = to_tk else {
            return Err(LinkedErr::token(E::NoClosing(right), &from_tk));
        };
        Ok(Some((self.inherit(from_idx, to_idx), from_tk, to_tk)))
    }

    fn is_done(&self) -> bool {
        let restore = self.pin();
        let is_done = self.token().is_none();
        restore(self);
        is_done
    }

    fn err_current(&self, err: E) -> LinkedErr<E> {
        LinkedErr {
            link: self
                .current()
                .map(|tk| (&tk.to_owned()).into())
                .unwrap_or(LinkedPosition::new(
                    TextPosition::default(),
                    TextPosition::default(),
                    &self.src,
                )),
            e: err,
        }
    }
    fn err_until_end(&self, err: E) -> LinkedErr<E> {
        LinkedErr {
            link: self
                .until_end()
                .map(|(from, to)| (&from.to_owned(), &to.to_owned()).into())
                .unwrap_or(LinkedPosition::new(
                    TextPosition::default(),
                    TextPosition::default(),
                    &self.src,
                )),
            e: err,
        }
    }
}

impl fmt::Display for Parser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.tokens.borrow()[self.pos().min(self.end)..=self.end]
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join("")
        )
    }
}
