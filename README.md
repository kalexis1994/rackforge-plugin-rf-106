# RackForge RF-106

RF-106 is a portable six-voice polyphonic synthesizer for RackForge. Its audio
engine, control model, host adapter and interactive panel are implemented in
Rust and ship as a WebAssembly component with a Rust/WebAssembly web interface.

RF-106 is a subtractive synthesizer rather than a PCM or sample-ROM player. Each
voice generates pulse, saw, sub-oscillator and noise signals, then shapes them
through high-pass and resonant low-pass filters, an envelope-controlled
amplifier and a stereo chorus. The implementation is circuit-informed and is
validated with deterministic DSP tests and repeatable offline experiments.

## Features

- Six-voice polyphony with two allocation modes, key transpose and portamento.
- Six persistent RackForge render units, one per physical voice, scheduled in
  parallel before the deterministic analog sum and stereo chorus stages.
- Pulse and saw oscillators, sub-oscillator, noise, PWM and LFO modulation.
- Four-position high-pass filter and a resonant, envelope-modulated low-pass
  filter with keyboard tracking.
- ADSR envelope, envelope/gate amplifier modes and output-stage modelling.
- Two stereo chorus modes with clock, delay-line and compander behaviour.
- Performance bender with pitch bend and LFO trigger gestures.
- 128 factory programs exposed through the panel and RackForge preset browser.
- Eight cassette-shaped banks for original 18-byte JUNO-106 tone Programs,
  with independent `.106`/`.syx` import and hardware-compatible SysEx export.
- Mouse, touch and keyboard interaction, plus host-owned MIDI parameter linking.

## Repository layout

```text
crates/rf-106-contract/  Public parameters, factory programs and state contract
crates/rf-106-control/   MIDI, keyboard modes and voice assignment
crates/rf-106-voice/     Oscillator, filter, amplifier and envelope voice model
crates/rf-106-chorus/    Stereo chorus model
crates/rf-106-output/    Output-stage model
crates/rf-106-dsp/       Six-voice synthesis engine
plugin/                  RackForge processor and package resources
plugin-ui/               Rust/WebAssembly control surface
tools/rf-106-metadata/   Deterministic metadata generator
```

The plugin exposes 128 factory programs and the complete synthesis panel: LFO,
DCO, HPF, VCF, VCA, envelope, chorus, performance controls, MIDI channel and
output controls. Desktop, touch and compact RackForge surfaces operate on the
same public parameter contract.

Program transfer preserves the original Roland tone bytes inside versioned
RackForge Program documents. CONFIG can install complete APR dumps as a bank;
every saved Program also produces hardware-shaped single-tone and bank `.syx`
artifacts. See [`docs/PROGRAM_TRANSFER.md`](docs/PROGRAM_TRANSFER.md).

## Fidelity work

The audio path is kept separate from the visual panel so circuit hypotheses can
be measured and replaced without changing the public parameter contract. The
current evidence, open questions and acceptance thresholds are recorded in
[`docs/CIRCUIT_FIDELITY_PLAN.md`](docs/CIRCUIT_FIDELITY_PLAN.md) and the
[`docs/circuit-fidelity/`](docs/circuit-fidelity/) laboratory notes. These notes
distinguish sourced behaviour, bounded inference and unresolved uncertainty.

## Build and test

A RackForge checkout is expected next to this repository. For local development,
copy `.cargo/config.toml.example` to `.cargo/config.toml` to use that SDK checkout.

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --release --workspace
cargo install wasm-bindgen-cli --version 0.2.127 --locked
bash tools/build-package.sh
```

On Windows:

```powershell
Copy-Item .cargo/config.toml.example .cargo/config.toml
cargo +stable-x86_64-pc-windows-msvc install wasm-bindgen-cli --version 0.2.127 --locked
./tools/build-package.ps1
```

The package is written to `artifacts/rf-106-0.2.19.rfplugin`. It contains the
audio component, versioned metadata and the plugin-owned web interface; it does
not contain a platform-specific executable.

GitHub Actions also builds the portable package after the x86-64 and ARM test
jobs pass. Every workflow run publishes `RF-106.rfplugin` together with its
SHA-256 checksum as a downloadable artifact retained for 30 days.

The RackForge icon, banner and splash use original artwork based on the plugin
UI's palette, split fader markers and shared graduations. Only the final package
assets are stored in the repository. Validate their dimensions, encoding and
file-size limits with Pillow:

```bash
python tools/generate-branding.py
```

## License

RF-106 is distributed under GPL-3.0-only. See [LICENSE](LICENSE) and
[NOTICE.md](NOTICE.md).
