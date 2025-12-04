#!/usr/bin/env bash

set -eu

dir=$(pwd)

pushd "$(dirname "$0")/../tweed" > /dev/null

cargo clippy -- -D warnings

popd > /dev/null
