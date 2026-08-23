# Chorus BBD clock and bandwidth derivation

## Direct evidence

The reference circuit contains two independent 256-stage BBD lines and two
two-phase clock drivers. The BBD manufacturer specifies:

- clock range: 10 kHz to 200 kHz;
- delay range: 0.64 ms to 12.8 ms;
- input bandwidth: 14 kHz at a 40 kHz clock, measured at -3 dB;
- nominal maximum input swing: 1.7 Vrms under the stated test conditions;
- typical THD: 0.3% at 40 kHz clock, 1 kHz and 0.78 Vrms;
- typical S/N: 88 dB under the manufacturer test condition.

The clock-driver manufacturer specifies two reverse-phase 50% duty outputs at
half of its oscillator frequency.

## Reconstructed clock channels

The target schematic contains two nominally mirrored voltage-controlled clock
networks, not one clock copied to both BBDs:

| Channel | Triangle input | Clock driver | BBD | Visible oscillator components |
| --- | --- | --- | --- | --- |
| A | TP3 through R128 1 kOhm | IC9 MN3101 | IC7 MN3009 | C53 150 pF, R134 22 kOhm, R135 1.8 kOhm, Tr19-Tr23, D9 |
| B | TP4 through R148 1 kOhm | IC11 MN3101 | IC10 MN3009 | C57 150 pF, R140 22 kOhm, R139 1.8 kOhm, Tr24-Tr28, D10 |

TP3 and TP4 are opposite versions of the same continuous triangle. The two
clock networks have matching nominal values, so their clock excursions are
nominally equal and opposite in time. Component tolerances may prevent perfect
mirror symmetry in a physical unit; the present DSP intentionally models the
nominal topology rather than inventing an unmeasured mismatch.

The machine-readable audit now includes every legible nominal mirror pair:

| Channel A | Channel B | Value |
| --- | --- | ---: |
| R128 | R148 | 1 kOhm |
| R123 | R136 | 1.8 kOhm |
| R124 | R137 | 8.2 kOhm |
| R125 | R138 | 10 kOhm |
| R126 | R142 | 2.2 kOhm |
| R127 | R143 | 2.2 kOhm |
| R129 | R147 | 33 kOhm |
| R130 | R146 | 330 kOhm |
| R131 | R145 | 33 kOhm |
| R132 | R144 | 6.8 kOhm |
| R133 | R141 | 2.2 kOhm |
| R134 | R140 | 22 kOhm |
| R135 | R139 | 1.8 kOhm |
| C53 | C57 | 150 pF |
| C51 | C55 | 10 uF |
| R99 | R108 | 100 Ohm |
| C46 | C49 | 100 uF |

## Active clock-control boundary

The cleaner related-revision drawing confirms the bipolar polarity classes and
the exact channel mirrors that are difficult to read in the target scan:

| Channel A | Channel B | Schematic polarity |
| --- | --- | --- |
| Tr19 | Tr24 | PNP |
| Tr20 | Tr25 | PNP |
| Tr21 | Tr26 | NPN |
| Tr22 | Tr27 | NPN |
| Tr23 | Tr28 | NPN |

This closes the schematic-level device topology, not the semiconductor model.
The related-revision inventory lists four PNP and six NPN part families for the
whole instrument but does not map those parts to Tr19-Tr28. Assigning one by
proximity or popularity would invent evidence. D9/D10 likewise retain
`1SS-133` only as a medium-confidence candidate because it is the sole ordinary
small-signal diode in that board-wide inventory.

The two modeled boundaries are consequently explicit:

```text
TP3 -> R128 -> Tr19-Tr23/D9 -> C53 + IC9 OX1/OX2/OX3 -> IC9 CP1/CP2 -> IC7
TP4 -> R148 -> Tr24-Tr28/D10 -> C57 + IC11 OX1/OX2/OX3 -> IC11 CP1/CP2 -> IC10
```

Both networks operate between the +15 V and -15 V rails. Their readable R/C
values, transistor polarities and mirrored connectivity are fixtures. Exact
part assignments, BJT operating points and the resulting TP3/TP4-to-clock
transfer remain open.

## Solver-ready connection graph

Audit v41 traces every junction rather than retaining only a component list.
The channel-neutral node names below are instantiated once for channel A and
then mirrored mechanically for channel B:

