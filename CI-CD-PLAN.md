# CI/CD and supply chain

Work plan to bring this repo's CI/CD up to what `../gogdl-lib` runs since its CI/CD line
(`../gogdl-lib/devlog/v1.2.x-ci-cd.md`), adapted to a Flutter + Rust bridge, plus a public push mirror to
GitHub. It ends with `main` and the `v*` tags protected, the mirror live, and this plan turned into a devlog.

The steps are short, and several share one MR: **5 MRs** in all, against the lib's 13. Inside an MR, each
step is one commit on the branch, and the squash leaves one commit on `main`. Bite tests run in throwaway
MRs that are closed unmerged and don't count. Tick each checkbox as it's done.

| MR | Branch | Steps | Title | Status |
|---|---|---|---|---|
| — | (none) | **0** | Decisions, blockers | ✅ |
| **1** | `ci/mr-pipelines` | 1–4, then 5 (settings) | `CI: MR pipelines, pinned images, release flow` | 🟦 |
| **2** | `release/v1.3.1` | 6–11 | `v1.3.1: supply-chain checks, rustls 0.23.45` | ⬜ |
| **3** | `ci/renovate` | 12–13 | `CI: Renovate` | ⬜ |
| **4** | `ci/github-mirror` | 14–17 | `CI: GitHub mirror` | ⬜ |
| **5** | `docs/ci-devlog` | 18 | `Docs: CI/CD devlog, close out the plan` | ⬜ |

Status: ⬜ not started · 🟦 in progress · ✅ done · ⏸ blocked

Every MR: the gates in `README.md` "Development" pass locally, the MR pipeline is green. Only MR 2 is a
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

### Step 1: Pipelines on MRs and `main`, MR template

- [x] `workflow:rules` copied from gogdl-lib: `schedule`, `merge_request_event`, `web`, no branch
      pipeline when an MR is open, `main`, `vX.Y.Z` tags.
- [x] `default: interruptible: true`, plus a `.not-scheduled` rule on `rust`, `dart`, `codegen` and `integration`.
- [x] Rewrite the `.gitlab-ci.yml` header (the trigger paragraph).
- [x] `.gitlab/merge_request_templates/Default.md`: Summary; gates pass locally; `CHANGELOG.md` line if
      Lumen sees a change; bindings regenerated if `rust/src/api/` changed; `rust/Cargo.lock` or
      `example/pubspec.lock` changed → new packages listed (the bar comes in step 10).
- [ ] Project settings: merge method per D1, delete source branch, auto-cancel redundant pipelines,
      pipelines must succeed.

**Done when:** this MR's own pipeline runs the four jobs, and a pushed branch with no MR runs nothing.

### Step 2: Pre-commit hook

- [ ] `.githooks/pre-commit` from gogdl-lib: when a staged file matches `rust/*.rs`, run
      `cargo fmt --check` in `rust/`. It checks and never rewrites.
- [ ] README "Development": `git config core.hooksPath .githooks` once per clone.

**Done when:** a commit with a misformatted `.rs` file is refused.

### Step 3: Pin every image

