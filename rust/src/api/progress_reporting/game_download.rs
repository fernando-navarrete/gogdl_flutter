use flutter_rust_bridge::frb;

#[derive(Clone)]
#[cfg_attr(test, derive(Debug, PartialEq))]
#[frb(non_opaque)]
pub enum DownloadGameProgress {
    /// Totals known before any work starts. `total_bytes` counts *compressed*
    /// bytes as they arrive off the wire — it is the denominator for
    /// `DownloadProgress` and is smaller than the install's size on disk.
    Started { total_files: u64, total_bytes: u64 },
    /// Files whose on-disk size has been checked, out of `Started.total_files`.
    /// Coalesced on a 150ms ticker.
    FileSizeVerification { checked_files: u64 },
    /// The allocation pass is starting, with its own denominators: only files
    /// that failed size verification are allocated, so these are smaller than
    /// `Started`'s. `total_bytes` here is *uncompressed*. Sent once, and
    /// skipped entirely when every file already had the right size.
    FileAllocationStarted { total_files: u64, total_bytes: u64 },
    /// Files pre-allocated so far, against `FileAllocationStarted`'s totals.
    /// Coalesced on a 150ms ticker.
    FileAllocation {
        allocated_files: u64,
        allocated_bytes: u64,
    },
    /// A file's path could not be resolved on disk; the download is aborting
    /// with a file-allocation error. Sent once per path, immediately.
    AllocationError { path: String },
    /// Compressed bytes received so far, out of `Started.total_bytes`. Failed
    /// attempts are already subtracted, so this can stall or move backwards.
    /// Coalesced on a 150ms ticker.
    DownloadProgress { downloaded_bytes: u64 },
    /// Every chunk transferred and checksummed. Terminal.
    Finished,
    /// The job was cancelled with its `CancelToken`. The files written so
    /// far stay, and `repairGame` resumes the install. Terminal.
    Cancelled,
}
