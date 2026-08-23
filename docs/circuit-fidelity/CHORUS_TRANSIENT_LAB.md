# Chorus oscillator laboratory

`rf-106-circuit-lab` is an offline evidence tool. It is deliberately outside
the real-time DSP and may not supply production constants until its assumptions
are closed.

Run it with:

```text
cargo run -p rf-106-circuit-lab
```

## Schematic and firmware audit v41

The target 1984 schematic and related 1985 schematic disagree on R15. The
former says 1 MOhm; the latter says 1 kOhm. The laboratory preserves both
readings and evaluates them rather than choosing by image sharpness alone. The
1 kOhm branch predicts 37.5 Hz with Tr1 open and fails the chorus-range sanity
test, so it is quarantined as an internally inconsistent related-revision
marking. The target-source 1 MOhm value remains authoritative for RF-106.

With the source conflict explicit, the audited reduced stage computes:

```text
Tr1 open:        R_effective = R5 + R8
Tr1 ideal shunt: R_effective = R5 + R8 + R5*R8/R4
RC scale:        tau = R_effective * C3
Schmitt ratio:   beta = R15 / (R6 + R15)
Ideal rate:      f = 1 / (4 * beta * tau)
```

It enumerates the corners of 5% resistor and 10% capacitor tolerances instead
of treating nominal components as an exact instrument.

The ideal rates are 0.818 Hz with Tr1 open and 0.407 Hz with Tr1 treated as a
short. The existing 0.842 Hz mode-II coordinate lies inside the enumerated open
branch corners. The 0.514 Hz mode-I coordinate is about 2.4% above the
ideal-shunt upper corner. The old attempt to explain that small discrepancy as
roughly 483 kOhm of Tr1 ON resistance was not physically supported and has been
removed. It is now reported only as a coordinate-versus-corner comparison.

The laboratory also records the TL062 manufacturer bounds at +/-15 V: at a
10 kOhm load its output swing is guaranteed to +/-10 V and typically +/-13.5 V;
the unity-gain slew rate is 1.5 V/us minimum and 3.5 V/us typical under the
printed test condition. These bounds constrain a future transient solver but
are not used to force a target frequency.

Finally, the control audit separates CHORUS ON/OFF -> Tr6/Tr5 -> TP4 clamp from
CHORUS I/II -> Tr2/D1/Tr1 -> R4 rate control. The service truth table and the
jack-board rail-shifting stage close the steady states as 0/+15 V: I is 0 V
with Tr2 on, D1 reverse-biased and Tr1 conducting; II is +15 V with Tr2 off, D1
pulling the Tr1 gate negative and the shunt open. Tr5 is an enable clamp, not an
unresolved mode-rate element.

The verified A_5/B_2 firmware closes the digital side of those two lines. Three
one-hot front-panel buttons become Assigner switch-1 values `0x20`, `0x40` and
`0x80`; after the seven-bit serial transform, Voice produces only Off, I and II.
A coincident I+II scan reaches the same Module Board latch state as I. It does
not select a fourth oscillator branch, so the unsupported 7.85 Hz coordinate
has been removed from production and from the laboratory targets.

The v41 audit extends the evidence chain through both clock drivers and BBDs.
It records the complete MN3101 pin map, the manufacturer's three published
oscillator examples, and the mirrored target networks from TP3/R128/IC9 to
IC7 and TP4/R148/IC11 to IC10. It also distinguishes the internal MN3101
oscillator from CP1/CP2: the latter are antiphase 50% outputs at exactly half
the oscillator frequency.

Combining that divider with the 256-stage BBD relation closes the exact
clocked-device part of the chain:

```text
f_osc = 2 * f_cp
t_delay = 128 / f_cp
```

The current delay coordinates round-trip through those equations and remain
inside the MN3009 10-200 kHz clock envelope. This does not solve voltage to
frequency: the MN3101 data supplies curves and examples, while the target uses
150 pF capacitors inside active transistor control networks. Consequently v41
sets `clock_frequency_chain_solved` to `true` and
`mn3101_control_transfer_solved` to `false`.

