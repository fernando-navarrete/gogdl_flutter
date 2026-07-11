use flutter_rust_bridge::{frb, DartFnFuture};
use futures::stream::{self, StreamExt};
use gogdl_lib2::{
    Client, DownloadDetail, DownloadEvent, DownloadJobEvent, DownloadStage, DownloadableFiles,
    FileAllocationEvent, FileVerifyEvent, GogDl, GogDlError, RemoteConfig, RepairDetail,
    RepairEvent, RepairStage, SaveFile, SaveProgress, VerifyChunksEvent, VerifyEvent,
};
use std::collections::HashSet;
use std::path::Path;
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
    pub release_date_timestamp: i64,
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
    pub total_chunks: u64,
    pub verified_chunks: u64,
    pub error_chunks: Vec<String>,
}

#[frb(opaque)]
#[derive(Clone, Copy, PartialEq)]
pub enum RepairStatus {
    FetchingFiles,
    VerifyingFiles,
    Allocating,
    VerifyingChunks,
    Downloading,
    Repaired,
}

impl RepairStatus {
    // Opaque enums expose no variant info to Dart, so this accessor is what
    // lets a client tell which stage a `RepairStream` snapshot came from,
    // e.g. `switch (event.status.name()) { case "downloading": ... }`.
    #[frb(sync)]
    pub fn name(&self) -> String {
        match self {
            RepairStatus::FetchingFiles => "fetchingFiles",
            RepairStatus::VerifyingFiles => "verifyingFiles",
            RepairStatus::Allocating => "allocating",
            RepairStatus::VerifyingChunks => "verifyingChunks",
            RepairStatus::Downloading => "downloading",
            RepairStatus::Repaired => "repaired",
        }
        .to_string()
    }
}

#[frb(opaque)]
#[derive(Clone)]
pub struct RepairStream {
    pub status: RepairStatus,
    // file-verify stage progress
    pub total_files: u64,
    pub processed_files: u64,
    // download stage progress (populated from DownloadEvent::Total / Progress)
    pub total_chunks: u64,
    pub downloaded_chunks: u64,
    pub total_bytes: u64,
    pub downloaded_bytes: u64,
    // paths still broken as of this snapshot -- see `unresolved_files`
    pub error_files: Vec<String>,
}

#[frb(opaque)]
#[derive(Clone, Copy, PartialEq)]
pub enum DownloadStatus {
    FetchingFiles,
    Allocating,
    Downloading,
    Downloaded,
}

impl DownloadStatus {
    // Opaque enums expose no variant info to Dart, so this accessor is what
    // lets a client tell which stage a `DownloadStream` snapshot came from,
    // e.g. `switch (event.status.name()) { case "downloading": ... }`.
    #[frb(sync)]
    pub fn name(&self) -> String {
        match self {
            DownloadStatus::FetchingFiles => "fetchingFiles",
            DownloadStatus::Allocating => "allocating",
            DownloadStatus::Downloading => "downloading",
            DownloadStatus::Downloaded => "downloaded",
        }
        .to_string()
    }
}

#[frb(opaque)]
#[derive(Clone)]
pub struct DownloadStream {
    pub status: DownloadStatus,
    // allocation-stage progress
    pub total_files: u64,
    pub allocated_files: u64,
    // download-stage progress
    pub total_chunks: u64,
    pub downloaded_chunks: u64,
    pub total_bytes: u64,
    pub downloaded_bytes: u64,
    // files that hit an allocation or download error
    pub error_files: Vec<String>,
}

#[frb(opaque)]
#[derive(Clone)]
pub struct CloudSaveFile {
    inner: SaveFile,
}

impl CloudSaveFile {
    // The lib's `SaveFile` wraps a private raw listing line; this is the
    // only way a client can read a display path for it.
    #[frb(sync)]
    pub fn path(&self) -> String {
        self.inner.get_path()
    }
}

#[frb(opaque)]
#[derive(Clone)]
pub struct CloudSaveConfig {
    inner: RemoteConfig,
}

