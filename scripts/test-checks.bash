#!/usr/bin/env bash
set -eu

pushd "$(dirname "$0")/../tweed" > /dev/null

cargo test spec_tests  --package tweed
cargo test custom_page_size_tests --features custom_page_size  --package tweed

popd > /dev/null
