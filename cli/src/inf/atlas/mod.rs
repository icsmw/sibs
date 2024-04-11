mod api;
mod error;
mod footprints;
mod map;
mod maps;

pub use error::*;
pub use footprints::*;
pub use map::*;
pub use maps::*;
use std::{collections::HashMap, path::PathBuf};
use tokio::{
    spawn,
    sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
};
use tokio_util::sync::CancellationToken;

use self::api::Demand;

#[derive(Debug)]
pub struct Atlas {
    tx: UnboundedSender<api::Demand>,
    state: CancellationToken,
}

impl Atlas {
    pub fn new(mut maps: HashMap<PathBuf, Map>) -> Self {
        let (tx, mut rx): (UnboundedSender<api::Demand>, UnboundedReceiver<api::Demand>) =
            unbounded_channel();
        let state = CancellationToken::new();
        let instance = Self {
            tx,
            state: state.clone(),
        };
        spawn(async move {
            let mut footprints = Footprints::new(None);
            while let Some(demand) = rx.recv().await {
                match demand {
                    Demand::AddFootprint(token, value, rx) => {
                        let _ = rx.send(Ok(footprints.add(&token, value)));
                    }
                    Demand::ReportErr(token, err, rx) => {
                        let _ =
                            rx.send(footprints.report_err(&mut Maps::new(&mut maps), &token, err));
                    }
                    Demand::SetMapPosition(token, rx) => {
                        let mut maps = Maps::new(&mut maps);
                        let map = match maps.get(&token) {
                            Ok(map) => map,
                            Err(err) => {
                                let _ = rx.send(Err(err));
                                continue;
                            }
                        };
                        let _ = rx.send(Ok(map.set_cursor(token)));
                    }
                }
            }

            state.cancel();
        });
        instance
    }
}
