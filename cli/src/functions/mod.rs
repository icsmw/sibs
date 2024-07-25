mod api;
pub mod cx;
pub mod env;
mod error;
pub mod fs;
pub mod hash;
pub mod load;
pub mod logs;
pub mod process;
pub mod sig;
pub mod str;
pub mod test;

use crate::{
    elements::FuncArg,
    error::LinkedErr,
    inf::{AnyValue, Context, Journal, Scope, Store},
};
use api::*;
pub use error::E;
use std::{future::Future, pin::Pin};
use tokio::{
    spawn,
    sync::{
        mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
        oneshot,
    },
};
use tokio_util::sync::CancellationToken;

pub type ExecutorPinnedResult = Pin<Box<dyn Future<Output = ExecutorResult> + Send>>;
pub type ExecutorResult = Result<AnyValue, LinkedErr<E>>;
pub type ExecutorFn = fn(Vec<FuncArg>, usize, Context, Scope) -> ExecutorPinnedResult;

pub trait TryAnyTo<T> {
    fn try_to(&self) -> Result<T, E>;
}

pub fn register(store: &mut Store<ExecutorFn>) -> Result<(), E> {
    str::register(store)?;
    fs::register(store)?;
    env::register(store)?;
    logs::register(store)?;
    cx::register(store)?;
    process::register(store)?;
    sig::register(store)?;
    hash::register(store)?;
    test::register(store)?;
    Ok(())
}

#[derive(Clone, Debug)]
pub struct Functions {
    tx: UnboundedSender<api::Demand>,
    state: CancellationToken,
}

impl Functions {
    pub fn init(journal: &Journal) -> Result<Self, E> {
        let (tx, mut rx): (UnboundedSender<api::Demand>, UnboundedReceiver<api::Demand>) =
            unbounded_channel();
        let state = CancellationToken::new();
        let instance = Self {
            tx,
            state: state.clone(),
        };
        let mut store = Store::<ExecutorFn>::new();
        let journal = journal.owned("Functions".to_owned(), None);
        register(&mut store)?;
        spawn(async move {
            while let Some(demand) = rx.recv().await {
                match demand {
                    Demand::Execute(name, tx) => {
                        let Some(executor) = store.get(&name) else {
                            if tx.send(Err(E::FunctionNotExists(name))).is_err() {
                                journal.err("Fail send response for Execute command");
                            }
                            continue;
                        };
                        if tx.send(Ok(executor)).is_err() {
                            journal.err(format!("Fail send function's execute: {name}"));
                        }
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
        args: Vec<FuncArg>,
        args_token: usize,
        cx: Context,
        sc: Scope,
    ) -> Result<AnyValue, LinkedErr<E>> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::Execute(name.to_owned(), tx))?;
        let executor = rx.await??;
        executor(args, args_token, cx, sc).await
    }
}
