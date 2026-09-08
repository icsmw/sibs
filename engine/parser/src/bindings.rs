use crate::*;
use std::cell::RefMut;

#[derive(Debug)]
struct Binding {
    owner: Uuid,
    from: usize,
    to: usize,
}

impl Binding {
    fn new(owner: Uuid, from: usize, to: usize) -> Self {
        Self { owner, from, to }
    }
    fn bind(&self, tokens: &mut RefMut<Vec<Token>>, rejected: &[Uuid]) {
        for token in tokens[self.from..self.to].iter_mut() {
            if let Some((owner, _)) = &token.owner {
                if rejected.contains(owner) {
                    token.drop_owner();
                }
            }
            token.set_owner(&self.owner, self.to.saturating_sub(self.from));
        }
    }
}

#[derive(Default, Debug)]
pub struct BindingsList {
    bindings: Vec<Binding>,
    rejected: Vec<Uuid>,
}

impl BindingsList {
    pub fn add(&mut self, owner: Uuid, from: usize, to: usize) {
        self.bindings.push(Binding::new(owner, from, to));
    }
    pub fn add_rejected(&mut self, uuids: Vec<&Uuid>) {
        self.rejected
            .append(&mut uuids.into_iter().copied().collect());
    }
    pub fn try_flush(&mut self, tokens: Rc<RefCell<Vec<Token>>>) -> Result<bool, E> {
        let Ok(mut tokens) = tokens.try_borrow_mut() else {
            // This can happen if some reader is still holding a reference to the tokens.
            // At the end we will drop all references to the tokens
            return Ok(false);
        };
        self.inner_flush(&mut tokens);
        Ok(true)
    }
    pub fn flush(&mut self, tokens: Rc<RefCell<Vec<Token>>>) -> Result<(), E> {
        let mut tokens = tokens
            .try_borrow_mut()
            .map_err(|err| E::EarlyFlushCall(err.to_string()))?;
        self.inner_flush(&mut tokens);
        self.rejected.clear();
        Ok(())
    }

    fn inner_flush(&mut self, tokens: &mut RefMut<Vec<Token>>) {
        for binding in self.bindings.drain(..) {
            binding.bind(tokens, &self.rejected);
        }
    }
}
