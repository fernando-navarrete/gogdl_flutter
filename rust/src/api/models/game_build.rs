use flutter_rust_bridge::frb;

#[frb(non_opaque)]
pub struct GameBuild {
    pub build_id: String,
    pub version_name: String,
    pub release_date: String,
    pub release_date_timestamp: i64,
}

impl GameBuild {
    #[frb(ignore)]
    pub fn from_gogdl(build: &gogdl_lib::GameBuild) -> Self {
        Self {
            build_id: build.build_id.clone(),
            version_name: build.version_name.clone(),
            release_date: build.date_published.to_rfc2822(),
            release_date_timestamp: build.date_published.timestamp(),
        }
    }
}
