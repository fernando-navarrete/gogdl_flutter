use flutter_rust_bridge::frb;

#[derive(Clone)]
#[cfg_attr(test, derive(Debug, PartialEq))]
#[frb(non_opaque)]
pub enum UploadSavesProgress {
    /// The number of files that will be uploaded (0 if the directory holds no
    /// files). The library reports no byte total up front, so an overall byte
    /// percentage can only be derived by summing each `FileStarted.total_bytes`
    /// as they arrive. Sent once, immediately.
    Started { total_files: u64 },
    /// A file's transfer has begun; `total_bytes` is its gzip-compressed size
    /// and the denominator for the `file_uploaded_bytes` that follow. Sent once
    /// per file, immediately.
    FileStarted {
        name: String,
        source: String,
        total_bytes: u64,
    },
    /// Compressed bytes handed to the network so far: `uploaded_bytes` across
    /// the whole job, `file_uploaded_bytes` for the file opened by the last
    /// `FileStarted`. Counted as queued for sending, not as GOG acknowledges
    /// them, and never moves backwards — saves are not retried.
    /// Coalesced on a 150ms ticker.
    Progress {
        uploaded_bytes: u64,
        file_uploaded_bytes: u64,
    },
    /// The file was accepted by GOG. Sent once per file, immediately.
    FileFinished { name: String },
    /// Every file uploaded. Terminal.
    Finished,
}
