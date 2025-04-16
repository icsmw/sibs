use tokio_util::sync::CancellationToken;

use crate::*;

#[derive(Debug, Clone, Default)]
pub struct Signals {
    signals: HashMap<String, Option<CancellationToken>>,
    waiter: HashMap<String, usize>,
}

impl Signals {
    pub fn emit(&mut self, key: String) -> Result<(), E> {
        let entry = self
            .signals
            .entry(key.clone())
            .or_insert(Some(CancellationToken::new()));
        if let Some(tk) = entry.take() {
            tk.cancel();
            Ok(())
        } else {
            Err(E::MultipleSignalEmit(key))
        }
    }
    pub fn wait(&mut self, key: String) -> Option<CancellationToken> {
        *self.waiter.entry(key.clone()).or_insert(0) += 1;
        self.signals
            .entry(key)
            .or_insert(Some(CancellationToken::new()))
            .clone()
    }
    pub fn waiters(&self, key: String) -> usize {
        self.waiter.get(&key).copied().unwrap_or_default()
    }
}