The v41 report also uses the exact endpoint table instead of reading obscured
pixels from the plotted curves. Log-log endpoint fits for the 100 pF and 200 pF
examples bracket a log-capacitance interpolation at the target's 150 pF. The
current mode-I clock range corresponds to a simple-network R2 equivalent of
7.96-41.30 kOhm; mode II corresponds to 11.07-37.88 kOhm. Both are inside the
published sweep, which closes a feasibility check without pretending that the
active Tr19-Tr28 networks have already been reduced.

The report therefore keeps both `active_network_voltage_to_r2_solved` and
`production_promotion_ready` false. The next solver boundary is the transistor
network between TP3/TP4 and OX1/OX2/OX3, not another fit of the BBD delay.

All 17 legible R/C pairs in the two clock-control networks are now fixtures,
from R128/R148 through C53/C57, C51/C55, R99/R108 and C46/C49. This makes
accidental asymmetry in a future channel-A/channel-B netlist a test failure
rather than a listening judgment.

The factory-adjustment audit also records what is absent. `CHORUS BIAS` uses a
10 Vpp, 1 kHz audio stimulus and VR1/VR2 to center the two BBD audio waveforms.
It gives no clock or modulation target. The report therefore sets
`factory_clock_target_present` to false and does not misuse the bias trimmers
as rate/depth controls.

The v41 active-network audit adds the next defensible layer. The target and
cleaner related-revision schematics agree that Tr19/Tr20 and Tr24/Tr25 are PNP,
while Tr21-Tr23 and Tr26-Tr28 are NPN. It also fixes all five transistor mirror
pairs and the TP3/TP4-to-MN3101-to-MN3009 network boundaries in the report.

The related-revision semiconductor list is board-wide, so it is retained only
as a candidate inventory: four PNP families and six NPN families. It cannot
assign exact parts to Tr19-Tr28. For the same reason, D9/D10 remain a
medium-confidence `1SS-133` candidate rather than a direct assignment.

Converting the endpoint-bounded equivalent resistances with `G = 1/R` gives a
24.21-125.55 uS requirement for mode I and 26.40-90.36 uS for mode II. These
machine-tested conductance envelopes define what a future active-network model
must span without choosing its transistor parameters to reproduce the current
calibration. Accordingly `active_network_schematic_topology_captured` is true,
while `exact_active_device_assignments_solved`,
`active_network_operating_point_solved`,
`active_network_voltage_to_r2_solved` and `production_promotion_ready` remain
false.

The v41 report additionally replaces the former prose-only boundary with an
explicit connection graph. It records all 18 two-terminal elements per
channel, all collector/base/emitter assignments for the five BJTs, and all
eight MN3101 pins. Named-node degree tests protect the control/OX3, timing,
OX2, feedback/OX1, triangle-input and local-negative-rail junctions. The target
and related-revision drawings agree on every connection, and the earlier-family
drawing independently agrees on the five-BJT oscillator shape. Therefore
`active_network_dc_solver_topology_ready` is true with zero unresolved
schematic connections. This says only that a solver can be wired without
guessing; the confirmed physical operating point remains deliberately unsolved.

That distinction exposes one additional solver input that the earlier audit
did not name. The published MN3101 material describes the internal oscillator
as a two-stage inverter and gives frequency-versus-R/C examples. Its electrical
table also guarantees OX1 high and low input regions, 30 uA leakage limits and
minimum OX2/OX3 output currents at points 1 V from either supply rail. The v41
report preserves those values and derives only conservative endpoint resistance
ceilings: 1.667/2.000 kOhm for OX2 high/low and 0.667/0.500 kOhm for OX3
high/low. These are one-point bounds, not output-resistance models.

The transition transfer and continuous output I-V remain unpublished, so the
available endpoints cannot uniquely identify one DC load line for the
surrounding five-transistor cell. Audit v41 therefore supplies an explicit
*hypothesis family* rather than a fabricated nominal model. It crosses three
transition shapes (early, linear and late) with three output-strength corners
(strong, geometric and weak), yielding nine continuous OX-port macros. Every
member preserves the guaranteed OX1 logic regions, OX2/OX3 polarity and minimum
endpoint currents. Their combined output contribution is also capped by the
200 mW absolute device-dissipation ceiling; this is a necessary safety bound,
not a claim that all device dissipation belongs to the outputs.

