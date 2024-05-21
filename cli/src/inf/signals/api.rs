use std::fmt;
use tokio::sync::oneshot;
use tokio_util::sync::CancellationToken;

/// Represents API of tast's context. Because each task has own context and
/// multiple tasks could be runned concurrency, communication goes via channels.
pub enum Demand {
    /// Returns signal token. If signal token doesn't exist it will be created
    ///
    /// # Parameters
    ///
    /// * `String` - Name of signal
    /// * `oneshot::Sender<CancellationToken>` - Response channel.
    Get(String, oneshot::Sender<CancellationToken>),
    /// Emit signal. If nobody waits signal, it will be marked as "emitted" for
    /// future calls
    ///
    /// # Parameters
    ///
    /// * `String` - Name of signal
    /// * `oneshot::Sender<bool>` - Response channel. True - if signal had
    /// listeners; false - if no.
    Emit(String, oneshot::Sender<bool>),
    /// Emit shutdown of events loop
    Destroy,
}

impl fmt::Display for Demand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Get(..) => "Get",
                Self::Emit(..) => "Emit",
                Self::Destroy => "Destroy",
            }
        )
    }
}