- [ ] `ci/Dockerfile`: `debian:bookworm-slim@sha256:…` (the digest gogdl-lib uses), and `rustup-init`
      `1.29.1` by sha256 instead of `sh.rustup.rs` (copy the lib's block).
- [ ] Flutter: clone the tag, then check that `git rev-parse HEAD` equals a `FLUTTER_COMMIT` passed in by
      `ci/build-image.sh` (a small table in the script keyed by version). A moved tag fails the build.
- [ ] `ci/build-image.sh`: `IMAGE_REVISION=1`, tag `…-frb-2.13.0-r1`. The `.toolchain` check still compares
      versions only.
- [ ] `release`: `release-cli:v0.24.0@sha256:…` (gogdl-lib's digest).
- [ ] Rebuild on the runner host and update `image:`. **Lumen:** its `.ci-image` keeps the old tag until
      Lumen bumps it in its own MR. Both tags can sit on the host, so nothing breaks.

**Done when:** no `image:` or `FROM` lacks `@sha256`, and the pipeline is green on `-r1`.

### Step 4: Release flow for a protected `main`

- [ ] README "CI and releases" and ROADMAP "Versioning rules": (1) a release branch with both versions, both
      lock files, the `CHANGELOG.md` section and the GAPS tick; (2) an MR titled `vX.Y.Z: ...`, green,
      merged; (3) `git fetch`, tag the merged commit on `origin/main`, push the tag; (4) the tag
      pipeline's `release` is green.
- [ ] `release`: `needs` on every check job (MR 2 adds its jobs to the list), `GIT_DEPTH: 0`, and fail
      unless `git merge-base --is-ancestor "$CI_COMMIT_SHA" origin/main` (gogdl-lib's block).
- [ ] `sh tool/release_notes.sh v1.3.0` still passes locally.

**Done when:** merged. MR 2 is the first release that follows it.

### Step 5: Protect `main` and `v*` (settings, right after MR 1 merges)

Done early, not last as in the lib, because of two of its pitfalls: Renovate's protected variables only
reach protected refs (its step 9), and a tag bite test run before `v*` is protected really does move the
tag (its step 12).

- [ ] Protected branch `main`: push "No one", merge Maintainers, no force push.
- [ ] MR settings: all threads resolved, skipped pipelines not successful.
- [ ] Protected tags `v*`: create Maintainers. **Check that `v*` is listed before** running any tag test.
- [ ] Bite tests: a direct push to `main` is rejected; an MR with a failing test can't merge;
      `git push --delete origin v1.3.0` is rejected. No force-move test (B1 shows what that costs).
- [ ] README: `main` and `v*` are protected. That line goes in MR 2.

**Done when:** all three bite tests behave as expected.

---

## MR 2: `release/v1.3.1`

### Step 6: `cargo-deny` (`audit`)

- [ ] `ci/Dockerfile`: `cargo-deny@0.20.2` (`--locked`), `IMAGE_REVISION=2`, rebuild.
- [ ] `deny.toml` from gogdl-lib, with `allow-git = ["ssh://git@thinkcentre.home:2200/gogdl/gogdl-lib.git"]`
      and the license list adjusted from `cargo deny list`.
- [ ] `audit` job: `.ssh` (metadata fetches `gogdl-lib`), `cd rust && cargo deny --locked check`, cached
      advisory db. Runs on MRs, `main`, tags and `SCHEDULE=scan`.
- [ ] It goes red on RUSTSEC-2026-0285 first: `cargo update -p rustls --precise 0.23.45` (the lib's fix).
- [ ] Bite test: `main`'s old `rust/Cargo.lock` in a throwaway MR turns `audit` red.

**Done when:** `audit` is green here and was red on the bite test.

### Step 7: pub lockfile check

- [ ] `tool/check_lockfile.sh`, adapted from Lumen's: every `example/pubspec.lock` package is `hosted` from
      `https://pub.dev` with a `sha256`, `sdk`, or the `path: ".."` bridge. Anything else fails.
- [ ] `dart` job: run it, and use `flutter pub get --enforce-lockfile` in `example/`.

**Done when:** green here, and red on a hand-edited `sha256` in a throwaway MR.

### Step 8: OSV-Scanner (`scan`), weekly schedule

- [ ] `scan` job, gogdl-lib's image and digest, over `rust/Cargo.lock` and `example/pubspec.lock`. Fails on
      any `MAL-` id, and on any advisory for a **pub** package (nothing else gates those). Rust
      CVE/GHSA/RUSTSEC ids are printed and left to `audit`. Accepted findings go in `osv-scanner.toml` with
      a `reason` and an `ignoreUntil`, as in Lumen. 30-day JSON artifact.
- [ ] Schedule "Weekly scan" on `main`, `SCHEDULE=scan`. Play it by hand: only `audit` and `scan` run.

**Done when:** `scan` is green, and the schedule runs exactly those two jobs.

### Step 9: Secret scan (`secrets`), full-history audit

Must come before MR 4, since the mirror publishes the whole history.

- [ ] Local `gitleaks git --log-opts="--all"` over all 119 commits and every tag. Review each finding
      (frb-generated code, test fixtures in `rust/src`). A real leak means stop and decide before any mirror.
- [ ] `.gitleaks.toml`: default rules, plus allowlist entries only for reviewed false positives, each with
      a reason.
- [ ] `secrets` job copied from gogdl-lib (MR range on MRs, `HEAD --remotes --tags` otherwise).
- [ ] Bite test: a fake `ghp_` token in a throwaway MR turns it red. Close it and delete the branch.

**Done when:** history is clean or allowlisted with reasons, and the bite test went red.

### Step 10: `SECURITY.md`

- [ ] From gogdl-lib's: adding a crate (7 days, `cargo add <crate>@=<ver>`, `build.rs`/proc macro named
      in the MR) **and** a pub package (pub.dev, verified publisher, 7 days, dev dependency where possible:
      Lumen's list). Dev machines. If a package we use is compromised: rotate `GOGDL_LIB_DEPLOY_KEY_B64`,
      the Renovate tokens and the mirror token; pin a known-good version; play the weekly scan; cut a
      patch, which Lumen needs to pick up the fix.
- [ ] Note that `hook/build.dart` runs cargo in every consuming app's build, so a `build.rs` here runs on
      Lumen's machines and CI.
- [ ] MR template: the dependency checkbox links it. README "Layout": `SECURITY.md`, `deny.toml`,
      `.gitleaks.toml`, `osv-scanner.toml`, `.githooks/`.

### Step 11: The `v1.3.1` release

- [ ] `release` `needs`: add `audit`, `scan` and `secrets`.
- [ ] Bump to `1.3.1` in `pubspec.yaml`, `rust/Cargo.toml`, `rust/Cargo.lock` and `example/pubspec.lock`.
      `CHANGELOG.md`: rustls 0.23.45 (RUSTSEC-2026-0285), no API change. GAPS: tick "No release pipeline"
      §6 if this closes it. README "CI and releases": rewrite for the new jobs and protection.
- [ ] Merge, tag `v1.3.1` on `main`, and check the tag pipeline: every check job, then `release`. This is
      the first real run of a `needs` + on-`main` release in either repo, which closes gogdl-lib's GAPS
      entry for it in spirit; note it there.
- [ ] Tell Lumen: pin `v1.3.1`, and drop its `RUSTSEC-2026-0285` entry from `osv-scanner.toml`.

**Done when:** the `v1.3.1` release exists and its tag pipeline is green.

---

## MR 3: `ci/renovate`

### Step 12: `renovate.json` and its validation

- [ ] From gogdl-lib's: weekly, `minimumReleaseAge: "7 days"`, security fixes skip the wait, majors off,
      0.x minors treated as majors (cargo), one grouped minor/patch/digest MR, `lockFileMaintenance` off,
      `pinDigests` for `gitlabci` and `dockerfile`.
- [ ] `enabledManagers: ["cargo", "pub", "gitlabci", "dockerfile"]`, `rangeStrategy: "update-lockfile"`.
      No toolchain regex manager (D4).
- [ ] `ignoreDeps`: `gogdl-lib`, `gogdl-flutter-ci`, `flutter_rust_bridge`, `flutter_rust_bridge_hooks`
      (D4, D5). `constraints`: `rust` `1.98.1`, `flutter` `3.47.2` (what Renovate installs to run cargo
      and pub).
- [ ] `renovate-config` job (`--strict`, on `renovate.json` changes), plus `release`'s optional `needs`.

**Done when:** the validator passes in the MR pipeline.

### Step 13: Renovate job and schedule

- [ ] `renovate` job from gogdl-lib (environment `renovate`, CA handling, token check). **And the
      `gogdl-lib` deploy key:** cargo has to fetch the git dependency to rewrite `rust/Cargo.lock`. Copy
      how Lumen's `renovate` job loads its key, and Lumen's guard that fails the job when Renovate aborts
      (Lumen `d9e4b54`).
- [ ] Variables `RENOVATE_TOKEN` and `GITHUB_COM_TOKEN` (protected, masked, scope `renovate`), with calendar
      reminders. `main` is already protected (step 5).
- [ ] Schedule "Renovate", `SCHEDULE=renovate`. Do a dry run first (`RENOVATE_DRY_RUN=full`): check the
      pub manager on a root `pubspec.yaml` with no lock file (only `example/pubspec.lock` exists), the
      ignored deps, and the 0.x rule. Then a live run.
- [ ] Bite test: a throwaway job prints `${RENOVATE_TOKEN:+set}` on an unprotected branch and prints nothing.

**Done when:** a live run opened the grouped MR, its pipeline ran every check job, and it merged through an MR.

---

## MR 4: `ci/github-mirror`

Needs steps 9 and 5, and B1 for step 17.

### Step 14: What the mirror shows

- [ ] `LICENSE-MIT` and `LICENSE-APACHE` replace `LICENSE`. `rust/Cargo.toml`: `license`, `description`,
      `repository` (the GitHub URL, as the lib). `pubspec.yaml`: `repository`.
- [ ] README: "This repository is a mirror" (as the lib's), and how to build outside the LAN with Lumen's
      `insteadOf` line. `deny.toml` licenses: this crate's own license.

### Step 15: GitHub side (settings)

- [ ] Create `fernando-navarrete/gogdl_flutter`, empty, public, with a "mirror" description. Issues,
      Projects, Wiki and Actions off. Ruleset on `main` and `v*`: no deletion, no force push, except the
      mirror token. Make sure a restore can still get through, unlike gogdl-lib's (B1).
- [ ] Fine-grained token, `Contents: read and write` on that repo only, with an expiry and a reminder.

### Step 16: GitLab push mirror (settings)

- [ ] Settings → Repository → Mirroring: push, password auth with the token, "Mirror only protected
      branches", "Keep divergent refs" off. "Update now".
- [ ] Check: `main` at the same SHA, and **every `v*` tag at the same SHA as GitLab** (compare
      `git ls-remote` on both, not just tag names). No other branches.

### Step 17: The `gogdl-lib` pin check

- [ ] In `release`, before the release is created: read the `gogdl-lib` tag and SHA from `rust/Cargo.lock`,
      and fail unless `git ls-remote https://github.com/fernando-navarrete/gogdl-lib refs/tags/<tag>`
      gives the same SHA (peeled). This is Lumen's B2/B4: a bridge tag never pins a dependency that GitHub
      doesn't have.
- [ ] Check it locally against `v1.2.3`: red while B1 is open, green once it's fixed.
- [ ] README and SECURITY.md: the mirror, its token in the rotation list, the release-order rule.

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