| Node | Confirmed connections |
| --- | --- |
| triangle input | TP3/TP4 and R128/R148 terminal 1 |
| input/emitter | R128, R129, R130 and Tr21 emitter / mirrored equivalents |
| PNP switch base | R127, Tr20 base and Tr21 collector |
| diode/base | Tr21 base, R129 and D9 cathode |
| control / OX3 | R126, D9 anode, R132, Tr19 collector, Tr23 collector and MN3101 pin 5 |
| timing | R132, C53 and Tr22 collector |
| OX2 | MN3101 pin 6 and R134 |
| feedback/base / OX1 | R134, R135, Tr22 base and MN3101 pin 7 |
| local negative rail | R131, R133, R135, C53, C46, Tr23 emitter, MN3101 pin 3 and R99 |

The remaining one-connection transistor nodes are also explicit: R124 feeds
the Tr19 emitter, R126 receives the Tr20 collector, R130/R131 bias the Tr23
base, and R133 returns the Tr22 emitter to the local negative rail. R99 then
connects that local rail to -15 V; C46 bypasses it to ground. C51 bypasses VGG
pin 8 to ground. CP1 and CP2 remain explicit external phase nodes leading to
the corresponding BBD.

The target and related-revision drawings agree at every one of those
junctions, while the earlier-family drawing independently agrees on the
five-BJT/OX1-OX2-OX3 shape. The report therefore marks the *topology* ready for
a DC solver with zero unresolved schematic connections. It does not mark the
nonlinear device models or any operating point as solved.

The earlier-family chorus board provides an unusually close comparative case:
its active MN3101 cell has the same five-BJT shape and nominal value pattern,
and its schematic notes explicitly allow `2SA1015-GR/Y` or `2SA1115-F` for PNP
positions and `2SC1815-GR/Y` or `2SC2603-F` for NPN positions. Intersecting that
rule with the related-revision inventory narrows the candidate families to two
per polarity. With two PNP and three NPN positions in one mirrored channel, the
unconstrained assignment count falls from:

```text
4^2 * 6^3 = 3456 board-inventory combinations
2^2 * 2^3 =   32 cross-revision hypotheses
```

The second number is a search-space reduction, not a target-board
identification. The earlier-family diode is explicitly `1S2473`, while the
related-revision inventory lists `1SS-133`; that empty intersection prevents a
false diode promotion. Likewise, the target manual's `2SA1015`/`2SC1815`
parts-designation legend is scoped to the module board and cannot be reused for
the jack board.

Both diode candidates are now represented independently. Their common
published endpoint is 1.2 V maximum at 100 mA, but the 1SS133 record is rated
90/80 V repetitive/DC reverse versus 40/35 V for 1S2473. Only the recovered
1S2473 manufacturer scan includes a usable forward curve; its eight 25 C pixel
anchors form a monotonic candidate-specific typical center.

The common endpoints nevertheless bound the complete circuit domain: the
13.64 mA conservative forward peak lies below the shared 100 mA test point and
the 30 V reverse peak lies below the weaker 35 V candidate. Explicit
monotonicity hypotheses therefore give both identities a shared 0-1.2 V
forward interval and 0.5 uA reverse-leakage ceiling at 25 C. This closes the
interval model while leaving target identity and a shared nominal curve
unsolved.

Primary manufacturer records now cover all four shortlisted families. The
Toshiba Y/GR candidates span hFE 120-400 at 2 mA, while the Mitsubishi F
candidates span 250-500 at 1 mA. Their frequency and capacitance evidence is
not interchangeable: Toshiba publishes 80 MHz as a minimum, Mitsubishi
publishes 200 MHz as typical, and the PNP/NPN Cob values differ even inside a
complementary family. The laboratory retains those conditions and limit kinds
per device instead of constructing a generic BJT.

These tables close candidate bounds, not a nonlinear model. They omit the
complete IS/NF, Early-voltage, beta-rolloff, junction-capacitance curve,
transit-time and temperature parameter sets required for a defensible
Gummel-Poon transient.

The v41 laboratory now also stores 30 raw pixel anchors from the four 25 C
transfer plots, together with the exact plot-axis calibrations and a four-pixel
reading radius. The Toshiba records remain base-current curves and the
Mitsubishi records collector-current curves. A monotonic piecewise
interpolation in log-current space reproduces all anchors without
extrapolation. A single exponential is retained only as a diagnostic: it does
not cover the visible 2SA1015 curvature inside the reading envelope and is not
used as that family's transfer center.

The pixel bounds do not represent production spread. Until unit variation is
wrapped explicitly around the four typical centers, the 32 assignments remain
unevaluated.

