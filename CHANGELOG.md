## 1.3.1

No change to the Dart API or to the progress events Dart receives.

* `rustls` 0.23.43 → 0.23.45 in `rust/Cargo.lock`, for RUSTSEC-2026-0285. Lumen: pin `v1.3.1` and
  drop the `RUSTSEC-2026-0285` entry from its `osv-scanner.toml`.
* Other semver-compatible lock bumps found by the new `audit` job: `anyhow` 1.0.104
  (RUSTSEC-2026-0190), `backtrace` 0.3.76 (`adler2` replaces the unmaintained `adler`) and
  `unicode-ident` 1.0.26 (now `Unicode-3.0`).
* CI runs on merge requests and `main`, with every image pinned by digest. New check jobs: `audit`
  (`cargo deny`, `rust/deny.toml`), `scan` (OSV-Scanner on both lock files, also weekly) and
  `secrets` (gitleaks). `dart` and `integration` check `example/pubspec.lock`'s sources and run
  `pub get --enforce-lockfile`. `release` waits for every check job and needs the tag on `main`.
* `SECURITY.md`: what a new crate or pub package has to meet, and what to do if one is compromised.
  `rust/Cargo.toml` is `publish = false`.

## 1.3.0

Download management for Lumen `v1.3.0`: cancelling jobs, error kinds, install size and free space,
and request timeouts. **Breaking**: ships together with the Lumen change that adopts it.

### Breaking

* The six job methods (`downloadGame`, `repairGame`, `verifyDownload`, `downloadProtonRelease`,
  `downloadSaveFiles`, `uploadSaveFiles`) take a required `cancel: CancelToken`. Create one with
  `CancelToken()` and stop the job with `cancel()`, which is sync and idempotent. A token that's
  already cancelled when the job starts cancels it before it does anything.
* Each progress enum has a `Cancelled` variant (`DownloadGameProgress_Cancelled`, and so on). On a
  cancel, the progress collected so far is flushed, `Cancelled` is sent last, and the stream closes
  with no error. No `Finished` follows. A cancel while the job is still fetching what it needs
  (the product bundles, the Proton release) ends the same way and writes nothing to disk. A cancel
  that races a job that has already finished does nothing, and the job's own result comes through.
* What Lumen changes: pass a `CancelToken` to each job, handle `*_Cancelled` in its exhaustive
  `switch`es, and switch on `GogError.kind()` instead of showing only `message()`.

### Added

* `GogError.kind()` returns a `GogErrorKind`: `sessionExpired`, `network`, `notFound`,
  `notEnoughSpace`, `integrity`, `fileSystem`, `cloudSavesUnsupported` or `unknown`. Lumen's
  planned D7 names map onto these: `notEnoughFreeSpace` is `notEnoughSpace`,
  `couldNotResolveFreeSpace` and `fileAllocation` are `fileSystem`, `network` is `network`, `auth`
  is `sessionExpired`, and `other` is `unknown`. A revoked session is `unknown`, not
  `sessionExpired`, until the login error kinds in `v1.5.0`.
* `GogError.spaceShortfall()` returns a `SpaceShortfall` (`requiredBytes`, `availableBytes`) for
  `notEnoughSpace`, and null otherwise.
* `GogdlApi.getInstallSize(gameId, buildName, selectedProducts)` returns an `InstallSize`
  (`downloadBytes`, `diskBytes` and a `ProductSize` per selected product), from the build metadata
  with no manifest fetches. The sums equal `downloadGame`'s and `verifyDownload`'s totals.
* `GogdlApi.getFreeSpace(path)`: the free space the download's own check compares against. `path`
  needn't exist.

### Changed

* A job stops when its Dart subscription is cancelled, at its next progress update (normally
  within one 150ms tick). It used to keep running with nobody listening.
* Requests time out: 10s to connect, and 30s without reading anything. There's no overall cap, so
  long transfers aren't cut off. Game downloads and metadata requests retry a timeout. Proton
  downloads, saves transfers and login fail on the first one, and a save upload must finish within
  30s per file.
* The Dart docs say what cancelling leaves behind: a game download or repair leaves state that
  `repairGame` resumes, a Proton download leaves `path` as it was, a saves download keeps the old
  save or the complete new one, and a cancelled saves upload may or may not have stored the file in
  flight (upload again).
