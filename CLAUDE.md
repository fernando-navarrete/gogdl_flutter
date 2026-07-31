# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this repo is

`gogdl_flutter` is a Flutter FFI plugin (`project_type: plugin_ffi`) that wraps
[`gogdl-lib`](https://thinkcentre.home:2200/gogdl/gogdl-lib) (a Rust library implementing
GOG's Galaxy backend: auth, catalog, downloads, repairs, verification, cloud saves, Proton-GE
releases) and exposes it to Dart via `flutter_rust_bridge` 2.12.0. This package **is the
bridge itself**, not a GUI application — it was renamed from `gogdl2_flutter_bridge` to
`gogdl_flutter` on 2026-07-14. A separate consuming Flutter app (the actual GOG client GUI)
depends on this package; it does not live in this repo.

The only entry point consumers use is the `Gog` class (`lib/src/rust/api/gogdl/gogdl.dart`,
generated from `rust/src/api/gogdl/gogdl.rs`). **`api.md` at the repo root is the canonical,
hand-maintained API reference** — read it before changing or adding to the `Gog` surface; it
documents every method, the stream-based progress model, error handling, and known gaps.

## Architecture

```
gogdl-lib (Rust crate, external git dep)
   -> rust/src/api/gogdl/gogdl.rs   (#[frb] annotated wrapper types/methods)
   -> rust/src/frb_generated.rs      (generated glue, do not hand-edit)
   -> lib/src/rust/...               (generated Dart bindings, do not hand-edit)
   -> lib/gogdl_flutter.dart         (hand-written public export surface)
```

- `rust/src/api/gogdl/gogdl.rs` is the only Rust file meant to be hand-edited. It defines
  `#[frb(opaque)]` wrapper structs/enums around `gogdl-lib` types (e.g. `Gog`, `GameBuild`,
  `DownloadableProduct`, `DownloadStream`/`RepairStream`/`VerificationStream`,
  `GogError`) because flutter_rust_bridge needs its own visible types at
  the FFI boundary rather than exposing `gogdl-lib`'s types directly.
- **Long-running operations are `Stream<T>`, not `Future`.** Each `#[frb]` method that
  downloads/repairs/verifies/transfers takes a `StreamSink<T>` parameter on the Rust side
  (from `crate::frb_generated::StreamSink`) and pushes progress snapshots into it while a
  separate async future drives the actual work; flutter_rust_bridge turns that into a single
  Dart `Stream<T>`. Follow this existing pattern for any new long-running method instead of
  inventing a separate progress-callback mechanism.
- **One error type wraps everything**: `GogDlError` (from `gogdl-lib`) is mapped to `GogError`,
  a `freezed` sealed Dart union with one variant per subsystem (auth/game/depot/secureLinks/
  download/saves/proton), each carrying just a `message: String`. Every fallible method must
  surface failures this way — do not introduce a second error type.
- Opaque Dart-visible enums (`DownloadStatus`, `RepairStatus`, `VerificationStatus`, etc.)
  expose no variant info to Dart by default, so each has a `#[frb(sync)] fn name() -> String`
  accessor (e.g. `"downloading"`, `"repaired"`) that Dart code switches on. Add one when adding
  a new opaque status enum.
- This bridge intentionally narrows `gogdl-lib`'s raw flow: it skips the crate's
  `get_downloadable_files` per-chunk-manifest step and has `downloadGame`/`repairDownload`/
  `verifyDownload` take `selectedProducts` (product-id strings) directly, resolving manifests
  internally. Don't reintroduce `DepotFile`/`Chunk` types across the FFI boundary without a
  deliberate decision to do so — see `api.md` §1 and §9 ("Known gaps & caveats") for the
  tradeoffs already accepted.
- Native builds for all five platforms (android/ios/linux/macos/windows) go through
  **Cargokit** (`cargokit/`, vendored from the flutter_rust_bridge project) which invokes
  `cargo build` per-platform and bundles the resulting cdylib. Do not hand-edit files under
  `cargokit/` — treat it as a vendored dependency.

## Codegen workflow

After changing anything in `rust/src/api/**`:

```sh
flutter_rust_bridge_codegen generate
```

This regenerates `lib/src/rust/**` (Dart bindings) and `rust/src/frb_generated.rs` from
`flutter_rust_bridge.yaml` (`rust_input: crate::api`, `rust_root: rust/`,
`dart_output: lib/src/rust`). Never hand-edit generated files — re-run codegen instead.

If any new/changed type uses `freezed` (like `GogError`), also run:

```sh
dart run build_runner build --delete-conflicting-outputs
```

## Common commands

```sh
# Fetch Dart deps (run from repo root and from example/)
flutter pub get

# Analyze/lint
flutter analyze

# Build the Rust side directly (fast inner-loop check, no Dart/codegen involved)
cd rust && cargo build

# Run the example app (manual smoke test of the bridge)
cd example && flutter run

# Integration test (drives the example app; currently only checks the flutter_rust_bridge
# quickstart `greet` function — the Gog surface itself has no test coverage yet)
cd example && flutter test integration_test/simple_test.dart
```

Note: `gogdl-lib` is pulled via a git dependency
(`ssh://git@thinkcentre.home:2200/gogdl/gogdl-lib.git`, pinned to tag `v0.1.0` in
`rust/Cargo.toml`) — building requires SSH access to that host. When `gogdl-lib` gets a new
tag/release that this bridge needs to pick up, bump the `tag` (and `version`) in
`rust/Cargo.toml`.

## Notes

- `README.md` is still the unmodified `flutter create --template=plugin_ffi` boilerplate
  (references `ffigen`/`src/gogdl_flutter.h`, which don't exist in this project — it uses
  `flutter_rust_bridge`, not raw `dart:ffi`/`ffigen`). Don't trust it over `api.md` or this file.
- `example/lib/main.dart` and `example/integration_test/simple_test.dart` are also still the
  unmodified quickstart template (calls the scaffold `greet` function, not `Gog`). Treat the
  example app as not yet wired up to the real API surface.
- CHANGELOG.md is still a TODO placeholder.
