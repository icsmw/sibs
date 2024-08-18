pub mod atlas;
pub mod error;
pub mod scenario;
pub mod tracker;

use bstorage::Storage;
use std::process;

pub use atlas::*;
pub use error::E;
pub use scenario::*;
pub use tracker::*;

use crate::{
    elements::FuncArg,
    error::LinkedErr,
    functions::Functions,
    inf::{Journal, Scope, ScopeDomain, Signals, Value},
    reader::Sources,
};
use tokio::{
    spawn,
    sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
};
use tokio_util::sync::CancellationToken;

const SIBS_FOLDER: &str = ".sibs";

/// Defines a way to close application
pub enum ExitCode {
    /// Immediately close the application with given exit's code and message. It will
    /// not wait for operations, which might be still running.
    ///
    /// # Parameters
    ///
    /// * `i32` - exit code
    /// * `Option<String>` - message to post in stdout before exit
    Immediately(i32, Option<String>),
    /// Store given exit code and message to use it as soon destructor of context
    /// will be called in regular way.
    ///
    /// # Parameters
    ///
    /// * `i32` - exit code
    /// * `Option<String>` - message to post in stdout before exit
    Aborting(i32, Option<String>),
    /// Close application in regular way
    Regular,
}

#[derive(Clone, Debug)]
pub struct Context {
    pub tracker: Tracker,
    pub atlas: Atlas,
    pub scenario: Scenario,
    pub journal: Journal,
    pub funcs: Functions,
    pub aborting: CancellationToken,
    pub scope: ScopeDomain,
    pub signals: Signals,
    tx: UnboundedSender<ExitCode>,
    state: CancellationToken,
}

impl Context {
    pub fn init(scenario: Scenario, src: &Sources, journal: &Journal) -> Result<Self, E> {
        let state = CancellationToken::new();
        let tracker = Tracker::init(journal.clone());
        let atlas = Atlas::init(src, journal);
        let funcs = Functions::init(journal)?;
        let scope = ScopeDomain::init(&scenario.path, journal);
        let signals = Signals::init(journal);
        let (tx, mut rx): (UnboundedSender<ExitCode>, UnboundedReceiver<ExitCode>) =
            unbounded_channel();
        let instance = Self {
            scenario,
            tracker: tracker.clone(),
            journal: journal.clone(),
            atlas: atlas.clone(),
            funcs: funcs.clone(),
            state: state.clone(),
            scope: scope.clone(),
            signals: signals.clone(),
            aborting: CancellationToken::new(),
            tx,
        };
        let journal = journal.clone();
        spawn(async move {
            let shutdown = |journal: &Journal| {
                let journal = journal.clone();
                Box::pin(async move {
                    let _ = journal.err_if("tracker", tracker.destroy().await);
                    let _ = journal.err_if("atlas", atlas.destroy().await);
                    let _ = journal.err_if("functions", funcs.destroy().await);
                    let _ = journal.err_if("scope", scope.destroy().await);
                    let _ = journal.err_if("signals", signals.destroy().await);
                })
            };
            let mut exit_code = ExitCode::Regular;
            while let Some(code) = rx.recv().await {
                let breaking = !matches!(code, ExitCode::Aborting(..));
                if !matches!(code, ExitCode::Regular) {
                    exit_code = code;
                }
                if breaking {
                    break;
                }
            }
            match exit_code {
                ExitCode::Immediately(code, msg) | ExitCode::Aborting(code, msg) => {
                    journal.warn("", format!("Forced exit with code {code}"));
                    journal.flush().await;
                    shutdown(&journal).await;
                    if let Some(msg) = msg {
                        if code == 0 {
                            println!("{msg}");
                        } else {
                            eprintln!("{msg}");
                        }
                    }
                    process::exit(code);
                }
                ExitCode::Regular => {
                    shutdown(&journal).await;
                }
            }
            state.cancel();
        });
        Ok(instance)
    }

    pub async fn destroy(&self) -> Result<(), E> {
        self.tx.send(ExitCode::Regular)?;
        self.state.cancelled().await;
        Ok(())
    }

    /// Destroy context with dependencies and immediately exit from the application
    /// with given exit's code and message. It will not wait for operations, which
    /// might be still running.
    ///
    /// # Arguments
    ///
    /// * `code` - code to exit
    /// * `msg` - message to post into stdout before exit
    pub async fn exit(&self, code: i32, msg: Option<String>) -> Result<(), E> {
        self.tx.send(ExitCode::Immediately(code, msg))?;
        Ok(())
    }

    /// Send cancellation signal to context and waits for all. Set code of exit
    /// after all operation will be done/cancelled.
    ///
    /// # Arguments
    ///
    /// * `code` - code to exit
    /// * `msg` - message to post into stdout before exit
    pub async fn abort(&self, code: i32, msg: Option<String>) -> Result<(), E> {
        self.tx.send(ExitCode::Aborting(code, msg))?;
        self.aborting.cancel();
        Ok(())
    }

    pub fn is_aborting(&self) -> bool {
        self.aborting.is_cancelled()
    }

    pub fn get_storage(&self) -> Result<Storage, E> {
        Ok(Storage::create(
            self.scenario.path.join(SIBS_FOLDER).join("storage"),
        )?)
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
    /// `Ok(Value)` result of executing
    pub async fn execute(
        &self,
        name: &str,
        args: Vec<FuncArg>,
        args_token: usize,
        sc: Scope,
    ) -> Result<Value, LinkedErr<E>> {
        // TODO: switch to element instead "name"
        self.funcs
            .execute(name, args, args_token, self.clone(), sc)
            .await
            .map_err(|e| LinkedErr::new(e.e.into(), e.token))
    }

    // pub async fn test_func(&self, name: &str, args: Vec<Value>, sc: Scope) -> Result<Value, E> {
    //     Ok(self.funcs.execute(name, args, self.clone(), sc).await?)
    // }
}
