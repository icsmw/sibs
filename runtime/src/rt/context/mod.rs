mod api;
mod owned;
mod params;
mod parent;
mod scope;
mod store;

use crate::*;
use api::*;
pub use owned::*;
pub use params::*;
pub use parent::*;
pub(crate) use scope::*;
use store::*;

#[derive(Debug, Clone)]
pub struct RtContext {
    tx: UnboundedSender<Demand>,
}

impl RtContext {
    #[tracing::instrument]
    pub fn new(cwd: PathBuf) -> Self {
        let (tx, mut rx) = unbounded_channel();
        spawn(async move {
            let mut stores: HashMap<Uuid, Store> = HashMap::new();
            tracing::info!("init demand's listener");
            while let Some(demand) = rx.recv().await {
                match demand {
                    Demand::Command(owner, command) => {
                        let store = stores.entry(owner).or_insert(Store::new(cwd.clone()));
                        match command {
                            DemandCommand::OpenScope(uuid, tx) => {
                                chk_send_err!(
                                    {
                                        store.open(&uuid);
                                        tx.send(())
                                    },
                                    DemandCommandId::OpenScope
                                );
                            }
                            DemandCommand::CloseScope(tx) => {
                                chk_send_err!(tx.send(store.close()), DemandCommandId::CloseScope);
                            }
                            DemandCommand::SetParentValue(vl, tx) => {
                                chk_send_err!(
                                    tx.send(store.set_parent_vl(vl)),
                                    DemandCommandId::SetParentValue
                                );
                            }
                            DemandCommand::WithdrawParentValue(tx) => {
                                chk_send_err!(
                                    tx.send(store.withdraw_parent_vl()),
                                    DemandCommandId::WithdrawParentValue
                                );
                            }
                            DemandCommand::DropParentValue(tx) => {
                                chk_send_err!(
                                    tx.send(store.drop_parent_vl()),
                                    DemandCommandId::DropParentValue
                                );
                            }
                            DemandCommand::EnterScope(uuid, tx) => {
                                chk_send_err!(
                                    tx.send(store.enter(&uuid)),
                                    DemandCommandId::EnterScope
                                );
                            }
                            DemandCommand::LeaveScope(tx) => {
                                chk_send_err!(tx.send(store.leave()), DemandCommandId::LeaveScope);
                            }
                            DemandCommand::InsertVariable(name, vl, tx) => {
                                chk_send_err!(
                                    tx.send(store.insert(name, vl)),
                                    DemandCommandId::InsertVariable
                                );
                            }
                            DemandCommand::UpdateVariableValue(name, vl, tx) => {
                                chk_send_err!(
                                    tx.send(store.update(name, vl)),
                                    DemandCommandId::UpdateVariableValue
                                );
                            }
                            DemandCommand::GetVariableValue(name, tx) => {
                                chk_send_err!(
                                    tx.send(store.lookup(name)),
                                    DemandCommandId::GetVariableValue
                                );
                            }
                            DemandCommand::OpenLoop(uuid, tx) => {
                                if store.loops.contains(&uuid) {
                                    chk_send_err!(
                                        { tx.send(Err(E::LoopAlreadyExist(uuid))) },
                                        DemandCommandId::OpenLoop
                                    );
                                } else {
                                    store.loops.push(uuid);
                                    chk_send_err!({ tx.send(Ok(())) }, DemandCommandId::OpenLoop);
                                }
                            }
                            DemandCommand::CloseLoop(tx) => {
                                if let Some(uuid) = store.loops.pop() {
                                    store.breaks.remove(&uuid);
                                    chk_send_err!({ tx.send(Ok(())) }, DemandCommandId::CloseLoop);
                                } else {
                                    chk_send_err!(
                                        { tx.send(Err(E::NoOpenLoopsToClose)) },
                                        DemandCommandId::CloseLoop
                                    );
                                }
                            }
                            DemandCommand::SetBreakSignal(tx) => {
                                let Some(target) = store.loops.last() else {
                                    chk_send_err!(
                                        { tx.send(Err(E::NoOpenLoopsToBreak)) },
                                        DemandCommandId::SetBreakSignal
                                    );
                                    continue;
                                };
                                if store.breaks.contains(target) {
                                    chk_send_err!(
                                        { tx.send(Err(E::BreakSignalAlreadyExist(*target))) },
                                        DemandCommandId::SetBreakSignal
                                    );
                                } else {
                                    store.breaks.insert(*target);
                                    chk_send_err!(
                                        { tx.send(Ok(())) },
                                        DemandCommandId::SetBreakSignal
                                    );
                                }
                            }
                            DemandCommand::OpenReturnContext(uuid, tx) => {
                                if store.rcx.contains(&uuid) {
                                    chk_send_err!(
                                        { tx.send(Err(E::ReturnCXAlreadyExist(uuid))) },
                                        DemandCommandId::OpenReturnContext
                                    );
                                } else {
                                    store.rcx.push(uuid);
                                    chk_send_err!(
                                        { tx.send(Ok(())) },
                                        DemandCommandId::OpenReturnContext
                                    );
                                }
                            }
                            DemandCommand::CloseReturnContext(tx) => {
                                if let Some(uuid) = store.rcx.pop() {
                                    store.returns.remove(&uuid);
                                    chk_send_err!(
                                        { tx.send(Ok(())) },
                                        DemandCommandId::CloseReturnContext
                                    );
                                } else {
                                    chk_send_err!(
                                        { tx.send(Err(E::NoOpenReturnCXsToClose)) },
                                        DemandCommandId::CloseReturnContext
                                    );
                                }
                            }
                            DemandCommand::SetReturnValue(vl, tx) => {
                                let Some(target) = store.rcx.last() else {
                                    chk_send_err!(
                                        { tx.send(Err(E::NoOpenReturnCXToBreak)) },
                                        DemandCommandId::SetReturnValue
                                    );
                                    continue;
                                };
                                if store.returns.contains_key(target) {
                                    chk_send_err!(
                                        { tx.send(Err(E::ReturnValueAlreadyExist(*target))) },
                                        DemandCommandId::SetReturnValue
                                    );
                                } else {
                                    store.returns.insert(*target, vl);
                                    chk_send_err!(
                                        { tx.send(Ok(())) },
                                        DemandCommandId::SetReturnValue
                                    );
                                }
                            }
                            DemandCommand::WithdrawReturnValue(uuid, tx) => {
                                chk_send_err!(
                                    { tx.send(Ok(store.returns.remove(&uuid))) },
                                    DemandCommandId::WithdrawReturnValue
                                );
                            }
                            DemandCommand::IsLoopStopped(tx) => {
                                chk_send_err!(
                                    {
                                        tx.send(
                                            store
                                                .loops
                                                .last()
                                                .map(|uuid| store.breaks.contains(uuid))
                                                .unwrap_or(false)
                                                || store
                                                    .rcx
                                                    .last()
                                                    .map(|uuid| store.returns.contains_key(uuid))
                                                    .unwrap_or(false),
                                        )
                                    },
                                    DemandCommandId::IsLoopStopped
                                );
                            }
                            DemandCommand::GetCwd(tx) => {
                                chk_send_err!(
                                    { tx.send(store.cwd.clone()) },
                                    DemandCommandId::GetCwd
                                );
                            }
                            DemandCommand::SetCwd(path, tx) => {
                                store.cwd = path;
                                chk_send_err!(tx.send(()), DemandCommandId::SetCwd);
                            }
                        }
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

    pub fn create_owned(&self, owner: Uuid, journal: Journal, progress: Progress) -> Context {
        Context::new(owner, self.clone(), journal, progress)
    }

    pub(crate) async fn set_parent_vl(&self, owner: Uuid, vl: ParentValue) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::Command(
            owner,
            DemandCommand::SetParentValue(vl, tx),
        ))?;
        rx.await?
    }

    pub(crate) async fn withdraw_parent_vl(&self, owner: Uuid) -> Result<Option<ParentValue>, E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::Command(
            owner,
            DemandCommand::WithdrawParentValue(tx),
        ))?;
        rx.await?
    }

    pub(crate) async fn drop_parent_vl(&self, owner: Uuid) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx
            .send(Demand::Command(owner, DemandCommand::DropParentValue(tx)))?;
        rx.await?
    }

    pub(crate) async fn open(&self, owner: Uuid, uuid: &Uuid) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx
            .send(Demand::Command(owner, DemandCommand::OpenScope(*uuid, tx)))?;
        Ok(rx.await?)
    }

