use crate::*;
use std::collections::HashMap;

/// Shared token streams during parsing, retaining the first stream per source.
/// Repeated includes still parse independently and cannot overwrite its owners.
#[derive(Debug)]
pub(crate) struct RcTokenStore {
    root: Uuid,
    sources: HashMap<Uuid, Rc<RefCell<Tokens>>>,
}

impl RcTokenStore {
    pub fn new(root: Uuid, tokens: Rc<RefCell<Tokens>>) -> Self {
        Self {
            root,
            sources: HashMap::from([(root, tokens)]),
        }
    }

    pub fn register(&mut self, source: Uuid, tokens: Rc<RefCell<Tokens>>) {
        self.sources.entry(source).or_insert(tokens);
    }
}

impl TryFrom<RcTokenStore> for TokenStore {
    type Error = E;

    fn try_from(mut store: RcTokenStore) -> Result<Self, E> {
        fn unwrap(tokens: Rc<RefCell<Tokens>>) -> Result<Tokens, E> {
            Ok(Rc::try_unwrap(tokens)
                .map_err(|_| E::BorrowError)?
                .into_inner())
        }
        let root = store.sources.remove(&store.root).ok_or(E::BorrowError)?;
        let mut tokens = TokenStore::new(store.root, unwrap(root)?);
        for (source, stream) in store.sources {
            tokens.insert(source, unwrap(stream)?);
        }
        Ok(tokens)
    }
}
