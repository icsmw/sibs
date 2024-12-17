mod api;

use crate::*;
use api::*;
use std::{collections::HashMap, sync::Arc};
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
            let mut scopes: HashMap<Uuid, HashMap<String, Arc<RtValue>>> = HashMap::new();
            let mut location: Vec<Uuid> = Vec::new();
            let mut parent: Option<Arc<RtValue>> = None;
            tracing::info!("init demand's listener");
            'listener: while let Some(demand) = rx.recv().await {
                match demand {
                    Demand::SetParentValue(vl, tx) => {
                        parent = Some(Arc::new(vl));
                        chk_send!(tx.send(()), DemandId::SetParentValue);
                    }
                    Demand::GetParentValue(tx) => {
                        chk_send!(tx.send(parent.clone()), DemandId::GetParentValue);
                    }
                    Demand::DropParentValue(tx) => {
                        parent = None;
                        chk_send!(tx.send(()), DemandId::DropParentValue);
                    }
                    Demand::EnterScope(uuid, tx) => {
                        scopes.entry(uuid).or_default();
                        location.push(uuid);
                        chk_send!(tx.send(()), DemandId::EnterScope);
                    }
                    Demand::LeaveScope(tx) => {
                        chk_send!(
                            tx.send(if !location.is_empty() {
                                location.pop();
                                Ok(())
                            } else {
                                Err(E::AttemptToLeaveGlobalScope)
                            }),
                            DemandId::EnterScope
                        );
                    }
                    Demand::SetVariableValue(name, vl, tx) => {
                        let Some(uuid) = location.last() else {
                            chk_send!(tx.send(Err(E::NoCurrentScope)), DemandId::SetVariableValue);
                            continue 'listener;
                        };
                        let Some(sc) = scopes.get_mut(uuid) else {
                            chk_send!(
                                tx.send(Err(E::ScopeNotFound(*uuid))),
                                DemandId::SetVariableValue
                            );
                            continue 'listener;
                        };
                        sc.insert(name, Arc::new(vl));
                        chk_send!(tx.send(Ok(())), DemandId::SetVariableValue);
                    }
                    Demand::GetVariableValue(name, tx) => {
                        for uuid in location.iter().rev() {
                            let Some(sc) = scopes.get(uuid) else {
                                continue;
                            };
                            let Some(vl) = sc.get(&name) else {
                                continue;
                            };
                            chk_send!(tx.send(Some(vl.clone())), DemandId::GetVariableValue);
                            continue 'listener;
                        }
                        chk_send!(tx.send(None), DemandId::SetVariableValue);
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
        Ok(rx.await?)
    }

    pub async fn get_parent_ty(&self) -> Result<Option<Arc<RtValue>>, E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::GetParentValue(tx))?;
        Ok(rx.await?)
    }

    pub async fn drop_parent_ty(&mut self) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::DropParentValue(tx))?;
        Ok(rx.await?)
    }

    pub async fn enter(&mut self, uuid: &Uuid) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::EnterScope(*uuid, tx))?;
        Ok(rx.await?)
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
        Ok(rx.await?)
    }
}
