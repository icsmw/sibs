mod store;

#[cfg(test)]
mod tests;

use super::api::{Demand, GlobalCommand};
use crate::*;
pub(super) use store::GlobalValues;

/// Channel access to the global values of one execution.
pub struct GlobalAccess<'a> {
    pub(super) cxs: &'a ExecutionContexts,
}

impl GlobalAccess<'_> {
    pub async fn lookup(&self, name: &str) -> Result<Option<Arc<RtValue>>, E> {
        let (tx, rx) = oneshot::channel();
        self.cxs
            .tx
            .send(Demand::Global(name.into(), GlobalCommand::Lookup(tx)))?;
        rx.await?
    }

    pub async fn is_registered(&self, name: &str, link: &SrcLink) -> Result<bool, E> {
        let (tx, rx) = oneshot::channel();
        self.cxs.tx.send(Demand::Global(
            name.into(),
            GlobalCommand::IsRegistered(link.clone(), tx),
        ))?;
        rx.await?
    }

    pub async fn register(
        &self,
        name: String,
        ty: Ty,
        mutable: bool,
        link: SrcLink,
        value: RtValue,
    ) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.cxs.tx.send(Demand::Global(
            name,
            GlobalCommand::Register(ty, mutable, link, value, tx),
        ))?;
        rx.await?
    }

    /// Apply one short synchronous transformation and return the committed value.
    /// The transformation must not execute script code or wait for other context requests.
    /// Once sent, the update is processed even if the caller stops waiting for its result.
    pub async fn update(
        &self,
        name: &str,
        transform: impl FnOnce(&RtValue) -> Result<RtValue, E> + Send + 'static,
    ) -> Result<Arc<RtValue>, E> {
        let (tx, rx) = oneshot::channel();
        self.cxs.tx.send(Demand::Global(
            name.into(),
            GlobalCommand::Update(Box::new(transform), tx),
        ))?;
        rx.await?
    }
}
