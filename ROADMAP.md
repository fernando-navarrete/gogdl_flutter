# ROADMAP

The order in which the items in [GAPS.md](GAPS.md) will be fixed, and the version tag each one ships
under.

## Versioning rules

- Everything stays on **1.x.x**.
- **Patch** (`1.x.y`): one tag per small, self-contained fix or addition.
- **Minor** (`1.x.0`): a big update, meaning a feature set or cross-cutting refactor. Patches then
  continue from `.1` under that minor.
- Tags use the existing `vX.Y.Z` format. `v1.4.3` is the newest tag, so the next stage is
  **`v1.5.0`**.
- For each release: bump `version:` in `pubspec.yaml` **and** `version` in `rust/Cargo.toml` to
  match, tick the item(s) in `GAPS.md` and add a `CHANGELOG.md` entry on a release branch, merge it
  through an MR, then tag the merged commit on `main` (README "CI and releases").
- Lumen is the only consumer, so a breaking Dart API change is allowed in a **minor**, never in a
  patch. It ships together with the Lumen change that adopts it, and the entry below says so.
- A `gogdl-lib` bump that changes behavior gets its own patch tag, even if the bridge code didn't
  change. The `gogdl-lib` tag has to be on its GitHub mirror, at the SHA
  `rust/Cargo.lock` pins, before this repo is tagged: `release` checks it.
- Items that depend on `gogdl-lib` (**[lib]** in GAPS) start with the upstream request (GAPS §7).
  If upstream isn't ready, the entry says what ships here in the meantime.

## Lining up with Lumen

Lumen is the only consumer, so every bridge release after the foundation is ordered by the Lumen
roadmap stage that needs it (`../lumen/ROADMAP.md`). A bridge minor is tagged **before** Lumen starts
the stage it serves, so Lumen can pin it on day one.

| Lumen stage | Needs from the bridge | Bridge release |
|---|---|---|
| (none, internal) | Tests, CI/CD | `v1.2.0` ✅ |
| `v1.3.0` Download management | Cancel/pause, resume path, error kinds, install size, free space | `v1.3.0` ✅ |
| `v1.4.0` Library and account | Clear the cache (Lumen D3), remove a product's files (Lumen D1) | `v1.4.0` ✅ |
| `v1.5.0` UI polish and packaging | Login error kinds (Lumen D1), breaking API cleanups (Lumen D3) | `v1.5.0` ✅ |
| `v1.6.0` Extras | Nothing | — |
| Not on Lumen's roadmap yet | Cloud saves support/listing/sync, upstream-driven items | `v1.6.0`, `v1.7.0` |

