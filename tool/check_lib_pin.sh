#!/bin/sh
# Release gate for the gogdl-lib pin (Lumen's GITHUB-MIRROR.md B2). Reads the gogdl-lib tag and SHA
# that rust/Cargo.lock (or the file given as $1) pins, and fails unless gogdl-lib's GitHub mirror (or
# the remote given as $2) has that tag at the same commit, peeled if it is annotated. A bridge tag
# then never pins a gogdl-lib tag that builds off the LAN can't fetch, or fetch at another commit.
# Usage: tool/check_lib_pin.sh [lockfile] [remote]   (run from the repo root)
set -eu

lock="${1:-rust/Cargo.lock}"
remote="${2:-https://github.com/fernando-navarrete/gogdl-lib.git}"

source=$(awk '
/^\[\[package\]\]/ { in_lib = 0; next }
/^name = "gogdl-lib"$/ { in_lib = 1; next }
in_lib && /^source = / { v = $0; sub(/^source = "/, "", v); sub(/"$/, "", v); print v; exit }
' "$lock")
if [ -z "$source" ]; then
  echo "error: no gogdl-lib package with a source in $lock" >&2
  exit 1
fi

case "$source" in
  git+*\?tag=*#*) ;;
  *) echo "error: gogdl-lib is not pinned by tag in $lock: $source" >&2; exit 1 ;;
esac
tag="${source#*\?tag=}"
tag="${tag%%#*}"
sha="${source##*#}"

refs=$(git ls-remote "$remote" "refs/tags/$tag" "refs/tags/$tag^{}")
remote_sha=$(printf '%s\n' "$refs" | awk -v p="refs/tags/$tag^{}" '$2 == p { print $1; exit }')
if [ -z "$remote_sha" ]; then
  remote_sha=$(printf '%s\n' "$refs" | awk -v p="refs/tags/$tag" '$2 == p { print $1; exit }')
fi

if [ -z "$remote_sha" ]; then
  echo "error: gogdl-lib $tag is not on $remote: update gogdl-lib's mirror before tagging here" >&2
  exit 1
fi
if [ "$remote_sha" != "$sha" ]; then
  echo "error: gogdl-lib $tag is $remote_sha on $remote, but $lock pins $sha" >&2
  exit 1
fi
echo "ok: gogdl-lib $tag is $sha on $remote"
