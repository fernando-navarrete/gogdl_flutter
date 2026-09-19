use flutter_rust_bridge::frb;
use gogdl_lib::{GogDlError, SavesError};

#[derive(Debug, thiserror::Error)]
pub enum GogError {
    #[error("{0}")]
    GogdlError(#[from] GogDlError),
    #[error("{0}")]
    SavesError(#[from] SavesError),
}

impl GogError {
    #[frb(sync)]
    pub fn message(&self) -> String {
        self.to_string()
    }
}