The nine macros make the MN3101 port suitable for an interval sweep, while
`mn3101_control_transfer_solved` correctly remains false. The actual transition
shape and continuous output strength are still unidentified. Audit v41 now also
provides continuous BJT region hypotheses, so a preliminary coupled sensitivity
sweep can be wired next. Production promotion remains blocked because neither
the OX macros nor the BJT output-conductance and unit-spread axes are guaranteed
device bounds.

The earlier-family chorus schematic provides a useful but deliberately weaker
constraint. Its five-transistor MN3101 control cell has the same topology and
nominal component pattern, and its notes explicitly permit `2SA1015` or
`2SA1115` for PNP positions and `2SC1815` or `2SC2603` for NPN positions. Those
four families are also present in the related-revision inventory. Their
intersection reduces the mirrored channel-A assignment space from 3456 generic
board-inventory combinations to 32 comparative hypotheses; requiring one
family per polarity would leave four, but the schematic does not justify that
extra assumption.

This is E6 cross-revision evidence, not a target-board parts list. The 1984
`2SA1015`/`2SC1815` parts-designation legend is explicitly scoped to the module
board and is not applied to the jack-board clock network. The earlier circuit
also specifies `1S2473`, while the later inventory supplies `1SS-133`, leaving
the diode intersection empty. The report therefore narrows only the bipolar
candidate set and still refuses exact device promotion.

The diode evidence is now quantitative rather than name-only. Both documentary
candidates publish `VF <= 1.2 V` at 100 mA and `IR <= 0.5 uA`, but their reverse
ratings differ: the later-inventory 1SS133 record gives 90 V repetitive peak
and 80 V DC, while the comparative 1S2473 record gives 40 V and 35 V. The
1S2473 manufacturer scan additionally supplies 3 pF maximum capacitance, 4 ns
maximum recovery and a 25 C forward curve. Eight calibrated anchors preserve
that curve from approximately 0.5 to 100 mA.

Those common endpoints now close a deliberately broad interval model over the
complete network domain. The conservative 13.64 mA forward peak is below the
shared 100 mA test point, and the 30 V reverse peak is below the weaker 35 V
candidate rating. Assuming monotonic forward voltage and reverse leakage gives
both candidates a shared `0..1.2 V` forward interval and `IR <= 0.5 uA` reverse
bound at 25 C. The assumptions are serialized as hypotheses rather than
manufacturer guarantees.

The available curve remains typical and belongs only to 1S2473. Piecewise
log-current interpolation supplies that candidate's optional center between
approximately 0.5 and 100 mA and refuses extrapolation. The 1SS133 candidate
uses only the shared interval. Thus `bounded_diode_iv_model_ready` is true
without solving identity or inventing a shared nominal curve.

## Primary bipolar-device bounds

The v41 report closes the documentary gap for every member of that shortlist
with four original manufacturer records: Toshiba 2SA1015/2SC1815 and
Mitsubishi 2SA1115/2SC2603. Each record preserves the printed test condition
and whether a number is a minimum, typical or maximum rather than flattening
all of them into nominal simulator constants.

The active-network candidates now have these directly published envelopes:

| Candidate | Rank and hFE condition | hFE span | fT | Cob |
| --- | --- | ---: | ---: | ---: |
| 2SA1015 Y/GR | VCE=-6 V, IC=-2 mA | 120-400 | 80 MHz minimum | 4 pF typical, 7 pF maximum |
| 2SA1115 F | VCE=-6 V, IC=-1 mA | 250-500 | 200 MHz typical | 4 pF typical |
| 2SC1815 Y/GR | VCE=6 V, IC=2 mA | 120-400 | 80 MHz minimum | 2 pF typical, 3.5 pF maximum |
| 2SC2603 F | VCE=6 V, IC=1 mA | 250-500 | 200 MHz typical | 2.5 pF typical |

All four are 50 V parts. The Toshiba pair is rated at 150 mA and 400 mW;
the Mitsubishi pair at 200 mA and 300 mW. These values can reject impossible
operating points and drive rank/capacitance corner tests. They cannot yet
produce the active-network transfer.

