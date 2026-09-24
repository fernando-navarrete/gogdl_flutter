use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use flutter_rust_bridge::{DartFnFuture, frb};
use gogdl_lib::{Client, DownloadEvent, GogDl};
use tokio::sync::mpsc;
use tokio::time::interval;

use crate::api::auth_refresh_observer::AuthRefreshObserver;
pub use crate::api::error::GogError;
pub use crate::api::models::downloadable_product::DownloadableProduct;
pub use crate::api::models::game_build::GameBuild;
pub use crate::api::models::proton_release::ProtonRelease;
use crate::api::progress_reporting::{
    DownloadGameProgress, DownloadSavesProgress, ProtonDownloadProgress, RepairGameProgress,
    UploadSavesProgress, VerifyDownloadProgress,
};
use crate::frb_generated::StreamSink;
use crate::progress::{
    self,
    download::DownloadTranslator,
    proton::ProtonTranslator,
    saves::{SavesDownloadTranslator, SavesUploadTranslator},
    verify::VerifyTranslator,
};

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

        let (tx, rx) = mpsc::unbounded_channel();
        let download_future = self
            .inner
            .download_proton_release(&release, Path::new(path), tx);

        let progress_reporting_future = progress::drive(ProtonTranslator::new(), rx, |p| {
            let _ = sink.add(p);
        });

        let (result, _) = tokio::join!(download_future, progress_reporting_future);
        let extracted_path = result?;
        let _ = sink.add(ProtonDownloadProgress::Finished(
            extracted_path.to_string_lossy().into_owned(),
        ));
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

    pub async fn verify_download(
        &self,
        game_id: i32,
        path: &str,
        build_name: &str,
        selected_products: &[i32],
        sink: StreamSink<VerifyDownloadProgress>,
    ) -> Result<(), GogError> {
        let bundles = self
            .inner
            .get_product_bundles(game_id, build_name, selected_products)
            .await?;

        let total_bytes: u64 = bundles
            .iter()
            .flat_map(|bundle| bundle.product_files.iter())
            .filter_map(|file| file.size())
            .sum();

        let (tx, rx) = mpsc::unbounded_channel();
        let verify_future = self.inner.verify_files(bundles, path, tx);

        let _ = sink.add(VerifyDownloadProgress::Started(total_bytes));
        let progress_reporting_future = progress::drive(VerifyTranslator::new(), rx, |p| {
            let _ = sink.add(p);
        });

        let (result, _) = tokio::join!(verify_future, progress_reporting_future);

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
    pub async fn download_game(
        &self,
        game_id: i32,
        path: &str,
        build_name: &str,
        selected_products: &[i32],
        sink: StreamSink<DownloadGameProgress>,
    ) -> Result<(), GogError> {
        let bundles = self
            .inner
            .get_product_bundles(game_id, build_name, selected_products)
            .await?;

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

        let (tx, rx) = mpsc::unbounded_channel();
        let download_future = self.inner.download_game(bundles, path, tx);

        let _ = sink.add(DownloadGameProgress::Started {
            total_files,
            total_bytes,
        });
        let progress_reporting_future = progress::drive(DownloadTranslator::new(), rx, |p| {
            let _ = sink.add(p);
        });

        let (result, _) = tokio::join!(download_future, progress_reporting_future);
        result?;
        let _ = sink.add(DownloadGameProgress::Finished);
        Ok(())
    }
    pub async fn repair_game(
        &self,
        game_id: i32,
        path: &str,
        build_name: &str,
        selected_products: &[i32],
        sink: StreamSink<RepairGameProgress>,
    ) -> Result<(), GogError> {
        let bundles = self
            .inner
            .get_product_bundles(game_id, build_name, selected_products)
            .await?;

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

        let (tx, mut rx) = mpsc::unbounded_channel();
        let repair_future = self.inner.repair_game(bundles, path, tx);

        let progress_sink = sink.clone();
        let progress_reporting_future = async move {
            let _ = progress_sink.add(RepairGameProgress::Started {
                total_files,
                total_bytes,
            });

            // `repair_game` runs these four stages strictly in sequence (see
            // `DownloadStageEvent`'s docs): size verification, allocation,
            // chunk verification, then download. The download stage is
            // skipped entirely when chunk verification finds nothing broken.
            #[derive(PartialEq)]
            enum Stage {
                SizeVerification,
                Allocation,
                Verification,
                Download,
            }
            let mut stage = Stage::SizeVerification;

            let mut checked_files: u64 = 0;

            // Only files that fail size verification go on to allocation, so
            // these totals are discovered incrementally during that stage
            // and only known for certain once it ends.
            let mut allocation_total_files: u64 = 0;
            let mut allocation_total_bytes: u64 = 0;
            let mut allocated_files: u64 = 0;
            let mut allocated_bytes: u64 = 0;

            let mut checked_bytes: u64 = 0;
            let mut download_total_bytes: u64 = 0;

            let mut downloaded: u64 = 0;

            let mut dirty = false;
            // Only forward the first allocation/verification failure per
            // path so the UI sees one event per file; kept separate so a
            // path that failed allocation doesn't silence its later
            // verification event.
            let mut reported_allocation: HashSet<String> = HashSet::new();
            let mut reported_verification: HashSet<String> = HashSet::new();

            let mut ticker = interval(Duration::from_millis(150));
            ticker.tick().await; // first tick fires immediately, discard it

            loop {
                tokio::select! {
                    event = rx.recv() => {
                        match event {
                            Some(gogdl_lib::DownloadStageEvent::FileSizeVerificationStage(event)) => {
                                checked_files += 1;
                                dirty = true;
                                use gogdl_lib::FileSizeVerificationEvent as E;
                                match event {
                                    E::FileSizeVerificationSuccess(_, _) => {}
                                    E::FileWithNoChunks(_, size)
                                    | E::CouldNotResolvePath(_, size)
                                    | E::FileNotFound(_, size)
                                    | E::FileSizeVerificationFailed(_, size)
                                    | E::FileSizeMismatch(_, size) => {
                                        allocation_total_files += 1;
                                        allocation_total_bytes += size;
                                    }
                                }
                            }
                            Some(gogdl_lib::DownloadStageEvent::FileAllocationStage(event)) => {
                                if stage == Stage::SizeVerification {
                                    // final flush so the size-verification stage
                                    // doesn't sit at a stale count once allocation starts
                                    let _ = progress_sink.add(RepairGameProgress::FileSizeVerification { checked_files });
                                    if allocation_total_files > 0 {
                                        let _ = progress_sink.add(RepairGameProgress::FileAllocationStarted {
                                            total_files: allocation_total_files,
                                            total_bytes: allocation_total_bytes,
                                        });
                                    }
                                    stage = Stage::Allocation;
                                }
                                use gogdl_lib::FileAllocationEvent as E;
                                match event {
                                    E::FileAllocationSuccess(_, size) => {
                                        allocated_files += 1;
                                        allocated_bytes += size;
                                        dirty = true;
                                    }
                                    E::FileWithNoChunks(_, _) => {
                                        allocated_files += 1;
                                        dirty = true;
                                    }
                                    E::CouldNotResolvePath(path, _) => {
                                        allocated_files += 1;
                                        dirty = true;
                                        if reported_allocation.insert(path.clone()) {
                                            // one-shot signal, flush immediately regardless of ticker
                                            let _ = progress_sink.add(RepairGameProgress::AllocationError(path));
                                        }
                                    }
                                }
                            }
                            Some(gogdl_lib::DownloadStageEvent::VerificationStage(event)) => {
                                if stage != Stage::Verification {
                                    // final flush of whichever pre-verification stage just ended
                                    match stage {
                                        Stage::SizeVerification => {
                                            let _ = progress_sink.add(RepairGameProgress::FileSizeVerification { checked_files });
                                        }
                                        Stage::Allocation => {
                                            let _ = progress_sink.add(RepairGameProgress::FileAllocation { allocated_files, allocated_bytes });
                                        }
                                        Stage::Verification | Stage::Download => {}
                                    }
                                    stage = Stage::Verification;
                                }
                                use gogdl_lib::VerificationEvent as E;
                                match event {
                                    E::Verified(_, size) => {
                                        checked_bytes += size;
                                        dirty = true;
                                    }
                                    E::CouldNotResolvePath(path, size) => {
                                        checked_bytes += size;
                                        dirty = true;
                                        if let Some(compressed) = compressed_sizes
                                            .get_mut(&(path.clone(), size))
                                            .and_then(|sizes| sizes.pop())
                                        {
                                            download_total_bytes += compressed;
                                        }
                                        if reported_verification.insert(path.clone()) {
                                            // one-shot signal, flush immediately regardless of ticker
                                            let _ = progress_sink
                                                .add(RepairGameProgress::CouldNotResolvePath(path, size));
                                        }
                                    }
                                    E::FileNotFound(path, size) => {
                                        checked_bytes += size;
                                        dirty = true;
                                        if let Some(compressed) = compressed_sizes
                                            .get_mut(&(path.clone(), size))
                                            .and_then(|sizes| sizes.pop())
                                        {
                                            download_total_bytes += compressed;
                                        }
                                        if reported_verification.insert(path.clone()) {
                                            // one-shot signal, flush immediately regardless of ticker
                                            let _ = progress_sink
                                                .add(RepairGameProgress::FileNotFound(path, size));
                                        }
                                    }
                                    E::ChecksumMismatch(path, size) => {
                                        checked_bytes += size;
                                        dirty = true;
                                        if let Some(compressed) = compressed_sizes
                                            .get_mut(&(path.clone(), size))
                                            .and_then(|sizes| sizes.pop())
                                        {
                                            download_total_bytes += compressed;
                                        }
                                        if reported_verification.insert(path.clone()) {
                                            // one-shot signal, flush immediately regardless of ticker
                                            let _ = progress_sink
                                                .add(RepairGameProgress::ChecksumMismatch(path, size));
                                        }
                                    }
                                }
                            }
                            Some(gogdl_lib::DownloadStageEvent::FileAllocationError()) => {
                                // `repair_future` is about to resolve with
                                // `DownloadError::FileAllocationError`, which
                                // surfaces to Dart as a stream error; no
                                // progress event needed here.
                            }
                            Some(gogdl_lib::DownloadStageEvent::DownloadStage(DownloadEvent::Preparing)) => {
                                // final flush of the verification stage that just ended
                                let _ = progress_sink.add(RepairGameProgress::Verification { checked_bytes });
                                stage = Stage::Download;
                                dirty = false;
                                // one-shot signal, flush immediately regardless of ticker
                                let _ = progress_sink.add(RepairGameProgress::DownloadStarted {
                                    total_bytes: download_total_bytes,
                                });
                            }
                            Some(gogdl_lib::DownloadStageEvent::DownloadStage(DownloadEvent::Prepared)) => {}
                            Some(gogdl_lib::DownloadStageEvent::DownloadStage(DownloadEvent::Downloading)) => {}
                            Some(gogdl_lib::DownloadStageEvent::DownloadStage(DownloadEvent::Progress(delta))) => {
                                downloaded += delta as u64;
                                dirty = true;
                            }
                            Some(gogdl_lib::DownloadStageEvent::DownloadStage(DownloadEvent::ProgressRegression(delta))) => {
                                // a delta to subtract, not a replacement value; see `DownloadEvent`'s docs
                                downloaded = downloaded.saturating_sub(delta as u64);
                                dirty = true;
                            }
                            None => break, // sender dropped, repair_future is done
                        }
                    }
                    _ = ticker.tick() => {
                        if dirty {
                            match stage {
                                Stage::SizeVerification => {
                                    let _ = progress_sink.add(RepairGameProgress::FileSizeVerification { checked_files });
                                }
                                Stage::Allocation => {
                                    let _ = progress_sink.add(RepairGameProgress::FileAllocation { allocated_files, allocated_bytes });
                                }
                                Stage::Verification => {
                                    let _ = progress_sink.add(RepairGameProgress::Verification { checked_bytes });
                                }
                                Stage::Download => {
                                    let _ = progress_sink.add(RepairGameProgress::DownloadProgress { downloaded_bytes: downloaded });
                                }
                            }
                            dirty = false;
                        }
                    }
                }
            }

            // final flush so the UI doesn't sit at a stale % between the last tick
            // and the channel closing
            if dirty {
                match stage {
                    Stage::SizeVerification => {
                        let _ = progress_sink
                            .add(RepairGameProgress::FileSizeVerification { checked_files });
                    }
                    Stage::Allocation => {
                        let _ = progress_sink.add(RepairGameProgress::FileAllocation {
                            allocated_files,
                            allocated_bytes,
                        });
                    }
                    Stage::Verification => {
                        let _ =
                            progress_sink.add(RepairGameProgress::Verification { checked_bytes });
                    }
                    Stage::Download => {
                        let _ = progress_sink.add(RepairGameProgress::DownloadProgress {
                            downloaded_bytes: downloaded,
                        });
                    }
                }
            }
        };

        let (result, _) = tokio::join!(repair_future, progress_reporting_future);
        result?;
        let _ = sink.add(RepairGameProgress::Finished);
        Ok(())
    }
    pub async fn download_save_files(
        &self,
        game_id: i32,
        build_name: &str,
        prefix: &str,
        install_path: &str,
        sink: StreamSink<DownloadSavesProgress>,
    ) -> Result<(), GogError> {
        let (tx, rx) = mpsc::unbounded_channel();
        let download_future = self.inner.download_save_files(
            game_id,
            build_name,
            Path::new(prefix),
            Path::new(install_path),
            tx,
        );

        let progress_reporting_future = progress::drive(SavesDownloadTranslator::new(), rx, |p| {
            let _ = sink.add(p);
        });

        let (result, _) = tokio::join!(download_future, progress_reporting_future);
        result?;
        let _ = sink.add(DownloadSavesProgress::Finished);
        Ok(())
    }
    pub async fn upload_save_files(
        &self,
        game_id: i32,
        build_name: &str,
        prefix: &str,
        install_path: &str,
        sink: StreamSink<UploadSavesProgress>,
    ) -> Result<(), GogError> {
        let (tx, rx) = mpsc::unbounded_channel();
        let upload_future = self.inner.upload_save_files(
            game_id,
            build_name,
            Path::new(prefix),
            Path::new(install_path),
            tx,
        );

        let progress_reporting_future = progress::drive(SavesUploadTranslator::new(), rx, |p| {
            let _ = sink.add(p);
        });

        let (result, _) = tokio::join!(upload_future, progress_reporting_future);
        result?;
        let _ = sink.add(UploadSavesProgress::Finished);
        Ok(())
    }
}