The forward-active adapter now supplies the first explicit hypothesis layer.
For IB-VBE sources it evaluates the curve at `IB = IC / beta`; for IC-VBE
sources beta changes base loading but not the digitized VBE center. Each family
has minimum, geometric-center and maximum beta corners plus low/center/high
pixel-reading offsets, yielding 36 per-device evidence corners. All four
digitized domains cover the conservative 30 V / 2.2 kOhm network peak-current
bound. Four additional DC-region records preserve the 0.1 uA cutoff endpoints
and the forced-beta-10 VCE(sat) maxima separately from the forward-active
curves. Two Mitsubishi h_oe values yield local 55.56 kOhm and 181.82 kOhm
output-resistance proxies, but not global Early voltages. Continuous
cutoff/saturation behavior, temperature spread, complete Early effect and
unit-to-unit VBE spread still prevent a complete DC model.

The thermal source audit now distinguishes power limits from transfer curves.
Each of the four candidates has a direct `PC-Ta` graph: full rated collector
power through 25 C, followed by a linear derating to zero at 125 C. The
laboratory evaluates those graphs only over the printed 0-125 C domain and
refuses extrapolation.

The Toshiba 2SA1015 and 2SC1815 pages additionally contain typical `IB-VBE`,
`hFE-IC` and `VCE(sat)-IC` traces at -25 C, 25 C and 100 C. The Mitsubishi
2SA1115 and 2SC2603 pages expose their DC transfer only at 25 C. Consequently
power-limited thermal checks are ready for all four families, but a continuous
direct temperature-dependent BJT transfer is not ready for all candidates.

The two direct Toshiba transfer plots are now calibrated from 600 dpi crops.
Five points on each of the -25 C, 25 C and 100 C traces yield 30 thermal pixel
anchors. Log-current piecewise interpolation round-trips every anchor, retains
the hot/room/cold order at every sampled current and rejects evaluation outside
the measured current or temperature domain. This closes a direct thermal
transfer center for the Toshiba pair only.

For both Mitsubishi candidates, v41 keeps the direct 25 C `IC-VBE` curve as
the center and creates an explicitly comparative temperature interval from
the union of both Toshiba shift families. Each source uses its own normalized
log-current coordinate, so donor `IB` is never treated as target `IC`. The
interval includes conservative trace-reading error, collapses to the direct
Mitsubishi center at 25 C, and rejects temperature or current-coordinate
extrapolation. It is ready for an uncertainty sweep but remains false as a
confirmed Mitsubishi temperature model or unit-spread bound.

Audit v41 additionally digitizes the direct Toshiba `hFE-IC` solid traces at
|VCE|=6 V and `VCE(sat)-IC` traces at forced beta 10. Sixty calibrated anchors
cover two devices, two characteristics, three temperatures and five current
coordinates. Piecewise log-log interpolation preserves the visibly non-linear
curves and refuses current extrapolation; linear temperature interpolation is
limited to -25..100 C. These curves close Toshiba thermal gain and saturation
centers only.

The v41 region layer enumerates 96 continuous 25 C hypotheses. It crosses two
cutoff shapes, zero/maximum leakage, two saturation shapes and two saturation
voltage corners; Mitsubishi additionally crosses zero with its locally held
typical `h_oe`. Toshiba typical VCE(sat) is used only inside its digitized
current domain, and all maximum values retain their original test-point status.
The family passes continuity, nonnegative-current, VCE-monotonicity and power
checks, but remains sensitivity infrastructure rather than a production model.
Audit v41 stamps the 13 unknown DC nodes into one KCL residual vector. It
includes all fourteen resistors, the five BJT C/B/E connections, the D9
1S2473 typical center and both MN3101 OX output ports; the three capacitors are
correctly open for this DC-only check. A damped Newton solve probes the uniform
Mitsubishi assignment at TP3 = 0 V with the early, linear and late OX1 shapes.
Only the linear/geometric macro converges below `1e-7 A`; that point also stays
inside the supply window, preserves every BJT VCE polarity and passes all 25 C
power limits. The other two probes are retained as rejected hypotheses.

This result is a mathematical fixed point and transient seed, not a measured
hardware operating point. Two devices reach the boundary of the digitized
transfer domain, the diode reverse branch uses a zero-current extension, TP3's
physical range is still unmeasured, and the MN3101 hidden supply current is not
published. Accordingly the report exposes the point but keeps
`active_network_operating_point_solved = false` and all production flags false.

The v41 continuation sweep then replaces the two PNP and three NPN positions
with every member of the 32-entry comparative shortlist while holding TP3 and
the linear/geometric MN3101 macro constant. All 32 assignments converge below
the KCL threshold and pass the supply, VCE-polarity and 25 C power gates. Their
three node-voltage metrics remain compact, with mean deviations of 0.08-0.15%
and maxima below 0.17%. Collector-current magnitude does not: its 9.57% mean
and 11.07% maximum deviations fail the 1%/3% gate. Maximum device power also
fails at 7.94% mean and 7.98% maximum deviation. Every assignment still clamps
two BJT transfer coordinates to the end of a digitized curve.

