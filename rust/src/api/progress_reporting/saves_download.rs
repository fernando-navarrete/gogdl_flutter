use flutter_rust_bridge::frb;

#[derive(Clone)]
#[cfg_attr(test, derive(Debug, PartialEq))]
#[frb(non_opaque)]
pub enum DownloadSavesProgress {
    /// Totals known before any file is transferred. `total_bytes` counts
    /// *stored* (compressed) bytes and is the denominator for
    /// `Progress.downloaded_bytes`. Both are 0 when the game has no cloud
    /// saves. Sent once, immediately.
    Started { total_files: u64, total_bytes: u64 },
    /// A file's transfer has begun; `total_bytes` is the denominator for the
    /// `file_downloaded_bytes` that follow. Sent once per file, immediately.
    FileStarted {
        name: String,
        destination: String,
        total_bytes: u64,
    },
    /// Stored bytes received so far: `downloaded_bytes` across the whole job,
    /// `file_downloaded_bytes` for the file opened by the last `FileStarted`.
    /// Never moves backwards — saves are not retried.
    /// Coalesced on a 150ms ticker.
    Progress {
        downloaded_bytes: u64,
        file_downloaded_bytes: u64,
    },
    /// The file is written, hash-verified against GOG's ETag, and its
    /// modification time restored. Sent once per file, immediately.
    FileFinished { name: String },
    /// Every file downloaded. Terminal.
    Finished,
}
