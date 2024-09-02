mod api;
mod error;
mod footprints;
mod map;
mod maps;

use self::api::Demand;
use crate::inf::Journal;
use crate::reader::Sources;
use crate::{error::LinkedErr, inf::Value};
pub use error::*;
pub use footprints::*;
pub use map::*;
pub use maps::*;
use std::{collections::HashMap, fmt, path::PathBuf};
use tokio::{
    spawn,
    sync::{
        mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
        oneshot,
    },
};
use tokio_util::sync::CancellationToken;

enum Action {
    Check(bool),
    Break,
}

#[derive(Clone, Debug)]
pub struct Atlas {
    tx: UnboundedSender<api::Demand>,
    state: CancellationToken,
}

impl Atlas {
    pub fn init(src: &Sources, journal: &Journal) -> Self {
        let (tx, mut rx): (UnboundedSender<Demand>, UnboundedReceiver<Demand>) =
            unbounded_channel();
        let state = CancellationToken::new();
        let instance = Self {
            tx,
            state: state.clone(),
        };
        let mut maps: HashMap<PathBuf, Map> = HashMap::new();
        src.iter().for_each(|(path, map)| {
            maps.insert(path.to_owned(), Map::from(map.borrow_mut().clone()));
        });
        let mut footprints = Footprints::new(journal, None);
        let journal = journal.owned("Atlas", None);
        spawn(async move {
            while let Some(demand) = rx.recv().await {
                let requested = demand.to_string();
                let action = match demand {
                    Demand::AddFootprint(token, value, rx) => {
                        footprints.add(&token, value);
                        Action::Check(rx.send(Ok(())).is_err())
                    }
                    Demand::ReportErr(err, rx) => Action::Check(
                        rx.send(footprints.report_err(&mut Maps::new(&mut maps), err))
                            .is_err(),
                    ),
                    Demand::SetMapPosition(token, rx) => {
                        let mut maps = Maps::new(&mut maps);
                        Action::Check(match maps.get(&token) {
                            Ok(map) => {
                                map.set_cursor(token);
                                rx.send(Ok(())).is_err()
                            }
                            Err(err) => rx.send(Err(err)).is_err(),
                        })
                    }
                    Demand::Destroy => Action::Break,
                };
                match action {
                    Action::Check(is_err) => {
                        if is_err {
                            journal.err(format!("Fail to send response for \"{requested}\""));
                            break;
                        }
                    }
                    Action::Break => {
                        break;
                    }
                }
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

    /// Adds footprint into trace
    ///
    /// # Arguments
    ///
    /// * `token` - Token
    /// * `value_as_str` - String representation of value; None if value isn't set
    ///
    /// # Returns
    ///
    /// `Ok(())` in case of footpring has been added
    pub async fn add_footprint(&self, token: usize, value: &Value) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx
            .send(Demand::AddFootprint(token, value.to_string(), tx))?;
        let _ = rx.await?;
        Ok(())
    }
    /// Adds footprint into trace
    ///
    /// # Arguments
    ///
    /// * `token` - Token
    /// * `value_as_str` - String representation of value; None if value isn't set
    ///
    /// # Returns
    ///
    /// `Ok(())` in case of footpring has been added
    pub async fn report_err<T: Clone + fmt::Display>(&self, err: &LinkedErr<T>) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::ReportErr(err.serialize(), tx))?;
        let _ = rx.await?;
        Ok(())
    }
    /// Sets position of cursor in current map
    ///
    /// # Arguments
    ///
    /// * `token` - Token
    ///
    /// # Returns
    ///
    /// `Ok(())` in case of footpring has been added
    pub async fn set_map_position(&self, token: usize) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::SetMapPosition(token, tx))?;
        let _ = rx.await?;
        Ok(())
    }
}
