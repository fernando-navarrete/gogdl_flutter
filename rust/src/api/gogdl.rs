use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use flutter_rust_bridge::{frb, DartFnFuture};
use gogdl_lib::{Client, GogDl};
use tokio::sync::mpsc;
use tokio::time::interval;

use crate::api::auth_refresh_observer::AuthRefreshObserver;
pub use crate::api::error::GogError;
pub use crate::api::models::downloadable_product::DownloadableProduct;
pub use crate::api::models::game_build::GameBuild;
pub use crate::api::models::proton_release::ProtonRelease;
use crate::api::progress_reporting::ProtonDownloadProgress;
use crate::frb_generated::StreamSink;

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

    pub async fn get_owned_games(&self) -> Result<Vec<i32>, GogError> {
        let owned_games = self.inner.get_owned_games().await?;
        Ok(owned_games.owned)
    }

    pub async fn restore_auth(&self, json_str: &str) -> Result<(), GogError> {
        self.inner.restore_auth(json_str).await?;
        Ok(())
    }

    pub async fn get_game_title(&self, game_id: i32) -> Result<String, GogError> {
        let game_details = self.inner.get_game_details(game_id).await?;
        Ok(game_details.title)
    }

    pub async fn get_background_image_link(&self, game_id: i32) -> Result<String, GogError> {
        let game_links = self.inner.get_game_links(game_id).await?;
        Ok(game_links.links.background_image.href)
    }

    pub async fn get_game_boxart_link(&self, game_id: i32) -> Result<String, GogError> {
        let game_links = self.inner.get_game_links(game_id).await?;
        Ok(game_links.links.box_art_image.href)
    }

    pub async fn get_game_summary(&self, game_id: i32) -> Result<String, GogError> {
        let game_summary = self.inner.get_game_summary(game_id).await?;
        Ok(game_summary.summary.english)
    }
    pub async fn get_game_screenshots(&self, game_id: i32) -> Result<Vec<String>, GogError> {
        let game_screenshots = self.inner.get_game_screenshots(game_id).await?;
        let links = match game_screenshots.resolve_links() {
            Ok(links) => links,
            Err(e) => return Err(GogError::GogdlError(gogdl_lib::GogDlError::GameError(e))),
        };
        Ok(links)
    }
    pub async fn get_game_builds(&self, game_id: i32) -> Result<Vec<GameBuild>, GogError> {
        let game_builds = self.inner.get_game_builds(game_id).await?;
        let builds = game_builds
            .items
            .iter()
            .map(|build| GameBuild::from_gogdl(build))
            .collect::<Vec<GameBuild>>();
        Ok(builds)
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

        let mut products = Vec::new();
        for product in &downloadable_products {
            let product_details = self.inner.get_product_details(&product.product_id).await?;
            products.push(DownloadableProduct::from_gogdl(&product_details));
        }

        Ok(products)
    }
    pub async fn get_proton_releases(&self, page: i32) -> Result<Vec<ProtonRelease>, GogError> {
        let proton_releases = self.inner.get_proton_releases(page as u32, 5).await?;
        let releases = proton_releases
            .releases()
            .iter()
            .filter(|release| release.get_release_size().is_ok())
            // Unwrap is safe here because we filtered out releases with an invalid size above
            .map(|release| ProtonRelease::from_gogdl(release).unwrap())
            .collect::<Vec<ProtonRelease>>();
        Ok(releases)
    }
    pub async fn download_proton_release(
        &self,
        path: &str,
        tag_name: &str,
        sink: StreamSink<ProtonDownloadProgress>,
    ) -> Result<(), GogError> {
        let release = self.inner.get_proton_release_by_tag(tag_name).await?;

        let (tx, mut rx) = mpsc::unbounded_channel();
        let download_future = self
            .inner
            .download_proton_release(&release, Path::new(path), tx);

        let progress_reporting_future = async move {
            let mut downloaded: u64 = 0;
            let mut last_extracted: Option<String> = None;
            let mut dirty = false;

            let mut ticker = interval(Duration::from_millis(150));
            ticker.tick().await; // first tick fires immediately, discard it

            loop {
                tokio::select! {
                    event = rx.recv() => {
                        match event {
                            Some(gogdl_lib::ProtonDownloadEvent::Downloading { total_bytes }) => {
                                // one-shot signal, flush immediately regardless of ticker
                                let _ = sink.add(ProtonDownloadProgress::Started(total_bytes));
                            }
                            Some(gogdl_lib::ProtonDownloadEvent::Progress(delta)) => {
                                downloaded += delta as u64;
                                dirty = true;
                            }
                            Some(gogdl_lib::ProtonDownloadEvent::Extracted(path)) => {
                                last_extracted = Some(path);
                                dirty = true;
                            }
                            None => break, // sender dropped, download_future is done
                        }
                    }
                    _ = ticker.tick() => {
                        if dirty {
                            let _ = sink.add(ProtonDownloadProgress::Progress(downloaded));
                            if let Some(p) = last_extracted.take() {
                                let _ = sink.add(ProtonDownloadProgress::Extracted(p));
                            }
                            dirty = false;
                        }
                    }
                }
            }

            // final flush so the UI doesn't sit at a stale % between the last tick
            // and the channel closing
            if dirty {
                let _ = sink.add(ProtonDownloadProgress::Progress(downloaded));
                if let Some(p) = last_extracted.take() {
                    let _ = sink.add(ProtonDownloadProgress::Extracted(p));
                }
            }
        };

        let (result, _) = tokio::join!(download_future, progress_reporting_future);
        result?;
        Ok(())
    }
    pub async fn login_with_code(&self, code: &str) -> Result<(), GogError> {
        self.inner.login_with_code(code).await?;
        Ok(())
    }
    pub async fn set_token_refresh_callback(
        &self,
        callback: impl Fn(String) -> DartFnFuture<()> + Send + Sync + 'static,
    ) {
        let (tx, mut rx) = mpsc::unbounded_channel::<String>();
        self.inner
            .set_token_observer(Arc::new(AuthRefreshObserver::new(tx)))
            .await;

        // Runs on flutter_rust_bridge's tokio runtime; ends when the observer
        // is replaced or removed and the sender drops, which also releases
        // the Dart handle held by `callback`.
        flutter_rust_bridge::spawn(async move {
            while let Some(auth_json) = rx.recv().await {
                callback(auth_json).await;
            }
        });
    }

    pub async fn remove_token_refresh_callback(&self) {
        self.inner.remove_token_observer().await;
    }
}
