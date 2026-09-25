# GAPS

Known gaps, bugs and improvement opportunities in `gogdl_flutter`, the Flutter/Rust bridge between
[Lumen](../lumen) and [`gogdl-lib`](../gogdl-lib). Tick an item off (`[x]`) when it's fixed, and add
new findings under the matching section.

Audit baseline (2026-09-24): `feat/cloud-saves` at `f37eba8` (`v1.1.3`), `gogdl-lib` pinned to
`v1.0.11`, Lumen pinned to `v1.1.3`. `fvm flutter analyze` reports **2 errors** (both in `example/`,
see §1), `cargo clippy` reports **1 warning**, and `cargo fmt --check` is not clean.

Priority tags: **P0** = broken behavior users will hit · **P1** = important gap · **P2** = polish / cleanup.

Items marked **[lib]** can't be fully fixed here: they need a change in `gogdl-lib` first. Section 7
collects them as the list of things to request upstream.

---

## 1. Bugs

- [ ] **P1 — Cancelling a stream on the Dart side doesn't stop the job.** Every job
  (`download_game`, `repair_game`, `verify_download`, `download_proton_release`,
  `download_save_files`, `upload_save_files`) discards the result of `sink.add` (`let _ = ...`).
  When Lumen cancels its subscription, the Rust future keeps going until it finishes, so a
  multi-GB download keeps using the network and disk with nobody listening. It could stop early
  here: treat a failed `sink.add` as "listener gone" and drop the job future. A clean stop that
  leaves resumable state on disk is **[lib]** (see §2 "Cancel and pause").
- [x] **P1 — The example app and its integration test don't compile.** `example/lib/main.dart:19`
  and `example/integration_test/simple_test.dart:9` still call the frb template's `greet()`, which no
  longer exists. These are the analyzer's only two errors, so `flutter analyze` can't gate CI until
  they're fixed.
  Fixed in `v1.2.0`: the example is an offline smoke app with a passing Linux integration test.
- [ ] **P2 — `get_proton_releases` can't tell a filtered page from the end of the list.** It
  fetches a fixed page of 5 and silently drops releases with no suitable asset
  (`gogdl.rs:106-112`). A page where every release gets filtered out returns `[]`, which Lumen
  reads as the end of the list. Return the unfiltered count or a `has_more` flag.
- [ ] **P2 — `get_proton_releases(page)` casts `i32` to `u32` without checking** (`page as u32`). A
  negative page wraps around to a huge page number instead of returning an error.
- [ ] **P2 — `get_downloadable_products` fails the whole call if one product lookup fails.** The
  per-product `get_product_details` loop uses `?`, so one bad DLC hides the base game and every
  other DLC on Lumen's DLC tab (Lumen `v1.4.0` DLC install/remove).
