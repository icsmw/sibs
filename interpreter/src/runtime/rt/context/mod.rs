mod api;
mod params;

use crate::*;
use api::*;
pub use params::RtParameters;
use tokio::spawn;

#[derive(Debug, Clone)]
pub struct RtContext {
    tx: UnboundedSender<Demand>,
}

impl RtContext {
    #[tracing::instrument]
    pub fn new(params: RtParameters) -> Self {
        let (tx, mut rx) = unbounded_channel();
        spawn(async move {
            tracing::info!("init demand's listener");
            while let Some(demand) = rx.recv().await {
                match demand {
                    Demand::GetTargetComponent(tx) => {
                        chk_send_err!(
                            { tx.send(params.component.clone(),) },
                            DemandId::GetTargetComponent
                        );
                    }
                    Demand::GetTaskParams(tx) => {
                        chk_send_err!(
                            { tx.send((params.task.clone(), params.args.to_vec(),)) },
                            DemandId::GetTaskParams
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

    pub async fn get_target_component(&self) -> Result<String, E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::GetTargetComponent(tx))?;
        rx.await.map_err(|e| e.into())
    }

    pub async fn get_task_params(&self) -> Result<(String, Vec<String>), E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::GetTaskParams(tx))?;
        rx.await.map_err(|e| e.into())
    }

    pub async fn destroy(&self) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::Destroy(tx))?;
        Ok(rx.await?)
    }
}
