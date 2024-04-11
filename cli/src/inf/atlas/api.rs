use std::fmt;

use crate::{
    error::{LinkedErr, LinkedErrSerialized},
    inf::atlas::E,
};
use tokio::sync::{mpsc::UnboundedSender, oneshot};

/// Represents API of tast's context. Because each task has own context and
/// multiple tasks could be runned concurrency, communication goes via channels.
pub enum Demand {
    /// Add footprint into trace
    ///
    /// # Parameters
    ///
    /// * `usize` - Token
    /// * `Option<String>` - String representation of value; None if value isn't set
    /// * `oneshot::Sender<()>` - Response channel
    AddFootprint(usize, Option<String>, oneshot::Sender<Result<(), E>>),
    /// Add error report into trace
    ///
    /// # Parameters
    ///
    /// * `usize` - Token
    /// * `LinkedErrSerialized` - Related error
    /// * `oneshot::Sender<()>` - Response channel
    ReportErr(usize, LinkedErrSerialized, oneshot::Sender<Result<(), E>>),
    /// Set position of cursor in current map
    ///
    /// # Parameters
    ///
    /// * `usize` - Token
    /// * `oneshot::Sender<()>` - Response channel
    SetMapPosition(usize, oneshot::Sender<Result<(), E>>),
}

/// Represents API of tast's context.
#[derive(Clone, Debug)]
pub struct Coupling {
    tx: UnboundedSender<Demand>,
}

impl Coupling {
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
    pub async fn add_footprint(&self, token: usize, value_as_str: Option<String>) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx
            .send(Demand::AddFootprint(token, value_as_str, tx))?;
        rx.await?;
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
    pub async fn report_err<T: fmt::Display>(
        &self,
        token: usize,
        err: LinkedErr<T>,
    ) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx
            .send(Demand::ReportErr(token, err.serialize(), tx))?;
        rx.await?;
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
        rx.await?;
        Ok(())
    }
}
