pub mod atlas;
pub mod error;
pub mod scenario;
pub mod tracker;

pub use atlas::*;
pub use error::E;
pub use scenario::*;
pub use tracker::*;

use crate::{executors::Functions, inf::journal::Journal, reader::Sources};
use tokio::spawn;
use tokio_util::sync::CancellationToken;

use super::AnyValue;

#[derive(Clone, Debug)]
pub struct Context {
    pub tracker: Tracker,
    pub atlas: Atlas,
    pub scenario: Scenario,
    pub journal: Journal,
    pub funcs: Functions,
    state: CancellationToken,
    signal: CancellationToken,
}

impl Context {
    pub fn init(scenario: Scenario, src: &Sources, journal: &Journal) -> Result<Self, E> {
        let state = CancellationToken::new();
        let signal = CancellationToken::new();
        let tracker = Tracker::init(journal.clone());
        let atlas = Atlas::init(src, journal);
        let funcs = Functions::init()?;
        let instance = Self {
            scenario,
            tracker: tracker.clone(),
            journal: journal.clone(),
            atlas: atlas.clone(),
            funcs: funcs.clone(),
            state: state.clone(),
            signal: signal.clone(),
        };
        spawn(async move {
            signal.cancelled().await;
            let _ = tracker.destroy().await;
            let _ = atlas.destroy().await;
            let _ = funcs.destroy().await;
            state.cancel();
        });
        Ok(instance)
    }

    pub async fn destroy(&self) -> Result<(), E> {
        self.signal.cancel();
        self.state.cancelled().await;
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
    pub async fn execute(&self, name: &str, args: Vec<AnyValue>) -> Result<AnyValue, E> {
        Ok(self.funcs.execute(name, args, self.clone()).await?)
    }
}
