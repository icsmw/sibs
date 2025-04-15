mod api;
mod progress;
mod render;
mod state;

pub(crate) use progress::*;
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
        let this = instance.clone();
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
                        Demand::Create(owner, alias, parent, tx) => {
                            let job = Progress::new(owner, alias, parent, this.clone());
                            if let Err(err) = render.add(&job) {
                                chk_send_err!(tx.send(Err(err)), DemandId::Create);
                                continue;
                            };
                            chk_send_err!(tx.send(Ok(job)), DemandId::Create);
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

    pub(crate) async fn create<S: ToString>(
        &self,
        owner: Uuid,
        alias: S,
        parent: Option<Uuid>,
    ) -> Result<Progress, E> {
        let (tx, rx) = oneshot::channel();
        self.tx
            .send(Demand::Create(owner, alias.to_string(), parent, tx))?;
        rx.await?
    }

    pub fn set_state(&self, uuid: &Uuid, state: ProgressState) {
        chk_send_err!(
            self.tx.send(Demand::SetState(*uuid, state)),
            DemandId::SetState
        );
    }

    pub fn set_msg<S: ToString>(&self, uuid: &Uuid, msg: S) {
        chk_send_err!(
            self.tx.send(Demand::SetMsg(*uuid, msg.to_string())),
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
#[tokio::test]
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
