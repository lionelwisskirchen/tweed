#!/usr/bin/env bash

set -eu

pushd "$(dirname "$0")/../tweed" > /dev/null
cargo llvm-cov clean --workspace
cargo llvm-cov test --no-report --test spec_tests
cargo llvm-cov test --no-report --features custom_page_size --test custom_page_size_tests
cargo llvm-cov report --cobertura --output-path target/llvm-cov-target/cobertura.xml
cargo llvm-cov report --summary-only
popd > /dev/null
