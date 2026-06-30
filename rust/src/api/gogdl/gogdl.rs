use flutter_rust_bridge::frb;
use gogdl_lib2::{Client, GogDl};

#[frb(opaque)]
pub struct Gog {
    inner: GogDl,
}

impl Gog {
    #[frb(sync)]
    pub fn new() -> Self {
        let client = Client::new();
        let gogdl = GogDl::new_from_client(client);
        Gog { inner: gogdl }
    }

    #[frb(sync)]
    pub fn get_login_url(&self) -> String {
        self.inner.get_login_url().to_string()
    }
}
