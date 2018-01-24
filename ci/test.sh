#!/usr/bin/env bash
set -euxo pipefail

cd "$(dirname "$0")/.."

export RUST_BACKTRACE=1

if [[ "${TRAVIS_RUST_VERSION:-}" = "nightly" ]] ; then
    cargo fmt -- --write-mode=diff
    cargo clippy
fi
cargo build --verbose --all
cargo test --verbose --all

cd test

yarn install
yarn run build:debug
yarn test

cd ../test_macro

yarn install
yarn run build:debug
yarn test
