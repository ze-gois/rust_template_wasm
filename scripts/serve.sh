#!/bin/bash
set -e

# 1. build Rust
cargo build --target wasm32-unknown-unknown

# 2. wasm-bindgen
wasm-bindgen target/wasm32-unknown-unknown/debug/rust_template_wasm_bindgen.wasm --out-dir ./pkg --target web

firefox --new-window http://0.0.0.0:8080/

# 3. serve com Python
python3 -m http.server 8080
