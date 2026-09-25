#!/bin/sh
# Builds the CI image that .gitlab-ci.yml runs its jobs in.
#
# Usage: ci/build-image.sh [--load-into <container>]
#
# Run it on the runner host (thinkcentre.home) from a checkout. The runner is a container with
# its own docker-in-docker daemon, and job containers only see images in that inner daemon, so
# pass the container that owns it with --load-into: the image is built with the host's docker
# (fast, cached layers) and piped into the inner daemon with `docker save | docker load`. No
# registry is needed. Without the flag it only builds.
#
# Run it again after bumping the Flutter version in .fvmrc or FRB_VERSION below; the tag changes,
# so .gitlab-ci.yml's `image:` must be updated to the printed name. The inner daemon's
# /var/lib/docker should be a volume: if it isn't, the image is lost when that container is
# recreated and jobs fail trying to pull gogdl-flutter-ci:* from Docker Hub. Rerun with
# --load-into to fix it.
set -eu

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

IMAGE="gogdl-flutter-ci:flutter-$FLUTTER_VERSION-frb-$FRB_VERSION"
docker build -t "$IMAGE" \
  --build-arg "FLUTTER_VERSION=$FLUTTER_VERSION" \
  --build-arg "FRB_VERSION=$FRB_VERSION" \
  ci/
echo "built $IMAGE"

if [ -n "$load_into" ]; then
  docker save "$IMAGE" | docker exec -i "$load_into" docker load
  echo "loaded $IMAGE into $load_into"
fi
