use flutter_rust_bridge::frb;

#[derive(Clone)]
#[frb(non_opaque)]
pub enum VerifyDownloadProgress {
    /// Total bytes to be checked, known before any chunk is read.
    Started(u64),
    /// Bytes checked so far, verified and failed alike. Coalesced on a
    /// 150ms ticker.
    Progress(u64),
    /// The file's relative path could not be resolved on disk. Sent once
    /// per path; the `u64` is the first failing chunk's size, not the
    /// file's.
    CouldNotResolvePath(String, u64),
    /// No file exists yet at the resolved path. Sent once per path.
    FileNotFound(String, u64),
    /// A chunk's on-disk MD5 didn't match the manifest. Sent once per path.
    ChecksumMismatch(String, u64),
    /// Verification ran to completion; the `u64` is how many chunks need
    /// re-downloading. Zero means the install is intact.
    Finished(u64),
}
