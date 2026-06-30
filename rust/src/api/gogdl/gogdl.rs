use flutter_rust_bridge::frb;
use gogdl_lib2::{Client, GogDl, GogDlError};
use thiserror::Error;

#[frb(opaque)]
pub struct Gog {
    inner: GogDl,
}

#[derive(Error, Debug)]
pub enum GogError {
    #[error("gogdl error: {0}")]
    GogDlError(#[from] GogDlError),
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

    pub async fn login_with_code(&self, code: String) -> Result<(), GogError> {
        let _ = self.inner.login_with_code(&code).await;
        Ok(())
    }
}
