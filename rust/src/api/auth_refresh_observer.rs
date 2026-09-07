use flutter_rust_bridge::frb;
use gogdl_lib::{Auth, TokenObserver};
use tokio::sync::mpsc::UnboundedSender;

/// Bridges gogdl-lib's synchronous token-refresh callback onto an async task
/// that can await a Dart function. `on_token_refreshed` only serializes and
/// enqueues, so it never blocks the request that triggered the refresh.
#[frb(ignore)]
pub struct AuthRefreshObserver {
    tx: UnboundedSender<String>,
}

impl AuthRefreshObserver {
    pub fn new(tx: UnboundedSender<String>) -> Self {
        Self { tx }
    }
}

impl TokenObserver for AuthRefreshObserver {
    fn on_token_refreshed(&self, auth: Auth) {
        // `Auth::to_string` produces the same JSON `restore_auth` accepts.
        if let Ok(auth_json) = auth.to_string() {
            // A send error just means the pump task is gone (callback replaced
            // or removed); there is nothing useful to do from here.
            let _ = self.tx.send(auth_json);
        }
    }
}
