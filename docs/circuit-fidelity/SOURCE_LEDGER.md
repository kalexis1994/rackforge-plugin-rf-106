# Circuit-fidelity source ledger

The source files themselves are not part of the repository. Hashes identify the
exact external documents used for a derivation.

| ID | Description | SHA-256 | Relevant pages | Level | Use |
| --- | --- | --- | --- | --- | --- |
| SRC-SVC-1984 | Reference-hardware service notes dated 1984-07-31 | `1429DB35C0F4CE6DE62AF00FA6BE8A5C56CAEBA3BA1073B55CE62A2CCF1692B5` | PDF 1-4, 8-12; printed 1, 5, 8, 9, 13, 15, 18, 19 | E1 | Architecture, wave generator, module/jack schematics and factory adjustment targets |
| SRC-SVC-106S-1985 | Related 106S/HS-60 service notes dated 1985-07 | `55118EA03A995EEAD22977E2BA5185B6971A5D7FCB1074E1E202EC706F3585EB` | PDF/printed 5, 6, 17 | E1 for the related revision | Cleaner jack-board topology, semiconductor inventory and explicit source-conflict checks; never silently overrides the target revision |
| SRC-SVC-60-1983 | Related earlier-family service notes dated 1983-04-10 | `A2376644C6D211D39ADBFE58A381CE37B2FBBECD2723FDFBC02EE107A63F7127` | PDF/printed 11 | E6 for RF-106 | Comparison of the dual-BBD architecture and hypothesis generation only |
| SRC-OM-106 | Roland JUNO-106 owner's manual and MIDI implementation | `E9315EFE58F26236AD26D1CD6D0359AAB284741186ED31CF0BBADB1D08902EFE` | PDF 28, 34-35 | E1 | APR/IPR message framing, the sixteen tone values and two packed switch bytes |
| SRC-BBD-CAT | Original BBD manufacturer catalog | `A99DC2CBB7AA735FF2F69A1CC06D863BA6AC25798EADA44B6558748AEE445DEC` | PDF 38-40; printed 36-38 | E2 | 256-stage topology, 10-200 kHz clock range, delay, bandwidth, headroom, THD and noise |
| SRC-CLK-3101 | Original BBD clock-driver data | `03A4B9134130E4A40567F5159CAB5BA37D601C92F83D1D721B67EAF661E1CCF8` | PDF 1-4; printed 58-61 | E2 | Two-phase clock behavior, divide-by-two relation, supply and oscillator network |
| SRC-OP-062 | TL062 manufacturer data | `8B4D9405B319A9D38F1C847E8F5B52AF74E83FDAB85295AFF2C529076DE72EF4` | PDF 8-10 | E2 | Output swing, slew rate and operating-condition bounds for the chorus Schmitt/integrator core |
| SRC-OP-5218 | Original dual-amplifier data | `0DBEF5A11C20BDE9875F6222E11CC9B9490A19F382F8B6582EF59ED082CB1C9F` | PDF 1-5 | E2 | Output swing, slew rate, gain bandwidth, noise, load and distortion behavior |
| SRC-FET-2SK30A | Toshiba discrete-device catalog table for 2SK30A | `0017AD3472ECB10B9BFE528E0C486B900E707B02DC18EAF07AE036911658F4BF` | PDF 1 | E2 | Corroborates the Y 1.2-3.0 mA and GR 2.6-6.5 mA IDSS ranks named by the service inventory; it does not by itself solve low-voltage channel resistance |
| SRC-BJT-2SA1015 | Toshiba 2SA1015 manufacturer data, archived copy | `6EC062D55C95DCA1595E23286509B70DF58DD82AC053A7216A1A655B72A8FCEA` | PDF 1-3 | E2 | PNP Y/GR hFE ranks, absolute ratings, saturation, transition-frequency and capacitance bounds; 25 C IC-VCE-by-IB output family; -25/25/100 C IB-VBE, hFE and VCE(sat) traces; PC-Ta derating |
| SRC-BJT-2SC1815 | Toshiba 2SC1815 manufacturer data, archived copy | `ADCD6EF3CB5D485BD163E87DA639ABDE5BD2A7C59A51B508B8D92DCC41B29FF4` | PDF 1-4 | E2 | NPN Y/GR hFE ranks, absolute ratings, saturation, transition-frequency and capacitance bounds; 25 C IC-VCE-by-IB output family; -25/25/100 C IB-VBE, hFE and VCE(sat) traces; PC-Ta derating |
| SRC-BJT-2SA1115 | Mitsubishi 2SA1115 manufacturer scan | `F6E7069288E9288D25D2C8459A935CA772C484C22CA9A31733DCA3002B3084EE` | PDF 1-3; printed 2-17 to 2-19 | E2 | PNP F-rank hFE, absolute ratings, saturation, transition-frequency and capacitance; 25 C IC-VCE-by-IB and transfer curves; PC-Ta derating |
| SRC-BJT-2SC2603 | Mitsubishi 2SC2603 manufacturer scan | `E65EA23ADC9C2E5FC1326CEE8DC737BC54E376E5DA2D527BA51B327C20D8A05B` | PDF 1-3; printed 4-34 to 4-36 | E2 | NPN F-rank hFE, absolute ratings, saturation, transition-frequency and capacitance; 25 C IC-VCE-by-IB and transfer curves; PC-Ta derating |
| SRC-ROM-A | Verified Assigner A_5 control ROM | `D43CCE5578EE2F16B27C8B06BFF30743E3E2DFFC796D033811E565D5D578C52E` | Private 8 KiB image; chorus decode at `0x0BEB-0x0C0A` | E3 | Three one-hot panel states, switch-1 storage and seven-bit serial encoding |
| SRC-ROM-B | Verified Voice B_2 module ROM | `F3C48E14434E29E264407E9163F30C0473F94C20957799ECF75746099D1BD2A2` | Private 8 KiB image; chorus latch decode at `0x014F-0x0164` | E3 | Module latch polarity, enable line and I/II rate line |

