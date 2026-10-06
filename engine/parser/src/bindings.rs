use crate::*;

#[derive(Debug)]
pub(crate) struct Binding {
    owner: Uuid,
    from: usize,
    to: usize,
}

impl Binding {
    fn bind(&self, tokens: &mut [Token]) {
        for token in &mut tokens[self.from..self.to] {
            token.set_owner(&self.owner, self.to.saturating_sub(self.from));
        }
    }
}

/// Pending token ownership for one file, shared by its subparsers.
#[derive(Default, Debug)]
pub(crate) struct BindingsList {
    bindings: Vec<Binding>,
    active: usize,
}

impl BindingsList {
    pub fn add(&mut self, owner: Uuid, from: usize, to: usize) {
        self.bindings.push(Binding { owner, from, to });
    }

    pub fn append(&mut self, bindings: &mut Vec<Binding>) {
        self.bindings.append(bindings);
    }

    pub fn flush(&mut self, tokens: Rc<RefCell<Tokens>>) -> Result<(), E> {
        if self.active != 0 {
            return Err(E::EarlyFlushCall(
                "A parsing attempt is still active".into(),
            ));
        }
        if self.bindings.is_empty() {
            return Ok(());
        }
        let mut tokens = tokens
            .try_borrow_mut()
            .map_err(|err| E::EarlyFlushCall(err.to_string()))?;
        for binding in self.bindings.drain(..) {
            binding.bind(&mut tokens.tokens);
        }
        Ok(())
    }
}

/// A speculative attempt. Dropping it rolls back all nested ownership effects.
/// Successful candidates take their journal; accepted readers commit to the
/// enclosing attempt, which can still discard the entire subtree later.
pub(crate) struct BindingScope {
    bindings: Rc<RefCell<BindingsList>>,
    start: usize,
    committed: bool,
}

impl BindingScope {
    pub fn new(bindings: Rc<RefCell<BindingsList>>) -> Self {
        let start = {
            let mut list = bindings.borrow_mut();
            list.active += 1;
            list.bindings.len()
        };
        Self {
            bindings,
            start,
            committed: false,
        }
    }

    pub fn take(self) -> Vec<Binding> {
        self.bindings.borrow_mut().bindings.split_off(self.start)
    }

    pub fn commit(mut self) {
        self.committed = true;
    }
}

impl Drop for BindingScope {
    fn drop(&mut self) {
        let mut list = self.bindings.borrow_mut();
        if !self.committed {
            list.bindings.truncate(self.start);
        }
        list.active = list.active.saturating_sub(1);
    }
}