- [ ] **P2 — Requests have no timeouts.** `GogdlApi::new` uses a default `Client::new()` with no
  connect or read timeout, so a stalled connection can hang metadata calls (and Lumen's spinners)
  indefinitely. Build the client with timeouts via `gogdl_lib::Client::builder()` (it re-exports
  `reqwest::Client`). `gogdl-lib`'s own retry layer only helps once a request fails.

## 2. Missing API surface (what Lumen needs)

What Lumen needs from the bridge. The Lumen version in brackets is the Lumen roadmap stage that
needs it; ROADMAP.md schedules each item to land before that stage starts.

- [ ] **P1 — Cancel and pause for downloads, verifications and repairs** (Lumen `v1.3.0`). Nothing
  can be stopped once it starts. The bridge needs a cancel handle per job, for example a
  `JobHandle` opaque returned alongside the stream, or a `cancel(job_id)`. **[lib]** needs
  cooperative cancellation (a `CancellationToken` or a cancel-safe future contract) that leaves
  partial files in a state `repair_game` can resume.
- [ ] **P1 — Structured errors** (Lumen `v1.3.0` downloads page error text, and `v1.5.0` login
  errors). `GogError` exposes only `message()`, a display string, so Lumen
  can't tell "session expired, sign in again" from "offline", "not found", "not enough disk space"
  or "cloud saves not supported" without matching on text. Add a `kind` (a flat enum Dart can
  switch on) next to the message. Several kinds are **[lib]**: a dead refresh token and a routine
  expiry come back as the same opaque error, and the free-space check throws away its numbers
  (`gogdl-lib` GAPS, Medium/Low).
- [ ] **P1 — Sign out** (Lumen `v1.4.0`). Lumen can delete its stored token, but the `GogDl`
  instance keeps the authenticated session in memory until the app restarts. **[lib]** `GogDl`
  has no `logout`/`clear_auth`. Workaround until then: expose `GogdlApi::reset()` that swaps
  `inner` for a fresh `GogDl`, which also drops the token observer.
- [ ] **P1 — Install and download size before installing** (Lumen `v1.3.0` disk space check).
  `get_product_bundles` already has everything needed. Expose
  `get_install_size(game_id, build, products)` returning the download size (compressed) and the
  on-disk size (uncompressed), plus per-product sizes for the DLC tab (Lumen `v1.4.0`).
- [ ] **P1 — Free space at the install path** (Lumen `v1.3.0` disk space check). `gogdl-lib` checks free space internally before a
  download but doesn't let callers ask. **[lib]** export `get_free_space` (or a `statvfs` wrapper).
  Otherwise, implement it here.
- [ ] **P1 — Cloud saves support and listing** (not on Lumen's roadmap yet). `get_remote_config` (`is_supported`,
  `get_locations`) and `get_save_files` exist in `gogdl-lib` but aren't bridged, so Lumen can only
  find out a game has no cloud saves by starting a sync and waiting for it to fail. Bridge them so
  Lumen can hide or disable the Saves tab and show what's in the cloud (names, sizes, timestamps).
- [ ] **P2 — Two-way save sync with conflict detection.** Download and upload are each a blind
  copy. Picking a direction per file (newer wins, or ask the user) needs local and remote
  timestamps/hashes. **[lib]** ("Save 'sync' is a blind copy", and "Download and upload ignore
  `cloudStorage.enabled`").
- [ ] **P2 — Batched metadata for the library** (Lumen `v1.4.0`, its 5.1). Lumen fetches titles one
  `await` at a time. Add `get_game_titles(ids)` that runs the lookups in parallel (bounded) in Rust,
  or a `get_library_entries` returning title, boxart and background together.
- [ ] **P2 — Resume an interrupted install.** `gogdl-lib` documents that `repair_game`, not
  `download_game`, resumes one. The bridge doesn't pass that on. Document it on the Dart API (or add
  a `resume_download` alias) so Lumen `v1.3.0` calls the right one.
- [ ] **P2 — Language and OS selection.** Builds are always Windows and the language filter is
  hardcoded. **[lib]** ("Language and OS are hardcoded with no selection surface").

## 3. Bridge API design / consistency

- [ ] **P2 — Boxart and background each fetch `get_game_links` separately** (`gogdl.rs:55-63`).
  Lumen asks for both per game, so every game costs two identical requests. Return both from one
  `get_game_links`.
- [ ] **P2 — Progress enums mix positional and named fields.** `ProtonDownloadProgress` and
  `VerifyDownloadProgress` use tuple variants (`Started(u64)`, `FileNotFound(String, u64)`), while
  the game and saves enums use named fields. Positional fields show up in Dart as `field0`/`field1`.
  Switch everything to named fields. This is a breaking change, so it lands in a minor bump
  alongside a Lumen update.
- [ ] **P2 — `ProtonDownloadProgress::Extracted` is lossy and undocumented.** Only the last
  extracted path per 150ms tick is forwarded. That's fine for a status line, but the doc comment
  should say so.
- [ ] **P2 — `GameBuild` carries the release date twice** (an RFC 2822 string and a timestamp).
  Keep the timestamp and let Dart format it.
- [ ] **P2 — `DownloadableProduct.product_type` is a free-form string.** Expose an enum (game/DLC)
  so Lumen doesn't string-compare.
- [ ] **P2 — Game ids are `i32`.** GOG product ids already reach ~1.5 billion, which is close to
  `i32::MAX`. `gogdl-lib` uses `i32` too, so change both together. **[lib]**
- [ ] **P2 — `GogError` has a separate `SavesError` variant** only because the saves methods in
  `gogdl-lib` return `SavesError` instead of `GogDlError`. **[lib]**

## 4. Code structure

- [x] **P1 — The progress coalescing is untestable and duplicated.** `gogdl.rs` is ~1000 lines,
  almost all event-to-progress translation inlined in each job method: the same
  ticker/`select!`/dirty-flag/final-flush scaffolding six times, and `download_game` and
  `repair_game` share ~200 near-identical lines of stage handling. Move each translator into its own
  module as a plain state machine (`on_event(event) -> Vec<Progress>`, `on_tick()`, `finish()`) and
  share the tick loop, so the logic can be unit-tested with synthetic events and no network.
  Fixed in `v1.2.0`: translators under `rust/src/progress/` and one shared tick loop.
- [x] **P2 — One clippy warning** (redundant closure, `GameBuild::from_gogdl` in `.map`) and
  `cargo fmt` drift.

## 5. Performance

- [ ] **P2 — `get_downloadable_products` fetches product details serially** (`gogdl.rs:97-100`),
  one request per DLC. Games with many DLCs (dozens) make the DLC tab slow. Use a bounded
  `join_all`.
- [ ] **P2 — Progress channels are unbounded.** The coalescer drains fast enough today, but a
  stalled Dart isolate lets memory grow without limit during a large download. Worth revisiting
  once the coalescers are extracted (§4).

## 6. Testing, CI/CD, tooling and docs

- [x] **P0 — No tests at all.** There are no Rust `#[cfg(test)]` modules, no Dart tests, and the
  only integration test is the broken frb template (§1).
  Fixed in `v1.2.0`: Rust unit tests for the translators, tick loop and model adapters, plus the
  Linux integration test.
- [x] **P1 — No CI.** Nothing runs analyze, clippy, fmt, tests or codegen on push or tag. Lumen
  already has GitLab CI on the self-hosted runner (`thinkcentre.home`), so reuse that setup.
  Fixed in `v1.2.0`: `.gitlab-ci.yml` with `rust`, `dart`, `codegen` and `integration` jobs.
- [x] **P1 — Generated bindings can drift from the Rust API.** `lib/src/rust/**` and
  `rust/src/frb_generated.rs` are committed, and nothing checks they match
  `flutter_rust_bridge_codegen generate` output for the pinned `2.13.0`.
  Fixed in `v1.2.0`: the `codegen` job regenerates and fails on any diff.
- [x] **P1 — Built with the vendored cargokit.** The crate was built by a vendored copy of cargokit
  (`cargokit/`, ~30 files) hooked into each platform's CMake, Gradle and podspec files, which the
  package carried as root `android/`, `ios/`, `linux/`, `macos/` and `windows/` dirs.
  Fixed in `v1.2.1`: migrated to Native Assets (`hook/build.dart`, through
  `flutter_rust_bridge_hooks`), with the toolchain pinned in `rust/rust-toolchain.toml`. cargokit and
  the glue dirs are gone; the Rust and Dart APIs are unchanged and the `.so` is still bundled as
  `lib/libgogdl_flutter.so`.
- [ ] **P1 — No release pipeline.** Tags are cut by hand, and nothing checks that `pubspec.yaml`,
  `rust/Cargo.toml` and the tag agree. Every Lumen build (and Lumen's CI) compiles the whole Rust
  crate from source, which is slow.
  Partly fixed in `v1.2.0`: the version check and GitLab release on tags. Precompiled binaries are
  still open: the cargokit mechanism this pointed to is gone, and `native_toolchain_rust` has no
  precompiled-binary support. Deferred to `v1.5.0`, which needs a prebuilt `.so` for Lumen's
  packaging anyway.
- [ ] **P1 — The build isn't reproducible outside the LAN.** `gogdl-lib` is a git dependency over
  `ssh://git@thinkcentre.home:2200`, the same situation Lumen documented and deferred in its
  `v1.1.0`. CI on the self-hosted runner works around it with a deploy key.
- [x] **P2 — `main` is behind the released tags.** `v1.1.0`–`v1.1.3` only exist on
  `feat/cloud-saves`, which is 5 commits ahead of `main`. Merge it before starting the next stage.
- [x] **P2 — Template leftovers.** The README describes the FFI plugin template (`src/`, `ffigen`,
  `sum`/`sumAsync`). `CHANGELOG.md` is `0.0.1 — TODO`. The `pubspec.yaml` description is "A new
  Flutter FFI plugin project.", with boilerplate comments and an unused `ffigen` dev dependency.
  Fixed in `v1.2.2`: the README describes the bridge, `CHANGELOG.md` has real entries (backfilled
  from `1.0.0`), and `pubspec.yaml` has a real description and no `ffi`/`ffigen`.
- [x] **P2 — Declared platforms are never built.** `pubspec.yaml` declares Android, iOS, macOS and
  Windows, and `frb_generated.web.dart` exists, but only Linux is used or tested (Lumen needs
  Proton). Either drop the other platforms or build them in CI. As it is, they claim support
  nobody checks.
  Fixed in `v1.2.3`: Linux only. The example's `android/`, `ios/`, `macos/` and `windows/` dirs are
  gone (the root ones went with cargokit in `v1.2.1`), and `web: false` in
  `flutter_rust_bridge.yaml` stops frb from generating the web output.
- [x] **P2 — Tighten lints.** Enable stricter rules on top of `flutter_lints` and
  `clippy -D warnings`, once the example is fixed and CI is running.
  Fixed in `v1.2.4`: the Dart side has the strict analyzer modes and a list of extra rules
  (see `analysis_options.yaml`), with generated code excluded and `example/` inheriting them.
  Clippy stays at `-D warnings`, without the pedantic group.

## 7. Requests for `gogdl-lib`

Consolidated from the **[lib]** items above, in the order this repo needs them (see ROADMAP.md's
"Lining up with Lumen"). Where `gogdl-lib`'s own `GAPS.md` already tracks one, its title is quoted.

1. **Test-constructible public types** (`GameBuild`, `ProductDetails`, `ProtonGeRelease`, the event
   enums), so the bridge's model adapters can be unit-tested without the network. If `serde`
   round-tripping from fixtures already works, just document it. Wanted by bridge `v1.2.0`, which
   skips those tests without it.
   `gogdl-lib` `v1.1.0` documents it (`tests/public_types.rs`, and a `# Constructing in tests`
   section with a doctest on each type): structs deserialize from JSON, the event enums are built
   from their public variants. Caveats: `ProductDetails.title` is skipped by serde, so set it by
   hand; `productType` is upper case (`"GAME"`, `"DLC"`) in real responses; and the events and
   Proton types have no `Debug`/`PartialEq` yet, so use `matches!` (tracked upstream).
2. **Cooperative cancellation** for `download_game`, `repair_game`, `verify_files`,
   `download_proton_release` and the save transfers (a `CancellationToken` parameter, or a
   documented cancel-safe contract for dropping the future), leaving state that `repair_game` can
   resume. Blocks bridge `v1.3.0` / Lumen `v1.3.0`.
3. **Structured, distinguishable errors**: session expired vs refresh token revoked, offline, not
   found, insufficient space (with required/available bytes), invalid login code, cloud saves
   unsupported. Tracked upstream as "A dead refresh token and a routine expiry both surface as the
   same opaque, unstructured error" and "The free-space check discards its own error detail". Also:
   "The four saves methods return `SavesError`, not `GogDlError`". Bridge `v1.3.0` / Lumen
   `v1.3.0` (downloads page), and the login kinds by Lumen `v1.5.0`.
4. **A public free-space query** (`PathResolver::get_free_space` is internal today). Bridge
   `v1.3.0` / Lumen `v1.3.0`, or the bridge implements it itself.
5. **`logout`/`clear_auth` on `GogDl`**, which also drops the token observer. Bridge `v1.4.0` /
   Lumen `v1.4.0`, or the bridge falls back to `GogdlApi::reset()`.
6. **Save sync metadata**: local and remote timestamps/hashes per file, and honoring
   `cloudStorage.enabled` (upstream: "Save 'sync' is a blind copy in each direction" and "Download
   and upload ignore `cloudStorage.enabled`"). Also upstream's "`download_save_files` truncates a
   good local save before it has the new one on disk", which is data loss and worth fixing sooner
   than bridge `v1.6.0`, since Lumen already syncs saves.
7. **Language and OS selection** for builds (upstream: "Language and OS are hardcoded with no
   selection surface").
8. **Proton tarball integrity check** (upstream: "Proton tarballs are extracted without any
   integrity check").
9. **`i64` (or `u64`) product ids.**
