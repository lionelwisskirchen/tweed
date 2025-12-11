#!/usr/bin/env bash
set -eu

pushd "$(dirname "$0")/../tweed" > /dev/null

cargo test --package tweed

popd > /dev/null
