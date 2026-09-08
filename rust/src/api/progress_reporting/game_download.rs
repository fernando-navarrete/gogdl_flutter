use flutter_rust_bridge::frb;

#[derive(Clone)]
#[frb(non_opaque)]
pub enum DownloadGameProgress {
    Started(u64),
    FileSizeVerification,
    FileAllocation,
    Verification,
    AllocationError,
    DownloadProgress(u64),
    DownloadProgressRegression(u64),
    Finished(u64),
}
