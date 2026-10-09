# gogdl_flutter

Dart bindings for [`gogdl-lib`](https://github.com/fernando-navarrete/gogdl-lib), generated with
[flutter_rust_bridge](https://cjycode.com/flutter_rust_bridge) `2.13.0`. It exposes GOG login,
library and game metadata, game download, repair and verification, Proton-GE downloads and cloud
saves as a single `GogdlApi` class. Long-running jobs stream their progress as freezed sealed
classes (`DownloadGameProgress`, `RepairGameProgress`, ...). Everything public is exported from
`lib/gogdl_flutter.dart`.

Lumen is the only consumer. Linux is the only platform that is built and tested, and
frb's web output is turned off (`web: false` in `flutter_rust_bridge.yaml`).

`gogdl-lib` is a git dependency, pinned by tag in `rust/Cargo.toml` and served over the LAN at
`ssh://git@thinkcentre.home:2200`. Both repositories are mirrored to GitHub: off the LAN, see
"Building from the GitHub mirror" below.

## Using it

```yaml
dependencies:
  gogdl_flutter:
    git:
      url: ssh://git@thinkcentre.home:2200/gogdl/gogdl_flutter.git
      ref: v1.3.2
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

### Building from the GitHub mirror

The `thinkcentre.home` URLs stay in `rust/Cargo.toml` and in a consumer's `pubspec.yaml`. Off the
LAN, one git setting rewrites them to the GitHub mirrors:

```sh
git config --global url."https://github.com/fernando-navarrete/".insteadOf "ssh://git@thinkcentre.home:2200/gogdl/"
```

The mirrors keep commit SHAs, so `rust/Cargo.lock` and a consumer's `pubspec.lock` still match. No
SSH key and no `CARGO_NET_GIT_FETCH_WITH_CLI` are needed: pub and the build hook's cargo both go
through the rewrite.

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
- `ci/`, `tool/`: the CI image and the release gates. See below.
- `SECURITY.md`: what a new crate or pub package has to meet, and what to do if one is compromised.
- `rust/deny.toml`, `osv-scanner.toml`, `.gitleaks.toml`: config of the `audit`, `scan` and `secrets`
  CI jobs (advisories, sources and licenses; accepted findings; secret rules).
- `.githooks/`: the pre-commit hook (see "Development").
- `LICENSE-MIT`, `LICENSE-APACHE`: the license (see "License").
- `renovate.json`: the Renovate config (weekly, grouped minor and patch updates of the lock files and CI
  images; the toolchains, frb and `gogdl-lib` move by hand).

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
manual runs from the web UI. A pushed branch without an MR runs nothing. `main` and the `v*` tags are
protected: everything lands through an MR with a green pipeline, and only Maintainers create tags.
The `check` stage has seven jobs, plus one that runs only when its file changes and one that runs only
on its schedule:

- `rust`: `cargo fmt --check`, clippy and the tests.
- `dart`: `tool/check_lockfile.sh`, `pub get --enforce-lockfile` and `flutter analyze` in the root
  and `example/`.
- `codegen`: regenerates the bindings and fails on any diff.
- `integration`: builds the Linux example and runs its integration test under xvfb.
- `audit`: `cargo deny` per `rust/deny.toml` (advisories, sources, licenses).
- `scan`: OSV-Scanner on `rust/Cargo.lock` and `example/pubspec.lock`, minus what
  `osv-scanner.toml` accepts.
- `secrets`: gitleaks over the MR's commits, or the whole history on `main` and tags.
- `renovate-config`: `renovate-config-validator --strict`, when `renovate.json` changes.
- `renovate`: Renovate itself, which opens the dependency MRs per `renovate.json`.

The "Weekly scan" schedule on `main` runs only `audit` and `scan`, so a new advisory for a locked
package is noticed. The "Renovate" schedule on `main` runs only `renovate`, on Monday mornings;
its MRs go through the same checks as any other. The jobs run in a prebuilt image (`ci/Dockerfile`,
built with `ci/build-image.sh`), which has to be rebuilt after bumping `.fvmrc`, `rust/rust-toolchain.toml` or
the codegen version (bump `constraints` in `renovate.json` with them), or after changing
`ci/Dockerfile` (bump `IMAGE_REVISION`). Every base image is pinned by digest, `rustup-init` by sha256 and Flutter by commit. `.gitlab-ci.yml`'s header covers the runner and deploy-key setup.

To release:

1. On a `release/vX.Y.Z` branch, bump `version:` in `pubspec.yaml` and `version` in
   `rust/Cargo.toml`, along with the two lock files that record it (`rust/Cargo.lock` and
   `example/pubspec.lock`). Add a `## X.Y.Z` section to `CHANGELOG.md` and tick the item in
   `GAPS.md`. If the `gogdl-lib` pin moved, its tag has to be on its GitHub mirror first, at the
   SHA in `rust/Cargo.lock` (`git ls-remote https://github.com/fernando-navarrete/gogdl-lib
   refs/tags/<tag>`).
2. Open an MR titled `vX.Y.Z: ...`, wait for a green pipeline, and merge it. GitLab takes a new
   MR's title from the branch's first commit, and the squash commit copies the title, so check both
   the title and the squash message in the merge widget before merging.
3. `git fetch`, tag the merged commit (`git tag vX.Y.Z origin/main`) and push the tag.
4. Check that the tag pipeline is green: every check job, then `release`.

On a `vX.Y.Z` tag the `release` job waits for every check job, then fails unless the tagged commit
is on `main`, `tool/check_lib_pin.sh` finds the pinned `gogdl-lib` tag on GitHub at the locked SHA
(so the tag builds off the LAN), and `tool/release_notes.sh` passes: the tag, the four versions above
and a non-empty changelog section must all agree. It then creates a GitLab release from that section.
If only the `gogdl-lib` check failed, update that repo's mirror and retry the job.

[ROADMAP.md](ROADMAP.md) has the planned releases and [GAPS.md](GAPS.md) the known gaps.

## This repository is a mirror

Development happens on a self-hosted GitLab. This GitHub repository is a read-only push mirror of
`main` and the `v*` tags (GitLab's built-in push mirror, protected refs only, with a fine-grained
token limited to this repository): issues, pull requests and Actions are off here. Report a problem to the
maintainer directly (see `SECURITY.md`).

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at your option.