## Source handling

- The imported 128-program table retains the source table's parameter slot 25
  while the generated table is converted to the documented tone layout. That
  slot represented a synthetic `Sub Sw` control which has no destination on
  the physical front panel or in the documented patch data. RF-106 deliberately
  excludes it from synthesis. The SUB level DAC is the sole sub-oscillator
  control. B35 `Tomita` is the end-to-end regression for the corrected source
  order: its pulse, saw, noise and SUB levels are zero, so its audible output
  comes from the resonant VCF rather than a synthetic sub source.
- B35 also exposes a lifecycle constraint that is physical rather than a patch
  gain exception: its high-Q VCF is continuously biased while the VCA is
  closed. RF-106 therefore advances idle DCO/VCF state for three measured B_2
  foreground intervals after program selection and full state restoration.
  This deterministic interval is an E6 bound until program-change timing can
  be checked on hardware; it is never applied while a voice is active or
  releasing.
- `SRC-SVC-1984` is a scan, so relevant schematic and adjustment pages require
  visual inspection rather than text extraction.
- A sharper related-revision scan is not automatically more authoritative than
  the target revision. Conflicting component markings remain explicit fixtures
  and must pass topology and operating-range sanity checks.
- The related-revision semiconductor inventory is board-wide. It can constrain
  Tr19-Tr28 to the schematic-confirmed PNP/NPN candidate families, but it cannot
  assign an exact part number to an individual designator without another
  source. The same boundary keeps D9/D10 at candidate status.
- The earlier-family page-11 device rule intersects that inventory at
  `2SA1015`/`2SA1115` for PNP and `2SC1815`/`2SC2603` for NPN. This is an E6
  shortlist for model sweeps, not direct evidence of the target designators.
  Its `1S2473` diode rule conflicts with the later `1SS-133` inventory, so no
  diode type is promoted by intersection.
- The target manual's `2SA1015`/`2SC1815` parts-designation legend is scoped to
  the module board. It is not evidence for jack-board Tr19-Tr28.
- `SRC-BBD-CAT` supersedes an encrypted single-device copy that could not be
  inspected reliably.
- A datasheet typical value is not treated as a guaranteed unit value.
- Full-page comparison of `SRC-SVC-1984` and `SRC-SVC-106S-1985` fixes the
  complete mirrored Tr19-Tr28 connection graph, including C46/C49 and
  R99/R108. `SRC-SVC-60-1983` corroborates the five-BJT/OX topology but remains
  comparative evidence rather than a source for target designators.
