#!/usr/bin/env bash
set -euxo pipefail

cd "$(dirname "${BASH_SOURCE[0]}")/.."

export RUST_BACKTRACE=1

if [[ "${TRAVIS_RUST_VERSION:-}" = "nightly" ]] ; then
    cargo fmt -- --write-mode=diff
    cargo clippy
fi
cargo build --verbose --all
cargo test --verbose --all

cd test

(
    cd native
    if [[ "${TRAVIS_RUST_VERSION:-}" = "nightly" ]] ; then
         cargo fmt -- --write-mode=diff
         cargo clippy
    fi
)

yarn install
yarn run build:debug
yarn test