impl CloudSaveConfig {
    #[frb(sync)]
    pub fn is_supported(&self) -> bool {
        self.inner.is_supported()
    }
    // Returns (known_folder, relative_path), e.g. ("Saved Games", "MyGame/saves").
    #[frb(sync)]
    pub fn local_path(&self) -> Result<(String, String), GogError> {
        self.inner.get_path().map_err(|e| GogError::SavesError {
            message: e.to_string(),
        })
    }
}

#[frb(opaque)]
#[derive(Clone)]
pub struct SaveAuthIds {
    pub client_id: String,
    pub client_secret: String,
}

#[frb(opaque)]
#[derive(Clone, Copy, PartialEq)]
pub enum SaveDownloadStatus {
    Downloading,
    Downloaded,
}

impl SaveDownloadStatus {
    #[frb(sync)]
    pub fn name(&self) -> String {
        match self {
            SaveDownloadStatus::Downloading => "downloading",
            SaveDownloadStatus::Downloaded => "downloaded",
        }
        .to_string()
    }
}

#[frb(opaque)]
#[derive(Clone)]
pub struct SaveDownloadStream {
    pub status: SaveDownloadStatus,
    pub transferred: u64,
    pub total: u64,
}

#[frb(opaque)]
#[derive(Clone, Copy, PartialEq)]
pub enum SaveUploadStatus {
    Uploading,
    Uploaded,
}

impl SaveUploadStatus {
    #[frb(sync)]
    pub fn name(&self) -> String {
        match self {
            SaveUploadStatus::Uploading => "uploading",
            SaveUploadStatus::Uploaded => "uploaded",
        }
        .to_string()
    }
}

#[frb(opaque)]
#[derive(Clone)]
pub struct SaveUploadStream {
    pub status: SaveUploadStatus,
    pub transferred: u64,
    pub total: u64,
}

/// Reconciles paths flagged broken during the verify/allocate/chunk-verify
/// stages against download-stage outcomes, so callers only see paths that
/// are still broken -- not every path that was found broken and then fixed
/// by the repair's own download stage (the common case). A path counts as
/// still broken if the download stage never confirmed a chunk for it, or if
/// it hit a hard download error -- the latter sticks regardless of event
/// arrival order, since one path's chunks can succeed and fail in any order.
fn unresolved_files(
    flagged: &HashSet<String>,
    download_errors: &HashSet<String>,
    download_successes: &HashSet<String>,
) -> Vec<String> {
    flagged
        .difference(download_successes)
        .chain(download_errors.iter())
        .cloned()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect()
}

#[cfg(test)]
mod unresolved_files_tests {
    use super::unresolved_files;
    use std::collections::HashSet;

