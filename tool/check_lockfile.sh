#!/bin/sh
# Lockfile integrity check, adapted from lumen's. Run in CI before `flutter pub get` in example/, so a
# package from anywhere unexpected is never fetched. Reads example/pubspec.lock (or the file given as
# $1) and fails when:
#   - a hosted package isn't from https://pub.dev, or has no sha256;
#   - a path package isn't this bridge (`path: ".."`);
#   - any source other than hosted, path or sdk (the Flutter SDK's own packages) appears.
# `flutter pub get --enforce-lockfile` then guarantees pub resolves to exactly these entries.
# The root pubspec.lock is gitignored (this is a package), so only example/ has one to check.
# Usage: tool/check_lockfile.sh [lockfile]   (run from the repo root)
set -eu

lock="${1:-example/pubspec.lock}"

out=$(awk '
function check() {
  if (name == "") return
  n++
  if (source == "hosted") {
    if (url != "\"https://pub.dev\"") print name ": hosted outside pub.dev: " url
    else if (sha == "") print name ": hosted package has no sha256"
  } else if (source == "path") {
    if (path != "\"..\"") print name ": path package other than the bridge: " path
  } else if (source != "sdk") {
    print name ": unexpected source \"" source "\""
  }
}
/^packages:/ { inpk = 1; next }
!inpk { next }
/^  [^ ]+:$/ { check(); name = $1; sub(/:$/, "", name); source = ""; url = ""; sha = ""; path = ""; next }
/^    source: / { source = $2; next }
/^      url: / { url = $2; next }
/^      sha256: / { sha = $2; next }
/^      path: / { path = $2; next }
/^[^ ]/ { check(); name = ""; inpk = 0 }
END { check(); print "COUNT " n + 0 }
' "$lock")

count=${out##*COUNT }
problems=$(printf '%s\n' "$out" | grep -v '^COUNT ' || true)

if [ -n "$problems" ] || [ "$count" -eq 0 ]; then
	echo "check_lockfile: problem(s) in $lock" >&2
	[ -n "$problems" ] && printf '%s\n' "$problems" | sed 's/^/  /' >&2
	[ "$count" -eq 0 ] && echo "  no packages found" >&2
	exit 1
fi
echo "check_lockfile: $count packages ok"
