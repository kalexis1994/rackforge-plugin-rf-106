# RF-106 program transfer

RF-106 uses the original Roland JUNO-106 tone representation as its portable
program payload: sixteen seven-bit potentiometer values and two packed switch
bytes. A RackForge Program document keeps those eighteen bytes unchanged, plus
the user-facing name and versioned plugin metadata.

This choice gives each format one job:

- the RackForge Program document retains the name, catalog identity and schema;
- the embedded eighteen-byte tone remains lossless and hardware-shaped;
- `.syx` artifacts wrap the same tone in the original 24-byte APR message;
- MIDI channel, bender, tuning, allocation, key transpose, physical volume and
  host output remain instance/performance state rather than travelling with a
  tone.

## CONFIG workflow

The plugin declares an optional `program-bank` file resource. In CONFIG,
**INSTALL SYSEX BANK** asks RackForge for a `.syx` file. The surface sees only a
host grant, never the source path. RackForge delivers the bytes to a replacement
plugin instance, which accepts complete Roland APR messages (`0x30` program or
`0x31` manual) and publishes their tones under **Imported SysEx**. Factory
programs stay available. Imported tones are editable copies, so saving one
creates a normal host-owned Program under **Your programs**.

SysEx APR has no name field. An imported tone is therefore labelled from its
hardware patch number (`Imported A11` through `Imported B88`). Its RackForge
name can be changed when the tone is saved.

Every Program save writes these derived artifacts under the plugin's RackForge
data namespace:

```text
programs/<program-id>.syx
exports/rf106-programs.syx
exports/rf106-factory.syx
```

The individual file uses the original manual-buffer message (`0x31`) so it can
be auditioned without assigning a permanent hardware slot. The bank files are
concatenated original program reports (`0x30`) numbered from zero. The factory
export contains the exact 2,304 tone bytes from the imported original bank;
their SHA-256 is
`394ae874da33aa63fa4833932fbf415546d2ad66b1b6b9a36315601799eeec21`.

## Compatibility boundary

RackForge's real-time plugin ABI deliberately carries channel MIDI messages and
does not carry arbitrary SysEx on the audio callback. RF-106 therefore supports
original SysEx through files and Program artifacts. Live IPR slider messages
(`0x32`) can be added when the host exposes a bounded control-plane SysEx route;
the program format will not need to change.

The original PWM byte reaches 105 at the physical slider limit. RF-106 stores
the byte unchanged and maps it to the engine's normalized `0.0..1.0` range only
while sounding the tone. This also keeps imported out-of-range-but-valid
seven-bit values intact if a librarian needs to round-trip them.
