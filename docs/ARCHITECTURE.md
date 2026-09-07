# RF-106 architecture

RF-106 is split into small Rust crates so the audio model stays independent of
the host and the interface.

```text
contract ----------------+--------------> plugin UI
                         |
control ---+             |
voice -----+-> DSP ------+--------------> RackForge processor
chorus ----+
output ----+
```

## Boundaries

- `rf-106-contract` defines stable parameter indices, validation, factory
  programs, the original JUNO-106 SysEx codec, preset IDs and serialized-state
  versioning.
- `rf-106-control` handles MIDI reception, performance modes and voice
  allocation decisions.
- `rf-106-voice`, `rf-106-chorus` and `rf-106-output` model the audio blocks.
- `rf-106-dsp` composes six voices and renders stereo samples without host APIs.
- `plugin` adapts the engine to the RackForge processor ABI, owns dynamic
  imported/custom Program catalogs and contains the host-output safety boundary.
- `plugin-ui` renders the panel and exchanges typed parameter values with
  RackForge.

## Runtime rules

Audio processing performs no dynamic allocation, file access, logging or
locking. Control-thread Program editing and SysEx resource installation may
allocate bounded buffers. Factory programs and metadata are generated
deterministically from the contract crate. The package contains one portable
audio component and one portable UI module.
