#!/usr/bin/env bash
set -euxo pipefail

cd "$(dirname "$0")/.."

export RUST_BACKTRACE=1

cargo build --verbose --all
cargo test --verbose --all

cd test

yarn install
yarn run build:debug
yarn test
