#!/bin/sh

cargo build --release --target wasm32-unknown-unknown --example button
wasm-bindgen --out-dir ./export/button/ --target web ./target/wasm32-unknown-unknown/release/examples/button.wasm
cp -f ./index.html ./export/button/index.html
zip -r ./export/button.zip ./export/button/