    fn set(items: &[&str]) -> HashSet<String> {
        items.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn flagged_file_fully_downloaded_is_resolved() {
        let flagged = set(&["a.bin"]);
        let download_errors = HashSet::new();
        let download_successes = set(&["a.bin"]);
        assert!(unresolved_files(&flagged, &download_errors, &download_successes).is_empty());
    }

    #[test]
    fn flagged_file_never_downloaded_stays_unresolved() {
        let flagged = set(&["a.bin"]);
        let download_errors = HashSet::new();
        let download_successes = HashSet::new();
        assert_eq!(
            unresolved_files(&flagged, &download_errors, &download_successes),
            vec!["a.bin".to_string()]
        );
    }

    #[test]
    fn one_failed_chunk_keeps_file_unresolved_despite_other_successes() {
        // Same path can have multiple chunks; one failing keeps it broken
        // even though other chunks for that path succeeded.
        let flagged = set(&["a.bin"]);
        let download_errors = set(&["a.bin"]);
        let download_successes = set(&["a.bin"]);
        assert_eq!(
            unresolved_files(&flagged, &download_errors, &download_successes),
            vec!["a.bin".to_string()]
        );
    }

    #[test]
    fn error_before_success_still_marks_unresolved() {
        // Order independence: a download error recorded before a later
        // success for the same path must not be forgotten.
        let flagged = set(&["a.bin"]);
        let mut download_errors = HashSet::new();
        download_errors.insert("a.bin".to_string());
        let download_successes = set(&["a.bin"]);
        assert_eq!(
            unresolved_files(&flagged, &download_errors, &download_successes),
            vec!["a.bin".to_string()]
        );
    }

    #[test]
    fn unflagged_download_error_is_reported() {
        // A hard download error can occur without the path ever being
        // flagged broken up front (e.g. a redownload triggered by a sibling
        // chunk verify failure elsewhere in the same file).
        let flagged = HashSet::new();
        let download_errors = set(&["b.bin"]);
        let download_successes = HashSet::new();
        assert_eq!(
            unresolved_files(&flagged, &download_errors, &download_successes),
            vec!["b.bin".to_string()]
        );
    }

    #[test]
    fn no_flags_and_no_errors_is_resolved() {
        let flagged = HashSet::new();
        let download_errors = HashSet::new();
        let download_successes = HashSet::new();
        assert!(unresolved_files(&flagged, &download_errors, &download_successes).is_empty());
    }
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
    #[error("SavesError: {message}")]
    SavesError { message: String },
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
            GogDlError::SavesError(saves_error) => GogError::SavesError {
                message: saves_error.to_string(),
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
                release_date_timestamp: build.date_published.timestamp(),
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
                total_chunks: 0,
                verified_chunks: 0,
                error_chunks: Vec::new(),
            })
            .ok();
        let files = self
            .get_downloadable_files(game_id, build_name, selected_products)
            .await?;

        let total_chunks = files
            .iter()
            .map(|file| {
                file.product_files
                    .iter()
                    .map(|depot| depot.get_download_units().len() as u64)
            })
            .flatten()
            .sum::<u64>();

        stream
            .add(VerificationStream {
                status: VerificationStatus::Verifying,
                total_chunks: total_chunks,
                verified_chunks: 0,
                error_chunks: Vec::new(),
            })
            .ok();

        let (tx, mut rx) = mpsc::unbounded_channel();

        let mut verified_chunks: u64 = 0;
        let mut error_chunks: Vec<String> = Vec::new();

        let verify_future = self.inner.verify_files(files, path, tx);
        let drain_future = async {
            while let Some(event) = rx.recv().await {
                match event {
                    VerifyEvent::ChunkOk => {
                        verified_chunks += 1;
                    }
                    VerifyEvent::ChunkChecksumMismatch(file) => {
                        verified_chunks += 1;
                        error_chunks.push(file);
                    }
                    VerifyEvent::FileNotFound(file) => {
                        verified_chunks += 1;
                        error_chunks.push(file);
                    }
                    VerifyEvent::CouldNotResolvePath(file) => {
                        verified_chunks += 1;
                        error_chunks.push(file);
                    }
                }
                stream
                    .add(VerificationStream {
                        status: VerificationStatus::Verifying,
                        total_chunks,
                        verified_chunks,
                        error_chunks: error_chunks.clone(),
                    })
                    .ok();
            }
        };

        let (verify_result, ()) = tokio::join!(verify_future, drain_future);
        verify_result?;

        stream
            .add(VerificationStream {
                status: VerificationStatus::Verified,
                total_chunks,
                verified_chunks,
                error_chunks: error_chunks.clone(),
            })
            .ok();

