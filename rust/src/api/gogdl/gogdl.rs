use flutter_rust_bridge::{frb, DartFnFuture};
use futures::stream::{self, StreamExt};
use gogdl_lib2::{Client, DownloadableFiles, GogDl, GogDlError, VerifyEvent};
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::mpsc;

use crate::frb_generated::StreamSink;

#[frb(opaque)]
pub struct Gog {
    inner: Arc<GogDl>,
}

#[frb(opaque)]
pub struct GameBuild {
    pub build_id: String,
    pub version_name: String,
    pub release_date: String,
}

#[frb(opaque)]
#[derive(Clone)]
pub struct DownloadableProduct {
    pub name: String,
    pub id: String,
    pub product_type: String,
}

#[frb(opaque)]
#[derive(Clone)]
pub enum VerificationStatus {
    FetchingFiles,
    Verifying,
    Verified,
}

#[frb(opaque)]
#[derive(Clone)]
pub struct VerificationStream {
    pub status: VerificationStatus,
    pub total_files: u64,
    pub verified_files: u64,
    pub error_files: Vec<String>,
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
            .buffer_unordered(4)
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

    pub async fn get_downloadable_products(
        &self,
        game_id: i32,
        build_name: &str,
    ) -> Result<Vec<DownloadableProduct>, GogError> {
        let downloadable_products = self
            .inner
            .get_downloadable_products(game_id, build_name)
            .await?;

        let results: Vec<Result<DownloadableProduct, GogError>> =
            stream::iter(downloadable_products)
                .map(|product| {
                    let inner = self.inner.clone();
                    async move {
                        let details = inner.get_product_details(&product.product_id).await?;
                        let product_type = details.get_product_type();
                        Ok::<DownloadableProduct, GogError>(DownloadableProduct {
                            name: details.title,
                            id: product.product_id,
                            product_type: product_type,
                        })
                    }
                })
                .buffer_unordered(4)
                .collect()
                .await;

        let products: Vec<DownloadableProduct> = results
            .iter()
            .filter(|result| result.is_ok())
            .map(|result| result.as_ref().unwrap().clone())
            .collect();

        Ok(products)
    }

    pub async fn get_game_screenshots(&self, game_id: i32) -> Result<Vec<String>, GogError> {
        let game_screenshots = self.inner.get_game_screenshots(game_id).await?;
        let links = match game_screenshots.resolve_links() {
            Ok(links) => links,
            Err(_e) => {
                return Err(GogError::GameError {
                    message: "Could not obtain screenshot links".to_string(),
                })
            }
        };
        Ok(links)
    }

    pub async fn get_game_boxart_link(&self, game_id: i32) -> Result<String, GogError> {
        let game_links = self.inner.get_game_links(game_id).await?;
        let boxart_link = game_links.links.box_art_image.href;
        Ok(boxart_link)
    }

    pub async fn get_background_image_link(&self, game_id: i32) -> Result<String, GogError> {
        let game_links = self.inner.get_game_links(game_id).await?;
        let background_link = game_links.links.galaxy_background_image.href;
        Ok(background_link)
    }
    pub async fn get_game_summary(&self, game_id: i32) -> Result<String, GogError> {
        let game_summary = self.inner.get_game_summary(game_id).await?;
        let summary = game_summary.summary.english;
        Ok(summary)
    }

    pub async fn get_game_builds(&self, game_id: i32) -> Result<Vec<GameBuild>, GogError> {
        let game_builds = self.inner.get_game_builds(game_id).await?;
        let builds: Vec<GameBuild> = game_builds
            .items
            .iter()
            .map(|build| GameBuild {
                build_id: build.build_id.clone(),
                version_name: build.version_name.clone(),
                release_date: build.date_published.to_rfc2822(),
            })
            .collect();
        Ok(builds)
    }

    pub async fn restore_auth_from_string(&self, token: String) -> Result<(), GogError> {
        self.inner.restore_auth(&token).await?;
        Ok(())
    }
    pub async fn verify_download(
        &self,
        path: &str,
        build_name: &str,
        selected_products: &[String],
        game_id: i32,
        stream: StreamSink<VerificationStream>,
    ) -> Result<(), GogError> {
        stream
            .add(VerificationStream {
                status: VerificationStatus::FetchingFiles,
                total_files: 0,
                verified_files: 0,
                error_files: Vec::new(),
            })
            .ok();
        let files = self
            .get_downloadable_files(game_id, build_name, selected_products)
            .await?;

        let mut total_files = 0;
        for file_list in files.iter() {
            total_files += file_list.product_files.len() as u64;
        }

        stream
            .add(VerificationStream {
                status: VerificationStatus::Verifying,
                total_files,
                verified_files: 0,
                error_files: Vec::new(),
            })
            .ok();

        let (tx, mut rx) = mpsc::unbounded_channel();

        self.inner.verify_files(files, path, tx).await?;

        let mut verified_files = 0;
        let mut error_files = Vec::new();
        while let Some(event) = rx.recv().await {
            match event {
                VerifyEvent::FileOk => {
                    verified_files += 1;
                    stream
                        .add(VerificationStream {
                            status: VerificationStatus::Verifying,
                            total_files,
                            verified_files: verified_files,
                            error_files: error_files.clone(),
                        })
                        .ok();
                }
                VerifyEvent::FileChecksumMismatch(file) => {
                    error_files.push(file);
                    stream
                        .add(VerificationStream {
                            status: VerificationStatus::Verifying,
                            total_files,
                            verified_files: verified_files - 1,
                            error_files: error_files.clone(),
                        })
                        .ok();
                }
            }
        }
        Ok(())
    }

    async fn get_downloadable_files(
        &self,
        game_id: i32,
        build_name: &str,
        selected_products: &[String],
    ) -> Result<Vec<DownloadableFiles>, GogError> {
        let products: Vec<&str> = selected_products.iter().map(|p| p.as_str()).collect();
        let files = self
            .inner
            .get_downloadable_files(game_id, build_name, &products)
            .await?;
        Ok(files)
    }
}