Audit v41 additionally inspects the complete characteristic pages instead of
assuming that every family has equivalent thermal data. All four candidates
publish a `PC-Ta` graph that stays at the 25 C rating through 25 C and then
falls linearly to zero at 125 C. The laboratory turns those four direct graphs
into executable, non-extrapolating power limits over their printed 0-125 C
domain. This closes temperature-dependent power rejection, not transistor
transfer.

Only the Toshiba pair publishes three-temperature `IB-VBE`, `hFE-IC` and
`VCE(sat)-IC` traces at -25 C, 25 C and 100 C. The Mitsubishi pair publishes
only 25 C `IC-VBE` and gain curves plus the 25 C saturation endpoint. The
report therefore records direct temperature-transfer coverage for two of four
candidates. It does not borrow a Toshiba coefficient silently; the Mitsubishi
temperature intervals described below remain labeled comparative hypotheses
and must be swept rather than selected as truth.

The Toshiba `IB-VBE` traces are now quantitative. Two 600 dpi crop
calibrations preserve the printed logarithmic current axes, the three
temperatures and an eight-pixel reading radius that covers trace thickness and
grid overlap. Five levels per trace produce 30 new raw pixel anchors and 30
electrical anchors. Every trace is monotonic and every sampled current obeys
`VBE(100 C) < VBE(25 C) < VBE(-25 C)`.

Piecewise interpolation is allowed only between the five current anchors and
linearly between -25 C, 25 C and 100 C. It refuses both current and temperature
extrapolation. At the serialized geometric reference currents, the observed
hot shift is about -126 mV for 2SA1015 and -106 mV for 2SC1815; the cold shift
is about +74 mV and +92 mV respectively. These remain manufacturer-typical
centers with digitization uncertainty, not unit-to-unit bounds.

Audit v41 constructs that Mitsubishi comparison without equating the
Toshiba `IB` axis with the Mitsubishi `IC` axis. Each of the four source curves
is mapped to a normalized logarithmic coordinate over its own digitized current
domain. At each coordinate and temperature, the union of the PNP and NPN
Toshiba VBE shifts, expanded by the conservative error of both trace readings,
is applied around the direct 25 C Mitsubishi `IC-VBE` center. Thus 25 C
collapses exactly to the Mitsubishi curve while cold and hot evaluations
produce intervals rather than invented nominal coefficients.

The serialized report contains 30 reference intervals: two Mitsubishi targets,
five temperatures and three normalized current coordinates. A denser 77-point
grid per target verifies ordered bounds, cold-above-room and hot-below-room
behavior, exact recovery of the 25 C target curve, and rejection outside
-25..100 C or normalized coordinate 0..1. The interval includes digitization
error but still does not cover Mitsubishi production spread. It is executable
for comparative sweeps and explicitly unusable as a confirmed device model.

The Toshiba thermal gain and saturation curves are now quantitative as well.
Four additional 600 dpi calibrations isolate only the solid `hFE-IC` traces at
|VCE|=6 V and the `VCE(sat)-IC` traces at forced beta 10. Five collector-current
levels on each of three temperatures produce 30 gain anchors and 30 saturation
anchors. The dashed |VCE|=1 V gain curves are deliberately excluded, so bias
conditions cannot be mixed invisibly.

Both axes are reconstructed logarithmically and interpolation stays piecewise
log-log inside a printed trace, followed by linear interpolation only between
-25 C, 25 C and 100 C. All 60 anchors round-trip, remain positive, and preserve
`value(100 C) > value(25 C) > value(-25 C)` at every reference current for both
gain and saturation voltage. A ten-pixel reading radius covers trace thickness,
grid overlap and the source watermark. These are manufacturer-typical centers,
not hFE-rank limits, saturation maxima or Mitsubishi temperature laws.

The Mitsubishi scans additionally publish a complete common-emitter
h-parameter row at 1 mA, 6 V, 270 Hz and 25 C. The PNP row gives hie=7.0 kOhm,
hre=0.1e-3, hfe=250 and hoe=18 uS; the NPN row gives 8.5 kOhm, 0.1e-3, 300 and
5.5 uS. These are valid local small-signal anchors at that printed bias point,
not parameters for arbitrary voltage, current or transient operation. The
Toshiba sheets do not publish an equivalent four-value row, so the laboratory
leaves those optional fields empty rather than synthesizing counterparts.

