#!/bin/bash


set -eu

dir=$(pwd)

pushd "$(dirname "$0")/../inplace_wasm_interpreter" > /dev/null
cargo llvm-cov --fail-under-lines=100 --open 
popd > /dev/null

