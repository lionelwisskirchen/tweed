#!/usr/bin/env bash
set -eu

pushd "$(dirname "$0")/../tweed" > /dev/null

cargo test --test spec_tests  --package tweed
cargo test --test custom_page_size_tests --features custom_page_size  --package tweed
cargo test --test ota_tests  --package tweed
popd > /dev/null
