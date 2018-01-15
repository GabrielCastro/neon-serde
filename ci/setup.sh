#!/usr/bin/env bash
set -euxo pipefail

cd "$(dirname "$0")/.."

if [[ "$(uname)" = "Darwin" ]] ; then
    brew update
    brew install yarn
fi

which rustfmt || cargo install rustfmt-nightly

if [[ "${TRAVIS_RUST_VERSION:-}" = "nightly" ]] ; then
    which cargo-clippy || cargo install clippy
fi
