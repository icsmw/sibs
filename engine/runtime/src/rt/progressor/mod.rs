mod api;
mod render;
mod state;

pub(crate) use state::*;

use crate::*;
use api::*;
use render::*;

use tokio::time::{self, Duration};
enum NextTick {
    Demand(Demand),
    Print,
    Exit,
}

#[derive(Clone, Debug)]
pub struct RtProgress {
    tx: UnboundedSender<Demand>,
}

impl RtProgress {
    #[tracing::instrument]
    pub fn new() -> Result<Self, E> {
        let (tx, mut rx) = unbounded_channel();
        let mut render: ProgressRender = ProgressRender::new()?;
        let instance = Self { tx };
        spawn(async move {
            let mut interval = time::interval(Duration::from_millis(60));
            tracing::info!("init demand's listener");
            loop {
                let tick = tokio::select! {
                    Some(demand) = rx.recv() => {
                        NextTick::Demand(demand)
                    }
                    _ = interval.tick() => {
                        NextTick::Print
                    }
                    else => {
                        tracing::info!("demand's rx is closed");
                        NextTick::Exit
                    }
                };
                match tick {
                    NextTick::Demand(demand) => match demand {
                        Demand::Register(identity, tx) => {
                            if let Err(err) = render.add(identity) {
                                chk_send_err!(tx.send(Err(err)), DemandId::Register);
                                continue;
                            };
                            chk_send_err!(tx.send(Ok(())), DemandId::Register);
                        }
                        Demand::SetState(uuid, state) => {
                            render.set_state(uuid, state);
                        }
                        Demand::SetMsg(uuid, msg) => {
                            render.set_msg(uuid, msg);
                        }
                        Demand::Destroy(tx) => {
                            render.destroy();
                            tracing::info!("got shutdown signal");
                            chk_send_err!(tx.send(()), DemandId::Destroy);
                            break;
                        }
                    },
                    NextTick::Print => {
                        render.print();
                    }
                    NextTick::Exit => {
                        break;
                    }
                }
            }
            tracing::info!("shutdown demand's listener");
        });
        Ok(instance)
    }

    pub(crate) async fn register(&self, identity: &JobIdentity) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::Register(identity.clone(), tx))?;
        rx.await?
    }

    pub fn set_state(&self, identity: &JobIdentity, state: ProgressState) {
        chk_send_err!(
            self.tx.send(Demand::SetState(identity.uuid(), state)),
            DemandId::SetState
        );
    }

    pub fn set_msg<S: ToString>(&self, identity: &JobIdentity, msg: S) {
        chk_send_err!(
            self.tx
                .send(Demand::SetMsg(identity.uuid(), msg.to_string())),
            DemandId::SetMsg
        );
    }

    pub async fn destroy(&self) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::Destroy(tx))?;
        Ok(rx.await?)
    }
}

#[ignore]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn visual_test() {
    use tokio::time::{self, Duration};
    let progressor = RtProgress::new().expect("RtProgress has been created");
    let mut jobs = Vec::new();
    for job in ["a", "b", "c", "d"] {
        let master = progressor
            .create(Uuid::new_v4(), format!("Job {job}"), None)
            .await
            .expect("Job's progress created");
        for sub in 0..5 {
            let child = master
                .child(format!("sub job {job} #{sub}"))
                .await
                .expect("Sub job is created");
            if sub % 2 == 0 {
                child.pending(Some("Pending task"));
            }
            jobs.push(child);
        }
        jobs.push(master);
    }
    let interval_duration = Duration::from_millis(500);
    let mut interval = time::interval(interval_duration);
    let mut iterations = 10;
    loop {
        interval.tick().await;
        iterations -= 1;
        if iterations == 0 {
            break;
        }
    }
}
