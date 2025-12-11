#!/usr/bin/env bash

set -eu

pushd "$(dirname "$0")/../tweed" > /dev/null
cargo llvm-cov test
cargo llvm-cov report --cobertura --output-path target/llvm-cov-target/cobertura.xml
popd > /dev/null