The report therefore exposes ten missing large-signal parameter groups,
including IS/NF, VAF/IKF, reverse gain, both junction-capacitance curves,
transit-time parameters and temperature/noise coefficients. It keeps
`complete_large_signal_model_available = false`; no SPICE default is allowed
to masquerade as device evidence.

The first curve-extraction pass is now machine-readable. Each of the four
25 C plots has an explicit page size, plot-axis calibration, four-pixel reading
radius and 30 raw pixel anchors across the four families. This yields 30
electrical anchors while preserving an important source distinction: the
Toshiba plots are `IB-VBE`,
whereas the Mitsubishi plots are `IC-VBE`. The former are not silently
multiplied by a guessed beta.

A single exponential diagnostic fits three traces inside their combined pixel
and axis-reading error. The 2SA1015 trace retains visible curvature and misses
that envelope, so the laboratory does not force it into one IS/NF pair.
Instead, every typical center uses piecewise-linear VBE interpolation in
log-current space. All anchors round-trip exactly, remain strictly monotonic,
and evaluation outside the plotted current interval is rejected.

These bands quantify digitization error only. Manufacturer-typical traces do
not publish unit-to-unit VBE spread, so `bounded_bjt_dc_fits_ready` remains
false. The next valid step is an explicit production-spread hypothesis around
each piecewise center, constrained by the published hFE ranks and local
h-parameters; it must remain labeled as a hypothesis rather than a guarantee.

The first evidence-only corner grid is now present. The common adapter keeps
IB-VBE and IC-VBE semantics distinct, applies the printed hFE rank at minimum,
geometric-center and maximum values, and combines each with low/center/high
pixel-reading offsets. This creates 36 per-device corners and four serialized
reference points. The complete digitized domain covers the conservative
13.64 mA peak implied by 30 V across the smallest 2.2 kOhm network resistor.
Applying a rank away from its printed test current is itself marked as a
hypothesis.

The non-forward-active evidence is now a separate four-family envelope. Every
record carries the manufacturer-maximum 25 C collector and emitter cutoff
leakage endpoints, plus the forced-beta-10 VCE(sat) test at 100 mA. The
conservative 13.64 mA network peak is below that saturation test current, but
using the 0.25-0.30 V maxima at lower current is explicitly a monotonicity
hypothesis rather than a published guarantee. Toshiba additionally publishes
maximum VBE(sat) endpoints for the complementary pair; the Mitsubishi sheets
do not, so those fields remain absent.

The Mitsubishi tables publish typical h_oe values at 1 mA, 6 V and 270 Hz.
Their reciprocals are local output resistances of 55.56 kOhm for 2SA1115 and
181.82 kOhm for 2SC2603. Multiplying that local resistance by the nominal test
current yields 55.56 V and 181.82 V voltage scales, but the audit labels these
only as local Early-effect proxies: they are neither guaranteed production
bounds nor SPICE VAF parameters. Toshiba has no equivalent numeric table in
the selected records.

Audit v41 turns those distinct sources into 96 continuous 25 C region
hypotheses rather than one fabricated transistor. Linear and smoothstep cutoff
joins cross zero/published-maximum leakage corners. Saturation joins cross the
direct Toshiba typical curve or the published maximum, while Mitsubishi uses a
zero lower corner and its published maximum because no typical curve exists.
The maximum below its 100 mA test point is always labeled an extension
hypothesis. Mitsubishi additionally crosses zero output conductance with a
constant held `h_oe` hypothesis; Toshiba remains at zero because no numeric
output-admittance record exists.

The 96 members are continuous at the first digitized forward-active point,
nonnegative on the validation grid, monotonic with VCE through saturation, and
power-audited against each 25 C rating. Direct Toshiba saturation is never
extrapolated. This closes a numerical family for sensitivity analysis, not a
bounded production device: Mitsubishi thermal gain/saturation, global output
conductance, temperature spread and unit VBE spread remain false.

## Ensemble uncertainty gate

