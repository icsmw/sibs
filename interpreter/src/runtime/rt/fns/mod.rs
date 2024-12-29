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
                                { tx.send(Err(E::NoLinkedFunctions(uuid))) },
                                DemandId::Execute
                            );
                            continue;
                        };
                        if let Err(err) = rt.scopes.enter(&fn_entity.uuid).await {
                            chk_send_err!({ tx.send(Err(err)) }, DemandId::Execute);
                            continue;
                        }
                        let mut err = None;
                        for (n, vl) in args.into_iter().enumerate() {
                            let Some(decl) = fn_entity.args.get(n) else {
                                err = Some(E::InvalidFnArgument);
                                break;
                            };
                            let Some(vl_ty) = vl.as_ty() else {
                                err = Some(E::InvalidFnArgument);
                                break;
                            };
                            if !decl.ty.compatible(&vl_ty) {
                                err = Some(E::InvalidFnArgument);
                                break;
                            }
                            if let Err(e) = rt.scopes.insert(&fn_entity.name, vl).await {
                                err = Some(e);
                                break;
                            }
                        }
                        if let Some(err) = err.take() {
                            if let Err(err) = rt.scopes.leave().await {
                                chk_send_err!({ tx.send(Err(err)) }, DemandId::Execute);
                                continue;
                            }
                            chk_send_err!({ tx.send(Err(err)) }, DemandId::Execute);
                            continue;
                        }
                        let result = fn_entity.node.interpret(rt.clone()).await;
                        if let Err(err) = rt.scopes.leave().await {
                            chk_send_err!({ tx.send(Err(err)) }, DemandId::Execute);
                            continue;
                        }
                        // chk_send_err!({ tx.send(result) }, DemandId::Execute);
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
    ) -> Result<RtValue, E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::Execute(*uuid, rt, args, tx))?;
        rx.await?
    }

    pub async fn destroy(&self) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::Destroy(tx))?;
        Ok(rx.await?)
    }
}
