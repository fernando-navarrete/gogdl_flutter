use std::sync::Arc;

use flutter_rust_bridge::frb;
pub use gogdl_lib::GogDlError;
use gogdl_lib::{Client, GogDl};

pub use crate::api::models::game_build::GameBuild;

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

    pub async fn get_game_summary(&self, game_id: i32) -> Result<String, GogDlError> {
        let game_summary = self.inner.get_game_summary(game_id).await?;
        Ok(game_summary.summary.english)
    }
    pub async fn get_game_screenshots(&self, game_id: i32) -> Result<Vec<String>, GogDlError> {
        let game_screenshots = self.inner.get_game_screenshots(game_id).await?;
        let links = game_screenshots.resolve_links()?;
        Ok(links)
    }
    pub async fn get_game_builds(&self, game_id: i32) -> Result<Vec<GameBuild>, GogDlError> {
        let game_builds = self.inner.get_game_builds(game_id).await?;
        let builds = game_builds
            .items
            .iter()
            .map(|build| GameBuild::from_gogdl(build))
            .collect::<Vec<GameBuild>>();
        Ok(builds)
    }
}
