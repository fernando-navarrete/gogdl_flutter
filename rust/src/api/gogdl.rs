use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use flutter_rust_bridge::{DartFnFuture, frb};
use gogdl_lib::GogDl;
use tokio::sync::mpsc;

use crate::api::auth_refresh_observer::AuthRefreshObserver;
pub use crate::api::cancel_token::CancelToken;
pub use crate::api::error::GogError;
pub use crate::api::models::downloadable_product::DownloadableProduct;
pub use crate::api::models::game_build::GameBuild;
pub use crate::api::models::install_size::{InstallSize, ProductSize};
pub use crate::api::models::product_removal::ProductRemoval;
pub use crate::api::models::proton_release::ProtonRelease;
use crate::api::progress_reporting::{
    DownloadGameProgress, DownloadSavesProgress, ProtonDownloadProgress, RepairGameProgress,
    UploadSavesProgress, VerifyDownloadProgress,
};
use crate::client::{self, CONNECT_TIMEOUT, READ_TIMEOUT};
use crate::frb_generated::StreamSink;
use crate::progress::{
    self, Outcome,
    download::DownloadTranslator,
    proton::ProtonTranslator,
    repair::RepairTranslator,
    saves::{SavesDownloadTranslator, SavesUploadTranslator},
    verify::VerifyTranslator,
};

#[frb(opaque)]
pub struct GogdlApi {
    inner: Arc<GogDl>,
}