These results prove that the assignment enumerator and continuation solver are
executable, but they do not prove device-family equivalence. The metrics are
static fixed-point diagnostics rather than BBD clock, delay, depth, stereo
differential or bandwidth trajectories, and two of them already reject the
predeclared uncertainty limits.

The next screening layer substitutes all nine MN3101 OX macros into each of
those 32 converged node vectors without re-solving them, producing 288 frozen-
node residual records. Every record remains below the MN3101 200 mW absolute
power ceiling. Only macro 4, the linear-transition/geometric-strength member,
preserves the `1e-7 A` KCL residual across all 32 assignments. The other eight
are not disproved: their residuals, from roughly 1.9 mA to 98 mA worst case,
show that they require a new nonlinear continuation rather than reuse of the
macro-4 voltages. This matrix is a solver-priority screen and cannot identify
the unpublished MN3101 transition.

Audit v41 follows that screen with a sixteen-step numerical homotopy from macro
4 to every exact OX macro. The blend exists only in the solver; `alpha = 1`
always restores the unmodified target equations. Five targets reach that exact
endpoint with valid KCL and DC gates: early/weak, all three linear-strength
corners, and late/weak (indices 2, 3, 4, 5 and 8). Early/strong and
early/geometric lose Newton convergence after 0.125 and 0.0625 respectively.
Late/strong and late/geometric still converge at their next stages, but cross a
BJT VCE-polarity boundary after 0.1875 and 0.75. None of the nine final paths
removes the BJT transfer-domain clamps, so these are preliminary nonlinear
seeds rather than confirmed operating points.

Audit v41 separately tests whether the published forced-beta-10 saturation
condition can close the missing base-drive behavior. Inside saturated points,
base current is gradually blended from `IC/beta` toward `IC/10`; this blend is
explicitly a sensitivity hypothesis, not a transistor law. The KCL path remains
valid at 6.25%, where saturated base current rises to 3.15 times the forward-
beta value and both transfer clamps remain. At 12.5%, one clamp disappears and
the multiplier reaches 5.29, but Newton no longer converges below the KCL
threshold. The exact beta-10 hypothesis is therefore rejected as a continuous
replacement. It remains useful only as its original datasheet test condition.

The same four primary pages contain a stronger next source that had not yet
entered the executable ledger: common-emitter `IC-VCE` families at 25 C with
every curve labeled by `IB`. The Toshiba plots span 0-8 V and 0-240 mA; the
Mitsubishi plots span 0-5 V and 0-50 mA. Together they provide 33 labeled base-
current curves, including `IB=0`, and every candidate directly covers the
conservative 0-1 V/0-13.64 mA saturation-knee domain of this network. Audit v41
stores the plot rectangles, axes and curve levels. It does not yet interpolate
or stamp them: the traces are manufacturer-typical, pixel uncertainty remains
to be measured, and interpolation between labeled base-current curves is still
a declared hypothesis rather than a recovered production law.

Audit v41 now measures that pixel limit before extracting any trace. Directly
extracting the Mitsubishi page rasters at their native 2176-pixel width improves
the four-pixel voltage uncertainty from 46.5/52.5 mV to 38.9/38.7 mV and the
current uncertainty to about 0.39 mA. The Toshiba full-page sources remain at
74.2/73.6 mV and up to 3.03 mA. The two transfer-clamped devices in the accepted
preliminary point sit at 1.8 mV and 41.4 mV. Their forward-beta base-current
estimates lie between `IB=0` and the first positive 20 uA curve, but their
voltages provide only 0.05 and 1.06 reading-radius separation from the axis.
The solver requires a two-radius margin, so neither point is promoted. Native
extraction materially improves the evidence without pretending that digital
enlargement creates missing low-voltage detail.

Audit v41 then tests one deliberately broad way to bridge that missing region
without calling it recovered device physics. Six laws cross linear/smoothstep
entry with 25%, 50% and 100% movement from the forward-beta base current toward
the first printed `IB` curve, and return exactly to the documented model at the
two-radius guard. The two 100% laws fail the reference supply/polarity/power
gates. Expanding the four moderate survivors over all 32 device assignments
creates 128 nonlinear solves: 48 converge, only 32 remain inside every DC gate,
and none produces a globally clamp-free family. The first-printed-curve target
is therefore rejected as a global low-`VCE` law. The next defensible candidate
must constrain carrier transport itself instead of adding another arbitrary
base-current multiplier.

