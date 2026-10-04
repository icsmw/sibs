mod error;
pub use error::*;

use crate::*;
use indexmap::IndexMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GlobalKind {
    Mutable,
    Constant,
    Environment,
}

#[derive(Debug)]
pub struct GlobalSymbol {
    pub kind: GlobalKind,
    pub binding: TypeEntity,
    pub link: SrcLink,
}

impl GlobalSymbol {
    pub fn new(kind: GlobalKind, node: Uuid, link: SrcLink, ty: Ty) -> Self {
        Self {
            kind,
            // Every global has an initializer (env entries get their value at startup).
            // Its fixed type is stored once; there is no separately inferred assignment type.
            binding: TypeEntity::new(node, link.pos.clone(), Some(ty), None),
            link,
        }
    }
}

/// Global declarations shared by analysis scopes. Contains no executable values or AST nodes.
#[derive(Debug, Default)]
pub struct Globals {
    // Keep the declaration order without storing a second list of symbols.
    symbols: IndexMap<String, GlobalSymbol>,
    // Deferred bodies see the prefix available when their scope was first entered.
    scopes: HashMap<Uuid, usize>,
    location: Vec<Uuid>,
}

impl Globals {
    pub fn register(&mut self, name: String, symbol: GlobalSymbol) -> Result<(), GlobalError> {
        if let Some(previous) = self.symbols.get(&name) {
            if previous.kind == symbol.kind
                && previous.binding.ty() == symbol.binding.ty()
                && (previous.binding.node == symbol.binding.node || previous.link == symbol.link)
            {
                return Ok(());
            }
            return Err(GlobalError::Conflict {
                name,
                previous: previous.link.clone(),
            });
        }
        self.symbols.insert(name, symbol);
        Ok(())
    }

    pub fn lookup(&self, name: &str) -> Option<&GlobalSymbol> {
        let (index, _, symbol) = self.symbols.get_full(name)?;
        (index < self.visible_len()).then_some(symbol)
    }

    pub fn enter(&mut self, scope: &Uuid) {
        let visible = self.visible_len();
        self.scopes.entry(*scope).or_insert(visible);
        self.location.push(*scope);
    }

    pub fn leave(&mut self) {
        self.location.pop();
    }

    fn visible_len(&self) -> usize {
        self.location
            .last()
            .map(|scope| self.scopes[scope])
            .unwrap_or(self.symbols.len())
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &GlobalSymbol)> {
        self.symbols
            .iter()
            .map(|(name, symbol)| (name.as_str(), symbol))
    }

    pub fn exists(&self, name: &str) -> bool {
        self.lookup(name).is_some()
    }

    pub fn is_mutable(&self, name: &str) -> bool {
        self.lookup(name)
            .is_some_and(|symbol| symbol.kind == GlobalKind::Mutable)
    }
}
