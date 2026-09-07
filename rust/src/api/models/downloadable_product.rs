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
