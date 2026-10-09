## Summary

<!-- What changes and why. -->

## Checklist

- [ ] The gates in `README.md` "Development" pass locally (fmt, clippy, test, analyze in the root and
      `example/`, the integration test)
- [ ] Lumen sees a change: added a line to `CHANGELOG.md`
- [ ] `rust/src/api/` changed: regenerated the bindings (`README.md` "Regenerating the bindings")
- [ ] `rust/Cargo.lock` or `example/pubspec.lock` changed: the new packages are listed (direct and
      transitive) in the description with the ones that have a `build.rs`, proc macro or build hook
      named, the lockfile diff is skimmed, and each one meets [`SECURITY.md`](SECURITY.md)
