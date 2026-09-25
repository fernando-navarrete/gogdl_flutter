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