The unresolved 32-hypothesis device sweep has a predeclared acceptance rule so
its threshold cannot be chosen after seeing the result. At every evaluated
coordinate, each candidate is compared with the arithmetic ensemble mean. A
metric passes only when:

```text
mean absolute relative deviation <= 1.0%
maximum absolute relative deviation <= 3.0%
finite hypotheses evaluated          = 32 of 32
```

The maximum guard prevents one divergent model from being hidden by a small
average. The rule must pass independently for the BBD clock trajectory, delay
trajectory, modulation depth, inter-channel differential trajectory and
clock-dependent bandwidth trajectory. Passing all five means device identity
is behaviorally immaterial within the modeled evidence envelope; it does not
identify the physical transistor.

The v41 report uses the accepted linear/geometric fixed point as a continuation
seed for all 32 assignments. Every assignment converges, remains inside the
supply and preserves VCE polarity and power limits. The three static node
voltages pass the 1% mean/3% maximum gate, but summed collector current and
maximum device power fail it by wide margins. Every result also clamps two BJT
coordinates to a digitized transfer boundary. Consequently the sweep is a
complete DC diagnostic but not a valid device-equivalence result.

All entries name the two PNP and three NPN choices for channel A and require
channel B to use the corresponding mirrored assignment. None has direct target
identity evidence. The five required audio-trajectory metrics remain entirely
unevaluated, so `accepted_as_behaviorally_equivalent` and
`production_promotion_ready` remain false.

Audit v41 also evaluates the complete 32-assignment by nine-MN3101-macro
frozen-node matrix. All 288 substitutions are finite and remain inside the
controller power ceiling. Only the linear/geometric macro retains KCL at the
already solved voltages; the other eight become prioritized nonlinear solves.
This is not evidence that macro 4 is physically installed, because every
non-passing member was tested without allowing its node voltages to move.

The subsequent sixteen-step homotopy does allow those voltages to move. It
reaches five exact macro endpoints (2, 3, 4, 5 and 8). Two early-transition
targets stop on non-convergence, while two late-transition targets encounter
invalid BJT VCE polarity. Since every reached endpoint still relies on at least
one transfer-domain clamp, the homotopy supplies solver seeds only and does not
select a physical MN3101 model.

The forced-beta-10 saturation endpoint is also tested as a possible base-drive
continuation. The path passes only through 6.25% of that substitution and loses
KCL convergence at 12.5%. Although the failed point reduces the clamp count
from two to one, it is not a valid operating point. This prevents a convenient
but unjustified promotion of a datasheet test setup into a continuous device
model and leaves the original two-clamp valid point as the last accepted seed.

Full-page inspection also reveals a direct evidence path beyond that rejected
shortcut. Each shortlisted transistor has a 25 C common-emitter `IC-VCE` plot
whose individual curves are labeled by base current. Across the four sources
there are 33 labeled curves: six for 2SA1015, eight for 2SC1815, ten for
2SA1115 and nine for 2SC2603. Even the smaller Mitsubishi axes cover 0-5 V and
0-50 mA, so all four plots contain the complete conservative 0-1 V and
0-13.64 mA knee region required by the circuit. The v41 report serializes the
page calibrations and exact printed base-current levels, but deliberately keeps
`digitized_output_surface_ready` false until trace anchors, reading bounds and
inter-curve interpolation tests exist.

The first resolution audit prevents premature use of that surface. Audit v41
extracts each Mitsubishi page's native 2176-pixel one-bit raster instead of
measuring a smaller rendered page. Its four-pixel reading radius is now
38.7-38.9 mV and about 0.39 mA; Toshiba remains 73.6-74.2 mV and up to 3.03 mA.
Tr22 at 1.8 mV reaches only 0.05 reading radii, while Tr23 at 41.4 mV reaches
1.06. Both provisional base currents remain bracketed by the zero and first
20 uA curves, but robust solver substitution requires two reading radii. The
report therefore allows digitization only in the visibly resolvable domain and
still requires a better source or separately bounded low-`VCE` model.

