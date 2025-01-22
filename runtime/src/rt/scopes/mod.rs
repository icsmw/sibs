mod api;
mod parent;
mod scope;
mod store;

use crate::*;
use api::*;
pub use parent::*;
pub(crate) use scope::*;
pub(crate) use store::*;

#[derive(Debug, Clone)]
pub struct RtScope {
    tx: UnboundedSender<Demand>,
}

impl RtScope {
    #[tracing::instrument]
    pub fn new() -> Self {
        let (tx, mut rx) = unbounded_channel();
        spawn(async move {
            let mut scopes = VlScopes::default();
            tracing::info!("init demand's listener");
            while let Some(demand) = rx.recv().await {
                match demand {
                    Demand::OpenScope(uuid, tx) => {
                        chk_send_err!(
                            {
                                scopes.open(&uuid);
                                tx.send(())
                            },
                            DemandId::OpenScope
                        );
                    }
                    Demand::CloseScope(tx) => {
                        chk_send_err!(tx.send(scopes.close()), DemandId::CloseScope);
                    }
                    Demand::SetParentValue(vl, tx) => {
                        chk_send_err!(tx.send(scopes.set_parent_vl(vl)), DemandId::SetParentValue);
                    }
                    Demand::WithdrawParentValue(tx) => {
                        chk_send_err!(
                            tx.send(scopes.withdraw_parent_vl()),
                            DemandId::WithdrawParentValue
                        );
                    }
                    Demand::DropParentValue(tx) => {
                        chk_send_err!(tx.send(scopes.drop_parent_vl()), DemandId::DropParentValue);
                    }
                    Demand::EnterScope(uuid, tx) => {
                        chk_send_err!(tx.send(scopes.enter(&uuid)), DemandId::EnterScope);
                    }
                    Demand::LeaveScope(tx) => {
                        chk_send_err!(tx.send(scopes.leave()), DemandId::LeaveScope);
                    }
                    Demand::InsertVariable(name, vl, tx) => {
                        chk_send_err!(tx.send(scopes.insert(name, vl)), DemandId::InsertVariable);
                    }
                    Demand::UpdateVariableValue(name, vl, tx) => {
                        chk_send_err!(
                            tx.send(scopes.update(name, vl)),
                            DemandId::UpdateVariableValue
                        );
                    }
                    Demand::GetVariableValue(name, tx) => {
                        chk_send_err!(tx.send(scopes.lookup(name)), DemandId::GetVariableValue);
                    }
                    Demand::Destroy(tx) => {
                        tracing::info!("got shutdown signal");
                        chk_send_err!(tx.send(()), DemandId::Destroy);
                        break;
                    }
                }
            }
            tracing::info!("shutdown demand's listener");
        });
        Self { tx }
    }

    pub async fn set_parent_vl(&self, vl: ParentValue) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::SetParentValue(vl, tx))?;
        rx.await?
    }

    pub async fn withdraw_parent_vl(&self) -> Result<Option<ParentValue>, E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::WithdrawParentValue(tx))?;
        rx.await?
    }

    pub async fn drop_parent_vl(&self) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::DropParentValue(tx))?;
        rx.await?
    }

    pub async fn open(&self, uuid: &Uuid) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::OpenScope(*uuid, tx))?;
        Ok(rx.await?)
    }

    pub async fn close(&self) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::CloseScope(tx))?;
        rx.await?
    }

    pub async fn enter(&self, uuid: &Uuid) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::EnterScope(*uuid, tx))?;
        rx.await?
    }

    pub async fn leave(&self) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::LeaveScope(tx))?;
        rx.await?
    }

    pub async fn insert<S: AsRef<str>>(&self, name: S, vl: RtValue) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx
            .send(Demand::InsertVariable(name.as_ref().to_owned(), vl, tx))?;
        rx.await?
    }

    pub async fn update<S: AsRef<str>>(&self, name: S, vl: RtValue) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::UpdateVariableValue(
            name.as_ref().to_owned(),
            vl,
            tx,
        ))?;
        rx.await?
    }

    pub async fn lookup<S: AsRef<str>>(&self, name: S) -> Result<Option<Arc<RtValue>>, E> {
        let (tx, rx) = oneshot::channel();
        self.tx
            .send(Demand::GetVariableValue(name.as_ref().to_owned(), tx))?;
        rx.await?
    }

    pub async fn destroy(&self) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::Destroy(tx))?;
        Ok(rx.await?)
    }
}
