#!/usr/bin/env bash
set -euxo pipefail

cd "$(dirname "$0")/.."

if [[ "$(uname)" = "Darwin" ]] ; then
    brew update
    brew install yarn
fi

cargo install rustfmt

if [[ "${TRAVIS_RUST_VERSION:-}" = "nightly" ]] ; then
    which rustfmt || cargo install clippy
fi
