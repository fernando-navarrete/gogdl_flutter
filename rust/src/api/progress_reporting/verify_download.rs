use flutter_rust_bridge::frb;

#[derive(Clone)]
#[cfg_attr(test, derive(Debug, PartialEq))]
#[frb(non_opaque)]
pub enum VerifyDownloadProgress {
    /// Total bytes to be checked, known before any chunk is read.
    Started { total_bytes: u64 },
    /// Bytes checked so far, verified and failed alike. Coalesced on a
    /// 150ms ticker.
    Progress { checked_bytes: u64 },
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
    /// Verification ran to completion. `bad_chunks` is how many chunks need
    /// re-downloading; zero means the install is intact. Terminal.
    Finished { bad_chunks: u64 },
    /// The job was cancelled with its `CancelToken`. Verifying writes
    /// nothing, so the install is as it was. Terminal.
    Cancelled,
}
