# RF-106 circuit-fidelity gap matrix

| Block | Current baseline | Open evidence gap | Audible risk | Can close without hardware? | Closure test |
| --- | --- | --- | --- | --- | --- |
| Control timing | Firmware-derived scan/control period and DAC words | Hardware jitter and analog settling distribution | Low to medium | Nominal: yes; distribution: no | Executed fixture plus settling-bound test |
| Saw/pulse/sub | Divider-correct oscillator with explicit PWM control; SUB follows its physical level DAC directly, with no synthetic enable gate | Wave-generator transfer, comparator curvature and level tracking | Medium | Mostly | 50%/95% PWM factory targets, SUB-only preset regression and spectrum sweeps |
| Noise | Deterministic excitation and panel level | Spectrum and absolute calibrated level | Medium | Nominal target: yes; texture: partly | 4 Vpp factory target and band-power tests |
| Four-pole filter | Correct topology with provisional input trim/resonance map and continuously running idle-state settling across program selection/state restore | Exact program-change settling time, CV law, self-oscillation onset/amplitude, bass loss and per-voice width/frequency trim | High | Nominal: mostly; variance: no | B35 SUB-only host-output regression plus factory 4.8 Vpp resonance, 248 Hz frequency and 992 Hz width targets |
| Voice amplifier | Nonlinear cell with provisional component behavior | Gain curve, offset, leakage, noise and trim distribution | Medium to high | Nominal: mostly; variance: no | Bias, offset and 6 Vpp gain targets |
| Chorus clock/LFO | Dual mirrored MN3101/MN3009 paths driven by TP3/TP4; exact `fosc -> fcp/2 -> 128/fcp` chain; v41 re-solves and attributes the rejected correlated target, then fixes source-search domains: Tr19 through 28.14 V/0.255 mA, Tr23 reverse VCE -39..-34 mV at 54-74 uA, Tr22 saturation 0.080-0.211 V with Mitsubishi base drive near 260 uA | Obtain bounds covering those exact domains in priority order Tr19, Tr23, Tr22, then sweep them through the stable nonlinear solve | High | Nominal: partly | A primary-evidence-bounded, re-solved 32-assignment BJT family must pass KCL plus voltage/current/power gates before any MN3101/transient/audio sweep; forward-only or low-voltage evidence cannot be extrapolated into the missing domains |
| Chorus bandwidth | Fixed board filters plus clock-dependent intrinsic BBD low-pass | Higher-order device response and unit spread | Low to medium | Nominal: yes; variance: no | Current 14 kHz at 40 kHz ratio test plus future E4 sweeps |
| Chorus distortion/noise | Physical-voltage bounded transfer reproduces 0.3% THD at 0.78 Vrms and 2.5% at the 1.7 Vrms input-swing point; no explicit noise floor | Bias dependence, clock feedthrough, noise color and unit spread | Medium | Nominal THD: yes; convincing variance/noise: no | Two-point harmonic test is closed; future E4 level/bias sweeps and noise spectrum |
| Output amplifier | Documented rails and a bounded large-signal stage | Datasheet slew disagreement, actual load and board gain | Low in normal use, high near clipping | Yes | Slew, swing and frequency-response tests |
| Switched high-pass | Circuit-derived selectable responses | Switching transient and component tolerance | Low to medium | Nominal: yes | Per-position response and continuity tests |
| Voice-to-voice character | Independent deterministic seeds | Real trim/tolerance distribution across six cards | Medium | No | Multi-unit E4 capture set |
| Bender performance control | Spring-centred bipolar lever drives the firmware-derived DCO/VCF bend path; forward push drives the physical LFO Trigger ramp; MIDI Pitch Bend and CC1 retain their documented independent inputs | Mechanical dead-zone and unit-to-unit lever travel | Low | Nominal: yes; variance: no | Endpoint/centre mapping, release-to-centre and pointer/touch regression |

## Priority

1. Chorus active-network BJT model bounds and 32 operating points.
2. Chorus clock-dependent bandwidth and operating envelope.
3. Filter/VCA factory-target harness.
4. Output amplifier reconciliation.
5. Oscillator/PWM factory-target harness.
6. Remote capture kit for real-unit distributions.
