#!/usr/bin/env bash


set -eu

pushd "$(dirname "$0")/../tweed" > /dev/null

cargo fmt -- --check 

popd > /dev/null

