use flutter_rust_bridge::frb;

/// The image links of a game, from one request.
#[frb(non_opaque)]
pub struct GameLinks {
    /// The box art image URL.
    pub boxart: String,
    /// The background image URL.
    pub background: String,
}

impl GameLinks {
    #[frb(ignore)]
    pub fn from_gogdl(links: &gogdl_lib::GameLinks) -> Self {
        Self {
            boxart: links.links.box_art_image.href.clone(),
            background: links.links.background_image.href.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copies_boxart_and_background() {
        let upstream: gogdl_lib::GameLinks = serde_json::from_str(
            r#"{"_links": {
                "boxArtImage": {"href": "https://img/box.jpg"},
                "backgroundImage": {"href": "https://img/bg.jpg"},
                "galaxyBackgroundImage": {"href": "https://img/galaxy.jpg"}
            }}"#,
        )
        .unwrap();
        let links = GameLinks::from_gogdl(&upstream);
        assert_eq!(links.boxart, "https://img/box.jpg");
        assert_eq!(links.background, "https://img/bg.jpg");
    }
}
