#!/bin/bash

cd ..

cargo build --release --no-default-features --target wasm32-unknown-unknown
cp -R assets dist/

wasm-bindgen --out-dir ./dist/ --target web ./target/wasm32-unknown-unknown/release/game.wasm 