Audit v41 adds that transport boundary as a reciprocal two-junction model. The
forward and reverse transport terms share one scale, preserving the Ebers-Moll
reciprocity structure; reverse current gain remains unpublished and is swept as
`beta_R = 1, 10, 100`. Unlike the earlier one-direction model, collector current
may reverse when the base-collector junction conducts. Both `VBE` and `VBC` must
remain inside the digitized transfer domain, so reverse operation is modeled
rather than rejected merely because `VCE` changes sign.

Direct substitution over three reverse-beta values and all 32 assignments is
poorly conditioned: only 6 of 96 points converge. A sixteen-step residual
homotopy at the reference assignment separates numerical reachability from
device evidence. The exact `beta_R=1` endpoint reaches `alpha=1`, remains inside
the supply, transfer-domain and power gates, and uses neither forward nor
reverse transfer clamps. `beta_R=10` fails at the first step and `beta_R=100`
stops at `alpha=0.1875`. This establishes one physics-constrained reference
candidate, not a production parameter.

Audit v41 then continues the exact `beta_R=1` endpoint from assignment 31 down
through the complete 32-assignment device space. All 32 endpoints converge,
remain inside the supply, transport-domain and power gates, and require no
forward or reverse transfer clamp; only the initial reference endpoint needs
the sixteen-stage homotopy. This rules out numerical conditioning as the
remaining obstacle. The voltage ensembles pass the predeclared 1% mean / 3%
maximum gate (`0.01/0.01%` at OX3, `0.21/0.22%` at the timing node and
`0.53/0.54%` at OX1), but collector-current magnitude (`5.23/8.79%`) and
maximum collector power (`7.95/7.96%`) do not. The exact reciprocal family is
therefore not accepted as DC-equivalent and cannot yet enter audible DSP.

Because the 32 assignments form a complete two-level factorial over Tr19-Tr23,
audit v41 decomposes the rejected spread without fitting any new parameter.
Orthogonal main effects explain 99.9969% of collector-current spread and more
than 99.99999% of maximum-power spread; interactions are only 0.0031% and
0.0000045%. Tr19 alone accounts for 79.92% of current spread and effectively
100% of maximum-power spread, and Tr19 is the maximum-dissipation device in all
32 endpoints. Tr23 and Tr22 account for another 16.04% and 4.04% of current
spread; Tr20 and Tr21 are negligible at this coordinate. This prioritizes Tr19
output/thermal evidence but does not identify the installed transistor.

Audit v41 evaluates that priority against the documents already archived. The
16 Toshiba Tr19 endpoints occupy `28.217-28.219 V`, `0.21418-0.21420 mA` and at
most `6.045 mW`; the 16 Mitsubishi endpoints occupy `27.890-27.891 V`,
`0.25414 mA` and at most `7.088 mW`. Both current and labeled-base-current
ranges are covered, and dissipation is only 1.51%/2.36% of the published 25 C
limits, leaving derating headroom to about 123.49/122.64 C. However, neither
Tr19 candidate lies inside its archived 5-8 V `IC-VCE` plot. Toshiba alone has
direct three-temperature transfer curves; Mitsubishi has only the explicitly
comparative thermal interval. Consequently the current documents cannot bound
high-`VCE` output conductance or narrow the rejected reciprocal family.

Audit v41 then asks how much differential high-`VCE` conductance future
evidence would have to support. A frozen-node screen applies `0-4 uS` to the
Toshiba Tr19 branch relative to 6 V in `0.001 uS` steps while holding Mitsubishi
at zero. Power passes between `1.441` and `1.893 uS`, but current never passes
and there is no joint interval. The best current point is `2.128 uS`, yet still
has 2.20% mean and 4.41% maximum deviation. The published Mitsubishi `h_oe` of
18 uS remains a local test-point value and is not substituted globally. This
proves that Tr19 output conductance alone cannot close the model; Tr23 and Tr22
transport remain required evidence axes.

Audit v41 separates those axes by their actual junction state. Both Tr22
candidates have positive `VCE` and lie inside the plot voltage/current domains
with at least two reading radii: Toshiba spans `0.1960-0.1966 V`, Mitsubishi
`0.0800-0.0801 V`. Both also have forward-biased base-collector junctions, so
this is saturation/reciprocal operation rather than a forward-active trace.
Only Toshiba stays within labeled `IB`; Mitsubishi requires about `260 uA`,
beyond its printed family, and no output surface is digitized. Both Tr23
candidates instead have negative `VCE` (`-39.0..-36.5 mV` Toshiba,
`-36.4..-34.0 mV` Mitsubishi) and nonzero reverse transport in every sample.
Forward common-emitter plots cannot bound that regime; a reverse-beta or
equivalent reciprocal-transport source is required.