- The four shortlisted bipolar families now have manufacturer records, but the
  published tables are not complete Gummel-Poon models. In particular, hFE,
  saturation, fT and one-point Cob values do not identify IS, VAF, junction
  capacitance curves, transit-time behavior or temperature coefficients.
  Those fields remain explicitly absent instead of receiving simulator
  defaults.
- All four bipolar records publish a `PC-Ta` graph over 0-125 C. Audit v41
  evaluates the direct flat-to-25-C then linear-to-zero-at-125-C derating and
  rejects temperatures outside the plotted domain. This is an ambient power
  limit, not a temperature law for VBE, beta, leakage or saturation.
- The Toshiba pair alone publishes `IB-VBE`, `hFE-IC` and `VCE(sat)-IC` traces
  at -25 C, 25 C and 100 C. The selected Mitsubishi pages publish their DC
  transfer only at 25 C. Direct thermal-transfer coverage is therefore two of
  four. Audit v41 creates a separate Mitsubishi sweep interval from the union
  of the Toshiba PNP/NPN shifts, explicitly as comparative E6-style evidence
  and never as a direct manufacturer guarantee.
- Audit v41 calibrates 600 dpi crops of the Toshiba `IB-VBE` plots with an
  eight-pixel reading radius. Five current levels on each -25 C, 25 C and
  100 C trace produce 30 raw and 30 electrical thermal anchors. Interpolation
  is piecewise in log current and linear only between the three printed
  temperatures; it refuses extrapolation and does not claim production spread.
- The comparative Mitsubishi interval uses a normalized log-current coordinate
  inside each source's own plotted domain, preserving donor `IB` and target
  `IC` as different quantities. It adds the conservative uncertainty of both
  digitized traces to the union of Toshiba shifts around the direct Mitsubishi
  25 C center. Thirty serialized references and two 77-point validation grids
  test the -25..100 C and 0..1 coordinate domains. This interval is usable for
  sweeps, not for identifying a Mitsubishi temperature law or production
  spread.
- Four further 600 dpi calibrations isolate Toshiba solid `hFE-IC` at |VCE|=6 V
  and `VCE(sat)-IC` at forced beta 10. Five current readings for each device,
  characteristic and -25/25/100 C trace yield 60 raw and 60 electrical anchors.
  Both axes are log calibrated with a ten-pixel reading radius. The dashed
  |VCE|=1 V gain curves are excluded, interpolation refuses extrapolation, and
  the result remains a set of manufacturer-typical Toshiba centers rather than
  rank limits, saturation maxima, production spread or Mitsubishi evidence.
- The four 25 C transfer plots are digitized as 30 raw pixel anchors with
  per-page axis calibrations and a four-pixel reading radius. Toshiba IB-VBE
  and Mitsubishi IC-VBE traces remain distinct. Piecewise interpolation is
  allowed only inside each plotted range; a single exponential is diagnostic,
  not a replacement for visible curvature. These uncertainty bands cover
  reading error and explicitly do not claim unit-to-unit production spread.
- The forward-active hypothesis adapter combines three printed hFE-rank
  corners with three pixel-reading offsets for each family, producing 36
  evidence-only corners. IB-VBE sources use `IB = IC / beta`; IC-VBE sources
  keep VBE independent of beta while base loading changes. The adapter rejects
  values outside the digitized domain and does not model cutoff, saturation,
  Early effect or unit VBE spread.
- Cutoff and saturation evidence is stored independently from that adapter.
  All four records publish 0.1 uA maximum ICBO/IEBO endpoints at 25 C and a
  VCE(sat) maximum at IC=100 mA, IB=10 mA. Those are exact test-point limits,
  not continuous I-V curves. Reusing a 0.25-0.30 V saturation maximum below
  100 mA requires an explicit monotonicity hypothesis; no temperature law is
  inferred from a 25 C table.
- Mitsubishi publishes typical h_oe values of 18 uS for 2SA1115 and 5.5 uS
  for 2SC2603 at magnitude IE=1 mA, magnitude VCE=6 V and 270 Hz. The audit
  serializes their reciprocal local output resistances and current-times-
  resistance voltage scales, but explicitly refuses to identify those local
  typical values with guaranteed SPICE VAF. The selected Toshiba records do
  not supply an equivalent numeric table.
