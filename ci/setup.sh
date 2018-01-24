#!/usr/bin/env bash
set -euxo pipefail

cd "$(dirname "$0")/.."

if [[ "$(uname)" = "Darwin" ]] ; then
    brew update
    brew install yarn
fi

if [[ "${TRAVIS_RUST_VERSION:-}" = "nightly" ]] ; then
    cargo install clippy --force
    cargo install rustfmt-nightly --force
fi
