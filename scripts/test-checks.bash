set -eu

dir=$(pwd)

pushd "$(dirname "$0")/../inplace_wasm_interpreter" > /dev/null

cargo test

popd > /dev/null
