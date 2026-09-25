#!/bin/sh
# Release gate for a vX.Y.Z tag. Fails unless pubspec.yaml, rust/Cargo.toml and the tag agree
# and CHANGELOG.md has a non-empty `## X.Y.Z` section; prints that section on stdout.
# Usage: tool/release_notes.sh <tag>   (run from the repo root)
set -eu

tag="${1:-}"
case "$tag" in
  v[0-9]*.[0-9]*.[0-9]*) ;;
  *) echo "error: '$tag' is not a vX.Y.Z tag" >&2; exit 1 ;;
esac
if ! printf '%s\n' "$tag" | grep -Eq '^v[0-9]+\.[0-9]+\.[0-9]+$'; then
  echo "error: '$tag' is not a vX.Y.Z tag" >&2
  exit 1
fi
version="${tag#v}"

pubspec_version=$(sed -n 's/^version: *//p' pubspec.yaml | head -n 1 | tr -d '\r')
cargo_version=$(awk '
  /^\[/ { in_package = ($0 == "[package]"); next }
  in_package && /^version[ \t]*=/ {
    v = $0
    sub(/^version[ \t]*=[ \t]*"/, "", v)
    sub(/".*$/, "", v)
    print v
    exit
  }
' rust/Cargo.toml)

if [ "$pubspec_version" != "$version" ] || [ "$cargo_version" != "$version" ]; then
  {
    echo "error: version mismatch"
    echo "  tag:                $version"
    echo "  pubspec.yaml:       ${pubspec_version:-<not found>}"
    echo "  rust/Cargo.toml:    ${cargo_version:-<not found>}"
  } >&2
  exit 1
fi

notes=$(awk -v heading="## $version" '
  {
    line = $0
    sub(/[ \t\r]+$/, "", line)
  }
  found && /^## / { exit }
  found { print line; next }
  line == heading { found = 1 }
' CHANGELOG.md | awk '
  NF { started = 1 }
  started { lines[++n] = $0 }
  END {
    while (n > 0 && lines[n] !~ /[^ \t]/) n--
    for (i = 1; i <= n; i++) print lines[i]
  }
')

if [ -z "$notes" ]; then
  echo "error: CHANGELOG.md has no non-empty '## $version' section" >&2
  exit 1
fi

printf '%s\n' "$notes"
