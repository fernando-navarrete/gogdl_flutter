//! Maps `gogdl-lib`'s nested error enums onto `GogErrorKind`, for `GogError::kind`. Kept outside
//! `api/` so frb doesn't list the helpers.

use gogdl_lib::{
    AuthError, ClientError, DepotError, DownloadError, GamesError, GogDlError, ProtonError,
    SavesError, SecureLinksError,
};

use crate::api::error::GogErrorKind;

pub(crate) fn gogdl(e: &GogDlError) -> GogErrorKind {
    match e {
        GogDlError::GameError(e) => games(e),
        GogDlError::DepotError(e) => depot(e),
        GogDlError::SecureLinksError(e) => secure_links(e),
        GogDlError::DownloadError(e) => download(e),
        GogDlError::ClientError(e) => client(e),
        GogDlError::ProtonError(e) => proton(e),
        GogDlError::SavesError(e) => saves(e),
        _ => GogErrorKind::Unknown,
    }
}

fn client(e: &ClientError) -> GogErrorKind {
    match e {
        ClientError::NetworkError(_)
        | ClientError::MaxRetriesReached
        | ClientError::AuthError(AuthError::NetworkError(_)) => GogErrorKind::Network,
        ClientError::HttpError { status, .. } => http(status.as_u16()),
        ClientError::AuthError(AuthError::NotAuthenticated | AuthError::TokenExpired) => {
            GogErrorKind::SessionExpired
        }
        _ => GogErrorKind::Unknown,
    }
}

fn http(status: u16) -> GogErrorKind {
    if status == 404 {
        GogErrorKind::NotFound
    } else {
        GogErrorKind::Unknown
    }
}

fn games(e: &GamesError) -> GogErrorKind {
    match e {
        // The downloads and saves methods used to say `BuildNotFound` for a DLC or a pack.
        GamesError::ProductNotAGame => GogErrorKind::NotFound,
        GamesError::ClientError(e) => client(e),
        _ => GogErrorKind::Unknown,
    }
}

fn depot(e: &DepotError) -> GogErrorKind {
    match e {
        DepotError::ClientError(e) => client(e),
        _ => GogErrorKind::Unknown,
    }
}

fn secure_links(e: &SecureLinksError) -> GogErrorKind {
    match e {
        SecureLinksError::GamesError(e) => games(e),
        SecureLinksError::ClientError(e) => client(e),
        _ => GogErrorKind::Unknown,
    }
}

fn download(e: &DownloadError) -> GogErrorKind {
    match e {
        DownloadError::GamesError(e) => games(e),
        DownloadError::DepotError(e) => depot(e),
        DownloadError::SecureLinksError(e) => secure_links(e),
        DownloadError::ClientError(e) => client(e),
        DownloadError::BuildNotFound => GogErrorKind::NotFound,
        DownloadError::NotEnoughFreeSpace { .. } => GogErrorKind::NotEnoughSpace,
        DownloadError::FileAllocationError
        | DownloadError::FileSystemError(_)
        | DownloadError::CouldNotResolveFreeSpace { .. } => GogErrorKind::FileSystem,
        DownloadError::ChunkHashMismatch { .. }
        | DownloadError::ChunkIntegrityCheckFailed(_)
        | DownloadError::DeflateError(_) => GogErrorKind::Integrity,
        _ => GogErrorKind::Unknown,
    }
}

fn proton(e: &ProtonError) -> GogErrorKind {
    match e {
        ProtonError::ClientError(e) => client(e),
        ProtonError::NotEnoughFreeSpace { .. } => GogErrorKind::NotEnoughSpace,
        ProtonError::CouldNotResolveFreeSpace { .. }
        | ProtonError::Io(_)
        | ProtonError::FileSystemError(_) => GogErrorKind::FileSystem,
        _ => GogErrorKind::Unknown,
    }
}

