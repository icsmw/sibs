mod ast;
mod error;

pub use error::E as ParserError;
use error::*;
mod bindings;
mod conflict;
mod interest;
mod modules;
mod nodes;
mod paths;
mod read;
mod tokens;

use conflict::*;
use interest::*;
use modules::*;
pub use nodes::*;
use paths::*;
pub use read::*;
use tokens::RcTokenStore;

use asttree::*;
use bindings::*;
use diagnostics::*;
use lexer::*;
use std::{
    cell::{Cell, Ref, RefCell},
    fmt,
    path::{Path, PathBuf},
    rc::Rc,
};
use uuid::Uuid;

#[derive(Debug)]
pub struct Parser {
    pub tokens: Rc<RefCell<Tokens>>,
    token_store: Rc<RefCell<RcTokenStore>>,
    source: CodeSourceContext,
    filename: Option<PathBuf>,
    cwd: Option<PathBuf>,
    srcs: Rc<RefCell<CodeSources>>,
    modules: Rc<RefCell<ModuleStore>>,
    errs: Rc<RefCell<Errors<E>>>,
    bindings: Rc<RefCell<BindingsList>>,
    end: usize,
    pos: Cell<usize>,
    resilience: bool,
}

type BetweenResult<'a> = Result<Option<(Parser, Ref<'a, Token>, Ref<'a, Token>)>, LinkedErr<E>>;

impl Parser {
    pub fn unbound<S: AsRef<str>>(
        tokens: Vec<Token>,
        src: &Uuid,
        content: S,
        resilience: bool,
    ) -> Self {
        let end = tokens.len().saturating_sub(1);
        let tokens = Rc::new(RefCell::new(Tokens::with(tokens)));
        let token_store = RcTokenStore::new(*src, tokens.clone());
        Self {
            tokens,
            token_store: Rc::new(RefCell::new(token_store)),
            pos: Cell::new(0),
            source: CodeSourceContext::new(*src),
            filename: None,
            cwd: None,
            srcs: Rc::new(RefCell::new(CodeSources::unbound(content, src))),
            modules: Rc::new(RefCell::new(ModuleStore::default())),
            errs: Rc::new(RefCell::new(Errors::default())),
            bindings: Rc::new(RefCell::new(BindingsList::default())),
            end,
            resilience,
        }
    }
    pub fn new<P: AsRef<Path>>(filename: P, resilience: bool) -> Result<Self, E> {
        let filename = filename.as_ref().to_path_buf();
        let cwd = filename.parent().ok_or(E::NoParentPath)?.to_path_buf();
        let mut srcs = CodeSources::default();
        let source = srcs.enter_file(&filename, None)?;
        let (_, _, tokens, _) = BoundLexer::new(&filename, source.source)?.inner();
        let end = tokens.len().saturating_sub(1);
        let tokens = Rc::new(RefCell::new(Tokens::with(tokens)));
        let token_store = RcTokenStore::new(source.source, tokens.clone());
        Ok(Self {
            tokens,
            token_store: Rc::new(RefCell::new(token_store)),
            pos: Cell::new(0),
            source,
            filename: Some(filename.clone()),
            srcs: Rc::new(RefCell::new(srcs)),
            modules: Rc::new(RefCell::new(ModuleStore::default())),
            errs: Rc::new(RefCell::new(Errors::default())),
            bindings: Rc::new(RefCell::new(BindingsList::default())),
            cwd: Some(cwd),
            end,
            resilience,
        })
    }
    pub fn new_child<P: AsRef<Path>>(&self, filename: P) -> Result<Self, E> {
        let filename = filename.as_ref().to_path_buf();
        // Keep the importing path for relative imports, including symlink aliases.
        let cwd = filename.parent().ok_or(E::NoParentPath)?.to_path_buf();
        let source = self.source_for_file(&filename)?;
        let (_, _, tokens, _) = BoundLexer::new(&filename, source.source)?.inner();
        let end = tokens.len().saturating_sub(1);
        let tokens = Rc::new(RefCell::new(Tokens::with(tokens)));
        self.token_store
            .borrow_mut()
            .register(source.source, tokens.clone());
        Ok(Self {
            tokens,
            token_store: self.token_store.clone(),
            pos: Cell::new(0),
            source,
            filename: Some(filename.clone()),
            srcs: self.srcs.clone(),
            modules: self.modules.clone(),
            errs: self.errs.clone(),
            // Token offsets are local to this file; only subparsers share bindings.
            bindings: Rc::new(RefCell::new(BindingsList::default())),
            cwd: Some(cwd),
            end,
            resilience: self.resilience,
        })
    }

    pub fn is_resilience(&self) -> bool {
        self.resilience
    }

    pub fn from_node<N: GetFilename>(&self, node: &N) -> Result<Parser, E> {
        self.from_file(node.get_filename()?)
    }

    pub fn from_file<P: AsRef<Path>>(&self, filename: P) -> Result<Parser, E> {
        self.new_child(self.resolve_path(filename)?)
    }

    pub fn pos(&self) -> usize {
        self.pos.get()
    }

    pub fn set_pos(&self, pos: usize) {
        self.pos.set(pos);
    }

    pub fn get_src_content(&self, src: Option<&Uuid>) -> Result<Option<String>, CodeSourceError> {
        self.srcs
            .borrow()
            .get_content(src.unwrap_or(&self.source.source))
    }

