# RF-106 circuit-fidelity plan

This branch is a research and validation lane for the single RF-106 engine. It
does not create an alternate sound mode. A change reaches the product only when
it is better supported than the current implementation and passes the same
public controls, presets and state contract.

## Ground rules

- The current engine is the comparison baseline, not an authority.
- Schematics describe nominal topology and component values; they do not prove
  the distribution of surviving instruments.
- Firmware establishes executed control behavior, not analog transfer curves.
- No production constant may depend on an unidentified recording or a
  secondary implementation alone.
- Every DSP change needs a stated source, derivation, confidence level and an
  automated regression.
- Source documents remain external research material. This repository stores
  only hashes, page references, derived equations and conclusions.

## Evidence levels

| Level | Evidence | Appropriate use |
| --- | --- | --- |
| E1 | Reference-hardware schematic or factory adjustment procedure | Topology, nominal parts, calibration targets |
| E2 | Original semiconductor manufacturer data | Device limits, typical transfer and clock relationships |
| E3 | Executed firmware fixture with a known ROM image | Quantized control behavior, timing and state transitions |
| E4 | Raw capture from an identified, documented hardware unit | End-to-end calibration and real-unit variance |
| E5 | Reproducible circuit derivation or simulation based on E1/E2 | Values not printed directly in a source |
| E6 | Secondary implementation, prose report or unidentified audio | Hypothesis generation only |

Confidence is recorded separately from evidence level: `high` means directly
specified and unambiguous; `medium` means a reproducible derivation is needed;
`low` means the result still depends on an unverified assumption.

## Work blocks

### A. Evidence baseline

1. Maintain the source ledger and gap matrix.
2. Record the baseline test result and package hash before each audible change.
3. Keep temporary scans and renders out of Git.

Exit criterion: every provisional constant has an owner block and a closure
test.

### B. Stereo chorus

1. Derive the 256-stage delay/clock relation and allowed clock range.
2. Separate fixed anti-alias/reconstruction networks from BBD bandwidth that
   follows clock frequency.
3. Reconstruct oscillator waveforms and switch combinations from the circuit.
4. Add signal-level, bias, bandwidth, modulation-range and transition tests.
5. Keep reduced/transient circuit solvers in the offline circuit laboratory;
   promote their output only after the complete network closes.
6. Treat source-reading corrections and revision conflicts as first-class test
   fixtures; each candidate value must survive topology and operating-range
   sanity checks before it can invalidate an existing derivation.

Exit criterion without hardware: all nominal values are E1/E2/E5 and the model
obeys the device operating envelope. E4 remains necessary for unit-specific
rate, depth, noise and distortion distributions.

### C. Output and high-pass path

1. Reconcile the amplifier large-signal limits, slew rate and load behavior.
2. Derive each switched high-pass transfer from the resistor/capacitor network.
3. Test frequency response, switching continuity and stereo gain matching.

### D. Voice filter and amplifier

1. Fit the four-pole topology to the factory resonance, gain, frequency and
   width procedures.
2. Separate nominal control law from voice-card trim offsets.
3. Model self-oscillation onset, resonance bass loss, VCA leakage and noise only
   where the evidence supports them.

### E. Oscillator and pulse-width path

1. Preserve the firmware-derived divider and DAC timing.
2. Derive the saw, pulse and sub transfer from the wave-generator circuit.
3. Use the factory 50% and 95% PWM procedures as invariant tests.

### F. Variance and remote capture

1. Provide a reproducible capture project for owners of reference hardware.
2. Archive raw dry recordings plus interface calibration, serial-independent
   unit ID, temperature/warm-up time and control settings.
3. Estimate distributions separately from the nominal model; never bake one
   arbitrary unit into every voice.

## Merge gate

A circuit-fidelity commit is eligible for the main branch only if:

1. the old and new behavior are reproducible from committed tests;
2. the new behavior has E1-E5 support and is not justified by E6 alone;
3. supported sample rates remain finite and real-time safe;
4. presets, parameter indices and serialized state do not change accidentally;
5. the full workspace tests and `.rfplugin` build pass.
