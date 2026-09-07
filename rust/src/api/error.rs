use gogdl_lib::GogDlError;

#[derive(Debug, thiserror::Error)]
pub enum GogError {
    #[error("{0}")]
    GogdlError(#[from] GogDlError),
}
