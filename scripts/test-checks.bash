#!/bin/bash
set -eu

dir=$(pwd)

pushd "$(dirname "$0")/../tweed" > /dev/null

cargo test --package tweed

popd > /dev/null
