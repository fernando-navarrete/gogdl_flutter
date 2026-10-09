# gogdl_flutter

Dart bindings for [`gogdl-lib`](ssh://git@thinkcentre.home:2200/gogdl/gogdl-lib.git), generated with
[flutter_rust_bridge](https://cjycode.com/flutter_rust_bridge) `2.13.0`. It exposes GOG login,
library and game metadata, game download, repair and verification, Proton-GE downloads and cloud
saves as a single `GogdlApi` class. Long-running jobs stream their progress as freezed sealed
classes (`DownloadGameProgress`, `RepairGameProgress`, ...). Everything public is exported from
`lib/gogdl_flutter.dart`.

Lumen is the only consumer. Linux is the only platform that is built and tested, and
frb's web output is turned off (`web: false` in `flutter_rust_bridge.yaml`).

`gogdl-lib` is a private git dependency (pinned by tag in `rust/Cargo.toml`) served only over the
LAN, at `ssh://git@thinkcentre.home:2200`. Nothing builds without access to that host.

## Using it

```yaml
dependencies:
  gogdl_flutter:
    git:
      url: ssh://git@thinkcentre.home:2200/gogdl/gogdl_flutter.git
      ref: v1.3.0
```

```dart
await RustLib.init();
final api = GogdlApi();
print(api.getLoginLink());
```

## How the crate is built

The crate in `rust/` is built with Flutter's Native Assets. `hook/build.dart` runs cargo through
`flutter_rust_bridge_hooks` and bundles the result as `lib/libgogdl_flutter.so` in the app. There is
nothing to configure in the consuming app, and `RustLib.init()` takes no arguments.

A build needs:

- Flutter `3.47.2` (`.fvmrc`). Build hooks need no flag.
- [rustup](https://rustup.rs). `rust/rust-toolchain.toml` pins the toolchain (`1.98.1`, target
  `x86_64-unknown-linux-gnu`) and rustup installs it on the first build.
- SSH access to `thinkcentre.home:2200` for `gogdl-lib`, with cargo fetching through the git CLI so
  it can use the ssh-agent: `CARGO_NET_GIT_FETCH_WITH_CLI=true`.

There are no precompiled binaries, so every consumer compiles the whole crate. Prebuilt binaries
are planned for `v1.5.0` (see [ROADMAP.md](ROADMAP.md)).

## Layout

- `rust/src/api/`: the Rust API that flutter_rust_bridge reads (`GogdlApi`, models, errors, progress
  types).
- `rust/src/progress/`: one plain state machine per job that turns `gogdl-lib` events into progress
  events, plus the shared tick loop. Not part of the bridged API.
- `rust/src/client.rs`, `rust/src/error_kind.rs`: the HTTP client with its timeouts, and the
  mapping from `gogdl-lib` errors to `GogErrorKind`. Not part of the bridged API.
- `lib/src/rust/`, `rust/src/frb_generated.rs`: generated. Don't edit them by hand.
- `hook/build.dart`: the Native Assets build hook.
- `example/`: an offline smoke app and its Linux integration test.
- `ci/`, `tool/`: the CI image and the release gate. See below.

## Development

In `rust/`:

```sh
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

Once per clone, `git config core.hooksPath .githooks` enables a pre-commit hook that runs
`cargo fmt --check` in `rust/` when a `.rs` file under `rust/` is staged. `git commit --no-verify` skips
it; CI still checks.

In the root and in `example/`:

```sh
fvm flutter analyze --fatal-infos
```

The integration test (loads the real library, no network):

```sh
cd example && fvm flutter test integration_test -d linux
```

### Regenerating the bindings

After changing anything under `rust/src/api/`:

```sh
cargo install flutter_rust_bridge_codegen --version 2.13.0 --locked
flutter_rust_bridge_codegen generate
dart run build_runner build --delete-conflicting-outputs
```

The explicit `build_runner` run is needed because codegen's own pass can delete the
`*.freezed.dart` files without rewriting them. `build.yaml` limits build_runner to `lib/**`, so it
doesn't walk the Flutter SDK cloned into the repo. Commit whatever changed: the `codegen` CI job
regenerates and fails on any diff.

## CI and releases

GitLab CI runs on the self-hosted runner, on merge requests, on `main`, on `vX.Y.Z` tags and for
manual runs from the web UI. A pushed branch without an MR runs nothing. The `check` stage has four
jobs: `rust`, `dart`, `codegen` and `integration`. They run in a prebuilt image (`ci/Dockerfile`,
built with `ci/build-image.sh`), which has to be rebuilt after bumping `.fvmrc`,
`rust/rust-toolchain.toml` or the codegen version, or after changing `ci/Dockerfile` (bump
`IMAGE_REVISION`). Every base image is pinned by digest, `rustup-init` by sha256 and Flutter by
commit. `.gitlab-ci.yml`'s header covers the runner and deploy-key setup.

To release:

1. Bump `version:` in `pubspec.yaml` and `version` in `rust/Cargo.toml`, along with the two lock
   files that record it (`rust/Cargo.lock` and `example/pubspec.lock`).
2. Add a `## X.Y.Z` section to `CHANGELOG.md` and tick the item in `GAPS.md`.
3. Commit, tag `vX.Y.Z` and push the tag.

On a `vX.Y.Z` tag the `release` job runs `tool/release_notes.sh`. It fails unless the tag, the four
versions above and a non-empty changelog section all agree, and then creates a GitLab release from
that section.

[ROADMAP.md](ROADMAP.md) has the planned releases and [GAPS.md](GAPS.md) the known gaps.
