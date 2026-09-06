use std::sync::Arc;

use flutter_rust_bridge::frb;
use gogdl_lib::{Client, GogDl, GogDlError, OwnedGames};

#[frb(opaque)]
pub struct GogdlApi {
    inner: Arc<GogDl>,
}

impl GogdlApi {
    #[frb(sync)]
    pub fn new() -> Self {
        let client = Client::new();
        Self {
            inner: Arc::new(GogDl::new_from_client(client)),
        }
    }
    #[frb(sync)]
    pub fn get_login_link(&self) -> String {
        self.inner.get_login_url().to_string()
    }

    pub async fn get_owned_games(&self) -> Result<OwnedGames, GogDlError> {
        self.inner.get_owned_games().await
    }
}
