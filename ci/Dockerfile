# CI image for .gitlab-ci.yml: every system package, the Rust toolchain, the frb codegen and
# Flutter, so jobs don't hit the network for them. Build it on the runner host with
# ci/build-image.sh (see the header of .gitlab-ci.yml).
FROM debian:bookworm-slim@sha256:7c7b2c966bc9ee8cedfeef67e0e279108992c77681fa595db4a9d65c06ccc587

ARG FLUTTER_VERSION
ARG RUST_VERSION
ARG FRB_VERSION=2.13.0
# The commit FLUTTER_VERSION's tag must point to (build-image.sh passes it in from its table), so a
# moved tag fails the build instead of shipping a different SDK.
ARG FLUTTER_COMMIT
# rustup-init is pinned by version and sha256. When bumping, take both from
# https://static.rust-lang.org/rustup/archive/<ver>/x86_64-unknown-linux-gnu/rustup-init.sha256
# (the hash stays hardcoded: one fetched from the same host would only prove the transfer).
ARG RUSTUP_VERSION=1.29.1
ARG RUSTUP_SHA256=dda7234360b7f578ca8b0ddcb80145646fa61a67c1720a5abc7051b35c9fcb71

# build-essential/cmake/pkg-config: aws-lc-sys and friends. The rest builds and runs the Linux
# desktop example under xvfb.
RUN apt-get update -qq \
 && apt-get install -qq -y --no-install-recommends \
      git curl unzip xz-utils ca-certificates openssh-client \
      build-essential cmake pkg-config \
      clang ninja-build libgtk-3-dev liblzma-dev libstdc++-12-dev xvfb xauth \
 && rm -rf /var/lib/apt/lists/*

# The toolchain is the one pinned in rust/rust-toolchain.toml (build-image.sh passes it in):
# the Native Assets build hook runs cargo through rustup, which would download a second
# toolchain if the image had a different one. The target must match the toml's `targets`.
ENV RUSTUP_HOME=/opt/rustup \
    CARGO_HOME=/opt/cargo \
    FLUTTER_HOME=/opt/flutter
ENV PATH=/opt/cargo/bin:/opt/flutter/bin:$PATH

RUN test -n "$RUST_VERSION" \
 && curl -sSf -o /tmp/rustup-init \
      "https://static.rust-lang.org/rustup/archive/$RUSTUP_VERSION/x86_64-unknown-linux-gnu/rustup-init" \
 && echo "$RUSTUP_SHA256  /tmp/rustup-init" | sha256sum -c - \
 && chmod +x /tmp/rustup-init \
 && /tmp/rustup-init -y --no-modify-path --profile minimal \
      --default-toolchain "$RUST_VERSION" -c rustfmt -c clippy -t x86_64-unknown-linux-gnu \
 && rm /tmp/rustup-init \
 && echo "$RUST_VERSION" > /opt/rust-version \
 && cargo install flutter_rust_bridge_codegen --version "$FRB_VERSION" --locked \
 && rm -rf /opt/cargo/registry /opt/cargo/git

RUN test -n "$FLUTTER_VERSION" && test -n "$FLUTTER_COMMIT" \
 && git clone --depth 1 -b "$FLUTTER_VERSION" https://github.com/flutter/flutter.git "$FLUTTER_HOME" \
 && have=$(git -C "$FLUTTER_HOME" rev-parse HEAD) \
 && { [ "$have" = "$FLUTTER_COMMIT" ] \
      || { echo "Flutter tag $FLUTTER_VERSION is at $have, not $FLUTTER_COMMIT: the tag moved" >&2; exit 1; }; } \
 && git config --system --add safe.directory "$FLUTTER_HOME" \
 && flutter config --no-analytics >/dev/null \
 && flutter precache --linux \
 && echo "$FLUTTER_VERSION" > /opt/flutter-version

# cargo-deny for the `audit` job, built from source so the job installs nothing. When bumping,
# take a version at least 7 days old from crates.io, and bump IMAGE_REVISION in build-image.sh.
ARG CARGO_DENY_VERSION=0.20.2
RUN cargo install --locked "cargo-deny@$CARGO_DENY_VERSION" \
 && rm -rf /opt/cargo/registry /opt/cargo/git
