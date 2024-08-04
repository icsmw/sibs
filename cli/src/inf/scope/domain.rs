use crate::inf::{
    context::E,
    scope::{Demand, Scope, Session, Sessions},
    Journal,
};
use std::path::{Path, PathBuf};
use tokio::{
    spawn,
    sync::{
        mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
        oneshot,
    },
};
use tokio_util::sync::CancellationToken;

#[derive(Debug, Clone)]
pub struct ScopeDomain {
    tx: UnboundedSender<Demand>,
    journal: Journal,
    state: CancellationToken,
}

impl ScopeDomain {
    pub fn init<P: AsRef<Path>>(root: P, journal: &Journal) -> Self {
        let (tx, mut rx): (UnboundedSender<Demand>, UnboundedReceiver<Demand>) =
            unbounded_channel();
        let state = CancellationToken::new();
        let instance = Self {
            tx,
            journal: journal.clone(),
            state: state.clone(),
        };
        let root = root.as_ref().to_path_buf();
        let journal = journal.clone();
        let own_journal = journal.owned("Scope Controller", None);
        spawn(async move {
            let mut sessions = Sessions::default();
            let mut globals = Session::new("globals", root.clone(), &journal);
            while let Some(demand) = rx.recv().await {
                let requested = demand.to_string();
                if matches!(demand, Demand::Destroy) {
                    break;
                } else if match demand {
                    Demand::AddSession(alias, cwd, tx) => {
                        let uuid = sessions.add(
                            alias,
                            cwd.map(|p| {
                                if p.is_absolute() {
                                    p
                                } else {
                                    globals.get_cwd().join(p)
                                }
                            })
                            .unwrap_or(globals.get_cwd()),
                            &journal,
                        );
                        tx.send(uuid).is_err()
                    }
                    Demand::RemoveSession(session, tx) => {
                        sessions.remove(session);
                        tx.send(()).is_err()
                    }
                    Demand::SetGlobalVariable(k, v, tx) => {
                        tx.send(Ok(globals.set_var(&k, v))).is_err()
                    }

                    Demand::GetGlobalVariable(k, tx) => tx.send(Ok(globals.get_var(&k))).is_err(),
                    Demand::GetGlobalCwd(tx) => tx.send(Ok(globals.get_cwd())).is_err(),
                    Demand::SetVariable(session, k, v, tx) => tx
                        .send(
                            sessions
                                .get(&session)
                                .map(|session| Ok(session.set_var(&k, v)))
                                .unwrap_or(Err(E::NoScopeSession(session))),
                        )
                        .is_err(),

                    Demand::GetVariable(session, k, tx) => tx
                        .send(
                            sessions
                                .get(&session)
                                .map(|session| Ok(session.get_var(&k)))
                                .unwrap_or(Err(E::NoScopeSession(session))),
                        )
                        .is_err(),
                    Demand::ImportVars(dest, src, tx) => {
                        let vars = sessions.get(&src).map(|session| session.get_vars().clone());
                        tx.send(
                            if let (Some(session), Some(vars)) = (sessions.get(&dest), vars) {
                                session.import_vars(vars);
                                Ok(())
                            } else {
                                Err(E::NoScopeSession(dest))
                            },
                        )
                        .is_err()
                    }
                    Demand::SetCwd(session, path, tx) => tx
                        .send(
                            sessions
                                .get(&session)
                                .map(|session| {
                                    session.set_cwd(path);
                                    Ok(())
                                })
                                .unwrap_or(Err(E::NoScopeSession(session))),
                        )
                        .is_err(),
                    Demand::GetCwd(session, tx) => tx
                        .send(
                            sessions
                                .get(&session)
                                .map(|session| Ok(session.get_cwd()))
                                .unwrap_or(Err(E::NoScopeSession(session))),
                        )
                        .is_err(),
                    Demand::OpenLoop(session, tx) => tx
                        .send(
                            sessions
                                .get(&session)
                                .map(|session| Ok(session.open_loop()))
                                .unwrap_or(Err(E::NoScopeSession(session))),
                        )
                        .is_err(),
                    Demand::CloseLoop(session, uuid, tx) => tx
                        .send(
                            sessions
                                .get(&session)
                                .map(|session| {
                                    session.close_loop(uuid);
                                    Ok(())
                                })
                                .unwrap_or(Err(E::NoScopeSession(session))),
                        )
                        .is_err(),
                    Demand::BreakLoop(session, tx) => tx
                        .send(
                            sessions
                                .get(&session)
                                .map(|session| Ok(session.break_loop()))
                                .unwrap_or(Err(E::NoScopeSession(session))),
                        )
                        .is_err(),
                    _ => true,
                } {
                    own_journal.err(format!("Fail to send response for \"{requested}\""));
                    break;
                };
            }
            state.cancel();
        });
        instance
    }

    pub async fn destroy(&self) -> Result<(), E> {
        self.tx.send(Demand::Destroy)?;
        self.state.cancelled().await;
        Ok(())
    }

    pub async fn create<S: AsRef<str>>(&self, alias: S, cwd: Option<PathBuf>) -> Result<Scope, E> {
        let (tx, rx) = oneshot::channel();
        self.tx
            .send(Demand::AddSession(alias.as_ref().to_string(), cwd, tx))?;
        Ok(Scope::new(
            self.tx.clone(),
            rx.await?,
            self.journal.owned(alias, None),
        ))
    }
}