- Audit v41 combines these sources only as a 96-member continuous 25 C
  sensitivity family. Cutoff uses linear/smoothstep joins and zero/maximum
  leakage corners. Saturation uses linear/smoothstep joins with direct Toshiba
  typical curves or published maximum endpoints; Mitsubishi uses zero and its
  maximum endpoint because no typical curve is present. Constant held `h_oe`
  is an explicit Mitsubishi hypothesis, never a global bound. Tests enforce
  continuity, nonnegative current, monotonic saturation response, source-domain
  refusal and power limits. None of the 96 members claims production spread or
  a confirmed transistor model.
- The two conflicting diode records remain separate. Both publish 1.2 V
  maximum at 100 mA and 0.5 uA reverse-current limits, but their reverse
  ratings differ. The recovered 1S2473 manufacturer scan contributes eight
  calibrated anchors from its 25 C forward curve plus capacitance and recovery
  limits. The 1SS133 primary product record contributes endpoint limits only;
  no curve is borrowed across identities.
- Their endpoint intersection now forms a circuit-domain interval, not a
  nominal diode curve. The 13.64 mA forward-current ceiling is below the common
  100 mA VF test and the 30 V reverse ceiling is below the weaker 35 V rating.
  Extending the printed maxima downward uses explicit monotonicity hypotheses,
  yielding 0-1.2 V forward and at most 0.5 uA reverse leakage at 25 C. Only
  1S2473 receives the digitized typical center, and its piecewise interpolation
  refuses extrapolation outside the visible curve.
- The MN3101 electrical table publishes guaranteed OX1 high/low regions,
  30 uA leakage limits, minimum OX2/OX3 drive currents at points 1 V from each
  supply rail and a 200 mW absolute device-dissipation ceiling. Audit v41 uses
  those facts to constrain nine continuous OX-port hypotheses: three transition
  shapes crossed with three output-strength corners. The weak corner preserves
  the endpoint resistance ceilings, the strong corner is a deliberately broad
  power-derived lower-resistance hypothesis, and the geometric corner lies
  between them. All nine preserve endpoint polarity/current and enforce the
  power ceiling. The transition transfer and continuous output I-V remain
  unpublished, so this family enables an interval solve but does not identify a
  nominal MN3101 model. Audit v41 couples this macro to the exact passive/BJT/
  diode graph and accepts one linear/geometric mathematical fixed point below
  `1e-7 A` KCL residual. It remains a solver seed only: hidden MN3101 supply
  current, measured TP3 range and two out-of-domain BJT transfer coordinates
  still prevent a confirmed physical operating point.
- Audit v41 continues that accepted seed through all 32 comparative BJT
  assignments. All converge and pass supply/polarity/power gates, but all use
  two transfer-domain clamps. Static node voltages pass the numerical spread
  threshold while total collector current and maximum transistor power fail;
  these are DC diagnostics and are never substituted for the five required
  audio trajectories.
- The resulting 32 assignment node vectors are crossed with all nine MN3101
  macros as a 288-point frozen-node residual screen. Every point respects the
  200 mW ceiling; only the linear/geometric member preserves KCL for all 32
  vectors. Non-passing members remain valid hypotheses awaiting a fresh
  nonlinear solve, not rejected controller identities.
- A sixteen-stage current-port homotopy subsequently reaches exact macros 2,
  3, 4, 5 and 8. Macros 0 and 1 stop on convergence; macros 6 and 7 cross an
  invalid BJT VCE-polarity boundary. The artificial blend is strictly a
  numerical path, never evidence of an internal MN3101 interpolation law.
- The forced-beta-10 saturation condition is then tested as a separate base-
  drive homotopy. It remains valid through 6.25% and fails convergence at
  12.5%; the full substitution is not reached. This preserves its actual
  source meaning as a 100 mA/10 mA test condition rather than inventing a
  continuous base-current law.
