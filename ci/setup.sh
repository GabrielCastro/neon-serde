#!/usr/bin/env bash
set -euxo pipefail

cd "$(dirname "$0")/.."

if [[ "$(uname)" = "Darwin" ]] ;
    brew update
    brew install yarn
fi
