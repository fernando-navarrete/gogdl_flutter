## 1.5.0

Login error kinds and API cleanup for Lumen `v1.5.0`. **Breaking**: ships together with the Lumen
change that adopts it (its Phase 7).

### Breaking

* Every progress enum uses named fields; no variant is positional any more. Old -> new:
  * `ProtonDownloadProgress`: `started(field0)` -> `started(totalBytes:)`, `progress(field0)` ->
    `progress(downloadedBytes:)`, `extracted(field0)` -> `extracted(path:)`, `finished(field0)` ->
    `finished(path:)`. `extracted` is documented now: only the last path per 150ms tick is sent,
    so it's for a status line, not a list of files.
  * `VerifyDownloadProgress`: `started(field0)` -> `started(totalBytes:)`, `progress(field0)` ->
    `progress(checkedBytes:)`, `couldNotResolvePath`, `fileNotFound` and `checksumMismatch`
    `(field0, field1)` -> `(path:, chunkSize:)`, `finished(field0)` -> `finished(badChunks:)`.
  * `RepairGameProgress`: `allocationError(field0)` -> `allocationError(path:)`, and the same
    three path variants as verify, with `(path:, chunkSize:)`.
  * `DownloadGameProgress`: `allocationError(field0)` -> `allocationError(path:)`.
  * `chunkSize` is the first failing chunk's uncompressed size, not the file's.
* `GameBuild.releaseDate` (the RFC 2822 string) is removed. Use `releaseDateTimestamp` (Unix
  seconds, UTC).
* `DownloadableProduct.productType` is a `ProductType` (`game`, `dlc` or `other`) instead of a
  `String`. GOG's value is compared case-insensitively, so `"GAME"` is `game`. `other` is anything
  else (a pack, a type GOG adds later) and drops the raw string. `ProductType` is exported from
  `gogdl_flutter.dart`.
* Removed: `getGameBoxartLink` and `getBackgroundImageLink` (use `getGameLinks`, `v1.4.2`), and
  `getProtonReleases` (use `getProtonReleasePage`, `v1.4.3`).
* New `GogErrorKind.invalidLoginCode`: `loginWithCode` fails with it when GOG rejects the code,
  whether it's mistyped, truncated, already used or expired (GOG doesn't tell them apart;
  `message()` has its description). A revoked session (a refresh token GOG no longer accepts) is
  `sessionExpired`, where it was `unknown`, and a network failure on login is `network`, where it
  was `unknown`. A Dart `switch` over `GogErrorKind` needs the new case.
* What Lumen changes: the `field0`/`field1` uses in `downloads_state.dart` and `proton_state.dart`,
  the `productType` label and `== "GAME"` filter, `GameBuild.releaseDate`, and an
  `invalidLoginCode` case on the login page.

### Changed

* `gogdl-lib` `v1.3.2` -> `v1.4.3` (`ca89691`). Its errors are classified by what GOG answered, and
  the `Preparing`/`Prepared` download events are gone (they never reached Dart).
* `repairGame` keeps its signature and `RepairGameProgress`, and calls upstream's `download_game`,
  since upstream deprecated `repair_game`. Whether it stays past `v1.6.0` is open.
* `GogError`'s separate saves variant is gone: upstream's saves methods return its main error type.
  `GogError` is opaque in Dart, so the only visible effect is that a saves failure gets the same
  kinds as any other call.
* Lumen: pin `v1.5.0`.

## 1.4.3

Proton pages. **Not breaking**: everything is added.

* `getProtonReleasePage(page)` returns a `ProtonReleasePage` with `releases` and `hasMore`.
  `hasMore` is true when GitHub returned a full page of 5, even if every release on it was left
  out for having no suitable asset, so a fully filtered page no longer looks like the end of the
  list. `ProtonReleasePage` is exported from `gogdl_flutter.dart`.
* `getProtonReleases` is deprecated and removed in `v1.5.0`. flutter_rust_bridge doesn't turn
  `#[deprecated]` into `@Deprecated`, so the deprecation is only in its doc comment: the analyzer
  won't warn.
