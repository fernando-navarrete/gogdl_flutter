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

## 0.0.1

* TODO: Describe initial release.
