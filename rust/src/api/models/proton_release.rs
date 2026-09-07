use flutter_rust_bridge::frb;
use gogdl_lib::ProtonError;

#[frb(non_opaque)]
pub struct ProtonRelease {
    pub tag_name: String,
    pub download_size: u64,
}

impl ProtonRelease {
    #[frb(ignore)]
    pub fn from_gogdl(release: &gogdl_lib::ProtonGeRelease) -> Result<Self, ProtonError> {
        let download_size = release.get_release_size()?;
        Ok(Self {
            tag_name: release.tag_name.clone(),
            download_size,
        })
    }
}
