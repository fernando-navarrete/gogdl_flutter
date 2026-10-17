use flutter_rust_bridge::frb;

#[derive(Clone)]
#[cfg_attr(test, derive(Debug, PartialEq))]
#[frb(non_opaque)]
pub enum RepairGameProgress {
    /// Totals known before any work starts. Unlike `DownloadGameProgress::Started`,
    /// `total_bytes` is *uncompressed* — it is the denominator for `Verification`,
    /// not for `DownloadProgress`, which gets its own total from `DownloadStarted`.
    Started { total_files: u64, total_bytes: u64 },
    /// Files whose on-disk size has been checked, out of `Started.total_files`.
    /// Coalesced on a 150ms ticker.
    FileSizeVerification { checked_files: u64 },
    /// The allocation pass is starting, with its own denominators: only files
    /// that failed size verification are allocated, so these are smaller than
    /// `Started`'s. `total_bytes` here is uncompressed. Sent once, and
    /// skipped entirely when every file already had the right size.
    FileAllocationStarted { total_files: u64, total_bytes: u64 },
    /// Files pre-allocated so far, against `FileAllocationStarted`'s totals.
    /// Coalesced on a 150ms ticker.
    FileAllocation {
        allocated_files: u64,
        allocated_bytes: u64,
    },
    /// A file's path could not be resolved on disk; the repair is aborting
    /// with a file-allocation error. Sent once per path, immediately.
    AllocationError { path: String },
    /// Bytes checksummed so far, verified and failed alike, out of
    /// `Started.total_bytes`. Coalesced on a 150ms ticker.
    Verification { checked_bytes: u64 },
    /// The file's relative path could not be resolved on disk. Sent once
    /// per path; `chunk_size` is the first failing chunk's uncompressed
    /// size, not the file's.
    CouldNotResolvePath { path: String, chunk_size: u64 },
    /// No file exists yet at the resolved path. Sent once per path;
    /// `chunk_size` is as in `CouldNotResolvePath`.
    FileNotFound { path: String, chunk_size: u64 },
    /// A chunk's on-disk MD5 didn't match the manifest. Sent once per path;
    /// `chunk_size` is as in `CouldNotResolvePath`.
    ChecksumMismatch { path: String, chunk_size: u64 },
    /// The re-download stage is starting; `total_bytes` is the *compressed*
    /// size of the chunks that failed verification. Sent once, and skipped
    /// entirely when the install was already intact.
    DownloadStarted { total_bytes: u64 },
    /// Compressed bytes received so far, out of `DownloadStarted.total_bytes`.
    /// Failed attempts are already subtracted, so this can stall or move
    /// backwards. Coalesced on a 150ms ticker.
    DownloadProgress { downloaded_bytes: u64 },
    /// The repair ran to completion, whether or not anything needed
    /// re-downloading. Terminal.
    Finished,
    /// The job was cancelled with its `CancelToken`. The files written so
    /// far stay, and another `repairGame` resumes the install. Terminal.
    Cancelled,
}