fn saves(e: &SavesError) -> GogErrorKind {
    match e {
        SavesError::ClientError(e) => client(e),
        SavesError::GamesError(e) => games(e),
        SavesError::DepotError(e) => depot(e),
        SavesError::BuildNotFound => GogErrorKind::NotFound,
        SavesError::CloudStorageNotSupported => GogErrorKind::CloudSavesUnsupported,
        SavesError::Io(_) | SavesError::FileSystemError(_) => GogErrorKind::FileSystem,
        SavesError::HashMismatch { .. } => GogErrorKind::Integrity,
        _ => GogErrorKind::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use std::io;
    use std::path::PathBuf;

    use gogdl_lib::FileSystemError;
    use reqwest::StatusCode;

    use super::*;
    use crate::api::error::GogError;

    fn network() -> reqwest::Error {
        gogdl_lib::Client::new()
            .get("not a url")
            .build()
            .unwrap_err()
    }

    fn io_error() -> io::Error {
        io::Error::other("disk")
    }

    fn http(status: u16) -> ClientError {
        ClientError::HttpError {
            status: StatusCode::from_u16(status).unwrap(),
            body: String::new(),
        }
    }

    fn download(e: DownloadError) -> GogError {
        GogError::GogdlError(GogDlError::DownloadError(e))
    }

    fn proton(e: ProtonError) -> GogError {
        GogError::GogdlError(GogDlError::ProtonError(e))
    }

    fn not_enough(required: u64, available: u64) -> DownloadError {
        DownloadError::NotEnoughFreeSpace {
            required,
            available,
        }
    }

    #[test]
    fn each_kind_maps_from_its_upstream_errors() {
        use GogErrorKind::*;
        let cases: Vec<(GogError, GogErrorKind)> = vec![
            // sessionExpired, nested as a download sees it and directly.
            (
                download(DownloadError::ClientError(ClientError::AuthError(
                    AuthError::TokenExpired,
                ))),
                SessionExpired,
            ),
            (
                GogError::GogdlError(GogDlError::ClientError(ClientError::AuthError(
                    AuthError::NotAuthenticated,
                ))),
                SessionExpired,
            ),
            (
                GogError::GogdlError(GogDlError::SavesError(SavesError::ClientError(
                    ClientError::AuthError(AuthError::TokenExpired),
                ))),
                SessionExpired,
            ),
            // network
            (
                GogError::GogdlError(GogDlError::ClientError(
                    ClientError::NetworkError(network()),
                )),
                Network,
            ),
            (
                download(DownloadError::ClientError(ClientError::MaxRetriesReached)),
                Network,
            ),
            (
                download(DownloadError::ClientError(ClientError::AuthError(
                    AuthError::NetworkError(network()),
                ))),
                Network,
            ),
            (
                proton(ProtonError::ClientError(ClientError::NetworkError(
                    network(),
                ))),
                Network,
            ),
            // notFound
            (download(DownloadError::BuildNotFound), NotFound),
            (download(DownloadError::ClientError(http(404))), NotFound),
            (
                download(DownloadError::SecureLinksError(
                    SecureLinksError::GamesError(GamesError::ClientError(http(404))),
                )),
                NotFound,
            ),
            (
                GogError::GogdlError(GogDlError::SavesError(SavesError::BuildNotFound)),
                NotFound,
            ),
            (
                download(DownloadError::GamesError(GamesError::ProductNotAGame)),
                NotFound,
            ),
            (
                GogError::GogdlError(GogDlError::SavesError(SavesError::GamesError(
                    GamesError::ProductNotAGame,
                ))),
                NotFound,
            ),
            // notEnoughSpace
            (download(not_enough(10, 3)), NotEnoughSpace),
            (
                proton(ProtonError::NotEnoughFreeSpace {
                    required: 10,
                    available: 3,
                }),
                NotEnoughSpace,
            ),
            // integrity
            (
                download(DownloadError::ChunkHashMismatch {
                    path: "a".into(),
                    offset: 0,
                    expected: "x".into(),
                    actual: "y".into(),
                }),
                Integrity,
            ),
            (
                download(DownloadError::ChunkIntegrityCheckFailed(2)),
                Integrity,
            ),
            (download(DownloadError::DeflateError(io_error())), Integrity),
            (
                GogError::GogdlError(GogDlError::SavesError(SavesError::HashMismatch {
                    name: "a".into(),
                    expected: "x".into(),
                    actual: "y".into(),
                })),
                Integrity,
            ),
            // fileSystem
            (download(DownloadError::FileAllocationError), FileSystem),
            (
                download(DownloadError::FileSystemError(
                    FileSystemError::FileRemovalError(io_error()),
                )),
                FileSystem,
            ),
            (
                download(DownloadError::CouldNotResolveFreeSpace {
                    path: PathBuf::from("/x"),
                }),
                FileSystem,
            ),
            (
                proton(ProtonError::CouldNotResolveFreeSpace {
                    path: PathBuf::from("/x"),
                }),
                FileSystem,
            ),
            (proton(ProtonError::Io(io_error())), FileSystem),
            (
                GogError::GogdlError(GogDlError::SavesError(SavesError::Io(io_error()))),
                FileSystem,
            ),
            // cloudSavesUnsupported
            (
                GogError::GogdlError(GogDlError::SavesError(SavesError::CloudStorageNotSupported)),
                CloudSavesUnsupported,
            ),
        ];
        for (error, kind) in cases {
            assert_eq!(error.kind(), kind, "{error:?}");
        }
    }

    #[test]
    fn a_saves_error_maps_like_the_same_error_from_a_download() {
        let revoked = || AuthError::SessionRevoked {
            description: "Invalid refresh token".into(),
        };
        let from_saves = GogError::GogdlError(GogDlError::SavesError(SavesError::ClientError(
            ClientError::AuthError(revoked()),
        )));
        let from_download = download(DownloadError::ClientError(
            ClientError::AuthError(revoked()),
        ));
        assert_eq!(from_saves.kind(), from_download.kind());
    }

    #[test]
    fn anything_unmapped_is_unknown() {
        let cases = [
            // The login kinds, until Step 4 maps them.
            download(DownloadError::ClientError(ClientError::AuthError(
                AuthError::SessionRevoked {
                    description: "Invalid refresh token".into(),
                },
            ))),
            download(DownloadError::ClientError(ClientError::AuthError(
                AuthError::InvalidLoginCode {
                    description: "Code doesn't exist".into(),
                },
            ))),
            download(DownloadError::ClientError(http(500))),
            download(DownloadError::ClientError(http(401))),
            proton(ProtonError::NoSuitableAsset("GE-Proton".into())),
            download(DownloadError::SecureLinksError(
                SecureLinksError::ProductNotOwned("1".into()),
            )),
        ];
        for error in cases {
            assert_eq!(error.kind(), GogErrorKind::Unknown, "{error:?}");
        }
    }

    #[test]
    fn space_shortfall_has_the_numbers_only_for_not_enough_space() {
        let shortfall = download(not_enough(10, 3)).space_shortfall().unwrap();
        assert_eq!(
            (shortfall.required_bytes, shortfall.available_bytes),
            (10, 3)
        );
        let shortfall = proton(ProtonError::NotEnoughFreeSpace {
            required: 7,
            available: 1,
        })
        .space_shortfall()
        .unwrap();
        assert_eq!(
            (shortfall.required_bytes, shortfall.available_bytes),
            (7, 1)
        );
        assert!(
            download(DownloadError::BuildNotFound)
                .space_shortfall()
                .is_none()
        );
    }
}
