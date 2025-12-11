#!/usr/bin/env bash

set -eu

pushd "$(dirname "$0")/../tweed" > /dev/null

cargo clippy -- -D warnings

popd > /dev/null
