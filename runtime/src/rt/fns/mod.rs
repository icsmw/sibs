mod api;

use crate::*;
use api::*;
use tokio::spawn;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct RtFns {
    tx: UnboundedSender<Demand>,
}

impl RtFns {
    #[tracing::instrument]
    pub fn new(fns: Fns) -> Self {
        let (tx, mut rx) = unbounded_channel();
        spawn(async move {
            tracing::info!("init demand's listener");
            while let Some(demand) = rx.recv().await {
                match demand {
                    Demand::Execute(uuid, rt, args, tx) => {
                        let Some(fn_entity) = fns.lookup_by_caller(&uuid) else {
                            chk_send_err!(
                                { tx.send(Err(LinkedErr::unlinked(E::NoLinkedFunctions(uuid)))) },
                                DemandId::Execute
                            );
                            continue;
                        };
                        chk_send_err!(
                            tx.send(fn_entity.execute(rt, args).await),
                            DemandId::Execute
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
    /// Asynchronously executes a function in the runtime with the given parameters.
    ///
    /// # Arguments
    ///
    /// * `uuid` - A reference to a `Uuid` of caller node.
    /// * `rt` - The runtime environment in which the function will be executed.
    /// * `args` - A vector of `RtValue` containing the arguments to pass to the function.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing either:
    /// * `RtValue` - The result of the executed function.
    /// * `E` - An error if the function execution fails.
    ///
    /// # Errors
    ///
    /// This function returns an error if:
    /// * Sending the execution demand to the runtime fails.
    /// * Awaiting the response from the runtime fails.
    pub async fn execute(
        &self,
        uuid: &Uuid,
        rt: Runtime,
        args: Vec<RtValue>,
    ) -> Result<RtValue, LinkedErr<E>> {
        let (tx, rx) = oneshot::channel();
        self.tx
            .send(Demand::Execute(*uuid, rt, args, tx))
            .map_err(|e| LinkedErr::unlinked(e.into()))?;
        rx.await.map_err(|e| LinkedErr::unlinked(e.into()))?
    }

    pub async fn destroy(&self) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::Destroy(tx))?;
        Ok(rx.await?)
    }
}
