#!/bin/bash


set -eu

dir=$(pwd)

pushd "$(dirname "$0")/../tweed" > /dev/null

cargo fmt -- --check 

popd > /dev/null