* A negative `page` fails instead of wrapping around to a huge page number, in both methods. It's
  a new `GogError` variant (`InvalidArgument`) whose `kind()` is `unknown`, so `GogErrorKind`
  doesn't change.
* Lumen: pin `v1.4.3`.

## 1.4.2

`getGameLinks()`. **Not breaking**: everything is added.

* `getGameLinks(gameId)` returns a `GameLinks` with `boxart` and `background` from one request,
  where the two old getters made one request each.
* `getGameBoxartLink` and `getBackgroundImageLink` are deprecated and removed in `v1.5.0`.
  flutter_rust_bridge doesn't turn `#[deprecated]` into Dart's `@Deprecated`, so the deprecation is
  only in their doc comments: the analyzer won't warn.
* Lumen: pin `v1.4.2`.

## 1.4.1

Faster, more tolerant `getDownloadableProducts()`. **Not breaking**: same signature and types.

* Product details are fetched 8 at a time instead of one by one, in upstream's order. Alien: Isolation
  (8 products) took 755 ms, down from 2.5 s.
* A product whose lookup fails is left out, so one bad DLC no longer hides the base game and the
  other DLCs. If the base game's own lookup fails the call fails, as before.
* Lumen: pin `v1.4.1`.

## 1.4.0

Library and account for Lumen `v1.4.0`. **Not breaking**: everything is added.

* `clearCache()` drops `gogdl-lib`'s memoized owned products, game builds and downloadable products
  (and its depot manifests and cloud-saves configs). Details, links, summary and screenshots stay.
  Auth and running jobs are untouched, and the next call to each cleared getter refetches. Call it
  before a library refresh or the Builds tab's Refresh.
* `removeProducts(gameId, path, buildName, keep, remove)` deletes the files only `remove`'s depots
  list, never one `keep`'s also lists (compared case-insensitively on normalized paths), prunes the
  directories it emptied, and treats a missing file as success. It returns a `ProductRemoval` with
  `filesRemoved` and `bytesFreed`, and is exported from `gogdl_flutter.dart`. It's a `Future`:
  removing one DLC from Alien: Isolation took 6.9 s. It fetches manifests, so it needs a login and
  the network.
* `gogdl-lib` `v1.2.3` -> `v1.3.2` (`8df36d0`), with no other change to the Dart API. Downloads skip
  verified chunks, and `downloadProgress` still ends at `started.totalBytes`.
  `GamesError::ProductNotAGame` maps to `notFound`.
* Lumen: pin `v1.4.0`.

## 1.3.3

Same code and dependency versions as `1.3.2`. No change to the Dart API or to the progress events
Dart receives.

* `gogdl-lib` (still `v1.2.3`, `988011c`) is fetched over HTTPS from its public GitHub mirror
  instead of `ssh://git@thinkcentre.home:2200`, so a clone of this repo, and any app that depends on
  it, builds with no SSH key, LAN access or git setting. Only its `source` in `rust/Cargo.lock`
  changed.
* The bridge itself can be pinned from GitHub too: `url:
  https://github.com/fernando-navarrete/gogdl_flutter.git`. Lumen: pin `v1.3.3` and drop the
  `RUSTSEC-2026-0285` entry from its `osv-scanner.toml`.
* CI fetches `gogdl-lib` the same way, so its deploy key is gone. `release` also fails if
  `rust/Cargo.lock` doesn't fetch `gogdl-lib` from GitHub.

## 1.3.2

Same code and dependencies as `1.3.1`. No change to the Dart API or to the progress events Dart
receives.

* Fixed the `release` job: it fetched `main` for its on-`main` check before trusting the runner's
  private CA, so `v1.3.1`'s release failed on a self-signed certificate. The `v1.3.1` tag stands
  but has no GitLab release. Lumen: pin `v1.3.2` and drop the `RUSTSEC-2026-0285` entry from its
  `osv-scanner.toml`.

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
