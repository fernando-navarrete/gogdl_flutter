# CI image for .gitlab-ci.yml: every system package, the Rust toolchain, the frb codegen and
# Flutter, so jobs don't hit the network for them. Build it on the runner host with
# ci/build-image.sh (see the header of .gitlab-ci.yml).
FROM debian:bookworm-slim

ARG FLUTTER_VERSION
ARG RUST_VERSION
ARG FRB_VERSION=2.13.0

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
 && curl -sSf https://sh.rustup.rs | sh -s -- -y --no-modify-path --profile minimal \
      --default-toolchain "$RUST_VERSION" -c rustfmt -c clippy -t x86_64-unknown-linux-gnu \
 && echo "$RUST_VERSION" > /opt/rust-version \
 && cargo install flutter_rust_bridge_codegen --version "$FRB_VERSION" --locked \
 && rm -rf /opt/cargo/registry /opt/cargo/git

RUN test -n "$FLUTTER_VERSION" \
 && git clone --depth 1 -b "$FLUTTER_VERSION" https://github.com/flutter/flutter.git "$FLUTTER_HOME" \
 && git config --system --add safe.directory "$FLUTTER_HOME" \
 && flutter config --no-analytics >/dev/null \
 && flutter precache --linux \
 && echo "$FLUTTER_VERSION" > /opt/flutter-version
