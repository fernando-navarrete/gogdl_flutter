use flutter_rust_bridge::frb;

#[frb(non_opaque)]
pub enum ProtonDownloadProgress {
    Started(u64),
    Progress(u64),
    Extracted(String),
}
