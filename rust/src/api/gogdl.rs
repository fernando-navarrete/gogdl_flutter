use std::sync::Arc;

use flutter_rust_bridge::frb;
pub use gogdl_lib::GogDlError;
use gogdl_lib::{Client, GogDl};

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

    pub async fn get_owned_games(&self) -> Result<Vec<i32>, GogDlError> {
        let owned_games = self.inner.get_owned_games().await?;
        Ok(owned_games.owned)
    }
}
