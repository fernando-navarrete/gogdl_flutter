# Security policy

This file covers the supply chain: what a new crate or pub package has to meet, how to keep a dev machine
safe while building here, and what to do when a package we use is reported compromised. The checks that
enforce most of it are in `.gitlab-ci.yml` and described in `README.md` ("Development", "CI and releases").

This is a single-maintainer project on a self-hosted GitLab. Report a problem to the maintainer directly,
not in a public issue.

What we depend on ships further than this repo: `hook/build.dart` runs cargo on `rust/` in every consuming
app's build, and Lumen builds this repo's `rust/Cargo.lock` as it is at the tag it pins. A crate added here
is compiled, and its build-time code run, on Lumen's dev machines and CI too.

## Adding a crate

A new crate in `rust/` has to meet all of these:

- **Needed.** It isn't replaceable by a few lines of our own code.
- **Maintained.** Recent releases, issues answered, and more than one maintainer or a known
  organisation where possible.
- **At least 7 days old**, the same wait Renovate applies. Cargo has no `--before`: pick the version on
  the crate's crates.io versions page, add it with `cargo add <crate>@=<version>` in `rust/` (relax the
  `=` once it is in), and never run a bare `cargo update` to get it.
- **Few transitive crates.** Check `cargo tree -e normal -i <crate>` and read the `Cargo.lock` diff;
  prefer the smaller option, and `default-features = false` where the crate allows it.

`gogdl-lib` is the exception: it is ours, pinned by tag, and a bump is an API change done by hand
(`ROADMAP.md`, "Versioning rules").

In the MR, list the new crates (direct and transitive) and skim the lockfile diff; the MR template has
a checkbox for it. CI then runs everything with `--locked`, `audit` (`cargo deny` per `rust/deny.toml`:
RustSec advisories, yanked crates, crates.io plus `gogdl-lib` as the only sources, the license
allowlist), `scan` (OSV-Scanner fails on any `MAL-` advisory in `rust/Cargo.lock`) and `secrets`
(gitleaks).

### Build-time code

Cargo has no `ignore-scripts`: a `build.rs` or a proc macro runs code on every machine and CI job that
builds the crate, and through `hook/build.dart` that includes every app that depends on this package. 64
crates in the graph have one today. Say in the MR which new crates do. List them in `rust/` with:

```sh
cargo metadata --locked --format-version 1 \
  | jq -r '.packages[] | select(any(.targets[].kind[]; . == "custom-build" or . == "proc-macro")) | .name'
```

## Adding a pub package

A new package in `pubspec.yaml` or `example/pubspec.yaml` has to meet all of these:

- **Needed**, **maintained**, and **at least 7 days old**, as for a crate.
- **A verified publisher** on pub.dev where possible.
- **Few transitive dependencies.** Check `fvm flutter pub deps` and the `example/pubspec.lock` diff, and
  prefer the smaller option.
- **A dev dependency** unless the bridge needs it at runtime. The root has no committed lock file (this
  is a package): a runtime dependency is resolved again in Lumen's `pubspec.lock`, so it widens Lumen's
  graph, not only ours.

A package with its own `hook/build.dart` runs code at build time, like a `build.rs`: name it in the MR.

CI then checks that every package in `example/pubspec.lock` comes from pub.dev with a sha256 (or is this
bridge, by path) with `tool/check_lockfile.sh`, resolves it with `pub get --enforce-lockfile`, and scans it:
`scan` fails on **any** advisory for a pub package, since nothing else gates them. To accept a finding, add
it to `osv-scanner.toml` with a `reason` and an `ignoreUntil` date (a month or less unless a fix is known to
be further away), so it comes back on its own:

```toml
[[IgnoredVulns]]
id = "GHSA-xxxx-xxxx-xxxx"
ignoreUntil = 2026-12-31
reason = "What it is, why it doesn't apply or can't be fixed yet, and what fixes it."
```

## Dev machines

- Keep no long-lived tokens in the shell environment while building here: `GITLAB_TOKEN`,
  `GITHUB_TOKEN`, cloud keys such as `AWS_*`. Quick check: `env | grep -iE 'token|secret|key'`.
- Build and test with `--locked`. Never `cargo install` without `--locked`. Run `cargo update` only on
  purpose and with `-p <crate>`: a plain one skips the 7-day wait for every transitive crate.
- In `example/`, `fvm flutter pub get --enforce-lockfile` unless you mean to change the lock file.

## If a package we use is reported compromised

1. **Find it.** `cargo tree -i <crate>` in `rust/`, and grep `rust/Cargo.lock` and `example/pubspec.lock`
   for the bad name and versions on `main`, on open branches and at the tag Lumen pins.
2. **Did a build run it?** Look at the job logs of `rust`, `codegen`, `integration`, `audit`, `dart` and
   `renovate` (it runs cargo and pub to rewrite the lock files) since the bad version was published,
   and at local dates (`ls -l --time-style=full-iso ~/.cargo/registry/src/*/<crate>-<version>`, `~/.pub-cache`). Lumen's
   builds ran it too if its pinned tag has it.
3. **Rotate secrets** if it could have run, for everything the machine or job could reach:
   `GOGDL_LIB_DEPLOY_KEY_B64` (generate a new key, replace the deploy key on `gogdl-lib`, update the
   variable), `RENOVATE_TOKEN` (a new project access token) and `GITHUB_COM_TOKEN` (a new GitHub
   token) if `renovate` ran it, and local tokens and SSH keys. The job token expires with the job.
4. **Pin a known-good version**, through an MR as usual: `cargo update -p <crate> --precise <version>`
   for a crate; an exact version in `pubspec.yaml`, or a `dependency_overrides` entry for a transitive
   one, for a pub package.
5. **Scan.** Play the weekly scan by hand (CI/CD, Schedules, "Weekly scan", Play) and check `audit` and
   `scan`.
6. **Cut a patch.** Lumen builds the `rust/Cargo.lock` of the tag it pins, so the fix reaches it only
   through a new tag (README "CI and releases"). Tell Lumen to pin it.