- Full-page inspection of those same four manufacturer records identifies a
  direct 25 C common-emitter `IC-VCE` family for every candidate. Their 33
  labeled `IB` curves include zero base current and share at least a 0-5 V,
  0-50 mA plot domain, which contains the conservative 0-1 V/0-13.64 mA
  saturation knee needed here. Audit v41 records each page rectangle, axis and
  printed base-current level. No curve has yet been digitized, and interpolation
  between adjacent `IB` traces remains an explicit sensitivity hypothesis.
- Audit v41 calibrates the usable resolution before reading those traces. The
  two Mitsubishi PDFs each contain a native 2176-pixel one-bit page raster;
  extracting it directly improves the four-pixel uncertainty to 38.7-38.9 mV
  and about 0.39 mA. Toshiba remains 73.6-74.2 mV and up to 3.03 mA. The two
  clamped points at 1.8/41.4 mV reach only 0.05/1.06 reading radii, below the
  predeclared two-radius solver margin. A separately found one-page 2SC2603
  reseller PDF contains only a 300x300 main raster and is rejected as lower-
  resolution non-primary evidence.
- Audit v41 evaluates, but does not promote, six continuous laws that move the
  low-`VCE` base current toward the first labeled `IB` curve inside the
  two-radius guard. The two full-strength laws fail the reference DC gates.
  The four moderate survivors produce 128 assignment-law solves: 48 converge,
  32 pass all DC gates and none removes the transfer clamps across the full
  assignment set. These are documented negative numerical results, not new
  manufacturer evidence; the whole first-labeled-curve family is rejected.
- Audit v41 adds a reciprocal two-junction transport sensitivity model. Its
  shared forward/reverse transport scale is a physics constraint; reverse beta
  1/10/100 is explicitly unpublished. Direct substitution converges in only 6
  of 96 assignment points. A sixteen-step reference homotopy reaches the exact
  reverse-beta-1 endpoint without forward or reverse transfer clamps and inside
  every DC gate; reverse-beta 10 and 100 do not reach their endpoints. This is
  a viable reference hypothesis, not evidence identifying reverse beta or a
  production-ready transistor model.
- Audit v41 adaptively continues that exact reverse-beta-1 endpoint across all
  32 active-device assignments. All endpoints converge inside every DC gate
  without transfer clamping, and only the reference requires homotopy. The
  OX3, timing and OX1 voltage ensembles pass the predeclared uncertainty limits;
  collector-current magnitude (5.23% mean / 8.79% maximum deviation) and peak
  collector power (7.95% / 7.96%) fail them. This is a numerical rejection at
  the DC gate, not new device evidence and not authorization for audible use.
- Audit v41 uses the balanced 32-run two-level factorial only to attribute that
  deterministic rejection. Main effects explain 99.9969% of current spread and
  more than 99.99999% of maximum-power spread. Tr19 contributes 79.92% and
  effectively 100% respectively, remains the maximum-dissipation transistor in
  all endpoints, and is therefore the next evidence priority. Tr23/Tr22 explain
  the remaining 16.04%/4.04% current shares. This sensitivity decomposition is
  not evidence of installed identity or permission to narrow any parameter.
- Audit v41 checks all ten position/candidate envelopes against their actual
  document domains. Dominant Tr19 occupies 28.217-28.219 V for 2SA1015 and
  27.890-27.891 V for 2SA1115, beyond the archived 8 V and 5 V output plots.
  Its current/base-drive ranges are covered and peak power is only 1.51%/2.36%
  of the direct 25 C limits, but safe dissipation does not constrain global
  output conductance. Toshiba supplies direct three-temperature transfer data;
  Mitsubishi remains comparative. Existing sources are therefore explicitly
  insufficient to narrow the reciprocal current/power spread.
- Audit v41 screens a purely diagnostic differential Tr19 conductance from
  0-4 uS at frozen exact-endpoint voltages. Power passes only over
  1.441-1.893 uS; current never passes, with its best 2.128 uS point still at
  2.20% mean / 4.41% maximum deviation. There is no joint interval. The direct
  Mitsubishi 18 uS `h_oe` datum is preserved as a local test-point value, not
  extended to 28 V. This negative result routes remaining primary evidence to
  Tr23/Tr22 transport as well as high-voltage Tr19 output conductance.
