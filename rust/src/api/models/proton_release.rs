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

#[cfg(test)]
mod tests {
    use super::*;

    fn release(assets: &[(&str, u64)]) -> gogdl_lib::ProtonGeRelease {
        let assets: Vec<_> = assets
            .iter()
            .map(|(name, size)| {
                serde_json::json!({
                    "name": name,
                    "browser_download_url": format!("https://example.invalid/{name}"),
                    "size": size,
                })
            })
            .collect();
        serde_json::from_value(serde_json::json!({
            "url": "https://example.invalid/release",
            "tag_name": "GE-Proton11-6",
            "id": 42,
            "created_at": "2026-01-01T00:00:00Z",
            "updated_at": "2026-01-02T00:00:00Z",
            "published_at": "2026-01-03T00:00:00Z",
            "assets": assets,
        }))
        .unwrap()
    }

    #[test]
    fn uses_the_x86_tarball_size() {
        let mapped = ProtonRelease::from_gogdl(&release(&[
            ("GE-Proton11-6.sha512sum", 10),
            ("GE-Proton11-6-aarch64.tar.gz", 200),
            ("GE-Proton11-6.tar.gz", 300),
        ]))
        .unwrap();
        assert_eq!(mapped.tag_name, "GE-Proton11-6");
        assert_eq!(mapped.download_size, 300);
    }

    #[test]
    fn errors_when_no_asset_fits() {
        let err = ProtonRelease::from_gogdl(&release(&[
            ("GE-Proton11-6.sha512sum", 10),
            ("GE-Proton11-6-aarch64.tar.gz", 200),
        ]))
        .err()
        .unwrap();
        assert!(matches!(err, ProtonError::NoSuitableAsset(tag) if tag == "GE-Proton11-6"));
    }
}
