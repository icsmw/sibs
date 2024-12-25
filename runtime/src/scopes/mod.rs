mod api;
mod parent;
mod scope;
mod store;

use crate::*;
use api::*;
pub(crate) use parent::*;
pub(crate) use scope::*;
use std::sync::Arc;
pub(crate) use store::*;
use tokio::spawn;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct RtScope {
    tx: UnboundedSender<Demand>,
}

impl RtScope {
    #[tracing::instrument]
    pub fn new() -> Self {
        let (tx, mut rx) = unbounded_channel();
        spawn(async move {
            let mut scopes = Scopes::default();
            tracing::info!("init demand's listener");
            while let Some(demand) = rx.recv().await {
                match demand {
                    Demand::OpenScope(uuid, tx) => {
                        chk_send!(
                            {
                                scopes.open(&uuid);
                                tx.send(())
                            },
                            DemandId::OpenScope
                        );
                    }
                    Demand::CloseScope(tx) => {
                        chk_send!(tx.send(scopes.close()), DemandId::CloseScope);
                    }
                    Demand::SetParentValue(vl, tx) => {
                        chk_send!(tx.send(scopes.set_parent_vl(vl)), DemandId::SetParentValue);
                    }
                    Demand::WithdrawParentValue(tx) => {
                        chk_send!(
                            tx.send(scopes.withdraw_parent_vl()),
                            DemandId::WithdrawParentValue
                        );
                    }
                    Demand::DropParentValue(tx) => {
                        chk_send!(tx.send(scopes.drop_parent_vl()), DemandId::DropParentValue);
                    }
                    Demand::EnterScope(uuid, tx) => {
                        chk_send!(tx.send(scopes.enter(&uuid)), DemandId::EnterScope);
                    }
                    Demand::LeaveScope(tx) => {
                        chk_send!(tx.send(scopes.leave()), DemandId::EnterScope);
                    }
                    Demand::SetVariableValue(name, vl, tx) => {
                        chk_send!(tx.send(scopes.insert(name, vl)), DemandId::SetVariableValue);
                    }
                    Demand::GetVariableValue(name, tx) => {
                        chk_send!(tx.send(scopes.lookup(name)), DemandId::GetVariableValue);
                    }
                    Demand::Destroy => {
                        tracing::info!("got shutdown signal");
                        break;
                    }
                }
            }
            tracing::info!("shutdown demand's listener");
        });
        Self { tx }
    }

    pub async fn set_parent_ty(&self, vl: RtValue) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::SetParentValue(vl, tx))?;
        rx.await?
    }

    pub async fn withdraw_parent_ty(&self) -> Result<Option<RtValue>, E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::WithdrawParentValue(tx))?;
        rx.await?
    }

    pub async fn drop_parent_ty(&mut self) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::DropParentValue(tx))?;
        rx.await?
    }

    pub async fn open(&mut self, uuid: &Uuid) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::OpenScope(*uuid, tx))?;
        Ok(rx.await?)
    }

    pub async fn close(&mut self) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::CloseScope(tx))?;
        rx.await?
    }

    pub async fn enter(&mut self, uuid: &Uuid) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::EnterScope(*uuid, tx))?;
        rx.await?
    }

    pub async fn leave(&mut self) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::LeaveScope(tx))?;
        rx.await?
    }

    pub async fn insert<S: AsRef<str>>(&mut self, name: S, vl: RtValue) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx
            .send(Demand::SetVariableValue(name.as_ref().to_owned(), vl, tx))?;
        rx.await?
    }

    pub async fn lookup<S: AsRef<str>>(&self, name: S) -> Result<Option<Arc<RtValue>>, E> {
        let (tx, rx) = oneshot::channel();
        self.tx
            .send(Demand::GetVariableValue(name.as_ref().to_owned(), tx))?;
        rx.await?
    }
}
