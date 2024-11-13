#!/bin/bash

set -eu

dir=$(pwd)

pushd "$(dirname "$0")/../inplace_wasm_interpreter" > /dev/null

cargo clippy -- -D warnings

popd > /dev/null
