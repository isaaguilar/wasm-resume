#!/bin/bash
set -o nounset -o errexit -o pipefail
source .private/env
# cleanup
rm -rf out
rm -rf dist

cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --no-typescript --out-name bevy_game --out-dir dist --target web target/wasm32-unknown-unknown/release/wasm-resume.wasm
cp wasm/* dist/
cp -r assets dist/

aws s3 sync dist/ "$S3_BUCKET"
