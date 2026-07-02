use flutter_rust_bridge::{frb, DartFnFuture};
use futures::stream::{self, StreamExt};
use gogdl_lib2::{Client, GogDl, GogDlError};
use std::sync::Arc;
use thiserror::Error;

#[frb(opaque)]
pub struct Gog {
    inner: Arc<GogDl>,
}

#[derive(Error, Debug)]
pub enum GogError {
    #[error("AuthError: {message}")]
    AuthError { message: String },
    #[error("GameError: {message}")]
    GameError { message: String },
    #[error("DepotError: {message}")]
    DepotError { message: String },
    #[error("SecureLinksError: {message}")]
    SecureLinksError { message: String },
    #[error("DownloadError: {message}")]
    DownloadError { message: String },
}

impl From<GogDlError> for GogError {
    fn from(value: GogDlError) -> Self {
        match value {
            GogDlError::AuthError(auth_error) => GogError::AuthError {
                message: auth_error.to_string(),
            },
            GogDlError::GameError(games_error) => GogError::GameError {
                message: games_error.to_string(),
            },
            GogDlError::DepotError(depot_error) => GogError::DepotError {
                message: depot_error.to_string(),
            },
            GogDlError::SecureLinksError(secure_links_error) => GogError::SecureLinksError {
                message: secure_links_error.to_string(),
            },
            GogDlError::DownloadError(download_error) => GogError::DownloadError {
                message: download_error.to_string(),
            },
        }
    }
}

impl Gog {
    #[frb(sync)]
    pub fn new() -> Self {
        let client = Client::new();
        let gogdl = GogDl::new_from_client(client);
        Gog {
            inner: Arc::new(gogdl),
        }
    }

    #[frb(sync)]
    pub fn get_login_url(&self) -> String {
        self.inner.get_login_url().to_string()
    }

    pub async fn login_with_code(&self, code: String) -> Result<String, GogError> {
        let token = self.inner.login_with_code(&code).await?;
        Ok(token)
    }

    pub async fn refresh_auth_with_callback(
        &self,
        callback: impl Fn(String) -> DartFnFuture<()>,
    ) -> Result<(), GogError> {
        let token = self.inner.refresh_auth().await?;
        callback(token).await;
        Ok(())
    }

    pub async fn get_owned_games(&self) -> Result<Vec<i32>, GogError> {
        let games = self.inner.get_owned_games().await?;
        let results: Vec<Result<(i32, String), GogError>> = stream::iter(games.owned)
            .map(|id| {
                let inner = self.inner.clone();
                async move {
                    let details = inner.get_game_details(id).await?;
                    Ok::<(i32, String), GogError>((id, details.title))
                }
            })
            .buffer_unordered(8)
            .collect()
            .await;

        let id_list: Vec<i32> = results
            .iter()
            .filter(|result| result.is_ok())
            // Unwrap is safe here because we know the result is Ok
            .map(|result| result.as_ref().unwrap().0)
            .collect();
        Ok(id_list)
    }

    pub async fn get_game_title(&self, game_id: i32) -> Result<String, GogError> {
        let game_details = self.inner.get_game_details(game_id).await?;
        let title = game_details.title;
        Ok(title)
    }

    pub async fn restore_auth_from_string(&self, token: String) -> Result<(), GogError> {
        self.inner.restore_auth(&token).await?;
        Ok(())
    }
}