Audit v41 converts that missing source into a correlated numerical requirement
without identifying a transistor law. With Tr19 fixed at `1.667 uS`, the center
of its frozen-node power-pass interval, it screens 90,601 Tr22/Tr23 Toshiba
reverse-beta pairs from `0.25` through `1.0` while Mitsubishi remains at the
reference `beta_R=1`. Exactly 2,694 pairs pass both current and power gates. The
accepted projections are Tr22 `0.910-0.995` and Tr23 `0.615-0.8125`; they are
not independent rectangular bounds. The best pair, `0.95/0.6875`, reduces
collector-current dispersion to `0.21%` mean and `0.39%` maximum while power
remains at `0.010%/0.014%`. This proves that bounded reciprocal transport could
close the frozen-node metrics, but there is no direct reverse-beta evidence and
base-current KCL was not re-solved. The region is therefore a target for future
evidence, not a solver substitution or production parameter.

Audit v41 now restamps that best pair into the actual 13-node KCL graph while
holding the solved `beta_R=1` voltages fixed. It includes the Tr19 collector/
emitter conductance delta and the equal-and-opposite Tr22/Tr23 collector/base
redistribution implied by reciprocal transport. 28 of 32 assignments exceed
the solver's `0.1 uA` residual limit; the four all-Mitsubishi targeted-device
cases have no parameter delta. The worst assignment is 8 at the OX3 control
node: `55.551 uA`, or `555.5x` the accepted residual. Maximum separate changes
are `37.038 uA` at Tr19 collector transport, `8.892 uA` at the Tr22 base and
`18.512 uA` at the Tr23 base. The frozen screen is therefore decisively not a
KCL solution. A full nonlinear collector/base re-solve is mandatory before the
current/power equivalence can be tested again.

Audit v41 performs that full nonlinear test. Starting from each exact
`beta_R=1` endpoint, all 32 correlated targets converge directly—no adaptive
homotopy is needed—and remain inside supply, transport, power and unclamped
transfer gates. Node feedback is material: the largest displacement is
`85.681 mV` at the Tr19 bias-emitter node in assignment 12. The three voltage
ensembles still pass (`0.005/0.009%` OX3, `0.169/0.175%` timing and
`0.521/0.527%` OX1 mean/maximum deviation), but current returns to
`3.793/7.196%` and maximum power to `5.703/5.706%`; both fail. Thus the frozen
pair does not close the model after KCL feedback. Its only retained value is a
stable numerical path on which future evidence-bounded parameters can be
tested.

Audit v41 recomputes the balanced factorial after that nonlinear solve rather
than carrying forward the frozen attribution. Main effects explain `99.9959%`
of collector-current spread; interactions contribute only `0.0041%`. Tr19
remains dominant but falls from the earlier frozen share to `67.90%`, while
Tr23 rises to `28.04%` and Tr22 remains `4.05%`; Tr20/Tr21 are negligible.
Tr19 explains effectively all maximum-power spread and is the maximum-power
device in every assignment. Node feedback therefore changes the magnitude, not
the evidence order: high-voltage Tr19 first, reverse-transport Tr23 second and
saturated Tr22 third.

Audit v41 recalculates the source domains at those re-solved endpoints. Toshiba
Tr19 moves to `28.131-28.132 V` and `224.769-224.770 uA`; Mitsubishi remains
near `27.890-27.891 V` and `254.136 uA`. A useful output source must therefore
reach at least `28.14 V` and `0.255 mA`, not merely the archived 5/8 V plots.
Toshiba Tr22 moves to `0.2105-0.2109 V`, while Mitsubishi remains
`0.0800-0.0801 V`; both are resolvable, but the latter still needs roughly
`260 uA` base drive and neither surface is digitized. Both Tr23 candidates stay
negative-VCE (`-38.4..-34.0 mV` collectively), with collector currents spanning
about `54-74 uA`, so no forward common-emitter family can bound them. Node
feedback changes coordinates but closes none of the three documentary gaps.

The resulting 32-model ensemble will be accepted as behaviorally equivalent
only if every required trajectory has at most 1% mean absolute relative
deviation from the ensemble mean and at most 3% maximum deviation. All 32
hypotheses must remain finite and inside the existing device envelopes. This
predeclared gate determines whether exact transistor identity matters; it does
not turn a consensus into evidence of a particular installed part.

