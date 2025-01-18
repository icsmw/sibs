mod api;

use crate::*;
use api::*;
use tokio::spawn;

#[derive(Debug, Clone)]
pub struct RtEvents {
    tx: UnboundedSender<Demand>,
}

impl RtEvents {
    #[tracing::instrument]
    pub fn new() -> Self {
        let (tx, mut rx) = unbounded_channel();
        spawn(async move {
            tracing::info!("init demand's listener");
            let mut breaks: HashSet<Uuid> = HashSet::new();
            let mut loops: Vec<Uuid> = Vec::new();
            let mut rcx: Vec<Uuid> = Vec::new();
            let mut returns: HashMap<Uuid, RtValue> = HashMap::new();
            while let Some(demand) = rx.recv().await {
                match demand {
                    Demand::OpenLoop(uuid, tx) => {
                        if loops.contains(&uuid) {
                            chk_send_err!(
                                { tx.send(Err(E::LoopAlreadyExist(uuid))) },
                                DemandId::OpenLoop
                            );
                        } else {
                            loops.push(uuid);
                            chk_send_err!({ tx.send(Ok(())) }, DemandId::OpenLoop);
                        }
                    }
                    Demand::CloseLoop(tx) => {
                        if let Some(uuid) = loops.pop() {
                            breaks.remove(&uuid);
                            chk_send_err!({ tx.send(Ok(())) }, DemandId::CloseLoop);
                        } else {
                            chk_send_err!(
                                { tx.send(Err(E::NoOpenLoopsToClose)) },
                                DemandId::CloseLoop
                            );
                        }
                    }
                    Demand::SetBreakSignal(tx) => {
                        let Some(target) = loops.last() else {
                            chk_send_err!(
                                { tx.send(Err(E::NoOpenLoopsToBreak)) },
                                DemandId::SetBreakSignal
                            );
                            continue;
                        };
                        if breaks.contains(target) {
                            chk_send_err!(
                                { tx.send(Err(E::BreakSignalAlreadyExist(*target))) },
                                DemandId::SetBreakSignal
                            );
                        } else {
                            breaks.insert(*target);
                            chk_send_err!({ tx.send(Ok(())) }, DemandId::SetBreakSignal);
                        }
                    }
                    Demand::OpenReturnContext(uuid, tx) => {
                        if rcx.contains(&uuid) {
                            chk_send_err!(
                                { tx.send(Err(E::ReturnCXAlreadyExist(uuid))) },
                                DemandId::OpenReturnContext
                            );
                        } else {
                            rcx.push(uuid);
                            chk_send_err!({ tx.send(Ok(())) }, DemandId::OpenReturnContext);
                        }
                    }
                    Demand::CloseReturnContext(tx) => {
                        if let Some(uuid) = rcx.pop() {
                            returns.remove(&uuid);
                            chk_send_err!({ tx.send(Ok(())) }, DemandId::CloseReturnContext);
                        } else {
                            chk_send_err!(
                                { tx.send(Err(E::NoOpenReturnCXsToClose)) },
                                DemandId::CloseReturnContext
                            );
                        }
                    }
                    Demand::SetReturnValue(vl, tx) => {
                        let Some(target) = rcx.last() else {
                            chk_send_err!(
                                { tx.send(Err(E::NoOpenReturnCXToBreak)) },
                                DemandId::SetReturnValue
                            );
                            continue;
                        };
                        if returns.contains_key(target) {
                            chk_send_err!(
                                { tx.send(Err(E::ReturnValueAlreadyExist(*target))) },
                                DemandId::SetReturnValue
                            );
                        } else {
                            returns.insert(*target, vl);
                            chk_send_err!({ tx.send(Ok(())) }, DemandId::SetReturnValue);
                        }
                    }
                    Demand::WithdrawReturnValue(uuid, tx) => {
                        chk_send_err!(
                            { tx.send(Ok(returns.remove(&uuid))) },
                            DemandId::WithdrawReturnValue
                        );
                    }
                    Demand::IsStopped(tx) => {
                        chk_send_err!(
                            {
                                tx.send(
                                    loops
                                        .last()
                                        .map(|uuid| breaks.contains(uuid))
                                        .unwrap_or(false)
                                        || rcx
                                            .last()
                                            .map(|uuid| returns.contains_key(uuid))
                                            .unwrap_or(false),
                                )
                            },
                            DemandId::IsStopped
                        );
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

    pub async fn open_loop(&self, uuid: &Uuid) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::OpenLoop(*uuid, tx))?;
        rx.await?
    }

    pub async fn close_loop(&self) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::CloseLoop(tx))?;
        rx.await?
    }

    pub async fn set_break(&self) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::SetBreakSignal(tx))?;
        rx.await?
    }

    pub async fn open_return_cx(&self, uuid: &Uuid) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::OpenReturnContext(*uuid, tx))?;
        rx.await?
    }

    pub async fn close_return_cx(&self) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::CloseReturnContext(tx))?;
        rx.await?
    }

    pub async fn set_return_vl(&self, vl: RtValue) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::SetReturnValue(vl, tx))?;
        rx.await?
    }

    pub async fn is_stopped(&self) -> Result<bool, E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::IsStopped(tx))?;
        Ok(rx.await?)
    }

    pub async fn withdraw_return_vl(&self, uuid: &Uuid) -> Result<Option<RtValue>, E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::WithdrawReturnValue(*uuid, tx))?;
        rx.await?
    }

    pub async fn destroy(&self) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::Destroy(tx))?;
        Ok(rx.await?)
    }
}
