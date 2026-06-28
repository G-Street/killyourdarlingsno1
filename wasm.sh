#!/bin/sh

set -eux -o pipefail

cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./export/main/ --target web ./target/wasm32-unknown-unknown/release/main.wasm
cp -f ./index.html ./export/main/index.html
cp -rf ./assets ./export/main/assets/
zip -r ./export/main.zip ./export/main/