The MN3101 pin functions used by this reconstruction are:

| Pin | Name | Function |
| ---: | --- | --- |
| 1 | GND | Ground |
| 2 | CP1 | Clock phase 1 output |
| 3 | VDD | Negative supply |
| 4 | CP2 | Clock phase 2 output |
| 5-7 | OX3, OX2, OX1 | External oscillator network |
| 8 | VGG | BBD gate-bias output |

At `VDD = -15 V`, the electrical table guarantees OX1 high from 0 to -1 V and
low from -14 to -15 V, with at most 30 uA input leakage. OX2 guarantees at
least 0.6 mA high at -1 V and 0.5 mA low at -14 V; OX3 guarantees at least
1.5 mA and 2.0 mA at the same respective points. Both outputs list 30 uA
maximum leakage. Treating the test point as 1 V from its rail gives conservative
endpoint resistance ceilings of 1.667/2.000 kOhm for OX2 and 0.667/0.500 kOhm
for OX3.

The v41 report turns those endpoints into nine deliberately non-nominal macro
hypotheses: early, linear and late OX1 transitions crossed with strong,
geometric and weak output-strength corners. The weak corner uses the endpoint
resistance ceilings. The strong corner uses 5 ohm as a deliberately broad
lower resistance derived from `1 V^2 / 200 mW`; the geometric corner lies
between them. This derivation is labeled as a hypothesis because 200 mW is the
absolute limit for the entire device, not a published output resistance.

All macros preserve the published logic endpoints, output polarity and minimum
drive, and cap combined OX2/OX3 output power at 200 mW. They permit a bounded
interval solve without claiming that any one transition is the installed
device's transfer. The actual continuous output I-V and transition region
therefore remain unidentified, and `mn3101_control_transfer_solved` remains
false.

## Delay relation

One complete BBD transfer uses both phases, so for `N = 256` stages and BBD
clock `f_cp`:

```text
t_delay = N / (2 * f_cp) = 128 / f_cp
f_cp = N / (2 * t_delay)
```

This reproduces both datasheet endpoints:

```text
128 / 200000 Hz = 0.64 ms
128 / 10000 Hz  = 12.8 ms
```

RF-106 clamps the internal delay to these two device endpoints. Current modes
do not reach the clamp, but a future calibration or circuit simulation cannot
silently overclock the modeled part.

The complete exact frequency chain is therefore:

```text
f_MN3101 oscillator = 2 * f_cp
f_cp                 = f_MN3101 oscillator / 2
t_MN3009 delay       = 128 / f_cp
```

The DSP API exposes `f_cp`, the BBD clock, not the upstream MN3101 oscillator
frequency. Keeping those quantities separate prevents a factor-of-two error.

## Manufacturer oscillator anchors

The clock-driver data gives example ranges rather than a closed transfer
equation:

| R1 | R2 sweep | C1 | MN3101 oscillator | CP1/CP2 |
| ---: | ---: | ---: | ---: | ---: |
| 0 | 5 kOhm-1 MOhm | 33 pF | 15-1500 kHz | 7.5-750 kHz |
| 22 kOhm | 5 kOhm-1 MOhm | 100 pF | 5.2-440 kHz | 2.6-220 kHz |
| 22 kOhm | 5 kOhm-1 MOhm | 200 pF | 1.4-280 kHz | 0.7-140 kHz |

These rows verify the divide-by-two relation and bound plausible operation.
They do not directly solve RF-106 because its 150 pF timing capacitors are
embedded in active transistor networks driven by TP3/TP4. A future model must
use explicit manufacturer bounds and simulate or otherwise validate the
complete active network. Treating one fitted equation as the device itself
would be less faithful than retaining the current explicit calibration.

## Endpoint-bounded 150 pF hypothesis

A 1200 dpi inspection confirms that the printed labels obscure substantial
parts of all three plotted traces. The laboratory therefore does not encode
subjectively selected pixels as manufacturer measurements. Instead, it fits a
straight line in log-log space through each pair of exact table endpoints for
the two examples that share the target's `R1 = 22 kOhm`:

```text
100 pF: R2 5 kOhm -> 220 kHz, R2 1 MOhm -> 2.6 kHz
200 pF: R2 5 kOhm -> 140 kHz, R2 1 MOhm -> 0.7 kHz
```

It then interpolates log-frequency by log-capacitance to 150 pF, matching C53
and C57. Inverting this provisional curve maps the current clock coordinates
to the following simple-network equivalents:

| Mode | Clock extrema | Equivalent R2 span |
| --- | ---: | ---: |
| I | 23.57-109.40 kHz | 7.96-41.30 kOhm |
| II | 25.55-80.50 kHz | 11.07-37.88 kOhm |

