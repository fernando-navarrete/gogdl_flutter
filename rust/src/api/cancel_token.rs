use flutter_rust_bridge::frb;
use tokio_util::sync::CancellationToken;

/// Stops a job. Pass one to a job method and call `cancel()` to stop it: the job's stream then
/// ends with its `cancelled` event instead of `finished`. Use one token per job. Cancelling twice,
/// or after the job ended, does nothing.
#[frb(opaque)]
pub struct CancelToken(pub(crate) CancellationToken);

impl CancelToken {
    #[frb(sync)]
    pub fn new() -> Self {
        Self(CancellationToken::new())
    }

    #[frb(sync)]
    pub fn cancel(&self) {
        self.0.cancel();
    }
}