Lumen items that need no bridge change: uninstall, Proton version removal, downloads page
dismiss/Retry (Lumen `v1.3.0`); update detection, since `get_game_builds` already returns build ids
and dates, sign out, since a fresh `GogdlApi` is unauthenticated (Lumen's Phase 0 dropped its D2),
parallel titles (an 8-wide limiter in Lumen), installing DLC on an installed game (`repair_game` with
the larger product set) and Proton releases in providers (Lumen `v1.4.0`); everything in Lumen
`v1.6.0`.

**Before `v1.2.0`** (done): merge `feat/cloud-saves` into `main` (GAPS §6). `v1.1.0`–`v1.1.3` only
existed on that branch.

---

## v1.2.0 ✅ — Foundation: tests, CI and CD

This goes before any feature work so that everything after it lands with tests and a pipeline.

- Fix the example app and its integration test (GAPS §1). The example becomes a minimal smoke test:
  `RustLib.init()`, construct `GogdlApi`, `getLoginLink()` returns a GOG URL, all offline.
- Move each job's progress translation out of `gogdl.rs` into its own module as a plain state
  machine (`on_event` / `on_tick` / `finish`), and share one tick loop across all six jobs. Download
  and repair share their stage handling. No change to the events Dart sees (GAPS §4).
- Rust unit tests for the extracted translators, driven by synthetic events with
  `tokio::time::pause`. They cover stage transitions and their final flushes, one-event-per-path
  dedupe (including repair's separate allocation/verification sets), `ProgressRegression`
  saturation, repair's compressed-size lookup for `DownloadStarted`, the saves per-file byte
  counters, and the final flush before `Finished` (GAPS §6 P0).
- Rust unit tests for the model adapters (`GameBuild`, `DownloadableProduct`, `ProtonRelease`), built
  from JSON fixtures if `gogdl-lib`'s types deserialize. If they don't, ask upstream (GAPS §7.1)
  and skip these tests until then.
- A Linux integration test (`example/integration_test`) that loads the real native library and
  runs the offline smoke test above, so broken FFI loading or a codegen mismatch fails CI.
- CI: GitLab CI on the self-hosted runner (`thinkcentre.home`), reusing Lumen's setup (Flutter from
  `.fvmrc`, SSH deploy key for `gogdl-lib`, `extra_hosts` for the LAN). Jobs:
  - `rust`: `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`.
  - `dart`: `flutter analyze --fatal-infos` (root and `example/`).
  - `codegen`: run `flutter_rust_bridge_codegen generate` pinned to `2.13.0` and fail on
    `git diff` (GAPS §6).
  - `integration`: build the example for Linux and run the integration test under `xvfb`.
  Same trigger policy as Lumen (tags only), plus manual runs on branches.
- CD, on a `vX.Y.Z` tag: fail if `pubspec.yaml`, `rust/Cargo.toml` and the tag disagree, then
  create a GitLab release with that version's `CHANGELOG.md` section.
- Fix the clippy warning and `cargo fmt` drift so the new gates start green (GAPS §4).

Follow-up patches (`v1.2.1+`), one per tag:
1. `v1.2.1`: migrate from the vendored cargokit to Native Assets (`hook/build.dart`, toolchain
   pinned in `rust/rust-toolchain.toml`). Precompiled binaries move to `v1.5.0`, since Native
   Assets has no support for them (GAPS §6).
2. `v1.2.2`: replace the template README, `CHANGELOG.md` and `pubspec.yaml` text, and drop the
   unused `ffigen` dev dependency (GAPS §6).
3. `v1.2.3`: Linux only. Drop the example's Android/iOS/macOS/Windows dirs and the web output
   (GAPS §6).
4. `v1.2.4`: tighten Dart lints on top of `flutter_lints` (GAPS §6).

## v1.3.0 ✅ — Download management (for Lumen `v1.3.0`)

Everything Lumen's download management stage needs. The full cancel/pause needs GAPS §7.2–7.3
upstream first.

- Stop a job when its Dart listener goes away (a failed `sink.add` drops the job future). This
  doesn't need upstream and can go first (GAPS §1).
- An explicit cancel handle per job for cancel/pause, on top of `gogdl-lib`'s cooperative
  cancellation (GAPS §2).
- Resume after an app exit: document (or alias) `repair_game` as the resume path, so Lumen's
  "resume interrupted downloads" calls the right method (GAPS §2).
- A structured `GogError.kind` Dart can switch on, alongside `message()`, for the downloads page's
  real error text. Kinds that `gogdl-lib` can't distinguish yet map to `unknown` until it can
  (GAPS §2).
- `get_install_size(game_id, build, products)` with per-product sizes, and a free-space query
  (implemented here if upstream hasn't exported one), for the disk space check (GAPS §2).
- Request timeouts on the HTTP client, so a stalled job fails with an error instead of hanging
  (GAPS §1).
- **Breaking** for Lumen `v1.3.0`: job methods take a `CancelToken`, each progress enum gains a
  `Cancelled` variant, and errors carry a kind.

## v1.4.0 ✅ — Library and account (for Lumen `v1.4.0`)

Lumen's work plan (`../lumen/v1.4.0-LIBRARY.md`, "Dependency changes") asks for two things, found
during its stage rather than before it. Its Phases 1–4 need nothing from the bridge and are done, so
this is tagged before Lumen's **Phase 6** (adopt the new bridge) instead of before the stage. Both
are **[lib]**: `gogdl-lib` ships them first (GAPS §7.12–7.13) and gets tagged, then this repo pins
that tag.

- Pin the `gogdl-lib` release that has `clear_cache` and `remove_products` (its `v1.3.0`, pinned as
  `v1.3.2`). Absorb anything breaking in that release (its `OwnedGames.unresolved` field,
  `download_game`'s new `VerificationStage`) without changing what Dart sees.
- `clearCache()` (Lumen D3): forwards to `GogDl::clear_cache()`, which drops the memoized owned
  products, game builds and downloadable products (details, links, summary and screenshots stay).
  Without it, Lumen's library refresh re-reads the same owned list and the Builds tab's Refresh is a
  no-op (GAPS §2).
- `removeProducts(gameId, path, buildName, keep, remove)` (Lumen D1): deletes the files only
  `remove`'s depots list (never one `keep`'s list too, compared case-insensitively on normalized
  paths), prunes directories it emptied and never touches a file outside the manifests. Returns a
  `ProductRemoval { filesRemoved, bytesFreed }`. A `Future`, not a stream: it's manifest fetches plus
  deletes. Make it a stream with a `Cancelled` variant only if a large DLC measures slow (GAPS §2).
- Not breaking: everything is added, nothing removed.
- **If `remove_products` stalls upstream**, tag `v1.4.0` with `clearCache()` alone so Lumen's Phase 6
  isn't held up, and ship `removeProducts` as the next patch. Lumen then ships DLC install only and
  moves removal to its own `v1.4.x` patch (its Phase 8).

Dropped from `v1.4.0`, because Lumen no longer needs them for this stage:
- Sign out: Lumen's Phase 0 confirmed that a fresh `GogdlApi` starts unauthenticated, so
  `ref.invalidate` on its backend provider is the sign-out. No `reset()` or `logout` (GAPS §2, §7.5).
- Batched titles (`get_game_titles`): Lumen runs `getGameTitle` through its own 8-wide limiter and
  caches it in providers. Left open in GAPS §2 as polish, unscheduled.

Follow-up patches (`v1.4.1+`), one per tag. None blocks Lumen; they make its new screens cheaper:
1. `v1.4.1`: `get_downloadable_products` fetches details in parallel (bounded) and tolerates one
   failed product lookup. Lumen measured 2.5 s for 8 products, serially, on its DLC tab (GAPS §1,
   §5).
2. `v1.4.2`: `get_game_links` returns boxart and background together. The two old getters stay,
   deprecated, until `v1.5.0` (GAPS §3).
3. `v1.4.3`: `get_proton_releases` validates `page` and reports whether more pages exist. Lumen's
   Proton dialog now watches one provider per page, so a fully filtered page still ends its list
   early (GAPS §1).

## v1.5.0 ✅ — Polish and API cleanup (for Lumen `v1.5.0`)

Work plan: [v1.5.0-POLISH-AND-CLEANUP.md](v1.5.0-POLISH-AND-CLEANUP.md). Lumen's polish and packaging
stage is its cleanup release, so the bridge's breaking cleanups ship here and Lumen migrates once (its
Phase 7). Lumen's work plan (`../lumen/v1.5.0-POLISH-AND-PACKAGING.md`) asks for D1 and D3 below.
Upstream's half is `gogdl-lib` `v1.4.0` (GAPS §7.3), already tagged.

- Pin `gogdl-lib` `v1.4.3` (its breaking `v1.4.0` plus three internal patches). Absorb its error
  enum cleanup, the removed `Preparing`/`Prepared` events and the deprecated `repair_game` without a
  Dart change: `repairGame` keeps its signature and calls upstream's `download_game`.
- Login error kinds (Lumen D1): `GogErrorKind.invalidLoginCode` for a bad, used or expired code, a
  network failure on login as `network`, and a revoked session as `sessionExpired` (GAPS §2).
- Named fields in every progress enum, documenting `ProtonDownloadProgress::Extracted`'s per-tick
  coalescing on the way (was `v1.5.1`). `GameBuild` keeps only the timestamp.
  `DownloadableProduct.product_type` becomes an enum (GAPS §3).
- Remove the getters deprecated in `v1.4.2` (GAPS §3) and `getProtonReleases`, deprecated in
  `v1.4.3` (GAPS §1).
- Collapse `GogError::SavesError` into the main variant: upstream's saves methods return
  `GogDlError` since its `v1.4.0` (GAPS §3; was `v1.7.0`). `GogError` is opaque in Dart, so this
  isn't visible there.
- **Breaking** for Lumen `v1.5.0`: the new error kind, the progress enum field names, the model
  changes and the removed getters.

Dropped from `v1.5.0`: the precompiled Linux `.so`. Lumen's Phase 0 chose an AppImage built from
the bundle its CI image already compiles, so nothing consumes one. It stays open in GAPS §6 as a
build-time speedup, unscheduled.

Follow-up patches (`v1.5.1+`), one per tag:
1. `v1.5.1`: revisit bounded progress channels (GAPS §5). Upstream's senders are unbounded by
   signature, so this measures the bridge's own hops first, and ships a tag only if something here
   changes.

## v1.6.0 — Cloud saves (not on Lumen's roadmap yet)

Lumen's `v1.6.0` needs nothing from the bridge, so this can run alongside it. Add the matching items
to Lumen's `GAPS.md` and `ROADMAP.md` before starting, so both sides agree on when Lumen adopts it.

- Bridge `get_remote_config` (supported or not, plus locations) and `get_save_files` (remote
  listing) (GAPS §2).
- Per-file sync direction and conflict detection once `gogdl-lib` exposes timestamps/hashes and
  honors `cloudStorage.enabled`. Also pick up upstream's fix for the save-truncation data loss
  (GAPS §2, §7.6).
- Upstream removes `repair_game` in its `v1.5.0`. The bridge's `repairGame` already calls
  `download_game` (`v1.5.0`), so decide then whether it stays as a name for Lumen's resume path.

## v1.7.0 — Upstream-driven

Items that wait on `gogdl-lib` and have no Lumen stage yet. Each ships as a patch as soon as its
upstream change lands.

- Language and OS selection for builds (GAPS §2, §7.7).
- `i64` product ids (**breaking**, lockstep with `gogdl-lib` and Lumen) (GAPS §3, §7.9).
