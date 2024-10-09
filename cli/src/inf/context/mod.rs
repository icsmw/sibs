pub mod atlas;
pub mod closures;
pub mod error;
pub mod scenario;
pub mod tracker;
pub mod variables;

use bstorage::Storage;
use closures::Closures;
use std::{fmt, process, sync::Arc};

use crate::{
    elements::FuncArg,
    error::LinkedErr,
    functions::{ExecutorFnDescription, Functions},
    inf::{Journal, Scope, ScopeDomain, Signals, Value},
    reader::Sources,
};
pub use atlas::*;
pub use error::E;
pub use scenario::*;
use tokio::{
    spawn,
    sync::{
        mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
        oneshot,
    },
};
use tokio_util::sync::CancellationToken;
pub use tracker::*;
pub use variables::*;

use super::ValueRef;

const SIBS_FOLDER: &str = ".sibs";

#[derive(Debug)]
/// Cases to close application
pub enum ExitCode {
    /// Immediately close the application with given exit's code and message. It will
    /// not wait for operations, which might be still running.
    ///
    /// # Parameters
    ///
    /// * `i32` - exit code
    Immediately(i32),
    /// Store given exit code and message to use it as soon destructor of context
    /// will be called in regular way.
    ///
    /// # Parameters
    ///
    /// * `i32` - exit code
    Aborting(i32),
    /// Close application in regular way
    Success,
}

impl ExitCode {
    pub fn code(&self) -> i32 {
        match self {
            Self::Aborting(code) | Self::Immediately(code) => *code,
            Self::Success => 0,
        }
    }
}

impl From<&ExitCodeMessage> for ExitCode {
    fn from(value: &ExitCodeMessage) -> Self {
        match value {
            ExitCodeMessage::Aborting(code, ..) => Self::Aborting(*code),
            ExitCodeMessage::Immediately(code, ..) => Self::Immediately(*code),
            ExitCodeMessage::Regular(..) => Self::Success,
        }
    }
}

impl fmt::Display for ExitCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Aborting(..) => "abort",
                Self::Immediately(..) => "exit",
                Self::Success => "success",
            }
        )
    }
}
/// Defines a way to close application
pub enum ExitCodeMessage {
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
    /// # Parameters
    ///
    /// * `ExitCode` - exit code
    Regular(oneshot::Sender<ExitCode>),
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
    pub variables: VariablesMeta,
    pub closures: Closures,
    tx: UnboundedSender<ExitCodeMessage>,
}

impl Context {
    pub fn init(scenario: Scenario, src: &Sources, journal: &Journal) -> Result<Self, E> {
        let tracker = Tracker::init(journal);
        let atlas = Atlas::init(src, journal);
        let funcs = Functions::init(journal)?;
        let scope = ScopeDomain::init(&scenario.path, journal);
        let variables = VariablesMeta::init(journal);
        let closures = Closures::init(journal);
        let signals = Signals::init(journal);
        let (tx, mut rx): (
            UnboundedSender<ExitCodeMessage>,
            UnboundedReceiver<ExitCodeMessage>,
        ) = unbounded_channel();
        let instance = Self {
            scenario,
            tracker: tracker.clone(),
            journal: journal.clone(),
            atlas: atlas.clone(),
            funcs: funcs.clone(),
            scope: scope.clone(),
            signals: signals.clone(),
            variables: variables.clone(),
            closures: closures.clone(),
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
                    let _ = journal.err_if("variables", variables.destroy().await);
                    let _ = journal.err_if("closures", closures.destroy().await);
                })
            };
            let mut exit_code = ExitCode::Success;
            while let Some(cmd) = rx.recv().await {
                let recv_exit_code: ExitCode = (&cmd).into();
                let immediately = matches!(cmd, ExitCodeMessage::Immediately(..));
                match cmd {
                    ExitCodeMessage::Aborting(code, msg)
                    | ExitCodeMessage::Immediately(code, msg) => {
                        exit_code = recv_exit_code;
                        journal.warn("", format!("Forced {exit_code} with code {code}"));
                        journal.flush().await;
                        if let Some(msg) = msg {
                            if code == 0 {
                                println!("{msg}");
                            } else {
                                eprintln!("{msg}");
                            }
                        }
                        if immediately {
                            shutdown(&journal).await;
                            process::exit(code);
                        } else {
                            continue;
                        }
                    }
                    ExitCodeMessage::Regular(tx) => {
                        shutdown(&journal).await;
                        if tx.send(exit_code).is_err() {
                            eprintln!("Fail to confirm context destroy")
                        }
                        break;
                    }
                }
            }
        });
        Ok(instance)
    }

    pub async fn destroy(&self) -> Result<ExitCode, E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(ExitCodeMessage::Regular(tx))?;
        Ok(rx.await?)
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
        self.tx.send(ExitCodeMessage::Immediately(code, msg))?;
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
        self.tx.send(ExitCodeMessage::Aborting(code, msg))?;
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
        ty: Option<Value>,
        sc: Scope,
    ) -> Result<Value, LinkedErr<E>> {
        // TODO: switch to element instead "name"
        self.funcs
            .execute(name, args, args_token, ty, self.clone(), sc)
            .await
            .map_err(|e| LinkedErr::new(e.e.into(), e.token))
    }
    pub async fn get_func_desc(
        &self,
        name: &str,
        ty: Option<ValueRef>,
    ) -> Result<Arc<ExecutorFnDescription>, LinkedErr<E>> {
        self.funcs
            .get_func_desc(name, ty)
            .await
            .map_err(|e| LinkedErr::new(e.e.into(), e.token))
    }
}
