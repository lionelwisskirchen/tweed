#!/bin/bash


set -eu

dir=$(pwd)

pushd "$(dirname "$0")/../tweed" > /dev/null
cargo +nightly llvm-cov --branch --open 
popd > /dev/null

