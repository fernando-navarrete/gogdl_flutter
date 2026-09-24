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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_the_build_and_formats_the_release_date() {
        let build: gogdl_lib::GameBuild = serde_json::from_str(
            r#"{
                "build_id": "58123",
                "version_name": "1.2.3",
                "date_published": "2024-03-01T12:00:00Z",
                "link": "https://example.invalid/depot"
            }"#,
        )
        .unwrap();
        let mapped = GameBuild::from_gogdl(&build);
        assert_eq!(mapped.build_id, "58123");
        assert_eq!(mapped.version_name, "1.2.3");
        assert_eq!(mapped.release_date, "Fri, 1 Mar 2024 12:00:00 +0000");
        assert_eq!(mapped.release_date_timestamp, 1_709_294_400);
    }
}
