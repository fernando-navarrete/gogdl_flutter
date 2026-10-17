use flutter_rust_bridge::frb;
use gogdl_lib::{DownloadError, GogDlError, ProtonError};

use crate::error_kind;

#[derive(Debug, thiserror::Error)]
pub enum GogError {
    #[error("{0}")]
    GogdlError(#[from] GogDlError),
    /// An argument the bridge rejected before calling `gogdl-lib`.
    #[error("invalid argument: {0}")]
    InvalidArgument(String),
}

/// What kind of failure a `GogError` is, for code that needs to react to it rather than show
/// `message()`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GogErrorKind {
    /// Not logged in, or the session expired and couldn't be refreshed.
    SessionExpired,
    /// A request failed or timed out, including after retrying.
    Network,
    /// The build, or the resource a request asked for, doesn't exist (HTTP 404).
    NotFound,
    /// Not enough free space at the install path. `spaceShortfall()` has the numbers.
    NotEnoughSpace,
    /// Downloaded data didn't match its checksum or couldn't be decompressed.
    Integrity,
    /// A file couldn't be created, allocated or written, or the path's disk couldn't be found.
    FileSystem,
    /// The game doesn't support cloud saves.
    CloudSavesUnsupported,
    /// Anything else. `message()` has the details.
    Unknown,
}

/// How much free space a job needed, and how much there was, in bytes.
#[frb(non_opaque)]
pub struct SpaceShortfall {
    pub required_bytes: u64,
    pub available_bytes: u64,
}

impl GogError {
    #[frb(sync)]
    pub fn message(&self) -> String {
        self.to_string()
    }

    /// The error's kind. A rejected argument (`InvalidArgument`) is `unknown`. A revoked session (a refresh token GOG no longer accepts) is `unknown`,
    /// not `sessionExpired`, since `gogdl-lib` doesn't tell it apart from other refresh failures
    /// yet (upstream `v1.5.0`).
    #[frb(sync)]
    pub fn kind(&self) -> GogErrorKind {
        match self {
            GogError::GogdlError(e) => error_kind::gogdl(e),
            GogError::InvalidArgument(_) => GogErrorKind::Unknown,
        }
    }

    /// The required and available bytes when `kind()` is `notEnoughSpace`, and null otherwise.
    #[frb(sync)]
    pub fn space_shortfall(&self) -> Option<SpaceShortfall> {
        match self {
            GogError::GogdlError(GogDlError::DownloadError(
                DownloadError::NotEnoughFreeSpace {
                    required,
                    available,
                },
            ))
            | GogError::GogdlError(GogDlError::ProtonError(ProtonError::NotEnoughFreeSpace {
                required,
                available,
            })) => Some(SpaceShortfall {
                required_bytes: *required,
                available_bytes: *available,
            }),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_argument_is_unknown_and_names_the_argument() {
        let e = GogError::InvalidArgument("page must not be negative, got -1".into());
        assert_eq!(e.kind(), GogErrorKind::Unknown);
        assert!(e.message().contains("page"));
    }
}
