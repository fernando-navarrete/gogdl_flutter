use flutter_rust_bridge::frb;

/// What `removeProducts` deleted. It counts only what that call removed: files that were already
/// gone, and files a product being kept also lists, are in neither number.
#[frb(non_opaque)]
pub struct ProductRemoval {
    /// Files deleted from the install.
    pub files_removed: u64,
    /// Their combined size on disk, in bytes.
    pub bytes_freed: u64,
}

impl ProductRemoval {
    #[frb(ignore)]
    pub fn from_gogdl(removal: &gogdl_lib::ProductRemoval) -> Self {
        Self {
            files_removed: removal.files_removed,
            bytes_freed: removal.bytes_freed,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copies_both_counts() {
        let removal = ProductRemoval::from_gogdl(&gogdl_lib::ProductRemoval {
            files_removed: 12,
            bytes_freed: 3_456,
        });
        assert_eq!(removal.files_removed, 12);
        assert_eq!(removal.bytes_freed, 3_456);
    }

    #[test]
    fn the_default_is_zero() {
        let removal = ProductRemoval::from_gogdl(&gogdl_lib::ProductRemoval::default());
        assert_eq!((removal.files_removed, removal.bytes_freed), (0, 0));
    }
}
