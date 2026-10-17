use std::collections::HashMap;
use std::future::Future;
use std::path::Path;
use std::sync::Arc;

use flutter_rust_bridge::{DartFnFuture, frb};
use futures_util::{StreamExt, stream};
use gogdl_lib::GogDl;
use tokio::sync::mpsc;

use crate::api::auth_refresh_observer::AuthRefreshObserver;
pub use crate::api::cancel_token::CancelToken;
pub use crate::api::error::GogError;
pub use crate::api::models::downloadable_product::{DownloadableProduct, ProductType};
pub use crate::api::models::game_build::GameBuild;
pub use crate::api::models::game_links::GameLinks;
pub use crate::api::models::install_size::{InstallSize, ProductSize};
pub use crate::api::models::product_removal::ProductRemoval;
pub use crate::api::models::proton_release::{ProtonRelease, ProtonReleasePage};
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

const PROTON_PAGE_SIZE: u32 = 5;

fn proton_page(page: i32) -> Result<u32, GogError> {
    u32::try_from(page)
        .map_err(|_| GogError::InvalidArgument(format!("page must not be negative, got {page}")))
}

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

    /// The box art and background image links of a game, from one request.
    pub async fn get_game_links(&self, game_id: i32) -> Result<GameLinks, GogError> {
        let game_links = self.inner.get_game_links(game_id).await?;
        Ok(GameLinks::from_gogdl(&game_links))
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
    /// The products of `buildName` that the account owns (the base game and its DLCs), in
    /// upstream's order, with their names and types. Their details are fetched 8 at a time.
    ///
    /// A product whose lookup fails is left out, so one bad DLC doesn't hide the rest. If the base
    /// game's own lookup fails the call fails, since the list is meaningless without it: with
    /// `notFound`, `network` or `sessionExpired` like any lookup.
    pub async fn get_downloadable_products(
        &self,
        game_id: i32,
        build_name: &str,
    ) -> Result<Vec<DownloadableProduct>, GogError> {
        let downloadable_products = self
            .inner
            .get_downloadable_products(game_id, build_name)
            .await?;
        let product_ids = downloadable_products
            .into_iter()
            .map(|product| product.product_id)
            .collect();

        let inner = &self.inner;
        resolve_products(game_id, product_ids, |id| async move {
            inner.get_product_details(&id).await
        })
        .await
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
    /// One page of Proton-GE releases, 5 to a page, newest first. Pages start at 1 (GitHub treats
    /// 0 as 1). Releases with no suitable asset are left out of `releases`, so `hasMore` says
    /// whether to ask for the next page: it's true when GitHub returned a full page, even if
    /// `releases` is empty.
    ///
    /// A negative `page` fails with `unknown` instead of wrapping around.
    pub async fn get_proton_release_page(&self, page: i32) -> Result<ProtonReleasePage, GogError> {
        let page = proton_page(page)?;
        let upstream = self
            .inner
            .get_proton_releases(page, PROTON_PAGE_SIZE)
            .await?;
        Ok(ProtonReleasePage::from_gogdl(&upstream, PROTON_PAGE_SIZE))
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
                let _ = sink.add(ProtonDownloadProgress::Finished {
                    path: extracted_path.to_string_lossy().into_owned(),
                });
            }
            Outcome::Cancelled => {
                let _ = sink.add(ProtonDownloadProgress::Cancelled);
            }
            Outcome::ListenerGone => {} // nobody to report to
        }
        Ok(())
    }
    /// Exchanges the code from the login redirect for a session, and returns the string to persist
    /// and give back to `restoreAuth`.
    ///
    /// Fails with `invalidLoginCode` when GOG rejects the code, and `network` when it can't be
    /// reached.
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
            .add(VerifyDownloadProgress::Started { total_bytes })
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
                let _ = sink.add(VerifyDownloadProgress::Finished { bad_chunks: 0 });
            }
            Err(gogdl_lib::GogDlError::DownloadError(
                gogdl_lib::DownloadError::ChunkIntegrityCheckFailed(failed),
            )) => {
                let _ = sink.add(VerifyDownloadProgress::Finished {
                    bad_chunks: failed as u64,
                });
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
    // This is `gogdl-lib`'s `download_game` (which verifies and resumes) with repair's progress
    // shape: verification first, then only the failed chunks. It is kept for Lumen's resume path,
    // and whether it stays past `v1.6.0` is open. (A plain comment, so MR 1 changes no Dart docs.)
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
        let repair_future = self.inner.download_game(bundles, path, tx);

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

/// Product detail lookups in flight at once, like Lumen's limiter.
const PRODUCT_DETAILS_CONCURRENCY: usize = 8;

/// Looks up `product_ids` with `fetch`, keeping their order. A failed lookup drops its product,
/// except the base game's (the id equal to `game_id`), whose error is returned.
async fn resolve_products<F, Fut>(
    game_id: i32,
    product_ids: Vec<String>,
    fetch: F,
) -> Result<Vec<DownloadableProduct>, GogError>
where
    F: Fn(String) -> Fut,
    Fut: Future<Output = Result<gogdl_lib::ProductDetails, gogdl_lib::GogDlError>>,
{
    let base_game_id = game_id.to_string();
    let lookups = stream::iter(product_ids)
        .map(|id| {
            let lookup = fetch(id.clone());
            async move { (id, lookup.await) }
        })
        .buffered(PRODUCT_DETAILS_CONCURRENCY)
        .collect::<Vec<_>>()
        .await;

    let mut products = Vec::new();
    for (id, lookup) in lookups {
        match lookup {
            Ok(details) => products.push(DownloadableProduct::from_gogdl(&details)),
            Err(e) if id == base_game_id => return Err(e.into()),
            Err(_) => {}
        }
    }
    Ok(products)
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::Duration;

    use gogdl_lib::{ClientError, DepotError, GogDlError, ProductDetails};
    use reqwest::StatusCode;

    use super::*;

    #[test]
    fn a_negative_proton_page_is_rejected() {
        assert!(matches!(
            proton_page(-1),
            Err(GogError::InvalidArgument(m)) if m.contains("-1")
        ));
        assert_eq!(proton_page(0).unwrap(), 0);
        assert_eq!(proton_page(3).unwrap(), 3);
    }

    fn details(id: i32) -> ProductDetails {
        let mut details: ProductDetails = serde_json::from_str(&format!(
            r#"{{"_embedded": {{"productType": "DLC", "product": {{"id": {id}, "title": "P{id}"}}}}}}"#
        ))
        .unwrap();
        details.title = format!("P{id}");
        details
    }

    fn not_found() -> GogDlError {
        GogDlError::DepotError(DepotError::ClientError(ClientError::HttpError {
            status: StatusCode::NOT_FOUND,
            body: String::new(),
        }))
    }

    fn ids(range: std::ops::Range<i32>) -> Vec<String> {
        range.map(|id| id.to_string()).collect()
    }

    #[tokio::test(start_paused = true)]
    async fn products_keep_upstreams_order_when_lookups_finish_out_of_order() {
        let products = resolve_products(1, ids(1..6), |id| async move {
            let id: i32 = id.parse().unwrap();
            tokio::time::sleep(Duration::from_millis(100 * (10 - id as u64))).await;
            Ok(details(id))
        })
        .await
        .unwrap();
        let got: Vec<i32> = products.iter().map(|p| p.id).collect();
        assert_eq!(got, vec![1, 2, 3, 4, 5]);
    }

    #[tokio::test]
    async fn a_failed_dlc_lookup_is_dropped() {
        let products = resolve_products(1, ids(1..5), |id| async move {
            match id.as_str() {
                "3" => Err(not_found()),
                _ => Ok(details(id.parse().unwrap())),
            }
        })
        .await
        .unwrap();
        let got: Vec<i32> = products.iter().map(|p| p.id).collect();
        assert_eq!(got, vec![1, 2, 4]);
    }

    #[tokio::test]
    async fn a_failed_base_game_lookup_fails_the_call() {
        let result = resolve_products(1, ids(1..4), |id| async move {
            match id.as_str() {
                "1" => Err(not_found()),
                _ => Ok(details(id.parse().unwrap())),
            }
        })
        .await;
        let error = result.err().expect("the base game lookup failed");
        assert_eq!(error.kind(), crate::api::error::GogErrorKind::NotFound);
    }

    #[tokio::test(start_paused = true)]
    async fn lookups_run_concurrently_up_to_the_limit() {
        let in_flight = AtomicUsize::new(0);
        let peak = AtomicUsize::new(0);
        let products = resolve_products(1, ids(1..21), |id| {
            let (in_flight, peak) = (&in_flight, &peak);
            async move {
                let now = in_flight.fetch_add(1, Ordering::SeqCst) + 1;
                peak.fetch_max(now, Ordering::SeqCst);
                tokio::time::sleep(Duration::from_millis(50)).await;
                in_flight.fetch_sub(1, Ordering::SeqCst);
                Ok(details(id.parse().unwrap()))
            }
        })
        .await
        .unwrap();
        assert_eq!(products.len(), 20);
        assert_eq!(peak.load(Ordering::SeqCst), PRODUCT_DETAILS_CONCURRENCY);
    }

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
