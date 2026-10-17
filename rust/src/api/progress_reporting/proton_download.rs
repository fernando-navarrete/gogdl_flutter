use flutter_rust_bridge::frb;

#[derive(Clone)]
#[cfg_attr(test, derive(Debug, PartialEq))]
#[frb(non_opaque)]
pub enum ProtonDownloadProgress {
    /// Sent once, before the first byte is read. `total_bytes` is the release
    /// tarball's *compressed* size, as GitHub reports it, and the denominator
    /// for `Progress.downloaded_bytes`.
    Started { total_bytes: u64 },
    /// Compressed bytes received so far, out of `Started.total_bytes`. The
    /// tarball is extracted as it streams in, so this isn't an on-disk size.
    /// Coalesced on a 150ms ticker.
    Progress { downloaded_bytes: u64 },
    /// An entry was extracted, sent after the tick's `Progress`. `path` is
    /// relative to the download directory, in the archive's own layout. Only
    /// the *last* entry per 150ms tick is sent, so this is for a status line,
    /// not a list of the files.
    Extracted { path: String },
    /// Absolute path of the extracted release directory, `path/<tag_name>`
    /// with the tag sanitized for the filesystem. Not necessarily the
    /// directory name inside the tarball — see `Extracted`, whose paths
    /// reflect the archive's own layout, before the rename. Terminal.
    Finished { path: String },
    /// The job was cancelled with its `CancelToken`. `path` is as it was
    /// before the download: the release is extracted into a hidden staging
    /// directory, which the next download into `path` cleans up. Terminal.
    Cancelled,
}
