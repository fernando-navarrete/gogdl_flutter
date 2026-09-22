use flutter_rust_bridge::frb;

#[derive(Clone)]
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
}