* `repairGame` is documented as the resume path for a cancelled, failed or interrupted install.
  `downloadGame` always starts over and re-downloads everything.
* Updated `gogdl-lib` from `v1.0.11` to `v1.2.3`. Behavior changes that come with it: token-exchange
  errors no longer carry refresh tokens, the token expiry margin is refreshed, an unauthenticated
  401 no longer loops, an unowned product fails at once instead of retrying, `repairGame` skips
  checksumming chunks it has just allocated, and the free-space check picks the longest matching
  mount point (so it can fail where it used to pass).

## 1.2.4

No change to the Dart API or to the progress events Dart receives.

* Tightened the Dart lints on top of `flutter_lints`: `strict-casts`, `strict-inference` and
  `strict-raw-types`, plus rules for unawaited futures, missing types, `dynamic` calls, unclosed
  streams and subscriptions, single quotes, `final` locals and sorted directives and dependencies.
  `example/` inherits the root `analysis_options.yaml`.
* Generated code (`lib/src/rust/**`, `*.freezed.dart`) is excluded from analysis, so a regenerate
  never trips a lint.
* Sorted the exports in `lib/gogdl_flutter.dart` and the dependencies in both `pubspec.yaml` files.

## 1.2.3

No change to the Dart API or to the progress events Dart receives.

* Linux is the only declared platform: removed the example's `android/`, `ios/`, `macos/` and
  `windows/` dirs and its unused `cupertino_icons` dependency.
* Stopped generating frb's web output (`web: false`), which removes `frb_generated.web.dart` and the
  wasm section of `rust/src/frb_generated.rs`.

## 1.2.2

No change to the Dart API or to the progress events Dart receives.

* Replaced the template README, `CHANGELOG.md` and `pubspec.yaml` text: the README now describes the
  bridge, how it is built, how to regenerate the bindings, and CI and releases.
* Dropped the unused `ffi` and `ffigen` dev dependencies, and marked the package `publish_to: none`.

## 1.2.1

No change to the Dart API or to the progress events Dart receives.

* Migrated the native build from the vendored cargokit to Native Assets: a Dart build hook
  (`hook/build.dart`, through `flutter_rust_bridge_hooks`) runs cargo and bundles the library.
  Removed `cargokit/` and the root `android/`, `ios/`, `linux/`, `macos/` and `windows/` dirs.
* Pinned the Rust toolchain to `1.98.1` in `rust/rust-toolchain.toml`. Consumers need rustup.
* CI image and `.toolchain` check now use the pinned Rust version.
* Precompiled binaries are deferred to `v1.5.0` (Native Assets has no support for them).

## 1.2.0

No change to the Dart API or to the progress events Dart receives.

* Fixed the example app and its integration test: an offline smoke test that loads the native
  library, constructs `GogdlApi` and checks `getLoginLink()` returns a GOG URL.
* Moved each job's progress translation out of `gogdl.rs` into plain state machines under
  `rust/src/progress/`, driven by one shared 150ms tick loop.
* Added Rust unit tests for the progress translators, the tick loop and the model adapters.
* Added GitLab CI (tags and manual runs): `rust` (fmt, clippy, test), `dart` (analyze), `codegen`
  (fails on generated-bindings drift) and `integration` (Linux, under xvfb), in a prebuilt image.
* Added a release stage on `vX.Y.Z` tags: it checks that the versions and lock files match the tag,
  and creates a GitLab release from this changelog.
* Fixed the clippy warning and the `cargo fmt` drift.

## 1.1.3

* Updated `gogdl-lib` to `v1.0.11`.

## 1.1.2

* Added the `Finished` download progress event. Updated `gogdl-lib` to `v1.0.10`.

## 1.1.1

* Save downloads and uploads take separate prefix and installation paths. Updated `gogdl-lib` to
  `v1.0.9`.

## 1.1.0

* Added `GogdlApi.downloadSaveFiles` and `GogdlApi.uploadSaveFiles`, which stream
  `DownloadSavesProgress` and `UploadSavesProgress`. `GogError` gained a second variant for save
  errors. Updated `gogdl-lib` to `v1.0.8`.

## 1.0.0

* First release.
