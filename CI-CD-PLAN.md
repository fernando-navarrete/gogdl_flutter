# CI/CD and supply chain

Work plan to bring this repo's CI/CD up to what `../gogdl-lib` runs since its CI/CD line
(`../gogdl-lib/devlog/v1.2.x-ci-cd.md`), adapted to a Flutter + Rust bridge, plus a public push mirror to
GitHub. It ends with `main` and the `v*` tags protected, the mirror live, and this plan turned into a devlog.

The steps are short, and several share one MR: **7 MRs** in all, against the lib's 13. Inside an MR, each
step is one commit on the branch, and the squash leaves one commit on `main`. Bite tests run in throwaway
MRs that are closed unmerged and don't count. Tick each checkbox as it's done.

| MR | Branch | Steps | Title | Status |
|---|---|---|---|---|
| — | (none) | **0** | Decisions, blockers | ✅ |
| **1** | `ci/mr-pipelines` | 1 (merged early as `1edda62`) | `CI: MR pipelines, pinned images, release flow` | ✅ |
| **1b** | `ci/hooks-images-release` | 2–3 (merged as `3f2bb18`) | `CI: pre-commit hook, pinned images` | ✅ |
| **1c** | `ci/release-flow` | 4 (merged as `c9e48b2`), then 5 (settings) | `CI: release flow for a protected main` | ✅ |
| **2** | `release/v1.3.1` | 6 (merged as `5478d80`) | `CI: cargo-deny audit job` | ✅ |
| **2b** | `release/v1.3.1` | 7–8 (merged as `f1d7b93`) | `CI: pub lockfile check, OSV scan` | ✅ |
| **2c** | `release/v1.3.1` | 9–11 (merged as `50c48f2`) | `v1.3.1: supply-chain checks, rustls 0.23.45` | ✅ |
| **2d** | `release/v1.3.2` | 11 (merged as `2ee9b4f`) | `v1.3.2: release job trusts the CA before its fetch` | ✅ |
| **3** | `ci/renovate` | 12–13 (merged as `bec544e`) | `CI: Renovate` | ✅ |
| **4** | `ci/github-mirror` | 14–17 | `CI: GitHub mirror` | 🟦 |
| **5** | `docs/ci-devlog` | 18 | `Docs: CI/CD devlog, close out the plan` | ⬜ |

Status: ⬜ not started · 🟦 in progress · ✅ done · ⏸ blocked

Every MR: the gates in `README.md` "Development" pass locally, the MR pipeline is green. Only MR 2c is a
release. It has to be: `rustls 0.23.43` in `rust/Cargo.lock` has RUSTSEC-2026-0285, and the fix reaches
Lumen only through a tag (Lumen accepts the finding in `osv-scanner.toml` until 2026-11-06).

## Starting point (2026-10-08)

- `main` at `823f7d8`, newest tag `v1.3.0` (`a5b7b5e`, pinned by Lumen). Flutter `3.47.2`, Rust `1.98.1`,
  frb `2.13.0`. CI image `gogdl-flutter-ci:flutter-3.47.2-rust-1.98.1-frb-2.13.0`, which **Lumen's CI uses
  too**. Remote `ssh://git@thinkcentre.home:2200/gogdl/gogdl_flutter.git`, no GitHub remote.
- CI runs **only on tags and web runs**: `rust` (fmt, clippy, test), `dart` (analyze, root and `example/`),
  `codegen` (regenerate, fail on diff), `integration` (Linux example under xvfb), then `release` with
  `release-cli:latest`. The image is `FROM debian:bookworm-slim` (no digest), pipes `sh.rustup.rs` into
  `sh`, and clones Flutter by tag. No MR pipelines, security jobs, schedules or Renovate.
- Commits go straight to `main`, and tags are cut on it. There is no `CLAUDE.md`. The docs live in
  `README.md` ("Development", "CI and releases") and `ROADMAP.md` ("Versioning rules").
- `rust/Cargo.lock` has 266 packages: 264 from crates.io and `gogdl-lib` from the LAN over SSH (`v1.2.3`,
  `988011c`). `example/pubspec.lock` has 46 hosted packages, the bridge as `path: ".."`, and 6 SDK
  packages. The root `pubspec.lock` is gitignored. `LICENSE` reads "TODO: Add your license here."
