#!/bin/sh
# Builds the CI image that .gitlab-ci.yml runs its jobs in.
#
# Usage: ci/build-image.sh [--load-into <container>]
#
# Run it on the runner host (thinkcentre.home) from a checkout. The runner shares the host's
# docker through its mounted docker.sock, so a plain build is all that's needed: jobs see the
# image directly. --load-into <container> is only for a runner with its own docker-in-docker
# daemon: it pipes the image in with `docker save | docker exec -i <container> docker load`.
#
# Run it again after bumping the Flutter version in .fvmrc, the Rust channel in
# rust/rust-toolchain.toml or FRB_VERSION below, or after changing ci/Dockerfile (bump
# IMAGE_REVISION); the tag changes, so .gitlab-ci.yml's `image:` must be updated to the printed name.
set -eu

# Bump whenever ci/Dockerfile changes; never reset on a toolchain bump.
IMAGE_REVISION=4

cd "$(dirname "$0")/.."

load_into=
case "${1:-}" in
  --load-into)
    load_into="${2:-}"
    [ -n "$load_into" ] || { echo "error: --load-into needs a container" >&2; exit 1; }
    ;;
  "") ;;
  *) echo "usage: $0 [--load-into <container>]" >&2; exit 1 ;;
esac

FRB_VERSION=2.13.0
FLUTTER_VERSION=$(sed -n 's/.*"flutter": *"\([^"]*\)".*/\1/p' .fvmrc)
[ -n "$FLUTTER_VERSION" ] || { echo "error: no flutter version in .fvmrc" >&2; exit 1; }

# The commit each Flutter tag points to, checked in ci/Dockerfile after the clone. When bumping .fvmrc,
# add a line from `git ls-remote https://github.com/flutter/flutter.git refs/tags/<version>`.
case "$FLUTTER_VERSION" in
  3.47.2) FLUTTER_COMMIT=d3b14c876900e553bc736ca19295fc09e3853e8e ;;
  *) echo "error: no commit for Flutter $FLUTTER_VERSION: add its commit to the table in $0" >&2; exit 1 ;;
esac

RUST_VERSION=$(sed -n 's/^channel *= *"\([^"]*\)".*/\1/p' rust/rust-toolchain.toml)
[ -n "$RUST_VERSION" ] || { echo "error: no channel in rust/rust-toolchain.toml" >&2; exit 1; }

IMAGE="gogdl-flutter-ci:flutter-$FLUTTER_VERSION-rust-$RUST_VERSION-frb-$FRB_VERSION-r$IMAGE_REVISION"
docker build -t "$IMAGE" \
  --build-arg "FLUTTER_VERSION=$FLUTTER_VERSION" \
  --build-arg "FLUTTER_COMMIT=$FLUTTER_COMMIT" \
  --build-arg "RUST_VERSION=$RUST_VERSION" \
  --build-arg "FRB_VERSION=$FRB_VERSION" \
  ci/
echo "built $IMAGE"

if [ -n "$load_into" ]; then
  docker save "$IMAGE" | docker exec -i "$load_into" docker load
  echo "loaded $IMAGE into $load_into"
fi
