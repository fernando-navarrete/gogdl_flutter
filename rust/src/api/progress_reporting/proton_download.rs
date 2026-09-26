use flutter_rust_bridge::frb;

#[derive(Clone)]
#[cfg_attr(test, derive(Debug, PartialEq))]
#[frb(non_opaque)]
pub enum ProtonDownloadProgress {
    Started(u64),
    Progress(u64),
    Extracted(String),
    /// Absolute path of the extracted release directory, `path/<tag_name>`
    /// with the tag sanitized for the filesystem. Not necessarily the
    /// directory name inside the tarball — see `Extracted`, whose paths
    /// reflect the archive's own layout, before the rename.
    Finished(String),
    /// The job was cancelled with its `CancelToken`. `path` is as it was
    /// before the download: the release is extracted into a hidden staging
    /// directory, which the next download into `path` cleans up. Terminal.
    Cancelled,
}
