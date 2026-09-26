use flutter_rust_bridge::frb;

/// The sizes of the selected products of a build, in bytes.
#[frb(non_opaque)]
pub struct InstallSize {
    /// What the download transfers, compressed.
    pub download_bytes: u64,
    /// The size on disk once installed. The download's own free-space check compares this.
    pub disk_bytes: u64,
    /// One entry per selected product in the build, ordered by `id`.
    pub products: Vec<ProductSize>,
}

/// The size of one product (the base game or a DLC), in bytes.
#[frb(non_opaque)]
pub struct ProductSize {
    pub id: i32,
    /// What the download transfers, compressed.
    pub download_bytes: u64,
    /// The size on disk once installed.
    pub disk_bytes: u64,
}

impl InstallSize {
    /// Keeps the products in `selected_products`, matched the way `get_product_bundles` matches
    /// them (a `product_id` that isn't an `i32` is never selected), so the totals equal what
    /// `download_game` and `verify_download` sum.
    #[frb(ignore)]
    pub fn from_gogdl(sizes: &[gogdl_lib::ProductSize], selected_products: &[i32]) -> Self {
        let products: Vec<ProductSize> = sizes
            .iter()
            .filter_map(|size| {
                let id = size.product_id.parse::<i32>().ok()?;
                selected_products.contains(&id).then_some(ProductSize {
                    id,
                    download_bytes: size.compressed_size,
                    disk_bytes: size.size,
                })
            })
            .collect();
        Self {
            download_bytes: products.iter().map(|p| p.download_bytes).sum(),
            disk_bytes: products.iter().map(|p| p.disk_bytes).sum(),
            products,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn size(product_id: &str, size: u64, compressed_size: u64) -> gogdl_lib::ProductSize {
        gogdl_lib::ProductSize {
            product_id: product_id.into(),
            size,
            compressed_size,
        }
    }

    fn sizes() -> Vec<gogdl_lib::ProductSize> {
        vec![
            size("10", 100, 60),
            size("20", 200, 150),
            size("30", 300, 250),
        ]
    }

    fn ids(install: &InstallSize) -> Vec<(i32, u64, u64)> {
        install
            .products
            .iter()
            .map(|p| (p.id, p.download_bytes, p.disk_bytes))
            .collect()
    }

    #[test]
    fn keeps_the_selected_products_in_order_and_sums_them() {
        let install = InstallSize::from_gogdl(&sizes(), &[30, 10]);
        assert_eq!(ids(&install), vec![(10, 60, 100), (30, 250, 300)]);
        assert_eq!(install.download_bytes, 310);
        assert_eq!(install.disk_bytes, 400);
    }

    #[test]
    fn a_product_selected_twice_counts_once() {
        let install = InstallSize::from_gogdl(&sizes(), &[20, 20]);
        assert_eq!(ids(&install), vec![(20, 150, 200)]);
        assert_eq!(install.download_bytes, 150);
        assert_eq!(install.disk_bytes, 200);
    }

    #[test]
    fn unknown_selected_ids_and_non_numeric_product_ids_are_ignored() {
        let mut sizes = sizes();
        sizes.push(size("not-a-number", 1_000, 1_000));
        let install = InstallSize::from_gogdl(&sizes, &[10, 99]);
        assert_eq!(ids(&install), vec![(10, 60, 100)]);
        assert_eq!(install.download_bytes, 60);
        assert_eq!(install.disk_bytes, 100);
    }

    #[test]
    fn an_empty_selection_is_zero() {
        let install = InstallSize::from_gogdl(&sizes(), &[]);
        assert!(install.products.is_empty());
        assert_eq!(install.download_bytes, 0);
        assert_eq!(install.disk_bytes, 0);
    }
}