Both spans are comfortably inside the manufacturer's 5 kOhm-1 MOhm example
sweep. This is a useful feasibility bound: the current delay excursion does
not demand an impossible clock-driver resistance. It is not yet the target
circuit transfer. Tr19-Tr28 make a voltage-controlled active network, so the
next stage must demonstrate how TP3/TP4 produce those equivalent resistances.
No device-tolerance percentage is assigned to an endpoint interpolation, and
these values remain barred from production promotion.

The same requirement expressed as conductance is more useful for reducing the
active transistor network:

| Mode | Required equivalent conductance | Span ratio |
| --- | ---: | ---: |
| I | 24.21-125.55 uS | 5.19:1 |
| II | 26.40-90.36 uS | 3.42:1 |

These are load-domain requirements obtained by `G = 1/R`; they are not
transistor measurements. Any future DC or transient model must cover these
envelopes while preserving both mirrored channels, rather than fitting the
present audible coordinates directly.

## Factory adjustment boundary

The service procedure named `CHORUS BIAS` does not calibrate the LFO or clock.
With VCA LEVEL at zero and CHORUS I selected, it injects a 10 Vpp, 1 kHz sine
wave into module-board TP2, observes jack-board TP1 and TP2, and adjusts VR1
and VR2 until the positive and negative audio-waveform halves are symmetrical
about the center line.

This directly constrains per-channel BBD audio bias and clipping symmetry. The
procedure publishes no LFO frequency, CP1/CP2 frequency, delay excursion or
depth target. Consequently VR1/VR2 cannot be used to force the provisional
rate/depth calibration, and `factory_clock_target_present` remains false.

## Intrinsic bandwidth

The nominal clock-normalized -3 dB ratio printed in the BBD data is:

```text
k_bw = 14 kHz / 40 kHz = 0.35
f_bbd,-3dB = 0.35 * f_cp
```

This is distinct from the fixed external anti-alias and reconstruction
networks on the board. RF-106 therefore models:

1. fixed external low-pass sections from board components; and
2. a clock-dependent intrinsic BBD low-pass section.

The intrinsic section is a nominal approximation to the printed response, not
a claim that every device has an exact one-pole transfer. Its coefficient is
kept explicit and tested so a later SPICE fit or E4 capture can replace it.

## Nominal BBD distortion transfer

The previous production path used `tanh(0.12 x) / 0.12`. That kept the delay
bounded, but `0.12` had no physical unit and did not reproduce either printed
MN3009 distortion point. The replacement keeps the established service scale
of 3 physical peak volts per DSP unit and applies a bounded odd-symmetric
transfer in volts:

```text
z = Vin / 2.9814637 V
Vout = 2.9814637 V * tanh(z + 0.2682414 z^3)
```

The two parameters are solved jointly against the manufacturer's 40 kHz clock,
1 kHz conditions:

| Input condition | Published THD | Recomputed DSP THD |
| --- | ---: | ---: |
| 0.78 Vrms nominal test | 0.3% typical | 0.300% |
| 1.7 Vrms input-swing point | 2.5% | 2.500% |

The verification integrates harmonics 2 through 10 over 16,384 samples. The
transfer has unity slope at zero, remains odd and bounded, and is applied only
to the wet BBD paths. Chorus OFF therefore remains bit-exact bypass while the
two delay lines continue running warm. The model intentionally does not add
bias asymmetry, unit spread, clock feedthrough or a guessed noise spectrum;
those still require either a sufficiently detailed primary curve or E4 audio
captures. The datasheet's 88 dB typical S/N and 150 uVrms maximum A-weighted
noise constrain future work, but do not establish its unweighted color.

## Present modulation envelope

Using the current explicit calibration layer:

| Mode | Delay range | BBD clock range | MN3101 oscillator range | Datasheet envelope |
| --- | ---: | ---: | ---: | --- |
| I | 1.17-5.43 ms | 23.57-109.40 kHz | 47.15-218.80 kHz | inside |
| II | 1.59-5.01 ms | 25.55-80.50 kHz | 51.10-161.01 kHz | inside |

The added oscillator column is exactly twice the BBD clock column. These
numbers are a consistency audit of the existing calibration, not a promotion
of its center delay or depth to measured hardware truth.

The service schematic establishes one shared free-running triangular integrator
feeding opposing clock-control paths in every active mode. It does not establish
the absolute rate and depth without solving the complete transistor/R/C network.
Those numerical values therefore remain provisional. The topology and device
bandwidth improvements do not promote them to established constants.
