# RF-106 RackForge component

This crate is the RackForge-facing adapter for RF-106. It implements
`rackforge_plugin_sdk::Processor` and compiles to `wasm32-unknown-unknown`.
It owns lifecycle, MIDI decoding, sample-accurate automation, preset selection
and opaque state serialization.

The component depends only on RF-106's platform-neutral contract and DSP crates.
Public parameter indices are stable and append-only. The current state begins
with the `R106` signature and uses RF-106 state schema 1.

The custom panel lives in the separate `rf-106-web` crate. Its controls,
RackForge `postMessage` protocol, search and visualizations compile from Rust to
`app_bg.wasm`. HTML and CSS are static resources; `app.js` is generated loader
glue rather than maintained UI source.
