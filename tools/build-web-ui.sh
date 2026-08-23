#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
input_wasm="$repo_root/target/wasm32-unknown-unknown/release/rf_106_web.wasm"
web_root="$repo_root/plugin/package/web"

cd "$repo_root"
cargo build --locked --release --target wasm32-unknown-unknown -p rf-106-web
wasm-bindgen "$input_wasm" --out-dir "$web_root" --out-name app --target web --no-typescript
printf '\n// Generated bootstrap: all UI behavior lives in the Rust WebAssembly module.\n__wbg_init();\n' >> "$web_root/app.js"
printf 'RF_106_WEB_UI_BUILT wasm=%s\n' "$web_root/app_bg.wasm"
