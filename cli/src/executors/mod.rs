mod api;
mod error;
mod inspect;

use crate::{
    elements::{Element, Function, Task},
    inf::{Context, Journal, Scope, Store},
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
pub type ExecutorResult = Result<bool, E>;
pub type ExecutorFn = fn(&Task, &[Element], Context, Scope) -> ExecutorPinnedResult;

pub fn register(store: &mut Store<ExecutorFn>) -> Result<(), E> {
    inspect::register(store)?;
    Ok(())
}

#[derive(Clone, Debug)]
pub struct Executors {
    tx: UnboundedSender<api::Demand>,
    state: CancellationToken,
}

impl Executors {
    pub fn init(journal: &Journal) -> Result<Self, E> {
        let (tx, mut rx): (UnboundedSender<api::Demand>, UnboundedReceiver<api::Demand>) =
            unbounded_channel();
        let state = CancellationToken::new();
        let instance = Self {
            tx,
            state: state.clone(),
        };
        let mut store = Store::<ExecutorFn>::new();
        let journal = journal.owned("Executors".to_owned(), None);
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
    /// Execute target executor
    ///
    /// # Arguments
    ///
    /// * `function` - function to execute
    /// * `task` - related task
    /// * `cx` - global context
    /// * `sc` - related task's scope
    ///
    /// # Returns
    ///
    /// `Ok(bool)` true - if task has to be run; false - if task can be skipped
    pub async fn execute(
        &self,
        function: &Function,
        task: &Task,
        cx: Context,
        sc: Scope,
    ) -> Result<bool, E> {
        let (tx, rx) = oneshot::channel();
        self.tx
            .send(Demand::Execute(function.name.to_owned(), tx))?;
        let executor = rx.await??;
        executor(task, &function.args, cx, sc).await
    }
}