    pub(crate) async fn close(&self, owner: Uuid) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx
            .send(Demand::Command(owner, DemandCommand::CloseScope(tx)))?;
        rx.await?
    }

    pub(crate) async fn enter(&self, owner: Uuid, uuid: &Uuid) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx
            .send(Demand::Command(owner, DemandCommand::EnterScope(*uuid, tx)))?;
        rx.await?
    }

    pub(crate) async fn leave(&self, owner: Uuid) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx
            .send(Demand::Command(owner, DemandCommand::LeaveScope(tx)))?;
        rx.await?
    }

    pub(crate) async fn insert<S: ToString>(
        &self,
        owner: Uuid,
        name: S,
        vl: RtValue,
    ) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::Command(
            owner,
            DemandCommand::InsertVariable(name.to_string(), vl, tx),
        ))?;
        rx.await?
    }

    pub(crate) async fn update<S: ToString>(
        &self,
        owner: Uuid,
        name: S,
        vl: RtValue,
    ) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::Command(
            owner,
            DemandCommand::UpdateVariableValue(name.to_string(), vl, tx),
        ))?;
        rx.await?
    }

    pub(crate) async fn lookup<S: ToString>(
        &self,
        owner: Uuid,
        name: S,
    ) -> Result<Option<Arc<RtValue>>, E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::Command(
            owner,
            DemandCommand::GetVariableValue(name.to_string(), tx),
        ))?;
        rx.await?
    }

    pub(crate) async fn open_loop(&self, owner: Uuid, uuid: &Uuid) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx
            .send(Demand::Command(owner, DemandCommand::OpenLoop(*uuid, tx)))?;
        rx.await?
    }

    pub(crate) async fn close_loop(&self, owner: Uuid) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx
            .send(Demand::Command(owner, DemandCommand::CloseLoop(tx)))?;
        rx.await?
    }

    pub(crate) async fn set_break(&self, owner: Uuid) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx
            .send(Demand::Command(owner, DemandCommand::SetBreakSignal(tx)))?;
        rx.await?
    }

    pub(crate) async fn open_return_cx(&self, owner: Uuid, uuid: &Uuid) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::Command(
            owner,
            DemandCommand::OpenReturnContext(*uuid, tx),
        ))?;
        rx.await?
    }

    pub(crate) async fn close_return_cx(&self, owner: Uuid) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::Command(
            owner,
            DemandCommand::CloseReturnContext(tx),
        ))?;
        rx.await?
    }

    pub(crate) async fn set_return_vl(&self, owner: Uuid, vl: RtValue) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::Command(
            owner,
            DemandCommand::SetReturnValue(vl, tx),
        ))?;
        rx.await?
    }

    pub(crate) async fn is_loop_stopped(&self, owner: Uuid) -> Result<bool, E> {
        let (tx, rx) = oneshot::channel();
        self.tx
            .send(Demand::Command(owner, DemandCommand::IsLoopStopped(tx)))?;
        Ok(rx.await?)
    }

    pub(crate) async fn withdraw_return_vl(
        &self,
        owner: Uuid,
        uuid: &Uuid,
    ) -> Result<Option<RtValue>, E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::Command(
            owner,
            DemandCommand::WithdrawReturnValue(*uuid, tx),
        ))?;
        rx.await?
    }

    pub(crate) async fn get_cwd(&self, owner: Uuid) -> Result<PathBuf, E> {
        let (tx, rx) = oneshot::channel();
        self.tx
            .send(Demand::Command(owner, DemandCommand::GetCwd(tx)))?;
        rx.await.map_err(|e| e.into())
    }

    pub(crate) async fn set_cwd(&self, owner: Uuid, path: PathBuf) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx
            .send(Demand::Command(owner, DemandCommand::SetCwd(path, tx)))?;
        rx.await.map_err(|e| e.into())
    }

    pub(crate) async fn destroy(&self) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::Destroy(tx))?;
        Ok(rx.await?)
    }
}
