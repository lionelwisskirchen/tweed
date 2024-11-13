#!/bin/bash
set -eu

dir=$(pwd)

pushd "$(dirname "$0")/../inplace_wasm_interpreter" > /dev/null

cargo test --package inplace_wasm_interpreter

popd > /dev/null
