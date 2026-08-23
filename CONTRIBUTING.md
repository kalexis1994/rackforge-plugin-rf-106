# Contributing

Run the complete verification set before submitting a change:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --release --workspace
cargo install wasm-bindgen-cli --version 0.2.127 --locked
bash tools/build-package.sh
```

## Design rules

- Executable plugin, DSP, contract, tooling and maintained UI logic are Rust.
- The real-time audio path performs no allocation, I/O, logging or locking.
- Public parameter IDs and serialized state fields are changed only with an
  explicit contract version.
- Every new MIDI message or parameter needs a deterministic engine test.
- Controller-specific behavior belongs to RackForge controller packages.
- Web and `little@1` surfaces must operate on the same plugin state.
- `plugin/package/web/app.js` and `app_bg.wasm` are generated files.

Do not commit generated audio, build output, packaged binaries, proprietary
patch banks or third-party source material.
