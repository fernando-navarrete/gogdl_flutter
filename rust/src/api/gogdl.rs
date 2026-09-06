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

    pub async fn restore_auth(&self, json_str: &str) -> Result<(), GogDlError> {
        self.inner.restore_auth(json_str).await
    }

    pub async fn get_game_title(&self, game_id: i32) -> Result<String, GogDlError> {
        let game_details = self.inner.get_game_details(game_id).await?;
        Ok(game_details.title)
    }

    pub async fn get_background_image_link(&self, game_id: i32) -> Result<String, GogDlError> {
        let game_links = self.inner.get_game_links(game_id).await?;
        Ok(game_links.links.background_image.href)
    }

    pub async fn get_game_boxart_link(&self, game_id: i32) -> Result<String, GogDlError> {
        let game_links = self.inner.get_game_links(game_id).await?;
        Ok(game_links.links.box_art_image.href)
    }
}