- 119 commits, one author email. The 30 old `-restart`/`-debug` tags stay.

### How gogdl-lib maps here

| gogdl-lib | here |
|---|---|
| `lint`, `test`, `doc` | `rust`, `dart`, `codegen`, `integration` (exist, move to MR pipelines) |
| `.githooks/pre-commit`: `cargo fmt --check` | the same, in `rust/` |
| Images by digest, `rustup-init` by sha256, `-rN` image revision | the same, plus Flutter pinned to a commit |
| `audit`: `cargo deny` (advisories, crates.io only, licenses) | the same on `rust/Cargo.lock`, plus one allowed git source (`gogdl-lib`) |
| (crates.io-only `[sources]`) | plus `tool/check_lockfile.sh` on `example/pubspec.lock` (from Lumen) and `--enforce-lockfile` |
| `scan`: OSV on `Cargo.lock`, fails on `MAL-` only | on both lockfiles; for pub it gates every advisory, since nothing else does |
| `secrets`: gitleaks, full-history audit | the same |
| `SECURITY.md` | the same, plus pub packages and the Native Assets hook |
| `renovate.json` (`cargo`), job, schedules | plus `pub`; the toolchains and frb stay manual (D4) |
| `release`: needs every check, tag must be on `main` | the same, plus the `gogdl-lib` tag must be on GitHub (Lumen's B2) |
| GitHub push mirror (built-in), `LICENSE-*`, `README.md` | the same |
| Protected `main` and `v*` | the same, done early (step 5) |
| manual `downstream` (builds the bridge and lumen-cli) | not now (D9) |

---

## Step 0: Decisions and blockers

Recommended answers are in *italics*. The steps below assume them.

Decided 2026-10-08: all recommended answers accepted.

- [x] **D1. Merge method.** *As in gogdl-lib:* fast-forward with squash required, squash message = MR
      title, source branch deleted. The devlog cites SHAs on `main`.
- [x] **D2. Where tags go.** *On `main`, after the release MR merges* (gogdl-lib D2). Lumen needing
      unreleased work pins `ref:` to a commit on its own feature branch.
- [x] **D3. Runner load.** This is heavier than the lib: `integration` builds the Linux app, and `codegen`
      and `rust` each compile the crate. *Accept it* with `interruptible` and auto-cancel, and measure MR 1.
      Fallback: `integration` manual on MRs and kept on `main` and tags.
- [x] **D4. Toolchain bumps.** The image is shared with Lumen, so moving Flutter, Rust or frb means a
      rebuild and an `image:` change in both repos. *Renovate ignores all three* (`.fvmrc`,
      `rust/rust-toolchain.toml`, `flutter_rust_bridge*` in pubspec and Cargo, `FRB_VERSION`), as Lumen
      already does for Flutter. They're bumped by hand, together. This differs from gogdl-lib D4.
- [x] **D5. Renovate and releases.** Unlike the lib, this repo's `rust/Cargo.lock` is what Lumen builds,
      so a Renovate lockfile MR reaches Lumen at the next tag. *`update-lockfile` for both managers. A
      security fix gets its own patch tag, and routine updates ride along with the next release.*
      `gogdl-lib` is ignored by Renovate: a pin bump is an API change, done by hand (ROADMAP rule).
- [x] **D6. Mirror and license.** *Public, owner `fernando-navarrete`, repo `gogdl_flutter`* (the name
      Lumen's `insteadOf` expects). License: *`MIT OR Apache-2.0`, as the lib* (`LICENSE-MIT`,
      `LICENSE-APACHE`, `license` in `rust/Cargo.toml`). The mirror is read-only and builds only with LAN
      access, or with Lumen's `insteadOf` rewrite once `gogdl-lib` is mirrored (B1).
- [x] **D7. How the mirror pushes.** *GitLab's built-in push mirror, as the lib.* No CI job gets the
      token. Lumen's GITHUB-MIRROR.md B2 asks for a mirror job that checks that the pinned `gogdl-lib` tag is
      on GitHub; here that check moves into the `release` job (step 17), which runs before anyone pins the tag.
- [x] **D8. Lint scope.** *Unchanged* (fmt, clippy `-D warnings`, `analyze --fatal-infos`). `cargo deny`
      gets its own job. No `dart format` gate for now.
- [x] **D9. Downstream job.** *Not now:* Lumen already scans this repo's `Cargo.lock` (`bridge-lock`), and
      a breaking change ships with Lumen's adopting MR anyway. Goes in GAPS §6.
- [x] **D10. Keep the `thinkcentre.home` URL for `gogdl-lib`** in `rust/Cargo.toml` (gogdl-lib's
      `CLAUDE.md`, Lumen's Decisions). Its `downstream` job patches that URL, and SHAs match across the
      mirror.

**Blockers, in other repos:**

- [x] **B1. gogdl-lib's GitHub `v1.2.3` is wrong.** `git ls-remote` on
      `github.com/fernando-navarrete/gogdl-lib` shows `refs/tags/v1.2.3` at `51075b0` ("throwaway: direct
      push test"), not `988011c`. This is left over from the step 12 tag bite test there: GitLab was
      restored but the mirror wasn't (or GitHub's tag protection rejected the restore). Anyone pinning
      `v1.2.3` from GitHub builds that commit. Fix it there (restore the tag on GitHub, check the ruleset,
      add a GAPS line or devlog note), and check every other mirrored tag against GitLab. Step 17's check
      fails until this is fixed, which is correct.
      *Fixed 2026-10-08:* the `v*` ruleset was turned off for one mirror "Update now"; GitHub
      `v1.2.3` is `988011c`, and every GitHub ref matches GitLab. Noted in gogdl-lib MR 26 (devlog
      step 12, plus a GAPS line: the ruleset has no bypass for the mirror token).

**Done when:** each answer is recorded here and B1 is fixed.

---

## MR 1: `ci/mr-pipelines`

This plan is the branch's first commit.

MR 1 merged after step 1 alone (`1edda62`). Steps 2–3 merged as MR 1b on `ci/hooks-images-release`
(`3f2bb18`), step 4 as MR 1c on `ci/release-flow` (`c9e48b2`), and step 5 followed that MR (settings
only, ticked on MR 2's branch). Pitfall for the devlog:
the squash message became the first commit's message ("Add the CI/CD work plan"), not the MR title, and
it happened again in MR 1b ("CI: pre-commit hook"). Step 5 set the squash commit message template to
`%{title}`.

### Step 1: Pipelines on MRs and `main`, MR template

- [x] `workflow:rules` copied from gogdl-lib: `schedule`, `merge_request_event`, `web`, no branch
      pipeline when an MR is open, `main`, `vX.Y.Z` tags.
- [x] `default: interruptible: true`, plus a `.not-scheduled` rule on `rust`, `dart`, `codegen` and `integration`.
- [x] Rewrite the `.gitlab-ci.yml` header (the trigger paragraph).
- [x] `.gitlab/merge_request_templates/Default.md`: Summary; gates pass locally; `CHANGELOG.md` line if
      Lumen sees a change; bindings regenerated if `rust/src/api/` changed; `rust/Cargo.lock` or
      `example/pubspec.lock` changed → new packages listed (the bar comes in step 10).
- [x] Project settings: merge method per D1, delete source branch, auto-cancel redundant pipelines,
      pipelines must succeed. (Done in step 5.)

**Done when:** this MR's own pipeline runs the four jobs, and a pushed branch with no MR runs nothing.

### Step 2: Pre-commit hook

- [x] `.githooks/pre-commit` from gogdl-lib: when a staged file matches `rust/*.rs`, run
      `cargo fmt --check` in `rust/`. It checks and never rewrites.
- [x] README "Development": `git config core.hooksPath .githooks` once per clone.

**Done when:** a commit with a misformatted `.rs` file is refused.

### Step 3: Pin every image

- [x] `ci/Dockerfile`: `debian:bookworm-slim@sha256:…` (the digest gogdl-lib uses), and `rustup-init`
      `1.29.1` by sha256 instead of `sh.rustup.rs` (copy the lib's block).
- [x] Flutter: clone the tag, then check that `git rev-parse HEAD` equals a `FLUTTER_COMMIT` passed in by
      `ci/build-image.sh` (a small table in the script keyed by version). A moved tag fails the build.
- [x] `ci/build-image.sh`: `IMAGE_REVISION=1`, tag `…-frb-2.13.0-r1`. The `.toolchain` check still compares
      versions only.
- [x] `release`: `release-cli:v0.24.0@sha256:…` (gogdl-lib's digest).
- [x] Rebuild on the runner host and update `image:`. **Lumen:** its `.ci-image` keeps the old tag until
      Lumen bumps it in its own MR. Both tags can sit on the host, so nothing breaks.

**Done when:** no `image:` or `FROM` lacks `@sha256`, and the pipeline is green on `-r1`.

### Step 4: Release flow for a protected `main`

- [x] README "CI and releases" and ROADMAP "Versioning rules": (1) a release branch with both versions, both
      lock files, the `CHANGELOG.md` section and the GAPS tick; (2) an MR titled `vX.Y.Z: ...`, green,
      merged; (3) `git fetch`, tag the merged commit on `origin/main`, push the tag; (4) the tag
      pipeline's `release` is green.
- [x] `release`: `needs` on every check job (MR 2 adds its jobs to the list), `GIT_DEPTH: 0`, and fail
      unless `git merge-base --is-ancestor "$CI_COMMIT_SHA" origin/main` (gogdl-lib's block).
- [x] `sh tool/release_notes.sh v1.3.0` still passes locally.

**Done when:** merged. MR 2 is the first release that follows it.

### Step 5: Protect `main` and `v*` (settings, right after MR 1c merges)

Done early, not last as in the lib, because of two of its pitfalls: Renovate's protected variables only
reach protected refs (its step 9), and a tag bite test run before `v*` is protected really does move the
tag (its step 12).

- [x] Protected branch `main`: push "No one", merge Maintainers, no force push.
- [x] MR settings: all threads resolved, skipped pipelines not successful, squash commit message
      template `%{title}`.
- [x] Protected tags `v*`: create Maintainers. **Check that `v*` is listed before** running any tag test.
- [x] Bite tests: a direct push to `main` is rejected; an MR with a failing test can't merge;
      `git push --delete origin v1.3.0` is rejected. No force-move test (B1 shows what that costs).
- [x] README: `main` and `v*` are protected. Added in step 11.

**Done when:** all three bite tests behave as expected.

*Done 2026-10-09.* A direct push to `main` was rejected ("not allowed to push code to protected
branches"). Throwaway MR !4 (a panicking test in `rust/src/error_kind.rs`) went red with no Merge
button. It was closed and its branch deleted. `git push --delete origin v1.3.0` was rejected ("You can
only delete protected tags using the web interface"), so a Maintainer can still delete a tag from the UI.
`git ls-remote` on `main` and every `v*` tag matched the pre-test baseline afterwards.

---

## MR 2: `release/v1.3.1`

MR 2 merged after step 6 alone (`5478d80`), so steps 7–11 continue as MR 2b on a fresh
`release/v1.3.1` off `main`. Pitfall for the devlog: the squash message is "Step 5: main and v*
protected, bite tests". GitLab takes a new MR's title from the branch's first commit, and the
`%{title}` template then copies it. Set the MR title by hand when opening the MR.

MR 2b merged after step 8 (`f1d7b93`), so steps 9–11 continue as MR 2c on a fresh `release/v1.3.1`
off `main`. The same pitfall hit again: the squash message is "Plan: MR 2 merged early with step 6, MR 2b
for steps 7-11", the branch's first commit, although the MR was opened after the warning above. For
the devlog: retitle the MR in the UI before merging, and check the squash message in the merge widget.

### Step 6: `cargo-deny` (`audit`)

- [x] `ci/Dockerfile`: `cargo-deny@0.20.2` (`--locked`), `IMAGE_REVISION=2`.
- [x] Rebuild on the runner host (`…-frb-2.13.0-r2`, `image:` already points at it). Lumen keeps `-r1`.
- [x] `rust/deny.toml` from gogdl-lib, with `allow-git = ["ssh://git@thinkcentre.home:2200/gogdl/gogdl-lib.git"]`.
      The lib's license list fits as is. `rust/Cargo.toml` gets `publish = false` and `deny.toml`
      `[licenses.private] ignore = true`: this crate is unlicensed until step 14. `gogdl-lib` `v1.2.3`
      predates its `license` field, so a `[[licenses.clarify]]` says `MIT OR Apache-2.0` until the pin moves.
- [x] `audit` job: `.ssh` (metadata fetches `gogdl-lib`), `cd rust && cargo deny --locked check`, cached
      advisory db. Runs on MRs, `main`, tags and `SCHEDULE=scan`.
- [x] Red first, locally, on four findings, all fixed by semver-compatible lock bumps (each version over
      7 days old): RUSTSEC-2026-0285 (`rustls` 0.23.43 → 0.23.45), RUSTSEC-2026-0190 unsound (`anyhow`
      1.0.75 → 1.0.104), RUSTSEC-2025-0056 unmaintained `adler` (`backtrace` 0.3.69 → 0.3.76, which
      moves to `miniz_oxide` 0.8 and `adler2`), and `Unicode-DFS-2016` rejected (`unicode-ident` 1.0.12 →
      1.0.26, now `Unicode-3.0`). The last three come in through frb's `allo-isolate`.
- [x] The MR pipeline runs `audit` green on `-r2`.
- [x] Bite test: `main`'s old `rust/Cargo.lock` in a throwaway MR turns `audit` red (!6, closed).

**Done when:** `audit` is green here and was red on the bite test.

### Step 7: pub lockfile check

- [x] `tool/check_lockfile.sh`, adapted from Lumen's: every `example/pubspec.lock` package is `hosted` from
      `https://pub.dev` with a `sha256`, `sdk`, or the `path: ".."` bridge. Anything else fails.
      Locally: 53 packages ok; red on a missing `sha256`, a non-pub.dev URL, another path and a `git` source.
- [x] `dart` job: run it, and use `flutter pub get --enforce-lockfile` in `example/`. `integration` does
      the same, since it also runs `pub get` in `example/`.
- [x] Bite test: a hand-edited `sha256` in a throwaway MR turns `dart` red (!7, closed): `pub get
      --enforce-lockfile` rejects it even with a warm pub cache. `check_lockfile.sh` only checks that a hash is there.

**Done when:** green here, and red on a hand-edited `sha256` in a throwaway MR.

### Step 8: OSV-Scanner (`scan`), weekly schedule

- [x] `scan` job, gogdl-lib's image and digest, over `rust/Cargo.lock` and `example/pubspec.lock`. Fails on
      any `MAL-` id, and on any advisory for a **pub** package (nothing else gates those). Rust
      CVE/GHSA/RUSTSEC ids are printed and left to `audit`. Accepted findings go in `osv-scanner.toml` with
      a `reason` and an `ignoreUntil`, as in Lumen. 30-day JSON artifact.
      Two scanner runs, one per lock file, so the exit code splits cargo from pub (the image has no jq,
      and Lumen needed a second `scan-gate` job in Dart for that). Locally, with the job's script: both
      locks clean; `v1.3.0`'s `Cargo.lock` prints 4 ids (rustls, anyhow, adler) and passes; a pub lock
      with `http 0.13.0` fails on GHSA-4rgh-jx4f-qfcq, and passes once that id is in `osv-scanner.toml`.
- [x] Schedule "Weekly scan" on `main`, `SCHEDULE=scan`. Play it by hand: only `audit` and `scan` run.
      Played 2026-10-09 after MR 2b merged: exactly those two jobs ran.

**Done when:** `scan` is green, and the schedule runs exactly those two jobs.

### Step 9: Secret scan (`secrets`), full-history audit

Must come before MR 4, since the mirror publishes the whole history.

- [x] Local `gitleaks git --log-opts="--all"` over all 119 commits and every tag. Review each finding
      (frb-generated code, test fixtures in `rust/src`). A real leak means stop and decide before any mirror.
      Run 2026-10-08 with the job's image: 155 commits (every branch, `origin/*`, all 43 tags; the
      history grew past 119), **no findings**.
- [x] `.gitleaks.toml`: default rules, plus allowlist entries only for reviewed false positives, each with
      a reason. Nothing to allowlist, so default rules only, as in gogdl-lib.
- [x] `secrets` job copied from gogdl-lib (MR range on MRs, `HEAD --remotes --tags` otherwise).
      Locally with the job's command: both ranges exit 0, and a commit with a fake `ghp_` token exits 1.
- [x] Bite test: a fake `ghp_` token in a throwaway MR turns it red. Close it and delete the branch.
      !10 went red on `secrets` while !9 (this branch) was green. The branch is deleted.

**Done when:** history is clean or allowlisted with reasons, and the bite test went red.

### Step 10: `SECURITY.md`

- [x] From gogdl-lib's: adding a crate (7 days, `cargo add <crate>@=<ver>`, `build.rs`/proc macro named
      in the MR) **and** a pub package (pub.dev, verified publisher, 7 days, dev dependency where possible:
      Lumen's list). Dev machines. If a package we use is compromised: rotate `GOGDL_LIB_DEPLOY_KEY_B64`,
      the Renovate tokens and the mirror token; pin a known-good version; play the weekly scan; cut a
      patch, which Lumen needs to pick up the fix.
- [x] Note that `hook/build.dart` runs cargo in every consuming app's build, so a `build.rs` here runs on
      Lumen's machines and CI.
- [x] MR template: the dependency checkbox links it. README "Layout": `SECURITY.md`, `deny.toml`,
      `.gitleaks.toml`, `osv-scanner.toml`, `.githooks/`.

*Done 2026-10-08.* The secrets list names only `GOGDL_LIB_DEPLOY_KEY_B64` for now: the Renovate tokens join
it in step 13 and the mirror token in step 17. 64 crates have a `build.rs` or proc macro.

### Step 11: The `v1.3.1` release

- [x] `release` `needs`: add `audit`, `scan` and `secrets`.
- [x] Bump to `1.3.1` in `pubspec.yaml`, `rust/Cargo.toml`, `rust/Cargo.lock` and `example/pubspec.lock`.
      `CHANGELOG.md`: rustls 0.23.45 (RUSTSEC-2026-0285), no API change. GAPS: tick "No release pipeline"
      §6 if this closes it. README "CI and releases": rewrite for the new jobs and protection.
      It doesn't close it (precompiled binaries are still open), so GAPS gets a line instead.
- [x] Merge, tag `v1.3.1` on `main`, and check the tag pipeline: every check job, then `release`. This is
      the first real run of a `needs` + on-`main` release in either repo, which closes gogdl-lib's GAPS
      entry for it in spirit; note it there.
- [ ] Tell Lumen: pin `v1.3.2` (see below), and drop its `RUSTSEC-2026-0285` entry from `osv-scanner.toml`.

*`v1.3.1` tagged 2026-10-08 on `50c48f2`.* Every check job was green, but `release` failed:
its `git fetch` of `main` (for the on-`main` check) ran before the step that adds the runner's private
CA to the trust store, so git hit "SSL certificate problem: self-signed certificate". gogdl-lib's
`release` has the same order, so its next tag fails too. A tag pipeline runs the tagged commit's
`.gitlab-ci.yml`, so a retry can't pick up a fix: the `v1.3.1` tag stands with no GitLab release, and
MR 2d moves the CA step before the fetch and ships it as `v1.3.2` (same code). Reproduced and checked
locally in the pinned `release-cli` image: the fetch fails without the CA and gets past TLS with it.

- [x] MR 2d merged, `v1.3.2` tagged, its `release` green. Tagged 2026-10-08 on `2ee9b4f`: every
      check job, then `release`, green; the GitLab release shows the `1.3.2` notes.
- [ ] gogdl-lib: the same reorder, plus its GAPS entry ("`needs` and on-`main` rule untested"): first
      real run here, which found this bug.
- [ ] Lumen pins `v1.3.2` (not `v1.3.1`).

**Done when:** the `v1.3.2` release exists and its tag pipeline is green.

---

## MR 3: `ci/renovate`

### Step 12: `renovate.json` and its validation

- [x] From gogdl-lib's: weekly, `minimumReleaseAge: "7 days"`, security fixes skip the wait, majors off,
      0.x minors treated as majors (cargo), one grouped minor/patch/digest MR, `lockFileMaintenance` off,
      `pinDigests` for `gitlabci` and `dockerfile`. Unlike the lib, `ci/Dockerfile`'s base digest gets
      its own group (`ci image base`): the image is shared with Lumen, so that MR needs a rebuild with a
      bumped `IMAGE_REVISION` and a new `image:`, and would otherwise hold up the grouped MR.
- [x] `enabledManagers: ["cargo", "pub", "gitlabci", "dockerfile"]`, `rangeStrategy: "update-lockfile"`.
      No toolchain regex manager (D4).
- [x] `ignoreDeps`: `gogdl-lib`, `gogdl-flutter-ci`, `flutter_rust_bridge`, `flutter_rust_bridge_hooks`
      (D4, D5). `constraints`: `rust` `1.98.1`, `flutter` `3.47.2` (what Renovate installs to run cargo
      and pub). The `.gitlab-ci.yml` header and README say to bump them with the toolchains.
- [x] `renovate-config` job (`--strict`, on `renovate.json` changes), plus `release`'s optional `needs`.
      Locally with the job's image: passes; fails on a migratable option (`stabilityDays`) and on an
      unknown one.

**Done when:** the validator passes in the MR pipeline.

### Step 13: Renovate job and schedule

The job is a commit on this branch. A schedule runs `main`'s `.gitlab-ci.yml`, and the protected tokens
reach only protected refs, so the variables, schedule, runs and bite test come after MR 3 merges and are
ticked on MR 4's branch.

- [x] `renovate` job from gogdl-lib (environment `renovate`, CA handling, token check). **And the
      `gogdl-lib` deploy key:** cargo has to fetch the git dependency to rewrite `rust/Cargo.lock`. Copy
      how Lumen's `renovate` job loads its key, and Lumen's guard that fails the job when Renovate aborts
      (Lumen `d9e4b54`).
      Renovate passes its child processes only a few variables, so no ssh-agent: the key goes in
      `~/.ssh/config`, and `~/.cargo/config.toml` sets `git-fetch-with-cli` (cargo's built-in git
      only uses the agent).
- [x] Variables `RENOVATE_TOKEN` and `GITHUB_COM_TOKEN` (protected, masked, scope `renovate`), with calendar
      reminders. `main` is already protected (step 5).
- [x] `SECURITY.md`: both tokens in the rotation list, and `renovate` in the job logs to check.
- [x] Schedule "Renovate", `SCHEDULE=renovate`. Do a dry run first (`RENOVATE_DRY_RUN=full`): check the
      pub manager on a root `pubspec.yaml` with no lock file (only `example/pubspec.lock` exists), the
      ignored deps, and the 0.x rule. Then a live run.
- [x] Bite test: throwaway jobs print `${RENOVATE_TOKEN:+set}` on an unprotected branch, with and
      without `environment: renovate`, and print nothing (with it, so the empty output comes from the
      protection and not the scope).

**Done when:** a live run opened the grouped MR, its pipeline ran every check job, and it merged through an MR.

*Done 2026-10-08.* The first live run was started from the Dependency Dashboard checkbox (the schedule
is Monday mornings). Its grouped MR merged as `faec192`, touching `rust/Cargo.lock` only: `reqwest`
0.13.4 → 0.13.5 (which pulls in a second `base64`, 0.23.1, alongside `hyper-util`'s 0.22.1) and
`thiserror` 2.0.20 → 2.0.21. Every new version was more than 7 days old, so the checkbox skipped only the
schedule, not `minimumReleaseAge`. These updates reach Lumen with the next tag (D5). Bite test in MR !14
(closed unmerged): both token jobs printed `[]` for `RENOVATE_TOKEN` and `GITHUB_COM_TOKEN`, with and
without `environment: renovate`, and a control job printed `[set]` for the unprotected
`GOGDL_LIB_DEPLOY_KEY_B64`. The calendar reminders and the dry run aren't recorded.

---

## MR 4: `ci/github-mirror`

Needs steps 9 and 5, and B1 for step 17.

### Step 14: What the mirror shows

- [x] `LICENSE-MIT` and `LICENSE-APACHE` replace `LICENSE`. `rust/Cargo.toml`: `license`, `description`,
      `repository` (the GitHub URL, as the lib). `pubspec.yaml`: `repository`.
- [x] README: "This repository is a mirror" (as the lib's), and how to build outside the LAN with Lumen's
      `insteadOf` line. `deny.toml` licenses: this crate's own license.

*Done 2026-10-08.* `deny.toml` loses `[licenses.private] ignore`, so `audit` now checks this crate's
own license: green in the `-r2` image, and red with `license = "GPL-3.0-only"` on a scratch copy. That
green run had SSH disabled (`GIT_SSH_COMMAND=false`) and only the README's `insteadOf` line, so cargo
fetched `gogdl-lib` from its GitHub mirror at the locked SHA. The README also links `gogdl-lib` on
GitHub and moves "Using it" to `v1.3.2`. Both lock files are unchanged.

### Step 15: GitHub side (settings)

- [x] Create `fernando-navarrete/gogdl_flutter`, empty, public, with a "mirror" description. Issues,
      Projects, Wiki and Actions off. Ruleset on `main` and `v*`: no deletion, no force push, except the
      mirror token. Make sure a restore can still get through, unlike gogdl-lib's (B1).
- [x] Fine-grained token, `Contents: read and write` on that repo only, with an expiry and a reminder.

*Done 2026-10-08.* The repo is public and empty (`git ls-remote` lists nothing), with Issues, Projects,
Wiki, Discussions and Actions off. The description was skipped: the README says it's a mirror. Two
active rulesets, "Protect Main" (`refs/heads/main`) and "Tag Rules" (`refs/tags/v*`), each restrict
deletions and block force pushes. Their bypass list is the **Repository admin** role: on a personal
repo a fine-grained token acts as its owner, so this is the only bypass that lets the mirror push a
restore, which gogdl-lib's no-bypass ruleset couldn't (B1). The cost, for step 18's GAPS line: the
ruleset doesn't stop the owner or the token from moving a tag on GitHub. GitLab's `v*` protection is
the real guard, and step 16's `ls-remote` comparison is the check. The token is limited to this repo
with `Contents: read and write`. Its expiry and reminder aren't recorded here.

### Step 16: GitLab push mirror (settings)

- [x] Settings → Repository → Mirroring: push, password auth with the token, "Mirror only protected
      branches", "Keep divergent refs" off. "Update now".
- [x] Check: `main` at the same SHA, and **every `v*` tag at the same SHA as GitLab** (compare
      `git ls-remote` on both, not just tag names). No other branches.

*Done 2026-10-08.* After "Update now", GitHub has `main` at `faec192` and all 45 tags (the 30 old
`-restart`/`-debug` ones too, as they match `v*`). Every GitHub ref, including the one peeled
annotated tag, has the same SHA on GitLab, and no GitLab tag is missing. GitHub has no other
branches: `ci/github-mirror` stayed off it.

### Step 17: The `gogdl-lib` pin check

- [x] In `release`, before the release is created: read the `gogdl-lib` tag and SHA from `rust/Cargo.lock`,
      and fail unless `git ls-remote https://github.com/fernando-navarrete/gogdl-lib refs/tags/<tag>`
      gives the same SHA (peeled). This is Lumen's B2/B4: a bridge tag never pins a dependency that GitHub
      doesn't have.
- [x] Check it locally against `v1.2.3`: red while B1 is open, green once it's fixed.
- [x] README and SECURITY.md: the mirror, its token in the rotation list, the release-order rule.

*Done 2026-10-08.* `tool/check_lib_pin.sh` reads the pin from `rust/Cargo.lock`, and `release` runs
it after the on-`main` check. It reads the peeled `^{}` line when the tag is annotated, and fails on a
pin that isn't `?tag=`. Locally: green against GitHub (`v1.2.3` at `988011c`, a lightweight tag). B1 is
already fixed, so a lock with another SHA stood in for it, and that went red with the mismatch. Also
red: a `v9.9.9` pin (missing on GitHub), a `rev=` pin, and no `gogdl-lib` package. A local clone with an
annotated `v1.2.3` on `988011c` passed, and red once the tag moved to its parent. Green and the
missing-tag case behave the same in the pinned `release-cli` image (busybox). The first real run is the
next tag. README "CI and releases" and ROADMAP "Versioning rules" carry the release-order rule, and
SECURITY.md rotates the mirror token. The merge-propagation check below is ticked on MR 5's branch.

**Done when:** a merge into `main` shows up on GitHub within minutes, a feature branch never does, and the
pin check passes.

---

## MR 5: `docs/ci-devlog`

### Step 18: Close-out

- [ ] `devlog/v1.3.x-ci-cd.md`: the decisions and pitfalls, with SHAs on `main`, in the lib devlog's format.
- [ ] GAPS §6: a "CI/CD" group for what's left (runner load measured or not, D9's downstream job, token
      expiries, the first toolchain bump done by hand across both repos).
- [ ] Tell Lumen that its B2 is done (its GITHUB-MIRROR.md is updated in Lumen's own repo).
- [ ] Delete this file in the same commit.

**Done when:** merged under the protection, and this plan is a devlog.
