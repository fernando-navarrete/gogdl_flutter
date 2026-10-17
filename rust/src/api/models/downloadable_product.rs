use flutter_rust_bridge::frb;

/// What GOG says a downloadable product is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProductType {
    Game,
    Dlc,
    /// Anything else, such as a pack or a type GOG adds later. The raw string isn't kept.
    Other,
}

impl ProductType {
    /// GOG sends `"GAME"` and `"DLC"`; the match ignores case.
    #[frb(ignore)]
    pub fn from_gogdl(product_type: &str) -> Self {
        if product_type.eq_ignore_ascii_case("game") {
            Self::Game
        } else if product_type.eq_ignore_ascii_case("dlc") {
            Self::Dlc
        } else {
            Self::Other
        }
    }
}

#[frb(non_opaque)]
pub struct DownloadableProduct {
    pub id: i32,
    pub name: String,
    pub product_type: ProductType,
}

impl DownloadableProduct {
    #[frb(ignore)]
    pub fn from_gogdl(downloadable_product: &gogdl_lib::ProductDetails) -> Self {
        let product_type = downloadable_product.get_product_type();
        let product_id = downloadable_product.get_product_id();
        Self {
            id: product_id,
            name: downloadable_product.title.clone(),
            product_type: ProductType::from_gogdl(&product_type),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn details(product_type: &str) -> gogdl_lib::ProductDetails {
        let mut details: gogdl_lib::ProductDetails = serde_json::from_str(&format!(
            r#"{{"_embedded": {{"productType": "{product_type}", "product": {{"id": 1207658924, "title": "Some DLC"}}}}}}"#
        ))
        .unwrap();
        // `title` is skipped by serde; `get_product_details` fills it in
        details.title = "Some DLC".into();
        details
    }

    #[test]
    fn maps_id_title_and_product_type() {
        let mapped = DownloadableProduct::from_gogdl(&details("dlc"));
        assert_eq!(mapped.id, 1_207_658_924);
        assert_eq!(mapped.name, "Some DLC");
        assert_eq!(mapped.product_type, ProductType::Dlc);
    }

    #[test]
    fn product_type_ignores_case() {
        assert_eq!(
            DownloadableProduct::from_gogdl(&details("GAME")).product_type,
            ProductType::Game
        );
        assert_eq!(
            DownloadableProduct::from_gogdl(&details("DLC")).product_type,
            ProductType::Dlc
        );
    }

    #[test]
    fn unknown_product_type_is_other() {
        assert_eq!(
            DownloadableProduct::from_gogdl(&details("PACK")).product_type,
            ProductType::Other
        );
    }
}