The v41 low-`VCE` experiment explicitly tests and rejects a first-labeled-`IB`
sensitivity family. Linear and smoothstep joins at 25%, 50% and 100% strength
are continuous at the two-radius boundary and are all marked non-identifying.
The full-strength pair breaks the reference DC gates. The remaining four laws
are solved across all 32 assignments: 48 of 128 points converge, 32 pass every
supply/polarity/power gate, and zero are transfer-clamp free. Because no law
survives the complete matrix, no uncertainty metric or audible constant is
promoted. This closes an invalid shortcut and routes the next block toward a
physics-constrained transport formulation.

That formulation is now executable in v41. It uses reciprocal forward/reverse
junction transport and treats reverse beta values 1, 10 and 100 only as an
uncertainty axis. A 96-point direct assignment matrix removes reliance on the
old forward-transfer clamps but converges in only six cases, so direct Newton
failure is not treated as physical rejection. Sixteen-step homotopy from the
accepted reference seed reaches one exact endpoint: `beta_R=1` at `alpha=1`,
with both junction transfers inside their digitized domains and all supply and
power gates satisfied. The other reverse-beta endpoints remain unreachable.
Intermediate homotopy blends are numerical paths, never transistor laws. The
surviving endpoint is then continued exactly across all 32 assignments. Every
endpoint converges inside the supply, transport and power gates with zero
forward/reverse clamps; only assignment 31 uses the adaptive homotopy. The
three node-voltage ensembles pass the 1% mean / 3% maximum DC uncertainty gate,
but collector-current magnitude reaches 5.23% mean and 8.79% maximum deviation,
and maximum collector power reaches 7.95% mean and 7.96% maximum deviation.
The DC ensemble is consequently rejected before transient TP3/TP4 validation,
so none of these hypotheses affects production audio.

The complete factorial also makes that rejection attributable rather than
opaque. Position main effects explain 99.9969% of current spread and virtually
all maximum-power spread. Tr19 contributes 79.92% of current spread and nearly
100% of power spread; it is also the maximum-power device in every endpoint.
Tr23 contributes 16.04% and Tr22 4.04% of current spread, while Tr20/Tr21 and
all interactions are negligible here. These deterministic sensitivities route
future evidence work toward Tr19, but cannot select its part number or narrow a
production parameter without new primary evidence.

The v41 evidence-domain audit makes the missing source precise. Tr19 remains at
27.89-28.22 V across the entire factorial, beyond both archived manufacturer
output-curve axes (5 V Mitsubishi, 8 V Toshiba). Its collector current and
labeled base drive are inside the plotted ranges, while peak dissipation is
only 6.045/7.088 mW, or 1.51%/2.36% of the direct 25 C power limits. These facts
prove headroom but not high-voltage output conductance. One candidate has direct
thermal transfer curves and the other only a comparative interval, so neither
pixel resolution nor existing thermal evidence can close the rejected spread.
A higher-voltage output family or independently justified global conductance
bound is required before any new transient run is defensible.

The v41 frozen-node requirement screen evaluates 4,001 differential Tr19
conductances from 0 to 4 uS, referenced to 6 V. A `1.441-1.893 uS` interval
would make maximum power satisfy the numerical gate, but no value makes total
current pass, so no joint interval exists. Even the best current point at
`2.128 uS` remains at 2.20% mean and 4.41% maximum deviation. Intermediate
values are requirement probes, not resolved operating points, and the direct
18 uS Mitsubishi `h_oe` number is local rather than a global law. Any defensible
next model therefore needs both high-voltage Tr19 evidence and bounded Tr23/Tr22
transport; changing Tr19 alone cannot authorize transient or audible work.

The v41 junction-state audit then distinguishes the remaining devices. Tr22 is
inside the archived voltage/current rectangles and above the two-radius reading
threshold for both candidates, but both base-collector junctions conduct. The
Toshiba point stays inside printed base-drive coverage; Mitsubishi reaches about
260 uA, outside its labeled `IB` family, and neither output surface has been
digitized. Tr23 is more fundamental: all 32 samples have negative `VCE` and
nonzero reverse transport. Its forward common-emitter curves contain no such
quadrant. Thus Tr22 needs bounded saturation-surface/base-drive evidence, while
Tr23 needs direct reverse-transport or reverse-beta bounds; forward extrapolation
is invalid for both.

