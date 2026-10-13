use flutter_rust_bridge::frb;
use gogdl_lib::ProtonError;

#[frb(non_opaque)]
pub struct ProtonRelease {
    pub tag_name: String,
    pub download_size: u64,
}

/// One page of Proton-GE releases.
#[frb(non_opaque)]
pub struct ProtonReleasePage {
    /// The page's releases, newest first, without those that have no suitable asset.
    pub releases: Vec<ProtonRelease>,
    /// Whether there may be a next page: true when GitHub returned a full page, even if every
    /// release on it was left out of `releases`.
    pub has_more: bool,
}

impl ProtonReleasePage {
    #[frb(ignore)]
    pub fn from_gogdl(page: &gogdl_lib::ProtonGeReleasesPage, per_page: u32) -> Self {
        let releases = page.releases();
        Self {
            releases: releases
                .iter()
                .filter_map(|release| ProtonRelease::from_gogdl(release).ok())
                .collect(),
            has_more: releases.len() >= per_page as usize,
        }
    }
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

    fn page(releases: &[serde_json::Value]) -> gogdl_lib::ProtonGeReleasesPage {
        serde_json::from_value(serde_json::Value::Array(releases.to_vec())).unwrap()
    }

    fn raw(tag: &str, assets: &[(&str, u64)]) -> serde_json::Value {
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
        serde_json::json!({
            "url": "https://example.invalid/release",
            "tag_name": tag,
            "id": 42,
            "created_at": "2026-01-01T00:00:00Z",
            "updated_at": "2026-01-02T00:00:00Z",
            "published_at": "2026-01-03T00:00:00Z",
            "assets": assets,
        })
    }

    #[test]
    fn a_full_page_of_filtered_out_releases_has_more() {
        let none = [("x.sha512sum", 1)];
        let upstream = page(
            &(0..5)
                .map(|i| raw(&format!("t{i}"), &none))
                .collect::<Vec<_>>(),
        );
        let mapped = ProtonReleasePage::from_gogdl(&upstream, 5);
        assert!(mapped.releases.is_empty());
        assert!(mapped.has_more);
    }

    #[test]
    fn a_short_page_has_no_more_and_keeps_order() {
        let ok = [("a.tar.gz", 7)];
        let upstream = page(&[raw("t1", &ok), raw("t2", &[]), raw("t3", &ok)]);
        let mapped = ProtonReleasePage::from_gogdl(&upstream, 5);
        let tags: Vec<_> = mapped
            .releases
            .iter()
            .map(|r| r.tag_name.as_str())
            .collect();
        assert_eq!(tags, ["t1", "t3"]);
        assert!(!mapped.has_more);
    }
}
