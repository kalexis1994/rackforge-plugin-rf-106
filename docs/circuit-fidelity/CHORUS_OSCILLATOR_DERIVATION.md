# Chorus modulation oscillator derivation

## Firm topology

The jack-board schematic contains one free-running low-frequency oscillator,
not one oscillator per chorus mode:

1. one half of IC1 is the threshold/comparator section;
2. the other half is the integrating section around C3;
3. the integrator output is available at TP4;
4. IC2 buffers and inverts that signal to TP3;
5. TP3 and TP4 feed the two clock-control networks through R128 and R148;
6. the front-panel switch lines operate transistor/FET networks around this
   oscillator and the wet path.

Consequences with E1 confidence:

- both delay lines receive opposing versions of the same modulation;
- oscillator phase is shared and free-running across mode changes;
- changing mode may change rate and excursion, but it cannot select a separate
  sinusoidal oscillator.

RF-106 now encodes those invariants directly.

## Firmware state reconstruction

The verified A_5 Assigner ROM closes the front-panel encoding independently of
the schematic. Its routine at `0x0BEB-0x0BF8` copies the three momentary chorus
buttons into switch-1 bits 5-7. The normal one-hot states are therefore:

| Panel button | Assigner switch-1 | Seven-bit value sent to Voice |
| --- | ---: | ---: |
| Off | `0x20` | `0x5F` |
| I | `0x40` | `0x3F` |
| II | `0x80` | `0x7F` |

The Assigner transmitter at `0x0BF9-0x0C0A` complements switch-1 and masks it
to seven bits. The verified B_2 Voice ROM routine at `0x014F-0x0164` then maps
incoming bit 5 to the active-low enable latch and bit 6 to the I/II rate latch.
The resulting latch low bits are `3`, `0` and `2` for Off, I and II.

The Module Board connector truth table closes the electrical meaning of those
bits. The module/jack interface is rail-shifted by the jack-board pull-up and
high-side transistor stage, so logical `0` and `1` resolve there to
approximately 0 V and +15 V, not TTL 0/5 V:

| State | Chorus enable line | I/II rate line |
| --- | ---: | ---: |
| Off | +15 V | don't care (the verified ROM leaves it at +15 V) |
| I | 0 V | 0 V |
| II | 0 V | +15 V |

This also resolves the old synthetic `I+II` mode. A coincident I+II panel scan
would store `0xC0`, transmit `0x3F` and reach exactly the same two Module Board
lines as mode I. It is not a fourth sonic state. The former 7.85 Hz coordinate
was an E6 property of the emulator itself and has been removed. RF-106 exposes
only the three physical states and rejects any other selector value.

## Readable component inventory

The following values are legible on the schematic and define the next circuit
simulation boundary:

| Function | Components |
| --- | --- |
| Comparator/threshold network | R6 47 kOhm, R7 33 kOhm, R15 1 MOhm, D2 |
| Integrator path | R5 1 MOhm, R8 2.2 MOhm, R4 680 kOhm, C3 0.1 uF |
| Mode-controlled branch | Tr1 2SK30A, Tr2, D1, R1 33 kOhm, R2 47 kOhm, R11 150 kOhm |
| Inverting buffer | R10 33 kOhm, R9 33 kOhm, C4 220 pF |
| Clock-control feeds | R128 1 kOhm to channel A, R148 1 kOhm to channel B |

The related-revision parts inventory corroborates TL-062CP for IC1 and
2SK30A-GR or 2SK30A-Y for Tr1. A Toshiba catalog table gives IDSS ranks of
1.2-3.0 mA for Y and 2.6-6.5 mA for GR. Those rank limits corroborate the part
identity but do not uniquely determine low-voltage channel resistance. The
inventory also lists 1SS-133 as the only ordinary
small-signal diode, making it the strongest candidate for D1/D2, but it does
not map that part number to individual designators. The diode assignment stays
at medium confidence rather than being promoted as a direct schematic fact.

The target-hardware 1984 schematic marks R15 as 1 MOhm. A related 1985
106S/HS-60 schematic marks the same designator as 1 kOhm. That is a real source
conflict, not a reason to silently replace the target value: inserting 1 kOhm
into the visible Schmitt network predicts about 37.5 Hz with Tr1 open. The
target value instead produces a chorus-range result and is retained. Both
readings and the sanity test are machine-readable laboratory fixtures.

With D2 reverse-biased in the normal signal range, the comparator input current
is negligible and R7 does not alter the threshold. The reduced ideal core is:

```text
beta = R15 / (R6 + R15)
f = 1 / (4 * beta * R_effective * C3)

Tr1 open:        R_effective = R5 + R8
Tr1 ideal shunt: R_effective = R5 + R8 + R5*R8/R4
```

The comparator swing cancels from this first-order frequency equation because
it scales both the Schmitt threshold and the integrator slope. The TL062 output
swing and slew limits remain relevant to waveform corners and restart behavior,
but not to the nominal ideal rate.

The resulting nominal estimates are 0.818 Hz with Tr1 open and 0.407 Hz with an
ideal Tr1 short. Five-percent resistor and ten-percent capacitor corners expand
those ranges to 0.705-0.961 Hz and 0.333-0.502 Hz respectively. The current
0.842 Hz mode-II coordinate is inside the open-branch interval. The current
0.514 Hz mode-I coordinate is only about 2.4% above the ideal-shunt interval's
upper edge. That small miss is evidence for unresolved component, threshold and
output-swing detail, not evidence for hundreds of kilohms of JFET ON resistance.
The former 483 kOhm inference has therefore been removed.

The control paths are now separated as well. CHORUS ON/OFF drives Tr6 and Tr5,
which clamp TP4 when disabled. The I/II line drives Tr2, D1 and Tr1. In mode I,
0 V turns the high-side Tr2 stage on, D1 is reverse-biased and R3 holds the Tr1
gate near its grounded source: the R4 shunt is active and the oscillator is
slow. In mode II, +15 V turns Tr2 off, R11 pulls its collector negative, D1
pulls the Tr1 gate below its source and the R4 shunt opens: the oscillator is
fast. Tr5 is therefore not an active mode-rate unknown, and the steady-state
mode polarity is no longer provisional.

## Still provisional

The current numerical rate and excursion for I and II remain in
`ChorusCalibration`. They are useful comparison coordinates but are not
promoted by the topology or firmware corrections.

The next closure requires a transient circuit model that includes the actual
component corners, op-amp output swing, switching transitions and the two
clock-control networks. Its outputs
must be:

- TP3/TP4 period, amplitude, symmetry and phase;
- both MN3101 clock extrema;
- corresponding BBD center delay and excursion;
- switching behavior for the three firmware states Off, I and II.
