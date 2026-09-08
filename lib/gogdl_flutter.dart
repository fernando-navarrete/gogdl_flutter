library;

export 'src/rust/frb_generated.dart' show RustLib;
export 'src/rust/api/gogdl.dart' show GogdlApi;
export 'src/rust/api/models/game_build.dart' show GameBuild;
export 'src/rust/api/models/downloadable_product.dart' show DownloadableProduct;
export 'src/rust/api/models/proton_release.dart' show ProtonRelease;
export 'src/rust/api/error.dart' show GogError;
export 'src/rust/api/progress_reporting/proton_download.dart'
    show
        ProtonDownloadProgress,
        ProtonDownloadProgress_Started,
        ProtonDownloadProgress_Progress,
        ProtonDownloadProgress_Extracted,
        ProtonDownloadProgressPatterns;
export 'src/rust/api/progress_reporting/verify_download.dart'
    show
        VerifyDownloadProgress,
        VerifyDownloadProgress_Started,
        VerifyDownloadProgress_Progress,
        VerifyDownloadProgress_CouldNotResolvePath,
        VerifyDownloadProgress_FileNotFound,
        VerifyDownloadProgress_ChecksumMismatch,
        VerifyDownloadProgress_Finished,
        VerifyDownloadProgressPatterns;
export 'src/rust/api/progress_reporting/game_download.dart'
    show
        DownloadGameProgress,
        DownloadGameProgress_Started,
        DownloadGameProgress_FileSizeVerification,
        DownloadGameProgress_FileAllocationStarted,
        DownloadGameProgress_FileAllocation,
        DownloadGameProgress_AllocationError,
        DownloadGameProgress_DownloadProgress,
        DownloadGameProgress_Finished,
        DownloadGameProgressPatterns;