    /// Apply accepted token ownership after all speculative attempts have closed.
    pub fn flush(&self) -> Result<(), E> {
        let mut bindings = self
            .bindings
            .try_borrow_mut()
            .map_err(|err| E::EarlyFlushCall(err.to_string()))?;
        bindings.flush(self.tokens.clone())?;
        Ok(())
    }

    fn prepare_module<P: AsRef<Path>>(&self, filename: P) -> Result<ModuleLoad, E> {
        let path = self.resolve_path(filename)?.canonicalize()?;
        if let Some(body) = self.modules.borrow().get(&path) {
            // Cached imports still participate in cycle detection.
            self.source_for_file(&path)?;
            return Ok(ModuleLoad::Cached(body));
        }
        // The canonical path also fixes the base directory of nested module imports.
        Ok(ModuleLoad::Unparsed {
            parser: self.new_child(&path)?,
            path,
        })
    }

    /// Resolve a path against this source without changing its symlink context.
    fn resolve_path<P: AsRef<Path>>(&self, filename: P) -> Result<PathBuf, E> {
        let mut filename = filename.as_ref().to_path_buf();
        if filename.is_relative() {
            filename = self.cwd.as_ref().ok_or(E::NoParentPath)?.join(filename);
        }
        if !filename.exists() {
            return Err(E::FileNotFound(filename.to_string_lossy().to_string()));
        }
        Ok(filename)
    }

    /// Resolve source identity and check the active import chain without lexing.
    fn source_for_file<P: AsRef<Path>>(&self, filename: P) -> Result<CodeSourceContext, E> {
        Ok(self
            .srcs
            .borrow_mut()
            .enter_file(filename, Some(&self.source))?)
    }

    fn src(&self) -> Uuid {
        self.source.source
    }

    fn inherit(&self, from: usize, to: usize) -> Self {
        Self {
            tokens: self.tokens.clone(),
            token_store: self.token_store.clone(),
            pos: Cell::new(from),
            source: self.source.clone(),
            filename: self.filename.clone(),
            srcs: self.srcs.clone(),
            modules: self.modules.clone(),
            errs: self.errs.clone(),
            bindings: self.bindings.clone(),
            cwd: self.cwd.clone(),
            end: to.min(self.tokens.borrow().count().saturating_sub(1)),
            resilience: self.resilience,
        }
    }

    fn next_token_pos(&self) -> Option<usize> {
        self.tokens.borrow().next_token_pos(self.pos(), self.end)
    }

    fn token(&self) -> Option<Ref<'_, Token>> {
        let pos = self.next_token_pos()?;
        self.pos.set(pos + 1);
        let tokens_ref = self.tokens.borrow();
        Some(Ref::map(tokens_ref, |tokens| &tokens.tokens[pos]))
    }

    fn current(&self) -> Option<Ref<'_, Token>> {
        let tokens = self.tokens.borrow();
        let index = if self.pos() < tokens.count() {
            self.pos()
        } else {
            self.end
        };
        if index >= tokens.count() {
            return None;
        }
        Some(Ref::map(tokens, |tokens| &tokens.tokens[index]))
    }

    fn until_end(&self) -> Option<(Ref<'_, Token>, Ref<'_, Token>)> {
        let tokens = self.tokens.borrow();
        if self.end >= tokens.count() {
            return None;
        }
        let from = self.pos().min(self.end);
        Some(Ref::map_split(tokens, |tokens| {
            (&tokens.tokens[from], &tokens.tokens[self.end])
        }))
    }

    fn tokens(&self, nm: usize) -> Option<Vec<Ref<'_, Token>>> {
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

    fn next(&self) -> Option<Ref<'_, Token>> {
        let tokens = self.tokens.borrow();
        let pos = self.next_token_pos()?;
        Some(Ref::map(tokens, |tokens| &tokens.tokens[pos]))
    }

    fn pin(&self) -> impl Fn(&Parser) -> usize + use<> {
        let pos = self.pos();
        move |parser: &Parser| {
            let to_restore = parser.pos();
            parser.pos.set(pos);
            to_restore
        }
    }

    fn between(&self, left: KindId, right: KindId) -> BetweenResult<'_> {
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
                .map(|tk| (&*tk).into())
                .unwrap_or(LinkedPosition::new(
                    TextPosition::default(),
                    TextPosition::default(),
                    &self.source.source,
                )),
            e: err,
            severity: Severity::Error,
        }
    }
    fn err_until_end(&self, err: E) -> LinkedErr<E> {
        LinkedErr {
            link: self
                .until_end()
                .map(|(from, to)| (&*from, &*to).into())
                .unwrap_or(LinkedPosition::new(
                    TextPosition::default(),
                    TextPosition::default(),
                    &self.source.source,
                )),
            e: err,
            severity: Severity::Error,
        }
    }
}

impl TryInto<Diagnostics<E>> for Parser {
    type Error = E;
    fn try_into(self) -> Result<Diagnostics<E>, E> {
        self.flush()?;
        let Parser {
            tokens,
            token_store,
            srcs,
            errs,
            ..
        } = self;
        drop(tokens);
        let tokens = Rc::try_unwrap(token_store)
            .map_err(|_| E::BorrowError)?
            .into_inner()
            .try_into()?;
        let sources = Rc::try_unwrap(srcs)
            .map_err(|_| E::BorrowError)?
            .into_inner();
        let errors = Rc::try_unwrap(errs)
            .map_err(|_| E::BorrowError)?
            .into_inner();

        Ok(Diagnostics::new(sources, tokens, errors))
    }
}

impl fmt::Display for Parser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.tokens.borrow().tokens[self.pos().min(self.end)..=self.end]
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join("")
        )
    }
}
