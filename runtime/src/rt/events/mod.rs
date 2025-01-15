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
                    Demand::ChkBreakSignal(uuid, tx) => {
                        chk_send_err!(
                            { tx.send(breaks.contains(&uuid)) },
                            DemandId::ChkBreakSignal
                        );
                    }
                    Demand::IsBreakInCurrentScope(tx) => {
                        chk_send_err!(
                            {
                                tx.send(
                                    loops
                                        .last()
                                        .map(|uuid| breaks.contains(uuid))
                                        .unwrap_or(false),
                                )
                            },
                            DemandId::IsBreakInCurrentScope
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

    pub async fn chk_break(&self, target: &Uuid) -> Result<bool, E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::ChkBreakSignal(*target, tx))?;
        Ok(rx.await?)
    }

    pub async fn is_break_in_current_scope(&self) -> Result<bool, E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::IsBreakInCurrentScope(tx))?;
        Ok(rx.await?)
    }

    pub async fn destroy(&self) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::Destroy(tx))?;
        Ok(rx.await?)
    }
}
