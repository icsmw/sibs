mod api;

use crate::inf::AnyValue;
use std::{
    sync::Arc,
    {collections::HashMap, path::PathBuf},
};
use tokio::{
    spawn,
    sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
};
use tokio_util::sync::CancellationToken;

pub struct TaskContext {
    tx: UnboundedSender<api::Demand>,
    state: CancellationToken,
}

impl TaskContext {
    pub fn new(mut cwd: Option<PathBuf>) -> Self {
        let (tx, mut rx): (UnboundedSender<api::Demand>, UnboundedReceiver<api::Demand>) =
            unbounded_channel();
        let state = CancellationToken::new();
        let instance = Self {
            tx,
            state: state.clone(),
        };
        spawn(async move {
            let mut vars: HashMap<String, Arc<AnyValue>> = HashMap::new();
            while let Some(demand) = rx.recv().await {
                match demand {
                    api::Demand::SetVariable(k, v, tx) => {
                        let _ = tx.send(vars.insert(k, Arc::new(v)).is_some());
                    }
                    api::Demand::GetVariable(k, tx) => {
                        let _ = tx.send(vars.get(&k).cloned());
                    }
                    api::Demand::SetCwd(path, tx) => {
                        cwd = path;
                        let _ = tx.send(());
                    }
                    api::Demand::GetCwd(tx) => {
                        let _ = tx.send(cwd.clone());
                    }
                }
            }
            state.cancel();
        });
        instance
    }
    pub fn is_done(&self) -> bool {
        self.state.is_cancelled()
    }
    pub async fn wait(&self) {
        self.state.cancelled().await
    }
}