- Audit v41 records `VBE`, `VBC`, signed `VCE` and reverse-transport current for
  every device endpoint. Tr22 is plot-resolvable at 0.080-0.197 V but has both
  junctions forward biased; 2SC2603 also exceeds its last labeled `IB` curve at
  about 260 uA. Tr23 occupies only negative `VCE` (-39.0 to -34.0 mV) with
  reverse transport at all 32 samples. The archived forward common-emitter
  families cannot bound Tr23, and no digitized surface yet bounds Tr22. These
  are explicit source gaps, not permission to infer reverse beta.
- Audit v41 fixes Tr19 at the center of its frozen-node power-pass interval and
  screens 90,601 correlated Toshiba Tr22/Tr23 reverse-beta pairs against the
  Mitsubishi `beta_R=1` reference. 2,694 pairs pass both numerical gates; their
  projections are Tr22 `0.910-0.995` and Tr23 `0.615-0.8125`, with a best pair
  of `0.95/0.6875`. The projections are not independent bounds, no direct
  reverse-beta source supports them, and the screen omits base-current KCL
  feedback. They are documented evidence requirements only and cannot narrow
  solver or production parameters.
- Audit v41 restamps the best diagnostic pair into the audited KCL nodes at the
  frozen `beta_R=1` voltages. 28 of 32 assignments exceed the `0.1 uA` solver
  residual, with a `55.551 uA` (`555.5x`) maximum at OX3 control. Maximum Tr19
  collector, Tr22-base and Tr23-base deltas are `37.038`, `8.892` and
  `18.512 uA`. This is an exact consequence of the diagnostic parameter change,
  not new component evidence; it proves a full nonlinear base/collector KCL
  re-solve is mandatory and forbids frozen-node solver substitution.
- Audit v41 fully re-solves the diagnostic correlated target from every exact
  `beta_R=1` assignment endpoint. All 32 converge directly without clamps and
  inside absolute DC gates; maximum node movement is `85.681 mV`. OX3, timing
  and OX1 voltage ensembles pass, while collector current (`3.793%/7.196%`)
  and maximum power (`5.703%/5.706%`) fail. This rejects the apparent frozen
  equivalence after feedback. Convergence establishes only an evidence-testing
  numerical path; it supplies no reverse-beta or output-conductance evidence.
- Audit v41 repeats the complete two-level factorial attribution on the
  nonlinear re-solved endpoints. Main effects explain `99.9959%` of current
  spread: Tr19 `67.90%`, Tr23 `28.04%`, Tr22 `4.05%`, with `0.0041%`
  interactions and negligible Tr20/Tr21. Tr19 remains the maximum-power device
  in all endpoints and dominates effectively all power spread. This preserves
  the evidence priority Tr19 -> Tr23 -> Tr22 after node feedback; it is a
  numerical sensitivity result, not identity or parameter evidence.
- Audit v41 recomputes all ten device/candidate evidence envelopes at the
  nonlinear endpoints. Tr19 requires at least 28.14 V / 0.255 mA output
  coverage; Tr22 requires 0.080-0.211 V saturation surfaces and Mitsubishi base
  drive near 260 uA; Tr23 remains entirely at negative VCE (-39 to -34 mV) and
  54-74 uA collector current. The direct-domain audit still rejects existing
  Tr19 plots, the undigitized Tr22 surfaces and every forward-only Tr23 family.
  These are source-search coordinates, not inferred device laws.
- Toshiba's Y/GR candidate record spans the union of the published Y and GR
  ranks (120-400). Mitsubishi's F record uses its published 250-500 range.
  Rank bounds remain parameter corners inside each family hypothesis rather
  than multiplying the 32 topology assignments.
- The MN3101 example table and divide-by-two behavior are direct evidence. The
  150 pF feasibility audit uses endpoint power-law fits and log-capacitance
  interpolation explicitly marked as a hypothesis; obscured plot pixels are
  not represented as manufacturer measurements.
- IDSS rank is not silently converted into a low-voltage JFET channel
  resistance; that requires the output curves or a defensible device model.
- A factory trim target constrains the calibrated system but does not by itself
  reveal the complete nonlinear transfer between targets.
- ROM bytes remain private. The public laboratory records only whole-image
  hashes, relevant address ranges and independently reconstructed effects.