impl GogdlApi {
    /// Connections time out after 10s, and a request fails after 30s without reading anything.
    /// Metadata requests and game chunk downloads retry a timeout (six attempts, with backoff).
    /// Proton downloads, saves downloads and uploads, and `loginWithCode` don't: a timeout fails
    /// them with a network error.
    #[frb(sync)]
    pub fn new() -> Self {
        let client = client::build(CONNECT_TIMEOUT, READ_TIMEOUT);
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

    /// Drops the cached data that can go stale, so the next call to each getter fetches it again:
    /// the owned games (`getOwnedGames`), game builds (`getGameBuilds`), downloadable products
    /// (`getDownloadableProducts`), depot manifests and cloud saves remote configs.
    ///
    /// Details, links, summary, screenshots, product details and build metadata stay cached.
    /// Login, the token refresh callback and running downloads, repairs and checks are untouched.
    /// A call already in flight may store its own result afterwards, which is current as of its
    /// fetch.
    ///
    /// A library refresh and the Builds tab's Refresh should call it before invalidating their
    /// state, or they re-read the same data.
    pub async fn clear_cache(&self) {
        self.inner.clear_cache().await;
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
            .map(GameBuild::from_gogdl)
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
    /// The download and on-disk size of `selectedProducts` of `buildName`, in total and per
    /// product. It's one request for the build's metadata, with no depot manifests, and it equals
    /// the totals `downloadGame` and `verifyDownload` report. Ids in `selectedProducts` that aren't
    /// in the build are ignored.
    ///
    /// `diskBytes` is what the download's own check compares against `getFreeSpace`. A game
    /// that's already partly on disk (resumed with `repairGame`) needs less than that, so a check
    /// against it is a warning, not a guarantee.
    pub async fn get_install_size(
        &self,
        game_id: i32,
        build_name: &str,
        selected_products: &[i32],
    ) -> Result<InstallSize, GogError> {
        let sizes = self.inner.get_product_sizes(game_id, build_name).await?;
        Ok(InstallSize::from_gogdl(&sizes, selected_products))
    }
    /// Deletes a product's files from the install at `path`, for a DLC the player unselected.
    /// It fetches the depot manifests of `keep` and `remove` for `buildName` of `gameId` (the same
    /// ones `downloadGame` resolves) and deletes only the files that `remove`'s depots list and
    /// `keep`'s don't. Files are compared case-insensitively on normalized paths, since DLCs share
    /// files with the base game: pass every product that stays installed in `keep`, the base game
    /// included.
    ///
    /// - An id in both lists is kept.
    /// - An empty `remove` returns zeros without fetching anything.
    /// - Only files the manifests list are touched. Anything else under `path` survives, a
    ///   directory that leads outside `path` is skipped, and a file that is already gone isn't an
    ///   error.
    /// - Directories the removal emptied are pruned, never `path` itself.
    ///
    /// It needs a login and the network, since it fetches manifests. Don't call it while a
    /// download, repair or check runs on `path`.
    ///
    /// Fails with `notFound` when the build or a product doesn't exist, `sessionExpired` and
    /// `network` like any lookup, and `fileSystem` when a delete fails. After a failure, or if the
    /// caller stops waiting, some files may be removed: call it again to finish. The result counts
    /// only what that call removed.
    pub async fn remove_products(
        &self,
        game_id: i32,
        path: &str,
        build_name: &str,
        keep: &[i32],
        remove: &[i32],
    ) -> Result<ProductRemoval, GogError> {
        let removal = self
            .inner
            .remove_products(game_id, path, build_name, keep, remove)
            .await?;
        Ok(ProductRemoval::from_gogdl(&removal))
    }
    /// The free space, in bytes, on the disk `path` is on: the same figure the download, repair
    /// and Proton checks compare against. `path` needn't exist, and isn't created: its nearest
    /// existing ancestor stands in for it. Local only, with no network or login.
    ///
    /// Fails with `fileSystem` when no ancestor of `path` can be resolved or no disk matches it.
    pub async fn get_free_space(&self, path: &str) -> Result<u64, GogError> {
        let free_space = self.inner.get_free_space(Path::new(path)).await?;
        Ok(free_space)
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
    /// A 30s stall fails the download with a network error. It isn't retried.
    ///
    /// Cancelling `cancel` leaves `path` as it was, and the stream ends with `cancelled`.
    pub async fn download_proton_release(
        &self,
        path: &str,
        tag_name: &str,
        cancel: &CancelToken,
        sink: StreamSink<ProtonDownloadProgress>,
    ) -> Result<(), GogError> {
        let cancel = &cancel.0;
        let Some(release) = cancel
            .run_until_cancelled(self.inner.get_proton_release_by_tag(tag_name))
            .await
        else {
            let _ = sink.add(ProtonDownloadProgress::Cancelled);
            return Ok(());
        };
        let release = release?;

        let (tx, rx) = mpsc::unbounded_channel();
        let download_future = self
            .inner
            .download_proton_release(&release, Path::new(path), tx);

        let outcome = progress::run(download_future, ProtonTranslator::new(), rx, cancel, |p| {
            sink.add(p).is_ok()
        })
        .await;
        match outcome {
            Outcome::Finished(result) => {
                let extracted_path = result?;
                let _ = sink.add(ProtonDownloadProgress::Finished(
                    extracted_path.to_string_lossy().into_owned(),
                ));
            }
            Outcome::Cancelled => {
                let _ = sink.add(ProtonDownloadProgress::Cancelled);
            }
            Outcome::ListenerGone => {} // nobody to report to
        }
        Ok(())
    }
    pub async fn login_with_code(&self, code: &str) -> Result<String, GogError> {
        let auth = self.inner.login_with_code(code).await?;
        Ok(auth)
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

    /// Cancelling `cancel` stops the check, and the stream ends with `cancelled`.
    pub async fn verify_download(
        &self,
        game_id: i32,
        path: &str,
        build_name: &str,
        selected_products: &[i32],
        cancel: &CancelToken,
        sink: StreamSink<VerifyDownloadProgress>,
    ) -> Result<(), GogError> {
        let cancel = &cancel.0;
        let Some(bundles) = cancel
            .run_until_cancelled(self.inner.get_product_bundles(
                game_id,
                build_name,
                selected_products,
            ))
            .await
        else {
            let _ = sink.add(VerifyDownloadProgress::Cancelled);
            return Ok(());
        };
        let bundles = bundles?;

        let total_bytes: u64 = bundles
            .iter()
            .flat_map(|bundle| bundle.product_files.iter())
            .filter_map(|file| file.size())
            .sum();

        if sink
            .add(VerifyDownloadProgress::Started(total_bytes))
            .is_err()
        {
            return Ok(()); // listener gone before the job started
        }

        let (tx, rx) = mpsc::unbounded_channel();
        let verify_future = self.inner.verify_files(bundles, path, tx);

        let outcome = progress::run(verify_future, VerifyTranslator::new(), rx, cancel, |p| {
            sink.add(p).is_ok()
        })
        .await;
        let result = match outcome {
            Outcome::Finished(result) => result,
            Outcome::Cancelled => {
                let _ = sink.add(VerifyDownloadProgress::Cancelled);
                return Ok(());
            }
            Outcome::ListenerGone => return Ok(()), // nobody to report to
        };

        match result {
            Ok(()) => {
                let _ = sink.add(VerifyDownloadProgress::Finished(0));
            }
            Err(gogdl_lib::GogDlError::DownloadError(
                gogdl_lib::DownloadError::ChunkIntegrityCheckFailed(failed),
            )) => {
                let _ = sink.add(VerifyDownloadProgress::Finished(failed as u64));
            }
            Err(e) => return Err(e.into()),
        }
        Ok(())
    }
    /// Downloads `selectedProducts` of `buildName` into `path`.
    ///
    /// Chunks already on disk that verify are skipped, so running it again on a partial install
    /// resumes it, and on a complete one transfers nothing. The skipped bytes count as
    /// downloaded, so `downloadProgress` still ends at `started.totalBytes`. `repairGame` runs the
    /// same pipeline and is the explicit resume path.
    ///
    /// Cancelling `cancel` keeps the files written so far, and the stream ends with `cancelled`.
    /// Call `downloadGame` or `repairGame` again to resume the install.
    pub async fn download_game(
        &self,
        game_id: i32,
        path: &str,
        build_name: &str,
        selected_products: &[i32],
        cancel: &CancelToken,
        sink: StreamSink<DownloadGameProgress>,
    ) -> Result<(), GogError> {
        let cancel = &cancel.0;
        let Some(bundles) = cancel
            .run_until_cancelled(self.inner.get_product_bundles(
                game_id,
                build_name,
                selected_products,
            ))
            .await
        else {
            let _ = sink.add(DownloadGameProgress::Cancelled);
            return Ok(());
        };
        let bundles = bundles?;

        let total_files: u64 = bundles
            .iter()
            .map(|bundle| bundle.product_files.len() as u64)
            .sum();
        // The download stage reports compressed bytes off the wire (see
        // `DownloadEvent::Progress`'s docs), so the denominator has to match
        // or the percentage would never reach 100%.
        let total_bytes: u64 = bundles
            .iter()
            .flat_map(|bundle| bundle.product_files.iter())
            .filter_map(|file| file.chunks.as_ref())
            .flat_map(|chunks| chunks.iter())
            .map(|chunk| chunk.compressed_size)
            .sum();

        if sink
            .add(DownloadGameProgress::Started {
                total_files,
                total_bytes,
            })
            .is_err()
        {
            return Ok(()); // listener gone before the job started
        }

        let (tx, rx) = mpsc::unbounded_channel();
        let download_future = self.inner.download_game(bundles, path, tx);

        let outcome = progress::run(
            download_future,
            DownloadTranslator::new(total_bytes),
            rx,
            cancel,
            |p| sink.add(p).is_ok(),
        )
        .await;
        match outcome {
            Outcome::Finished(result) => {
                result?;
                let _ = sink.add(DownloadGameProgress::Finished);
            }
            Outcome::Cancelled => {
                let _ = sink.add(DownloadGameProgress::Cancelled);
            }
            Outcome::ListenerGone => {} // nobody to report to
        }
        Ok(())
    }
    /// Checks `selectedProducts` of `buildName` under `path` and downloads only the chunks that are
    /// missing or corrupt.
    ///
    /// This is the resume path: call it on an install that `downloadGame` (or an earlier
    /// `repairGame`) didn't finish, whether it was cancelled, failed or interrupted by an app exit.
    /// Chunks it has just allocated aren't read back, so a resume only reads what was already on
    /// disk.
    ///
    /// Cancelling `cancel` keeps the files written so far, and the stream ends with `cancelled`.
    /// Call `repairGame` again to resume the install.
    pub async fn repair_game(
        &self,
        game_id: i32,
        path: &str,
        build_name: &str,
        selected_products: &[i32],
        cancel: &CancelToken,
        sink: StreamSink<RepairGameProgress>,
    ) -> Result<(), GogError> {
        let cancel = &cancel.0;
        let Some(bundles) = cancel
            .run_until_cancelled(self.inner.get_product_bundles(
                game_id,
                build_name,
                selected_products,
            ))
            .await
        else {
            let _ = sink.add(RepairGameProgress::Cancelled);
            return Ok(());
        };
        let bundles = bundles?;

        let total_files: u64 = bundles
            .iter()
            .map(|bundle| bundle.product_files.len() as u64)
            .sum();
        // The verification stage reports uncompressed bytes, so the
        // denominator has to match (see `verify_download`'s equivalent).
        let total_bytes: u64 = bundles
            .iter()
            .flat_map(|bundle| bundle.product_files.iter())
            .filter_map(|file| file.size())
            .sum();

        // Only the chunks that fail verification get re-downloaded, and the
        // download stage only reports compressed bytes off the wire — this
        // is how each failing (path, uncompressed size) is resolved back to
        // the compressed size to sum for `DownloadStarted`'s total.
        let mut compressed_sizes: HashMap<(String, u64), Vec<u64>> = HashMap::new();
        for file in bundles
            .iter()
            .flat_map(|bundle| bundle.product_files.iter())
        {
            for chunk in file.chunks.iter().flatten() {
                compressed_sizes
                    .entry((file.path.clone(), chunk.size))
                    .or_default()
                    .push(chunk.compressed_size);
            }
        }

        if sink
            .add(RepairGameProgress::Started {
                total_files,
                total_bytes,
            })
            .is_err()
        {
            return Ok(()); // listener gone before the job started
        }

        let (tx, rx) = mpsc::unbounded_channel();
        let repair_future = self.inner.repair_game(bundles, path, tx);

        let translator = RepairTranslator::new(compressed_sizes);
        let outcome = progress::run(repair_future, translator, rx, cancel, |p| {
            sink.add(p).is_ok()
        })
        .await;
        match outcome {
            Outcome::Finished(result) => {
                result?;
                let _ = sink.add(RepairGameProgress::Finished);
            }
            Outcome::Cancelled => {
                let _ = sink.add(RepairGameProgress::Cancelled);
            }
            Outcome::ListenerGone => {} // nobody to report to
        }
        Ok(())
    }
    /// Cancelling `cancel` leaves each local save either the old one or the complete new one, and
    /// the stream ends with `cancelled`.
    pub async fn download_save_files(
        &self,
        game_id: i32,
        build_name: &str,
        prefix: &str,
        install_path: &str,
        cancel: &CancelToken,
        sink: StreamSink<DownloadSavesProgress>,
    ) -> Result<(), GogError> {
        let cancel = &cancel.0;
        let (tx, rx) = mpsc::unbounded_channel();
        let download_future = self.inner.download_save_files(
            game_id,
            build_name,
            Path::new(prefix),
            Path::new(install_path),
            tx,
        );

        let outcome = progress::run(
            download_future,
            SavesDownloadTranslator::new(),
            rx,
            cancel,
            |p| sink.add(p).is_ok(),
        )
        .await;
        match outcome {
            Outcome::Finished(result) => {
                result?;
                let _ = sink.add(DownloadSavesProgress::Finished);
            }
            Outcome::Cancelled => {
                let _ = sink.add(DownloadSavesProgress::Cancelled);
            }
            Outcome::ListenerGone => {} // nobody to report to
        }
        Ok(())
    }
    /// Each file's upload must finish within 30s, since the read timeout covers sending the
    /// request body. A slower one fails with a network error and isn't retried.
    ///
    /// Cancelling `cancel` leaves local files untouched, and the stream ends with `cancelled`. The
    /// file in flight may or may not have been stored, so upload again to be sure.
    pub async fn upload_save_files(
        &self,
        game_id: i32,
        build_name: &str,
        prefix: &str,
        install_path: &str,
        cancel: &CancelToken,
        sink: StreamSink<UploadSavesProgress>,
    ) -> Result<(), GogError> {
        let cancel = &cancel.0;
        let (tx, rx) = mpsc::unbounded_channel();
        let upload_future = self.inner.upload_save_files(
            game_id,
            build_name,
            Path::new(prefix),
            Path::new(install_path),
            tx,
        );

        let outcome = progress::run(
            upload_future,
            SavesUploadTranslator::new(),
            rx,
            cancel,
            |p| sink.add(p).is_ok(),
        )
        .await;
        match outcome {
            Outcome::Finished(result) => {
                result?;
                let _ = sink.add(UploadSavesProgress::Finished);
            }
            Outcome::Cancelled => {
                let _ = sink.add(UploadSavesProgress::Cancelled);
            }
            Outcome::ListenerGone => {} // nobody to report to
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn free_space_of_the_temp_dir_is_positive() {
        let dir = std::env::temp_dir();
        let free_space = GogdlApi::new()
            .get_free_space(dir.to_str().unwrap())
            .await
            .unwrap();
        assert!(free_space > 0);
    }

    // The API has no login here, so any manifest lookup would fail: `Ok` proves nothing was fetched.
    #[tokio::test]
    async fn removing_nothing_fetches_nothing() {
        let removal = GogdlApi::new()
            .remove_products(1, "/nonexistent", "build", &[1], &[])
            .await
            .unwrap();
        assert_eq!((removal.files_removed, removal.bytes_freed), (0, 0));
    }

    #[tokio::test]
    async fn an_id_in_both_lists_is_kept_and_fetches_nothing() {
        let removal = GogdlApi::new()
            .remove_products(1, "/nonexistent", "build", &[1, 2], &[2])
            .await
            .unwrap();
        assert_eq!((removal.files_removed, removal.bytes_freed), (0, 0));
    }

    #[tokio::test]
    async fn a_missing_path_uses_its_nearest_existing_ancestor() {
        let dir = std::env::temp_dir();
        let root = dir.join(format!("gogdl-missing-{}", std::process::id()));
        let missing = root.join("a").join("b");
        let api = GogdlApi::new();

        let for_dir = api.get_free_space(dir.to_str().unwrap()).await.unwrap();
        let for_missing = api.get_free_space(missing.to_str().unwrap()).await.unwrap();

        // Other processes can write to the temp dir between the two calls.
        assert!(for_dir.abs_diff(for_missing) < 64 * 1024 * 1024);
        assert!(!root.exists());
    }
}
