use flutter_rust_bridge::frb;

#[frb(non_opaque)]
pub struct DownloadableProduct {
    pub id: i32,
    pub name: String,
    pub product_type: String,
}

impl DownloadableProduct {
    #[frb(ignore)]
    pub fn from_gogdl(downloadable_product: &gogdl_lib::ProductDetails) -> Self {
        let product_type = downloadable_product.get_product_type();
        let product_id = downloadable_product.get_product_id();
        Self {
            id: product_id,
            name: downloadable_product.title.clone(),
            product_type: product_type.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_id_title_and_product_type() {
        let mut details: gogdl_lib::ProductDetails = serde_json::from_str(
            r#"{"_embedded": {"productType": "dlc", "product": {"id": 1207658924, "title": "Some DLC"}}}"#,
        )
        .unwrap();
        // `title` is skipped by serde; `get_product_details` fills it in
        details.title = "Some DLC".into();
        let mapped = DownloadableProduct::from_gogdl(&details);
        assert_eq!(mapped.id, 1_207_658_924);
        assert_eq!(mapped.name, "Some DLC");
        assert_eq!(mapped.product_type, "dlc");
    }
}
