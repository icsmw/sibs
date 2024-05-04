pub mod atlas;
pub mod error;
pub mod scenario;
pub mod tracker;

use std::process;

pub use atlas::*;
pub use error::E;
pub use scenario::*;
pub use tracker::*;

use crate::{
    executors::Functions,
    inf::{AnyValue, Journal, Scope},
    reader::Sources,
};
use tokio::{
    spawn,
    sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
};
use tokio_util::sync::CancellationToken;

pub type ExitCode = Option<(i32, Option<String>)>;

#[derive(Clone, Debug)]
pub struct Context {
    pub tracker: Tracker,
    pub atlas: Atlas,
    pub scenario: Scenario,
    pub journal: Journal,
    pub funcs: Functions,
    tx: UnboundedSender<ExitCode>,
    state: CancellationToken,
}

impl Context {
    pub fn init(scenario: Scenario, src: &Sources, journal: &Journal) -> Result<Self, E> {
        let state = CancellationToken::new();
        let tracker = Tracker::init(journal.clone());
        let atlas = Atlas::init(src, journal);
        let funcs = Functions::init()?;
        let (tx, mut rx): (UnboundedSender<ExitCode>, UnboundedReceiver<ExitCode>) =
            unbounded_channel();
        let instance = Self {
            scenario,
            tracker: tracker.clone(),
            journal: journal.clone(),
            atlas: atlas.clone(),
            funcs: funcs.clone(),
            state: state.clone(),
            tx,
        };
        let journal = journal.clone();
        spawn(async move {
            let shutdown = move || {
                Box::pin(async move {
                    let _ = tracker.destroy().await;
                    let _ = atlas.destroy().await;
                    let _ = funcs.destroy().await;
                })
            };
            let exit = rx.recv().await.expect("Correct destroy signal to context");
            if let Some((code, msg)) = exit {
                journal.warn("", format!("Forced exit with code {code}"));
                journal.flush().await;
                shutdown().await;
                if let Some(msg) = msg {
                    if code == 0 {
                        println!("{msg}");
                    } else {
                        eprintln!("{msg}");
                    }
                }
                process::exit(code);
            } else {
                shutdown().await;
            }
            state.cancel();
        });
        Ok(instance)
    }

    pub async fn destroy(&self) -> Result<(), E> {
        self.tx.send(None)?;
        self.state.cancelled().await;
        Ok(())
    }

    pub async fn exit(&self, code: i32, msg: Option<String>) -> Result<(), E> {
        self.tx.send(Some((code, msg)))?;
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
    pub async fn execute(&self, name: &str, args: Vec<AnyValue>, sc: Scope) -> Result<AnyValue, E> {
        Ok(self.funcs.execute(name, args, self.clone(), sc).await?)
    }
}