The v41 correlated requirement screen makes that evidence request quantitative.
At the frozen Tr19 conductance of `1.667 uS`, 2,694 of 90,601 Tr22/Tr23
reverse-beta pairs pass the current and power gates. Accepted one-dimensional
projections are `0.910-0.995` for Toshiba Tr22 and `0.615-0.8125` for Toshiba
Tr23 relative to Mitsubishi `beta_R=1`; combinations inside those projections
are not all accepted. The best pair is `0.95/0.6875`, giving `0.21%/0.39%`
mean/maximum current deviation and `0.010%/0.014%` power deviation. Because the
screen freezes the node voltages, does not re-solve base-current KCL, and has no
direct reverse-beta source, it authorizes neither a transient run nor an
audible-model change. It instead supplies a falsifiable acceptance region for
primary evidence and the subsequent full nonlinear re-solve.

The v41 residual audit tests whether that omitted re-solve is merely formal or
numerically material. Restamping the diagnostic Tr19/Tr22/Tr23 corrections into
their collector, base and emitter nodes makes 28 of 32 frozen assignments fail
the `0.1 uA` KCL criterion. The maximum induced residual is `55.551 uA` at the
OX3 control node, `555.5x` the solver limit; Tr19, Tr22-base and Tr23-base
changes reach `37.038`, `8.892` and `18.512 uA` respectively. Four assignments
have no delta because all three targeted positions use the held Mitsubishi
reference. This rejects the frozen-node pair as an operating-point substitute.
It remains only a numerical continuation target, pending direct parameter
evidence and a full nonlinear 32-assignment re-solve.

That v41 re-solve is now executable. All 32 targets converge directly from
their exact reciprocal endpoints, remain unclamped and pass every absolute DC
gate. The largest node movement is `85.681 mV` at the Tr19 bias-emitter node.
OX3, timing and OX1 voltage dispersion pass the uncertainty policy, but the
re-solved collector-current magnitude (`3.793%` mean, `7.196%` maximum) and
maximum collector power (`5.703%`, `5.706%`) fail it. Node feedback therefore
invalidates the apparent frozen current/power closure. The convergent solver
path is useful for testing future primary-evidence bounds, but neither these
unpublished target values nor any derived transient/audio behavior may be
promoted.

The v41 post-solve factorial makes the next evidence order explicit. Main
effects retain `99.9959%` of current variance, with Tr19 at `67.90%`, Tr23 at
`28.04%` and Tr22 at `4.05%`; interactions are `0.0041%` and Tr20/Tr21 remain
negligible. Tr19 also accounts for effectively all maximum-power variance and
is the highest-dissipation device in all 32 solutions. The nonlinear feedback
therefore strengthens the need for both Tr19 and Tr23 evidence, while keeping
Tr22 as the smaller third axis. These shares prioritize measurement/document
work only and do not identify installed parts.

The v41 post-solve evidence envelopes prevent searching for the wrong plot
region. Tr19 now requires output behavior through at least `28.14 V` at up to
`0.255 mA`. Tr22 needs saturation surfaces over `0.080-0.211 V`, including the
Mitsubishi branch at approximately `260 uA` base drive. Tr23 remains entirely
in reverse `VCE`, from roughly `-39` to `-34 mV`, with `54-74 uA` collector
current and nonzero reverse junction transport. Existing current/base ranges
and absolute power limits still pass, but they do not constrain these missing
behaviors. The next acceptable source must cover these exact domains or provide
an independently bounded equivalent model.

## Promotion rule

`production_promotion_ready` remains `false` until a transient model:

1. includes actual R/C corners, TL062 output behavior and the mode-switch
   transient around Tr2/D1/Tr1;
2. produces TP3 and TP4 as opposite continuous triangles;
3. maps both control voltages through the two MN3101 networks;
4. reproduces the three firmware states without inserting target rates;
5. reports sensitivity to component and semiconductor tolerances;
6. reproduces the firmware-proven I alias for a coincident I+II scan.

The production chorus continues using explicit provisional numerical
calibration in the meantime. The laboratory can disprove or bound those values
without silently changing the sound.
