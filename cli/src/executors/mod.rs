mod api;
pub mod cx;
pub mod env;
mod error;
pub mod fs;
pub mod load;
pub mod logs;
pub mod store;
pub mod str;
pub mod test;

use crate::inf::{any::AnyValue, context::Context, Scope};
use api::*;
pub use error::E;
use std::{future::Future, pin::Pin};
use store::*;
use tokio::{
    spawn,
    sync::{
        mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
        oneshot,
    },
};
use tokio_util::sync::CancellationToken;

pub type ExecutorPinnedResult = Pin<Box<dyn Future<Output = ExecutorResult> + Send>>;
pub type ExecutorResult = Result<AnyValue, E>;
pub type ExecutorFn = fn(Vec<AnyValue>, Context, Scope) -> ExecutorPinnedResult;

pub trait Executor {
    fn execute(args: Vec<AnyValue>, cx: Context, sc: Scope) -> ExecutorPinnedResult;
    fn get_name() -> String;
}

pub trait TryAnyTo<T> {
    fn try_to(&self) -> Result<T, E>;
}

pub fn register(store: &mut Store) -> Result<(), E> {
    str::register(store)?;
    fs::register(store)?;
    env::register(store)?;
    logs::register(store)?;
    cx::register(store)?;
    test::register(store)?;
    Ok(())
}

#[derive(Clone, Debug)]
pub struct Functions {
    tx: UnboundedSender<api::Demand>,
    state: CancellationToken,
}

impl Functions {
    pub fn init() -> Result<Self, E> {
        let (tx, mut rx): (UnboundedSender<api::Demand>, UnboundedReceiver<api::Demand>) =
            unbounded_channel();
        let state = CancellationToken::new();
        let instance = Self {
            tx,
            state: state.clone(),
        };
        let mut store = Store::new();
        register(&mut store)?;
        spawn(async move {
            while let Some(demand) = rx.recv().await {
                match demand {
                    Demand::Execute(name, args, cx, sc, tx) => {
                        let Some(executor) = store.get(&name) else {
                            let _ = tx.send(Err(E::FunctionNotExists(name)));
                            continue;
                        };
                        let result = executor(args, cx, sc).await;
                        let _ = tx.send(result);
                    }
                    Demand::Destroy => {
                        break;
                    }
                }
            }
            state.cancel();
        });
        Ok(instance)
    }
    pub async fn destroy(&self) -> Result<(), E> {
        self.tx.send(Demand::Destroy)?;
        self.state.cancelled().await;
        Ok(())
    }
    /// Execute target function
    ///
    /// # Arguments
    ///
    /// * `name` - name of target function
    /// * `args` - arguments
    ///
    /// # Returns
    ///
    /// `Ok(AnyValue)` result of executing
    pub async fn execute(
        &self,
        name: &str,
        args: Vec<AnyValue>,
        cx: Context,
        sc: Scope,
    ) -> Result<AnyValue, E> {
        let (tx, rx) = oneshot::channel();
        self.tx
            .send(Demand::Execute(name.to_owned(), args, cx, sc, tx))?;
        rx.await?
    }
}