        Ok(())
    }

    pub async fn repair_download(
        &self,
        path: &str,
        build_name: &str,
        selected_products: &[String],
        game_id: i32,
        stream: StreamSink<RepairStream>,
    ) -> Result<(), GogError> {
        stream
            .add(RepairStream {
                status: RepairStatus::FetchingFiles,
                total_files: 0,
                processed_files: 0,
                total_chunks: 0,
                downloaded_chunks: 0,
                total_bytes: 0,
                downloaded_bytes: 0,
                error_files: Vec::new(),
            })
            .ok();
        let files = self
            .get_downloadable_files(game_id, build_name, selected_products)
            .await?;

        let total_files = files
            .iter()
            .map(|file| file.product_files.len() as u64)
            .sum::<u64>();

        stream
            .add(RepairStream {
                status: RepairStatus::VerifyingFiles,
                total_files,
                processed_files: 0,
                total_chunks: 0,
                downloaded_chunks: 0,
                total_bytes: 0,
                downloaded_bytes: 0,
                error_files: Vec::new(),
            })
            .ok();

        let (tx, mut rx) = mpsc::unbounded_channel::<RepairEvent>();

        let mut processed_files: u64 = 0;
        let mut total_chunks: u64 = 0;
        let mut downloaded_chunks: u64 = 0;
        let mut total_bytes: u64 = 0;
        let mut downloaded_bytes: u64 = 0;

        // FileVerify/Allocation/ChunkVerify errors flag a path as needing
        // repair; that's the point of the exercise, not a failure. A path is
        // only genuinely still broken once the drain loop finishes if the
        // download stage never confirmed a chunk for it (`flagged` minus
        // `download_successes`) or it hit a hard download-stage error
        // (`download_errors`, sticky regardless of event arrival order since
        // downloads for one path can be a mix of successful and failed
        // chunks arriving in any order).
        let mut flagged: HashSet<String> = HashSet::new();
        let mut download_errors: HashSet<String> = HashSet::new();
        let mut download_successes: HashSet<String> = HashSet::new();

        let repair_future = self.inner.repair_files(files, path, tx);
        let drain_future = async {
            while let Some(RepairEvent { stage, detail }) = rx.recv().await {
                let status = match stage {
                    RepairStage::VerifyingFiles => RepairStatus::VerifyingFiles,
                    RepairStage::Allocating => RepairStatus::Allocating,
                    RepairStage::VerifyingChunks => RepairStatus::VerifyingChunks,
                    RepairStage::Downloading => RepairStatus::Downloading,
                };
                match detail {
                    RepairDetail::FileVerify(event) => {
                        processed_files += 1;
                        match event {
                            FileVerifyEvent::FileOk => {}
                            FileVerifyEvent::FileNotFound(path)
                            | FileVerifyEvent::CouldNotResolvePath(path)
                            | FileVerifyEvent::CouldNotReadFileSize(path)
                            | FileVerifyEvent::ChecksumMismatch(path)
                            | FileVerifyEvent::SizeMismatch(path, _, _) => {
                                flagged.insert(path);
                            }
                        }
                    }
                    RepairDetail::Allocation(event) => match event {
                        FileAllocationEvent::FileOk | FileAllocationEvent::FileAllocated(_) => {}
                        FileAllocationEvent::PathResolveError(path)
                        | FileAllocationEvent::AllocationError(path)
                        | FileAllocationEvent::FileSizeError(path) => {
                            flagged.insert(path);
                        }
                    },
                    RepairDetail::ChunkVerify(event) => match event {
                        VerifyChunksEvent::PathResolveError(path)
                        | VerifyChunksEvent::FileNotFound(path)
                        | VerifyChunksEvent::ChecksumCalculationError(path)
                        | VerifyChunksEvent::ChunkChecksumMismatch(path) => {
                            flagged.insert(path);
                        }
                    },
                    RepairDetail::Download(event) => match event {
                        DownloadEvent::Total { bytes, chunks } => {
                            total_bytes = bytes;
                            total_chunks = chunks as u64;
                        }
                        DownloadEvent::Progress { bytes } => {
                            downloaded_bytes += bytes;
                        }
                        DownloadEvent::ChunkDownloaded { path } => {
                            downloaded_chunks += 1;
                            download_successes.insert(path);
                        }
                        DownloadEvent::SecureLinkError(path)
                        | DownloadEvent::DownloadError(path)
                        | DownloadEvent::WriteError(path)
                        | DownloadEvent::PathResolveError(path) => {
                            download_errors.insert(path);
                        }
                    },
                }
                stream
                    .add(RepairStream {
                        status,
                        total_files,
                        processed_files,
                        total_chunks,
                        downloaded_chunks,
                        total_bytes,
                        downloaded_bytes,
                        error_files: unresolved_files(
                            &flagged,
                            &download_errors,
                            &download_successes,
                        ),
                    })
                    .ok();
            }
        };

        let (repair_result, ()) = tokio::join!(repair_future, drain_future);
        repair_result?;

        stream
            .add(RepairStream {
                status: RepairStatus::Repaired,
                total_files,
                processed_files,
                total_chunks,
                downloaded_chunks,
                total_bytes,
                downloaded_bytes,
                error_files: unresolved_files(&flagged, &download_errors, &download_successes),
            })
            .ok();

        Ok(())
    }

    pub async fn download_game(
        &self,
        path: &str,
        build_name: &str,
        selected_products: &[String],
        game_id: i32,
        stream: StreamSink<DownloadStream>,
    ) -> Result<(), GogError> {
        stream
            .add(DownloadStream {
                status: DownloadStatus::FetchingFiles,
                total_files: 0,
                allocated_files: 0,
                total_chunks: 0,
                downloaded_chunks: 0,
                total_bytes: 0,
                downloaded_bytes: 0,
                error_files: Vec::new(),
            })
            .ok();
        let files = self
            .get_downloadable_files(game_id, build_name, selected_products)
            .await?;

        let total_files = files
            .iter()
            .map(|file| file.product_files.len() as u64)
            .sum::<u64>();

        stream
            .add(DownloadStream {
                status: DownloadStatus::Allocating,
                total_files,
                allocated_files: 0,
                total_chunks: 0,
                downloaded_chunks: 0,
                total_bytes: 0,
                downloaded_bytes: 0,
                error_files: Vec::new(),
            })
            .ok();

        let (tx, mut rx) = mpsc::unbounded_channel::<DownloadJobEvent>();

        let mut allocated_files: u64 = 0;
        let mut total_chunks: u64 = 0;
        let mut downloaded_chunks: u64 = 0;
        let mut total_bytes: u64 = 0;
        let mut downloaded_bytes: u64 = 0;
        let mut error_files: HashSet<String> = HashSet::new();

        let download_future = self.inner.download_files(files, path, tx);
        let drain_future = async {
            while let Some(DownloadJobEvent { stage, detail }) = rx.recv().await {
                let status = match stage {
                    DownloadStage::Allocating => DownloadStatus::Allocating,
                    DownloadStage::Downloading => DownloadStatus::Downloading,
                };
                match detail {
                    DownloadDetail::Allocation(event) => {
                        allocated_files += 1;
                        match event {
                            FileAllocationEvent::FileOk | FileAllocationEvent::FileAllocated(_) => {
                            }
                            FileAllocationEvent::PathResolveError(path)
                            | FileAllocationEvent::AllocationError(path)
                            | FileAllocationEvent::FileSizeError(path) => {
                                error_files.insert(path);
                            }
                        }
                    }
                    DownloadDetail::Download(event) => match event {
                        DownloadEvent::Total { bytes, chunks } => {
                            total_bytes = bytes;
                            total_chunks = chunks as u64;
                        }
                        DownloadEvent::Progress { bytes } => {
                            downloaded_bytes += bytes;
                        }
                        DownloadEvent::ChunkDownloaded { path: _ } => {
                            downloaded_chunks += 1;
                        }
                        DownloadEvent::SecureLinkError(path)
                        | DownloadEvent::DownloadError(path)
                        | DownloadEvent::WriteError(path)
                        | DownloadEvent::PathResolveError(path) => {
                            error_files.insert(path);
                        }
                    },
                }
                stream
                    .add(DownloadStream {
                        status,
                        total_files,
                        allocated_files,
                        total_chunks,
                        downloaded_chunks,
                        total_bytes,
                        downloaded_bytes,
                        error_files: error_files.iter().cloned().collect(),
                    })
                    .ok();
            }
        };

        let (download_result, ()) = tokio::join!(download_future, drain_future);
        download_result?;

        stream
            .add(DownloadStream {
                status: DownloadStatus::Downloaded,
                total_files,
                allocated_files,
                total_chunks,
                downloaded_chunks,
                total_bytes,
                downloaded_bytes,
                error_files: error_files.into_iter().collect(),
            })
            .ok();

        Ok(())
    }

    pub async fn get_save_auth_ids(&self, game_id: i32) -> Result<SaveAuthIds, GogError> {
        let (client_id, client_secret) = self.inner.get_save_auth_ids(game_id).await?;
        Ok(SaveAuthIds {
            client_id,
            client_secret,
        })
    }

    pub async fn get_save_remote_config(
        &self,
        client_id: &str,
    ) -> Result<CloudSaveConfig, GogError> {
        let remote_config = self.inner.get_remote_config(client_id).await?;
        Ok(CloudSaveConfig {
            inner: remote_config,
        })
    }

    pub async fn get_save_file_list(
        &self,
        client_id: &str,
        client_secret: &str,
    ) -> Result<Vec<CloudSaveFile>, GogError> {
        let save_files = self
            .inner
            .get_save_file_list(client_id, client_secret)
            .await?;
        Ok(save_files
            .into_iter()
            .map(|inner| CloudSaveFile { inner })
            .collect())
    }

    pub async fn delete_save(
        &self,
        save_file: &CloudSaveFile,
        client_id: &str,
        client_secret: &str,
    ) -> Result<(), GogError> {
        self.inner
            .delete_save_file(&save_file.inner, client_id, client_secret)
            .await?;
        Ok(())
    }

    pub async fn download_save(
        &self,
        save_file: &CloudSaveFile,
        client_id: &str,
        client_secret: &str,
        path: String,
        stream: StreamSink<SaveDownloadStream>,
    ) -> Result<(), GogError> {
        stream
            .add(SaveDownloadStream {
                status: SaveDownloadStatus::Downloading,
                transferred: 0,
                total: 0,
            })
            .ok();

        let (tx, mut rx) = mpsc::unbounded_channel::<SaveProgress>();

        let mut transferred: u64 = 0;
        let mut total: u64 = 0;

        let download_future = self.inner.download_save_file(
            &save_file.inner,
            client_id,
            client_secret,
            Path::new(&path),
            tx,
        );
        let drain_future = async {
            while let Some(SaveProgress {
                transferred: event_transferred,
                total: event_total,
            }) = rx.recv().await
            {
                transferred = event_transferred;
                total = event_total;
                stream
                    .add(SaveDownloadStream {
                        status: SaveDownloadStatus::Downloading,
                        transferred,
                        total,
                    })
                    .ok();
            }
        };

        let (download_result, ()) = tokio::join!(download_future, drain_future);
        download_result?;

        stream
            .add(SaveDownloadStream {
                status: SaveDownloadStatus::Downloaded,
                transferred,
                total,
            })
            .ok();

        Ok(())
    }

    pub async fn upload_save(
        &self,
        client_id: &str,
        client_secret: &str,
        path: String,
        url_path: String,
        stream: StreamSink<SaveUploadStream>,
    ) -> Result<(), GogError> {
        stream
            .add(SaveUploadStream {
                status: SaveUploadStatus::Uploading,
                transferred: 0,
                total: 0,
            })
            .ok();

        let (tx, mut rx) = mpsc::unbounded_channel::<SaveProgress>();

        let mut transferred: u64 = 0;
        let mut total: u64 = 0;

        let upload_future =
            self.inner
                .upload_save_file(client_id, client_secret, Path::new(&path), &url_path, tx);
        let drain_future = async {
            while let Some(SaveProgress {
                transferred: event_transferred,
                total: event_total,
            }) = rx.recv().await
            {
                transferred = event_transferred;
                total = event_total;
                stream
                    .add(SaveUploadStream {
                        status: SaveUploadStatus::Uploading,
                        transferred,
                        total,
                    })
                    .ok();
            }
        };

        let (upload_result, ()) = tokio::join!(upload_future, drain_future);
        upload_result?;

        stream
            .add(SaveUploadStream {
                status: SaveUploadStatus::Uploaded,
                transferred,
                total,
            })
            .ok();

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
