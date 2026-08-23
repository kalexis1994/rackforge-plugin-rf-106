use serde::Serialize;

const R5_OHMS: f64 = 1_000_000.0;
const R8_OHMS: f64 = 2_200_000.0;
const R4_OHMS: f64 = 680_000.0;
const R6_OHMS: f64 = 47_000.0;
const R7_OHMS: f64 = 33_000.0;
const R9_OHMS: f64 = 33_000.0;
const R10_OHMS: f64 = 33_000.0;
const R15_OHMS: f64 = 1_000_000.0;
const C3_FARADS: f64 = 0.1e-6;
const C4_FARADS: f64 = 220.0e-12;
const RESISTOR_TOLERANCE: f64 = 0.05;
const CAPACITOR_TOLERANCE: f64 = 0.10;

const TL062_OUTPUT_SWING_GUARANTEED_VOLTS: f64 = 10.0;
const TL062_OUTPUT_SWING_TYPICAL_VOLTS: f64 = 13.5;
const TL062_SLEW_RATE_MIN_VOLTS_PER_MICROSECOND: f64 = 1.5;
const TL062_SLEW_RATE_TYPICAL_VOLTS_PER_MICROSECOND: f64 = 3.5;

const CURRENT_MODE_I_RATE_HZ: f64 = 0.514;
const CURRENT_MODE_II_RATE_HZ: f64 = 0.842;
const CURRENT_CENTER_DELAY_MS: f64 = 3.30;
const CURRENT_MODE_I_DEPTH_MS: f64 = 2.13;
const CURRENT_MODE_II_DEPTH_MS: f64 = 1.71;
const CONTROL_LOW_VOLTS: f64 = 0.0;
const CONTROL_HIGH_VOLTS: f64 = 15.0;
const MN3101_OSCILLATOR_TO_CP_DIVISOR: f64 = 2.0;
const MN3009_STAGES: f64 = 256.0;
const MN3009_MIN_CLOCK_HZ: f64 = 10_000.0;
const MN3009_MAX_CLOCK_HZ: f64 = 200_000.0;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
enum BranchState {
    Tr1Open,
    Tr1IdealShunt,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct ComponentValue {
    designator: &'static str,
    nominal: f64,
    unit: &'static str,
    role: &'static str,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct RcScale {
    branch: BranchState,
    nominal_resistance_ohms: f64,
    nominal_seconds: f64,
    minimum_seconds: f64,
    maximum_seconds: f64,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct IdealOscillatorEstimate {
    branch: BranchState,
    schmitt_feedback_ratio: f64,
    nominal_hz: f64,
    minimum_hz: f64,
    maximum_hz: f64,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct ProvisionalCoordinate {
    name: &'static str,
    current_hz: f64,
    evaluated_by_complete_model: bool,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct SourceConflict {
    designator: &'static str,
    target_source_value: &'static str,
    related_revision_value: &'static str,
    resolution: &'static str,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct ControlPath {
    input: &'static str,
    devices: &'static str,
    function: &'static str,
    oscillator_rate_dependency: &'static str,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct DeviceBounds {
    device: &'static str,
    condition: &'static str,
    guaranteed_output_swing_volts: f64,
    typical_output_swing_volts: f64,
    minimum_slew_rate_volts_per_microsecond: f64,
    typical_slew_rate_volts_per_microsecond: f64,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct SemiconductorAudit {
    designators: &'static str,
    device: &'static str,
    evidence: &'static str,
    confidence: &'static str,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct FetGrade {
    grade: &'static str,
    minimum_idss_ma: f64,
    maximum_idss_ma: f64,
    evidence: &'static str,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct HardwareControlState {
    panel_mode: &'static str,
    chorus_enable_line_volts: f64,
    chorus_rate_line_volts: f64,
    rate_line_is_dont_care: bool,
    tr5_enable_clamp: &'static str,
    tr2_state: &'static str,
    d1_state: &'static str,
    tr1_gate_state: &'static str,
    oscillator_branch: &'static str,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct CoordinateComparison {
    mode: &'static str,
    current_hz: f64,
    branch: BranchState,
    reduced_nominal_hz: f64,
    reduced_minimum_hz: f64,
    reduced_maximum_hz: f64,
    inside_assumed_component_corners: bool,
    distance_above_maximum_percent: f64,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct FirmwareChorusState {
    panel_selection: &'static str,
    assigner_switch1_bits: u8,
    transmitted_switch1_value: u8,
    module_latch_low_bits: u8,
    canonical_audio_mode: &'static str,
    distinct_audio_state: bool,
    normal_one_hot_state: bool,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct Mn3101Pin {
    pin: u8,
    name: &'static str,
    function: &'static str,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct Mn3101OscillatorExample {
    r1_ohms: f64,
    r2_min_ohms: f64,
    r2_max_ohms: f64,
    c1_picofarads: f64,
    oscillator_min_hz: f64,
    oscillator_max_hz: f64,
    cp_min_hz: f64,
    cp_max_hz: f64,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct Mn3101OxElectricalBounds {
    test_supply_v: f64,
    power_dissipation_absolute_max_mw: f64,
    ox1_guaranteed_high_range_v: [f64; 2],
    ox1_guaranteed_low_range_v: [f64; 2],
    ox1_input_leakage_max_ua: f64,
    output_high_test_voltage_v: f64,
    output_low_test_voltage_v: f64,
    ox2_output_high_current_min_ma: f64,
    ox2_output_low_current_min_ma: f64,
    ox2_output_leakage_max_ua: f64,
    ox3_output_high_current_min_ma: f64,
    ox3_output_low_current_min_ma: f64,
    ox3_output_leakage_max_ua: f64,
    ox2_high_endpoint_resistance_max_ohms: f64,
    ox2_low_endpoint_resistance_max_ohms: f64,
    ox3_high_endpoint_resistance_max_ohms: f64,
    ox3_low_endpoint_resistance_max_ohms: f64,
    transition_region_is_published: bool,
    continuous_output_iv_is_published: bool,
    evidence: &'static str,
    interpretation: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
enum Mn3101TransitionShape {
    Early,
    Linear,
    Late,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
enum Mn3101OutputStrengthCorner {
    Strong,
    Geometric,
    Weak,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
enum Mn3101Ox1Region {
    GuaranteedLow,
    UnpublishedTransition,
    GuaranteedHigh,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct Mn3101OxMacroHypothesis {
    index: usize,
    transition_shape: Mn3101TransitionShape,
    output_strength_corner: Mn3101OutputStrengthCorner,
    transition_shape_is_hypothesis: bool,
    minimum_resistance_from_total_power_is_conservative_hypothesis: bool,
    published_endpoint_currents_remain_hard_minima: bool,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct Mn3101OxMacroPoint {
    hypothesis_index: usize,
    ox1_input_v: f64,
    ox1_region: Mn3101Ox1Region,
    normalized_high_fraction: f64,
    ox2_target_v: f64,
    ox3_target_v: f64,
    ox2_resistance_ohms: f64,
    ox3_resistance_ohms: f64,
    ox2_output_current_a: f64,
    ox3_output_current_a: f64,
    ox2_output_power_w: f64,
    ox3_output_power_w: f64,
    combined_output_power_w: f64,
    combined_power_inside_absolute_limit: bool,
    transition_is_published: bool,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct Mn3101OxMacroReadiness {
    hypothesis_count: usize,
    guaranteed_logic_regions_preserved: bool,
    endpoint_current_minima_preserved: bool,
    combined_power_limit_enforced: bool,
    complete_supply_domain_covered: bool,
    actual_transition_transfer_solved: bool,
    suitable_for_interval_sweep: bool,
    suitable_as_identified_nominal_device_model: bool,
    interpretation: &'static str,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct ClockChannelNetwork {
    channel: &'static str,
    triangle_source: &'static str,
    modulation_input: &'static str,
    clock_driver: &'static str,
    bbd: &'static str,
    oscillator_capacitor: &'static str,
    oscillator_resistors: [&'static str; 2],
    active_network: &'static str,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct ClockDelayEnvelope {
    mode: &'static str,
    delay_min_ms: f64,
    delay_max_ms: f64,
    cp_min_hz: f64,
    cp_max_hz: f64,
    mn3101_oscillator_min_hz: f64,
    mn3101_oscillator_max_hz: f64,
    inside_mn3009_clock_range: bool,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct Mn3101EndpointFit {
    r1_ohms: f64,
    c1_picofarads: f64,
    r2_min_ohms: f64,
    r2_max_ohms: f64,
    cp_at_r2_min_hz: f64,
    cp_at_r2_max_hz: f64,
    log_log_exponent: f64,
    evidence: &'static str,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct EquivalentR2Envelope {
    mode: &'static str,
    cp_min_hz: f64,
    cp_max_hz: f64,
    r2_at_cp_max_ohms: f64,
    r2_at_cp_min_ohms: f64,
    inside_manufacturer_sweep: bool,
    interpretation: &'static str,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct EquivalentConductanceEnvelope {
    mode: &'static str,
    minimum_microsiemens: f64,
    maximum_microsiemens: f64,
    span_ratio: f64,
    interpretation: &'static str,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct MirroredClockComponent {
    channel_a_designator: &'static str,
    channel_b_designator: &'static str,
    nominal: f64,
    unit: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
enum ActiveNetworkNode {
    PositiveSupply,
    Ground,
    NegativeSupply,
    LocalNegativeRail,
    TriangleInput,
    InputEmitterNode,
    PnpSwitchBase,
    PnpSwitchCollector,
    PnpBiasBase,
    PnpBiasEmitter,
    DiodeBase,
    Control,
    Timing,
    LowerNpnBase,
    OscillatorFeedback,
    FeedbackBase,
    FeedbackEmitter,
    Vgg,
    ClockPhaseOne,
    ClockPhaseTwo,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
enum NetlistElementKind {
    Resistor,
    Capacitor,
    Diode,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct MirroredNetlistElement {
    channel_a_designator: &'static str,
    channel_b_designator: &'static str,
    kind: NetlistElementKind,
    terminal_1: ActiveNetworkNode,
    terminal_2: ActiveNetworkNode,
    terminal_1_meaning: &'static str,
    terminal_2_meaning: &'static str,
    nominal: Option<f64>,
    unit: Option<&'static str>,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct MirroredBipolarConnection {
    channel_a_designator: &'static str,
    channel_b_designator: &'static str,
    polarity: BipolarPolarity,
    collector: ActiveNetworkNode,
    base: ActiveNetworkNode,
    emitter: ActiveNetworkNode,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct MirroredMn3101PinConnection {
    channel_a_designator: &'static str,
    channel_b_designator: &'static str,
    pin: u8,
    function: &'static str,
    node: ActiveNetworkNode,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct ActiveNetworkNetlistAudit {
    channel_a_path: &'static str,
    channel_b_path: &'static str,
    two_terminal_elements_per_channel: usize,
    bipolar_devices_per_channel: usize,
    controller_pins_per_channel: usize,
    cross_revision_connectivity_agreement: bool,
    mirrored_by_construction: bool,
    unresolved_schematic_connections: usize,
    diode_orientation_confidence: &'static str,
    dc_solver_topology_ready: bool,
    nonlinear_device_models_ready: bool,
    evidence: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
enum BipolarPolarity {
    Pnp,
    Npn,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct MirroredActiveDevice {
    channel_a_designator: &'static str,
    channel_b_designator: &'static str,
    polarity: BipolarPolarity,
    evidence: &'static str,
    exact_part_assignment: &'static str,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct MirroredDiodeCandidate {
    channel_a_designator: &'static str,
    channel_b_designator: &'static str,
    candidate: &'static str,
    confidence: &'static str,
    evidence: &'static str,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct DiodeCandidateElectricalEvidence {
    candidate: &'static str,
    evidence_role: &'static str,
    reverse_repetitive_peak_v: f64,
    reverse_dc_v: f64,
    average_forward_current_ma: f64,
    peak_forward_current_ma: f64,
    surge_forward_current_ma: f64,
    forward_voltage_max_v: f64,
    forward_voltage_test_current_ma: f64,
    reverse_current_max_ua: f64,
    reverse_current_test_voltage_v: f64,
    terminal_capacitance_max_pf: Option<f64>,
    reverse_recovery_max_ns: Option<f64>,
    manufacturer_typical_forward_curve_available: bool,
    direct_target_designator_evidence: bool,
    source: &'static str,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct DiodeForwardPlotCalibration {
    candidate: &'static str,
    source: &'static str,
    page_width_px: u16,
    page_height_px: u16,
    x_left_px: f64,
    x_right_px: f64,
    voltage_left_v: f64,
    voltage_right_v: f64,
    y_top_px: f64,
    y_bottom_px: f64,
    current_top_a: f64,
    current_bottom_a: f64,
    reading_radius_px: f64,
    trace: &'static str,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct DiodeForwardAnchor {
    candidate: &'static str,
    x_px: f64,
    y_px: f64,
    forward_voltage_v: f64,
    forward_current_a: f64,
    voltage_reading_uncertainty_mv: f64,
    current_reading_uncertainty_percent: f64,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct DiodePiecewiseForwardValidation {
    candidate: &'static str,
    anchor_count: usize,
    minimum_forward_current_a: f64,
    maximum_forward_current_a: f64,
    minimum_forward_voltage_v: f64,
    maximum_forward_voltage_v: f64,
    anchors_strictly_monotonic: bool,
    maximum_anchor_round_trip_error_mv: f64,
    extrapolation_permitted: bool,
    interpretation: &'static str,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct DiodeSharedEndpointEnvelope {
    documentary_candidates: usize,
    temperature_c: f64,
    conservative_network_forward_peak_current_a: f64,
    shared_forward_endpoint_test_current_a: f64,
    shared_forward_voltage_minimum_v: f64,
    shared_forward_voltage_maximum_v: f64,
    network_forward_domain_covered: bool,
    extend_forward_endpoint_to_lower_current_is_monotonicity_hypothesis: bool,
    conservative_network_reverse_peak_v: f64,
    shared_reverse_test_voltage_minimum_v: f64,
    shared_reverse_current_maximum_a: f64,
    network_reverse_domain_covered: bool,
    extend_reverse_endpoint_to_lower_voltage_is_monotonicity_hypothesis: bool,
    candidate_specific_typical_centers: usize,
    interval_model_ready: bool,
    continuous_nominal_model_for_every_candidate_ready: bool,
    exact_target_identity_required_for_interval_model: bool,
    interpretation: &'static str,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct DiodeForwardVoltageInterval {
    candidate: &'static str,
    forward_current_a: f64,
    shared_voltage_minimum_v: f64,
    shared_voltage_maximum_v: f64,
    candidate_specific_typical_center_v: Option<f64>,
    typical_center_is_manufacturer_curve_not_guarantee: bool,
    interval_is_shared_endpoint_hypothesis: bool,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct DiodeModelReadiness {
    documentary_candidates: usize,
    candidates_with_endpoint_limits: usize,
    candidates_with_digitized_typical_curve: usize,
    digitized_curve_anchor_count: usize,
    digitized_curve_strictly_monotonic: bool,
    shared_endpoint_interval_model_ready: bool,
    network_forward_domain_covered: bool,
    network_reverse_domain_covered: bool,
    continuous_nominal_curve_for_every_candidate_ready: bool,
    exact_target_type_solved: bool,
    bounded_forward_iv_for_every_candidate: bool,
    bounded_reverse_iv_for_every_candidate: bool,
    safe_scope: &'static str,
    interpretation: &'static str,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct BoardWideBipolarInventory {
    pnp_candidates: [&'static str; 4],
    npn_candidates: [&'static str; 6],
    scope: &'static str,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct RelatedFamilyActiveDeviceRule {
    npn_candidates: [&'static str; 2],
    pnp_candidates: [&'static str; 2],
    diode: &'static str,
    topology_relation: &'static str,
    evidence_level: &'static str,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct ComparativeCandidateShortlist {
    npn_candidates: [&'static str; 2],
    pnp_candidates: [&'static str; 2],
    diode_intersection: [&'static str; 0],
    derivation: &'static str,
    direct_target_evidence: bool,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct ActiveDeviceAssignmentSpace {
    pnp_positions_per_channel: usize,
    npn_positions_per_channel: usize,
    board_inventory_assignments: usize,
    comparative_shortlist_assignments: usize,
    uniform_family_per_polarity_hypotheses: usize,
    channel_b_is_constrained_to_mirror_channel_a: bool,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct ActiveDeviceHypothesis {
    index: usize,
    channel_a_pnp: [&'static str; 2],
    channel_a_npn: [&'static str; 3],
    channel_b_uses_mirrored_assignments: bool,
    direct_target_evidence: bool,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct CandidateBipolarModelEvidence {
    candidate_key: &'static str,
    device: &'static str,
    polarity: BipolarPolarity,
    allowed_ranks: &'static str,
    hfe_minimum: f64,
    hfe_maximum: f64,
    hfe_condition: &'static str,
    vcbo_absolute_volts: f64,
    vceo_absolute_volts: f64,
    vebo_absolute_volts: f64,
    collector_current_absolute_maximum_ma: f64,
    collector_power_absolute_maximum_mw: f64,
    collector_cutoff_current_maximum_ua: f64,
    collector_cutoff_condition: &'static str,
    emitter_cutoff_current_maximum_ua: f64,
    emitter_cutoff_condition: &'static str,
    vce_saturation_typical_absolute_volts: Option<f64>,
    vce_saturation_maximum_absolute_volts: f64,
    vce_saturation_test_collector_current_absolute_ma: f64,
    vce_saturation_test_base_current_absolute_ma: f64,
    vce_saturation_condition: &'static str,
    vbe_saturation_maximum_absolute_volts: Option<f64>,
    vbe_saturation_condition: Option<&'static str>,
    transition_frequency_mhz: f64,
    transition_frequency_limit_kind: &'static str,
    transition_frequency_condition: &'static str,
    collector_output_capacitance_typical_pf: f64,
    collector_output_capacitance_maximum_pf: Option<f64>,
    collector_output_capacitance_condition: &'static str,
    base_intrinsic_resistance_typical_ohms: Option<f64>,
    common_emitter_h_parameter_condition: Option<&'static str>,
    common_emitter_h_parameter_emitter_current_absolute_ma: Option<f64>,
    common_emitter_hie_typical_ohms: Option<f64>,
    common_emitter_hre_typical: Option<f64>,
    common_emitter_hfe_typical: Option<f64>,
    common_emitter_hoe_typical_microsiemens: Option<f64>,
    source: &'static str,
    supports_complete_large_signal_model: bool,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct BjtOutputCharacteristicPlotEvidence {
    candidate_key: &'static str,
    source: &'static str,
    polarity: BipolarPolarity,
    temperature_c: f64,
    page_width_px: u16,
    page_height_px: u16,
    x_left_px: f64,
    x_right_px: f64,
    collector_emitter_voltage_left_v: f64,
    collector_emitter_voltage_right_v: f64,
    y_top_px: f64,
    y_bottom_px: f64,
    collector_current_top_a: f64,
    collector_current_bottom_a: f64,
    base_current_curve_levels_a: &'static [f64],
    includes_zero_base_current_curve: bool,
    reading_radius_px: f64,
    calibration_basis: &'static str,
    curve_status: &'static str,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct BjtOutputCharacteristicEvidenceValidation {
    candidate_count: usize,
    pnp_candidate_count: usize,
    npn_candidate_count: usize,
    total_labeled_base_current_curves: usize,
    every_candidate_has_zero_base_current_curve: bool,
    every_positive_base_current_family_strictly_increasing: bool,
    minimum_shared_collector_emitter_voltage_domain_v: f64,
    minimum_shared_collector_current_domain_a: f64,
    conservative_network_peak_current_a: f64,
    saturation_knee_voltage_domain_v: f64,
    every_family_covers_conservative_network_current: bool,
    every_family_covers_saturation_knee_voltage: bool,
    direct_base_drive_evidence_available_for_every_candidate: bool,
    minimum_voltage_reading_uncertainty_mv: f64,
    maximum_voltage_reading_uncertainty_mv: f64,
    minimum_collector_current_reading_uncertainty_ma: f64,
    maximum_collector_current_reading_uncertainty_ma: f64,
    interpolation_between_labeled_base_current_curves_is_hypothesis: bool,
    digitized_output_surface_ready: bool,
    suitable_as_confirmed_production_device_model: bool,
    interpretation: &'static str,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct BjtOutputCharacteristicClampedPointResolution {
    designator: &'static str,
    candidate_key: &'static str,
    collector_emitter_voltage_absolute_v: f64,
    collector_current_a: f64,
    preliminary_forward_beta_base_current_a: f64,
    smallest_positive_labeled_base_current_a: f64,
    preliminary_base_current_inside_zero_to_first_curve_bracket: bool,
    plot_voltage_reading_uncertainty_mv: f64,
    vce_exceeds_plot_reading_uncertainty: bool,
    vce_to_plot_reading_uncertainty_ratio: f64,
    minimum_solver_resolution_margin_radii: f64,
    vce_meets_solver_resolution_margin: bool,
    directly_resolvable_from_archived_plot: bool,
    interpretation: &'static str,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct BjtOutputCharacteristicClampedResolutionValidation {
    evaluated_clamped_points: usize,
    points_inside_zero_to_first_base_current_curve_bracket: usize,
    points_with_vce_above_plot_reading_uncertainty: usize,
    points_meeting_solver_resolution_margin: usize,
    minimum_solver_resolution_margin_radii: f64,
    every_clamped_point_directly_resolvable: bool,
    higher_resolution_source_or_bounded_low_vce_model_required: bool,
    digitization_may_proceed_for_resolvable_domain_only: bool,
    suitable_for_immediate_solver_substitution: bool,
    interpretation: &'static str,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct BjtLowVceBaseDriveHypothesis {
    index: usize,
    join_shape: BjtRegionJoinShape,
    blend_to_first_labeled_curve: f64,
    guard_margin_reading_radii: f64,
    first_labeled_curve_is_sensitivity_target_not_bound: bool,
    suitable_as_identified_device_law: bool,
}

#[derive(Clone, Copy, Debug)]
struct BjtLowVceBaseDriveEvaluation {
    base_current_a: f64,
    guard_voltage_v: f64,
    proximity_weight: f64,
    first_labeled_base_current_a: f64,
    base_current_adjustment_a: f64,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct BjtLowVceBaseDriveSweepPoint {
    hypothesis_index: usize,
    active_device_assignment_index: usize,
    converged: bool,
    maximum_kcl_residual_a: f64,
    control_voltage_v: f64,
    timing_voltage_v: f64,
    feedback_base_voltage_v: f64,
    total_collector_current_a: f64,
    maximum_collector_power_mw: f64,
    adjusted_bjt_count: usize,
    remaining_transfer_clamp_count: usize,
    maximum_base_current_adjustment_a: f64,
    inside_supply_polarity_and_power_gates: bool,
}

#[derive(Clone, Debug, Serialize)]
struct BjtLowVceBaseDriveSweepValidation {
    defined_hypothesis_count: usize,
    reference_screen_survivor_count: usize,
    reference_screen_rejected_count: usize,
    hypothesis_count: usize,
    assignment_count: usize,
    expected_matrix_points: usize,
    evaluated_matrix_points: usize,
    converged_matrix_points: usize,
    points_inside_supply_polarity_and_power_gates: usize,
    points_without_transfer_clamping: usize,
    hypotheses_converged_for_all_assignments: usize,
    hypotheses_eliminating_clamps_for_all_assignments: usize,
    metric_comparisons: [ActiveDeviceDcMetricComparison; 5],
    complete_dc_ensemble_accepted: bool,
    required_audio_trajectory_metrics_evaluated: bool,
    suitable_for_production_promotion: bool,
    interpretation: &'static str,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct BjtReciprocalTransportHypothesis {
    index: usize,
    reverse_beta: f64,
    forward_and_reverse_transport_share_reciprocal_scale: bool,
    reverse_beta_is_unpublished_sensitivity_axis: bool,
    suitable_as_identified_device_law: bool,
}

#[derive(Clone, Copy, Debug)]
struct BjtReciprocalTransportParameters {
    hypothesis_index: Option<usize>,
    default_reverse_beta: f64,
    tr19_toshiba_differential_conductance_us: f64,
    tr22_toshiba_reverse_beta: f64,
    tr23_toshiba_reverse_beta: f64,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct BjtReciprocalTransportSweepPoint {
    hypothesis_index: usize,
    active_device_assignment_index: usize,
    converged: bool,
    maximum_kcl_residual_a: f64,
    control_voltage_v: f64,
    timing_voltage_v: f64,
    feedback_base_voltage_v: f64,
    total_collector_terminal_current_absolute_a: f64,
    maximum_collector_power_mw: f64,
    devices_with_reverse_transport: usize,
    forward_transfer_clamp_count: usize,
    reverse_transfer_clamp_count: usize,
    inside_supply_polarity_and_power_gates: bool,
}

#[derive(Clone, Debug, Serialize)]
struct BjtReciprocalTransportSweepValidation {
    hypothesis_count: usize,
    assignment_count: usize,
    expected_matrix_points: usize,
    evaluated_matrix_points: usize,
    converged_matrix_points: usize,
    points_inside_supply_polarity_and_power_gates: usize,
    points_without_forward_transfer_clamping: usize,
    points_without_any_transfer_clamping: usize,
    hypotheses_converged_for_all_assignments: usize,
    hypotheses_eliminating_forward_clamps_for_all_assignments: usize,
    metric_comparisons: [ActiveDeviceDcMetricComparison; 5],
    complete_dc_ensemble_accepted: bool,
    required_audio_trajectory_metrics_evaluated: bool,
    suitable_for_production_promotion: bool,
    interpretation: &'static str,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct BjtReciprocalTransportHomotopyStage {
    hypothesis_index: usize,
    blend_fraction: f64,
    converged: bool,
    maximum_kcl_residual_a: f64,
    every_unknown_inside_supply_window: bool,
    exact_target_reached: bool,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct BjtReciprocalTransportHomotopySummary {
    hypothesis_index: usize,
    reverse_beta: f64,
    attempted_stages: usize,
    converged_stages: usize,
    largest_converged_blend_fraction: f64,
    exact_target_reached: bool,
    exact_target_inside_all_dc_gates: bool,
    final_every_unknown_inside_supply_window: bool,
    final_every_bjt_vce_polarity_valid: bool,
    final_every_bjt_inside_power_limit: bool,
    final_forward_transfer_clamp_count: usize,
    final_reverse_transfer_clamp_count: usize,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct BjtReciprocalTransportHomotopyValidation {
    hypothesis_count: usize,
    stages_per_hypothesis: usize,
    exact_targets_reached: usize,
    exact_targets_inside_all_dc_gates: usize,
    exact_targets_without_any_transfer_clamping: usize,
    homotopy_is_numerical_path_not_device_evidence: bool,
    reciprocal_transport_family_viable_at_reference: bool,
    suitable_for_production_promotion: bool,
    interpretation: &'static str,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct BjtReciprocalTransportAssignmentDevicePoint {
    designator: &'static str,
    candidate_key: &'static str,
    base_emitter_voltage_absolute_v: f64,
    base_collector_voltage_absolute_v: f64,
    collector_emitter_voltage_absolute_v: f64,
    collector_current_signed_a: f64,
    collector_current_absolute_a: f64,
    base_current_absolute_a: f64,
    reverse_transport_current_absolute_a: f64,
    collector_power_mw: f64,
}

#[derive(Clone, Debug, Serialize)]
struct BjtReciprocalTransportAssignmentPoint {
    active_device_assignment_index: usize,
    converged: bool,
    adaptive_homotopy_used: bool,
    homotopy_stages_completed: usize,
    maximum_kcl_residual_a: f64,
    control_voltage_v: f64,
    timing_voltage_v: f64,
    feedback_base_voltage_v: f64,
    total_collector_terminal_current_absolute_a: f64,
    maximum_collector_power_mw: f64,
    maximum_collector_power_designator: &'static str,
    forward_transfer_clamp_count: usize,
    reverse_transfer_clamp_count: usize,
    inside_supply_transport_and_power_gates: bool,
    node_voltages: Vec<ActiveNetworkDcNodeVoltage>,
    device_points: Vec<BjtReciprocalTransportAssignmentDevicePoint>,
}

#[derive(Clone, Debug, Serialize)]
struct BjtReciprocalTransportAssignmentValidation {
    attempted_assignments: usize,
    converged_assignments: usize,
    assignments_using_adaptive_homotopy: usize,
    assignments_inside_all_dc_gates: usize,
    assignments_without_any_transfer_clamping: usize,
    complete_32_assignment_ensemble: bool,
    metric_comparisons: [ActiveDeviceDcMetricComparison; 5],
    dc_metrics_accepted_as_equivalent: bool,
    required_audio_trajectory_metrics_evaluated: bool,
    accepted_as_behaviorally_equivalent: bool,
    suitable_for_production_promotion: bool,
    interpretation: &'static str,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct BjtReciprocalTransportFactorEffect {
    designator: &'static str,
    polarity: BipolarPolarity,
    first_candidate_key: &'static str,
    second_candidate_key: &'static str,
    samples_per_candidate: usize,
    collector_current_first_candidate_mean_a: f64,
    collector_current_second_candidate_mean_a: f64,
    collector_current_main_effect_a: f64,
    collector_current_main_effect_relative_to_grand_mean_percent: f64,
    collector_current_centered_spread_share_percent: f64,
    maximum_power_first_candidate_mean_mw: f64,
    maximum_power_second_candidate_mean_mw: f64,
    maximum_power_main_effect_mw: f64,
    maximum_power_main_effect_relative_to_grand_mean_percent: f64,
    maximum_power_centered_spread_share_percent: f64,
}

#[derive(Clone, Debug, Serialize)]
struct BjtReciprocalTransportDispersionAttribution {
    assignment_count: usize,
    balanced_complete_two_level_factorial: bool,
    collector_current_grand_mean_a: f64,
    maximum_power_grand_mean_mw: f64,
    factor_effects: Vec<BjtReciprocalTransportFactorEffect>,
    collector_current_main_effect_spread_share_percent: f64,
    collector_current_interaction_spread_share_percent: f64,
    maximum_power_main_effect_spread_share_percent: f64,
    maximum_power_interaction_spread_share_percent: f64,
    dominant_collector_current_designator: &'static str,
    dominant_maximum_power_designator: &'static str,
    minimum_collector_current_assignment_index: usize,
    maximum_collector_current_assignment_index: usize,
    minimum_maximum_power_assignment_index: usize,
    maximum_maximum_power_assignment_index: usize,
    maximum_power_device_designator: &'static str,
    maximum_power_device_is_constant_across_assignments: bool,
    attribution_is_numerical_sensitivity_not_device_identity: bool,
    suitable_for_production_parameter_narrowing: bool,
    interpretation: &'static str,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct BjtReciprocalTransportDeviceEvidenceEnvelope {
    designator: &'static str,
    polarity: BipolarPolarity,
    candidate_key: &'static str,
    sample_count: usize,
    minimum_vce_v: f64,
    maximum_vce_v: f64,
    minimum_vbe_v: f64,
    maximum_vbe_v: f64,
    minimum_vbc_v: f64,
    maximum_vbc_v: f64,
    minimum_collector_current_a: f64,
    maximum_collector_current_a: f64,
    minimum_base_current_a: f64,
    maximum_base_current_a: f64,
    minimum_reverse_transport_current_a: f64,
    maximum_reverse_transport_current_a: f64,
    maximum_collector_power_mw: f64,
    maximum_power_fraction_of_25c_limit_percent: f64,
    power_derating_critical_ambient_c: f64,
    minimum_output_plot_voltage_resolution_radii: f64,
    every_vce_inside_output_plot_domain: bool,
    every_collector_current_inside_output_plot_domain: bool,
    every_base_current_inside_labeled_curve_domain: bool,
    every_coordinate_resolvable_at_two_reading_radii: bool,
    output_characteristic_surface_digitized: bool,
    direct_three_temperature_transfer_available: bool,
    comparative_temperature_interval_available: bool,
    evidence_directly_bounds_operating_region: bool,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct BjtReciprocalTransportTr22Tr23EvidenceAudit {
    evaluated_candidate_envelopes: usize,
    evaluated_device_samples: usize,
    tr22_candidates_inside_output_plot_voltage_domain: usize,
    tr22_candidates_inside_output_plot_current_domain: usize,
    tr22_candidates_inside_labeled_base_current_domain: usize,
    tr22_candidates_resolvable_at_two_reading_radii: usize,
    tr23_candidates_with_negative_vce: usize,
    tr23_candidates_inside_forward_output_plot_domain: usize,
    tr23_candidates_with_reverse_transport_at_every_sample: usize,
    candidates_with_direct_thermal_transfer: usize,
    candidates_with_only_comparative_thermal_interval: usize,
    digitized_output_surfaces_available: usize,
    tr22_output_surface_digitization_required: bool,
    tr22_mitsubishi_base_drive_extension_evidence_required: bool,
    tr23_reverse_transport_evidence_required: bool,
    existing_documents_sufficient_to_bound_remaining_current_spread: bool,
    suitable_for_production_parameter_narrowing: bool,
    interpretation: &'static str,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct BjtTr22Tr23ReverseBetaRequirement {
    tr19_differential_conductance_us: f64,
    tr19_conductance_inside_power_acceptance_interval: bool,
    first_candidate_key: &'static str,
    second_candidate_key: &'static str,
    second_candidate_reverse_beta_held_at: f64,
    minimum_screened_reverse_beta: f64,
    maximum_screened_reverse_beta: f64,
    reverse_beta_step: f64,
    evaluated_beta_pairs: usize,
    current_accepted_beta_pairs: usize,
    joint_current_power_accepted_beta_pairs: usize,
    tr22_accepted_reverse_beta_minimum: Option<f64>,
    tr22_accepted_reverse_beta_maximum: Option<f64>,
    tr23_accepted_reverse_beta_minimum: Option<f64>,
    tr23_accepted_reverse_beta_maximum: Option<f64>,
    best_tr22_reverse_beta: f64,
    best_tr23_reverse_beta: f64,
    best_current_comparison: UncertaintyComparison,
    power_comparison: UncertaintyComparison,
    accepted_region_is_not_rectangular_bound: bool,
    direct_reverse_beta_evidence_available: bool,
    frozen_node_screen_omits_base_kcl_resolve: bool,
    suitable_for_solver_substitution: bool,
    suitable_for_production_parameter_narrowing: bool,
    interpretation: &'static str,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct BjtCorrelatedReverseTransportFrozenResidualPoint {
    active_device_assignment_index: usize,
    changed_device_count: usize,
    tr19_collector_current_delta_a: f64,
    tr22_base_current_delta_a: f64,
    tr23_base_current_delta_a: f64,
    maximum_parameter_delta_kcl_residual_a: f64,
    rms_parameter_delta_kcl_residual_a: f64,
    maximum_parameter_delta_kcl_residual_node: ActiveNetworkNode,
    exceeds_solver_kcl_acceptance: bool,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct BjtCorrelatedReverseTransportFrozenResidualAudit {
    evaluated_assignments: usize,
    assignments_without_parameter_delta: usize,
    assignments_exceeding_solver_kcl_acceptance: usize,
    solver_kcl_acceptance_a: f64,
    maximum_parameter_delta_kcl_residual_a: f64,
    maximum_parameter_delta_kcl_residual_multiple: f64,
    worst_assignment_index: usize,
    worst_residual_node: ActiveNetworkNode,
    maximum_tr19_collector_current_delta_a: f64,
    maximum_tr22_base_current_delta_a: f64,
    maximum_tr23_base_current_delta_a: f64,
    frozen_node_requirement_is_kcl_compatible: bool,
    full_nonlinear_base_and_collector_kcl_resolve_required: bool,
    direct_parameter_evidence_available: bool,
    suitable_for_solver_substitution: bool,
    suitable_for_production_parameter_narrowing: bool,
    interpretation: &'static str,
}

#[derive(Clone, Debug, Serialize)]
struct BjtCorrelatedReverseTransportResolvePoint {
    active_device_assignment_index: usize,
    direct_target_converged: bool,
    adaptive_homotopy_used: bool,
    homotopy_stages_completed: usize,
    converged: bool,
    maximum_kcl_residual_a: f64,
    maximum_node_voltage_displacement_v: f64,
    maximum_displacement_node: ActiveNetworkNode,
    control_voltage_v: f64,
    timing_voltage_v: f64,
    feedback_base_voltage_v: f64,
    total_collector_terminal_current_absolute_a: f64,
    maximum_collector_power_mw: f64,
    maximum_collector_power_designator: &'static str,
    forward_transfer_clamp_count: usize,
    reverse_transfer_clamp_count: usize,
    inside_supply_transport_and_power_gates: bool,
    device_points: Vec<BjtReciprocalTransportAssignmentDevicePoint>,
}

#[derive(Clone, Debug, Serialize)]
struct BjtCorrelatedReverseTransportResolveValidation {
    attempted_assignments: usize,
    direct_target_converged_assignments: usize,
    assignments_using_adaptive_homotopy: usize,
    converged_assignments: usize,
    assignments_inside_all_dc_gates: usize,
    assignments_without_any_transfer_clamping: usize,
    maximum_node_voltage_displacement_v: f64,
    maximum_displacement_assignment_index: usize,
    maximum_displacement_node: ActiveNetworkNode,
    metric_comparisons: [ActiveDeviceDcMetricComparison; 5],
    resolved_dc_metrics_accepted_as_equivalent: bool,
    target_parameters_have_direct_evidence: bool,
    numerical_path_suitable_for_evidence_testing: bool,
    required_audio_trajectory_metrics_evaluated: bool,
    suitable_for_production_parameter_narrowing: bool,
    suitable_for_production_promotion: bool,
    interpretation: &'static str,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct BjtReciprocalTransportTr19EvidenceAudit {
    evaluated_envelopes: usize,
    tr19_candidate_envelopes: usize,
    tr19_samples: usize,
    tr19_candidates_inside_output_plot_voltage_domain: usize,
    tr19_candidates_inside_output_plot_current_domain: usize,
    tr19_candidates_inside_labeled_base_current_domain: usize,
    tr19_candidates_resolvable_at_two_reading_radii: usize,
    tr19_candidates_with_direct_thermal_transfer: usize,
    tr19_candidates_with_comparative_thermal_interval: usize,
    tr19_candidates_inside_25c_power_limit: usize,
    tr19_operating_region_directly_bounded_by_current_evidence: bool,
    existing_documents_sufficient_to_narrow_reciprocal_current_power_spread: bool,
    global_output_conductance_evidence_required: bool,
    higher_voltage_output_characteristic_evidence_required: bool,
    suitable_for_production_parameter_narrowing: bool,
    interpretation: &'static str,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct BjtTr19HighVceConductanceRequirement {
    reference_voltage_v: f64,
    minimum_differential_conductance_us: f64,
    maximum_differential_conductance_us: f64,
    conductance_step_us: f64,
    evaluated_hypotheses: usize,
    correction_applied_to_candidate: &'static str,
    correction_held_zero_for_candidate: &'static str,
    current_acceptance_interval_minimum_us: Option<f64>,
    current_acceptance_interval_maximum_us: Option<f64>,
    power_acceptance_interval_minimum_us: Option<f64>,
    power_acceptance_interval_maximum_us: Option<f64>,
    joint_acceptance_interval_minimum_us: Option<f64>,
    joint_acceptance_interval_maximum_us: Option<f64>,
    best_current_conductance_us: f64,
    best_current_comparison: UncertaintyComparison,
    power_comparison_at_best_current: UncertaintyComparison,
    published_mitsubishi_local_hoe_us: f64,
    tr19_differential_conductance_alone_can_pass_current_gate: bool,
    tr19_differential_conductance_alone_can_pass_power_gate: bool,
    tr19_differential_conductance_alone_can_pass_both_gates: bool,
    remaining_transport_evidence_priorities: [&'static str; 2],
    frozen_node_linearized_screen_not_resolved_operating_points: bool,
    suitable_for_production_parameter_narrowing: bool,
    interpretation: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
enum PlotCurrentAxis {
    Linear,
    Logarithmic,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
enum TransferCurrentKind {
    Base,
    Collector,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
enum BjtThermalCurveKind {
    ForwardCurrentGain,
    CollectorEmitterSaturationVoltage,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct BjtTransferPlotCalibration {
    candidate_key: &'static str,
    source: &'static str,
    page_width_px: u16,
    page_height_px: u16,
    x_left_px: f64,
    x_right_px: f64,
    vbe_left_v: f64,
    vbe_right_v: f64,
    y_top_px: f64,
    y_bottom_px: f64,
    current_top_a: f64,
    current_bottom_a: f64,
    current_axis: PlotCurrentAxis,
    current_kind: TransferCurrentKind,
    reading_radius_px: f64,
    curve_status: &'static str,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct BjtTransferPixelAnchor {
    candidate_key: &'static str,
    x_px: f64,
    y_px: f64,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct BjtTransferElectricalAnchor {
    candidate_key: &'static str,
    current_kind: TransferCurrentKind,
    vbe_v: f64,
    current_a: f64,
    vbe_reading_uncertainty_mv: f64,
    current_reading_uncertainty_percent: f64,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct BjtTypicalTransferFit {
    candidate_key: &'static str,
    current_kind: TransferCurrentKind,
    anchor_count: usize,
    temperature_c: f64,
    thermal_voltage_v: f64,
    fitted_emission_coefficient: f64,
    fitted_scale_current_a: f64,
    rms_vbe_residual_mv: f64,
    maximum_absolute_vbe_residual_mv: f64,
    maximum_anchor_vbe_reading_uncertainty_mv: f64,
    maximum_anchor_current_reading_uncertainty_percent: f64,
    single_exponential_covers_plot_reading_error: bool,
    fit_covers_unit_to_unit_variation: bool,
    anchors_suitable_for_piecewise_hypothesis_center: bool,
    single_exponential_suitable_for_hypothesis_center: bool,
    suitable_as_guaranteed_device_bound: bool,
    interpretation: &'static str,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct BjtPiecewiseTransferValidation {
    candidate_key: &'static str,
    current_kind: TransferCurrentKind,
    anchor_count: usize,
    minimum_current_a: f64,
    maximum_current_a: f64,
    minimum_vbe_v: f64,
    maximum_vbe_v: f64,
    anchors_strictly_monotonic: bool,
    maximum_anchor_round_trip_error_mv: f64,
    extrapolation_permitted: bool,
    interpretation: &'static str,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct BjtForwardActiveEnvelope {
    candidate_key: &'static str,
    current_kind: TransferCurrentKind,
    beta_minimum: f64,
    beta_geometric_center: f64,
    beta_maximum: f64,
    digitized_source_current_minimum_a: f64,
    digitized_source_current_maximum_a: f64,
    common_collector_current_minimum_a: f64,
    common_collector_current_maximum_a: f64,
    conservative_network_peak_current_a: f64,
    digitized_domain_covers_network_peak: bool,
    maximum_vbe_reading_uncertainty_mv: f64,
    beta_rank_bounds_available: bool,
    beta_rank_applied_outside_test_point_is_hypothesis: bool,
    unit_vbe_spread_available: bool,
    cutoff_model_ready: bool,
    saturation_model_ready: bool,
    early_effect_model_ready: bool,
    preliminary_forward_active_domain_ready: bool,
    full_dc_model_ready: bool,
    interpretation: &'static str,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct BjtEvidenceCorner {
    candidate_key: &'static str,
    beta_corner: &'static str,
    beta: f64,
    vbe_reading_corner: &'static str,
    vbe_offset_mv: f64,
    evidence_domain_only: bool,
    covers_unit_to_unit_variation: bool,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct BjtForwardActivePoint {
    candidate_key: &'static str,
    beta: f64,
    collector_current_a: f64,
    base_current_a: f64,
    vbe_offset_mv: f64,
    vbe_v: f64,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct BjtDcRegionEvidenceEnvelope {
    candidate_key: &'static str,
    temperature_c: f64,
    collector_cutoff_test_voltage_absolute_v: f64,
    collector_cutoff_current_maximum_a: f64,
    emitter_cutoff_test_voltage_absolute_v: f64,
    emitter_cutoff_current_maximum_a: f64,
    cutoff_endpoints_are_maximum_limits: bool,
    cutoff_continuous_iv_ready: bool,
    saturation_test_collector_current_a: f64,
    saturation_forced_beta: f64,
    saturation_vce_typical_absolute_v: Option<f64>,
    saturation_vce_maximum_absolute_v: f64,
    saturation_vbe_maximum_absolute_v: Option<f64>,
    conservative_network_peak_current_a: f64,
    network_peak_below_saturation_test_current: bool,
    extend_saturation_limit_to_lower_current_is_hypothesis: bool,
    saturation_continuous_iv_ready: bool,
    local_output_admittance_typical_siemens: Option<f64>,
    local_output_resistance_typical_ohms: Option<f64>,
    local_effective_early_voltage_absolute_v: Option<f64>,
    local_output_admittance_condition: Option<&'static str>,
    effective_early_voltage_is_local_proxy_not_spice_vaf: bool,
    early_effect_continuous_model_ready: bool,
    temperature_spread_ready: bool,
    full_dc_region_model_ready: bool,
    interpretation: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
enum BjtRegionJoinShape {
    Linear,
    Smoothstep,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
enum BjtCutoffLeakageCorner {
    Zero,
    PublishedMaximum,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
enum BjtSaturationVoltageCorner {
    ZeroLowerBound,
    DirectTypicalCurve,
    PublishedMaximumUpperBound,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
enum BjtOutputConductanceCorner {
    Zero,
    LocalTypicalHeldConstant,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
enum BjtContinuousDcRegion {
    CutoffJoin,
    ForwardActive,
    SaturationJoin,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct BjtContinuousDcHypothesis {
    index: usize,
    candidate_key: &'static str,
    cutoff_join_shape: BjtRegionJoinShape,
    cutoff_leakage_corner: BjtCutoffLeakageCorner,
    saturation_join_shape: BjtRegionJoinShape,
    saturation_voltage_corner: BjtSaturationVoltageCorner,
    output_conductance_corner: BjtOutputConductanceCorner,
    cutoff_uses_published_maximum_as_upper_corner_only: bool,
    saturation_typical_is_direct_toshiba_curve: bool,
    saturation_maximum_extended_below_test_current_is_hypothesis: bool,
    local_output_conductance_held_away_from_test_point_is_hypothesis: bool,
    covers_unit_to_unit_variation: bool,
    suitable_as_confirmed_device_model: bool,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct BjtContinuousDcPoint {
    hypothesis_index: usize,
    candidate_key: &'static str,
    region: BjtContinuousDcRegion,
    beta: f64,
    base_emitter_voltage_absolute_v: f64,
    collector_emitter_voltage_absolute_v: f64,
    forward_active_center_collector_current_a: f64,
    cutoff_leakage_corner_a: f64,
    saturation_voltage_absolute_v: f64,
    saturation_threshold_evaluated: bool,
    saturation_current_scale: f64,
    output_conductance_correction_a: f64,
    collector_current_a: f64,
    base_current_a: f64,
    collector_power_mw: f64,
    collector_power_inside_25c_published_limit: bool,
    evidence_and_hypothesis_boundary: &'static str,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct BjtContinuousDcValidation {
    candidate_key: &'static str,
    hypothesis_count: usize,
    evaluated_grid_points: usize,
    every_point_finite_and_nonnegative: bool,
    cutoff_endpoints_respect_zero_to_published_maximum: bool,
    cutoff_to_forward_active_join_is_continuous: bool,
    saturation_current_is_monotonic_with_vce: bool,
    direct_typical_saturation_never_extrapolates: bool,
    published_maximum_extension_is_labeled_hypothesis: bool,
    local_hoe_never_promoted_to_global_early_voltage: bool,
    preliminary_region_sweep_ready: bool,
    bounded_production_device_model_ready: bool,
    interpretation: &'static str,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct ActiveNetworkDcCoordinate {
    name: &'static str,
    triangle_input_v: f64,
    active_device_assignment_index: usize,
    mn3101_macro_hypothesis_index: usize,
    temperature_c: f64,
    interpretation: &'static str,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct ActiveNetworkDcNodeVoltage {
    node: ActiveNetworkNode,
    voltage_v: f64,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct ActiveNetworkDcDevicePoint {
    designator: &'static str,
    candidate_key: &'static str,
    continuous_hypothesis_index: usize,
    beta: f64,
    region: BjtContinuousDcRegion,
    base_emitter_voltage_absolute_v: f64,
    collector_emitter_voltage_absolute_v: f64,
    collector_current_a: f64,
    base_current_a: f64,
    forward_beta_base_current_a: f64,
    forced_beta_ten_base_current_a: f64,
    saturation_base_drive_blend: f64,
    forced_beta_ten_is_datasheet_test_condition_not_device_law: bool,
    low_vce_base_drive_hypothesis_index: Option<usize>,
    low_vce_guard_voltage_v: f64,
    low_vce_proximity_weight: f64,
    low_vce_first_labeled_base_current_a: f64,
    low_vce_base_current_adjustment_a: f64,
    reciprocal_transport_hypothesis_index: Option<usize>,
    base_collector_voltage_absolute_v: f64,
    reciprocal_reverse_beta: f64,
    reverse_transport_current_a: f64,
    reverse_transfer_voltage_was_clamped_to_digitized_domain: bool,
    collector_power_mw: f64,
    transfer_voltage_was_clamped_to_digitized_domain: bool,
    collector_emitter_polarity_valid: bool,
    inside_published_power_limit: bool,
}

#[derive(Clone, Debug, Serialize)]
struct ActiveNetworkDcOperatingPoint {
    coordinate: ActiveNetworkDcCoordinate,
    converged: bool,
    iterations: usize,
    seed_index: usize,
    used_continuation_seed: bool,
    maximum_kcl_residual_a: f64,
    rms_kcl_residual_a: f64,
    node_voltages: Vec<ActiveNetworkDcNodeVoltage>,
    device_points: Vec<ActiveNetworkDcDevicePoint>,
    diode_forward_voltage_v: f64,
    diode_current_a: f64,
    diode_curve_extension_used: bool,
    mn3101_point: Option<Mn3101OxMacroPoint>,
    every_unknown_inside_supply_window: bool,
    every_bjt_vce_polarity_valid: bool,
    every_bjt_inside_published_power_limit: bool,
    suitable_only_as_preliminary_fixed_point: bool,
    suitable_as_confirmed_physical_operating_point: bool,
    interpretation: &'static str,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct ActiveNetworkDcSolverValidation {
    coordinate_count: usize,
    converged_coordinate_count: usize,
    maximum_accepted_kcl_residual_a: f64,
    all_converged_points_inside_supply_window: bool,
    all_converged_bjt_points_respect_vce_polarity: bool,
    all_converged_bjt_points_inside_power_limits: bool,
    all_converged_bjt_points_inside_digitized_transfer_domain: bool,
    residual_assembly_covers_resistors: bool,
    residual_assembly_covers_bipolars: bool,
    residual_assembly_covers_diode_center: bool,
    residual_assembly_covers_mn3101_ox_ports: bool,
    capacitors_are_open_at_dc: bool,
    mn3101_hidden_supply_current_is_modeled: bool,
    preliminary_fixed_points_ready: bool,
    confirmed_physical_operating_points_ready: bool,
    interpretation: &'static str,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct ActiveDeviceDcSweepPoint {
    active_device_assignment_index: usize,
    converged: bool,
    maximum_kcl_residual_a: f64,
    control_voltage_v: f64,
    timing_voltage_v: f64,
    feedback_base_voltage_v: f64,
    total_collector_current_a: f64,
    maximum_collector_power_mw: f64,
    clamped_bjt_count: usize,
    inside_supply_polarity_and_power_gates: bool,
    suitable_for_complete_ensemble_comparison: bool,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct ActiveDeviceDcMetricComparison {
    metric: &'static str,
    unit: &'static str,
    comparison: Option<UncertaintyComparison>,
}

#[derive(Clone, Debug, Serialize)]
struct ActiveDeviceDcSweepValidation {
    attempted_assignments: usize,
    converged_assignments: usize,
    assignments_inside_all_dc_gates: usize,
    assignments_without_transfer_clamping: usize,
    complete_32_assignment_ensemble: bool,
    continuation_seed_used: bool,
    metric_comparisons: [ActiveDeviceDcMetricComparison; 5],
    dc_metrics_accepted_as_equivalent: bool,
    required_audio_trajectory_metrics_evaluated: bool,
    accepted_as_behaviorally_equivalent: bool,
    interpretation: &'static str,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct Mn3101FrozenNodeCompatibilityPoint {
    active_device_assignment_index: usize,
    mn3101_macro_hypothesis_index: usize,
    transition_shape: Mn3101TransitionShape,
    output_strength_corner: Mn3101OutputStrengthCorner,
    evaluation_succeeded: bool,
    maximum_kcl_residual_a: f64,
    rms_kcl_residual_a: f64,
    frozen_node_residual_passes: bool,
    combined_output_power_w: f64,
    combined_power_inside_absolute_limit: bool,
    interpretation: &'static str,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct Mn3101FrozenNodeCompatibilitySummary {
    mn3101_macro_hypothesis_index: usize,
    transition_shape: Mn3101TransitionShape,
    output_strength_corner: Mn3101OutputStrengthCorner,
    evaluated_assignments: usize,
    residual_passing_assignments: usize,
    mean_maximum_kcl_residual_a: f64,
    worst_maximum_kcl_residual_a: f64,
    every_point_inside_mn3101_power_limit: bool,
    suitable_as_continuation_seed_for_all_assignments: bool,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct Mn3101FrozenNodeMatrixValidation {
    expected_matrix_points: usize,
    evaluated_matrix_points: usize,
    assignment_count: usize,
    macro_hypothesis_count: usize,
    macros_compatible_with_all_frozen_assignment_points: usize,
    every_macro_point_inside_absolute_power_limit: bool,
    matrix_is_screening_not_operating_point_solution: bool,
    actual_mn3101_transition_identified: bool,
    interpretation: &'static str,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct Mn3101HomotopyStage {
    target_macro_hypothesis_index: usize,
    blend_fraction: f64,
    converged: bool,
    maximum_kcl_residual_a: f64,
    inside_supply_polarity_and_power_gates: bool,
    exact_target_macro_reached: bool,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct Mn3101HomotopySummary {
    target_macro_hypothesis_index: usize,
    transition_shape: Mn3101TransitionShape,
    output_strength_corner: Mn3101OutputStrengthCorner,
    attempted_stages: usize,
    converged_stages: usize,
    largest_converged_blend_fraction: f64,
    exact_target_macro_reached: bool,
    final_maximum_kcl_residual_a: f64,
    final_converged: bool,
    final_every_unknown_inside_supply_window: bool,
    final_every_bjt_vce_polarity_valid: bool,
    final_every_bjt_inside_power_limit: bool,
    final_mn3101_inside_power_limit: bool,
    final_inside_supply_polarity_and_power_gates: bool,
    final_clamped_bjt_count: usize,
    suitable_as_preliminary_nonlinear_seed: bool,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct Mn3101HomotopyValidation {
    source_macro_hypothesis_index: usize,
    target_macro_hypotheses: usize,
    exact_targets_reached: usize,
    exact_targets_inside_all_dc_gates: usize,
    exact_targets_without_bjt_transfer_clamping: usize,
    homotopy_is_numerical_path_not_device_evidence: bool,
    actual_mn3101_transition_identified: bool,
    interpretation: &'static str,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct BjtSaturationBaseDriveHomotopyStage {
    blend_fraction: f64,
    converged: bool,
    maximum_kcl_residual_a: f64,
    every_unknown_inside_supply_window: bool,
    every_bjt_vce_polarity_valid: bool,
    every_bjt_inside_power_limit: bool,
    inside_supply_polarity_and_power_gates: bool,
    clamped_bjt_count: usize,
    maximum_base_current_multiplier_over_forward_beta: f64,
    exact_forced_beta_ten_hypothesis_reached: bool,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct BjtSaturationBaseDriveHomotopyValidation {
    attempted_stages: usize,
    converged_stages: usize,
    largest_converged_blend_fraction: f64,
    initial_clamped_bjt_count: usize,
    final_clamped_bjt_count: usize,
    exact_forced_beta_ten_hypothesis_reached: bool,
    exact_hypothesis_inside_all_dc_gates: bool,
    transfer_domain_clamps_eliminated: bool,
    forced_beta_ten_is_published_test_condition: bool,
    forced_beta_ten_is_continuous_device_law: bool,
    suitable_as_confirmed_saturation_model: bool,
    interpretation: &'static str,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct BjtPublishedThermalEvidence {
    candidate_key: &'static str,
    transfer_current_kind: TransferCurrentKind,
    transfer_curve_temperatures_c: &'static [f64],
    saturation_curve_temperatures_c: &'static [f64],
    hfe_curve_temperatures_c: &'static [f64],
    collector_power_graph_minimum_ambient_c: f64,
    collector_power_flat_through_ambient_c: f64,
    collector_power_zero_at_ambient_c: f64,
    collector_power_at_25c_mw: f64,
    temperature_curves_are_manufacturer_typical_not_limits: bool,
    power_derating_is_direct_published_graph: bool,
    direct_candidate_temperature_transfer_available: bool,
    source: &'static str,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct BjtThermalTransferPlotCalibration {
    candidate_key: &'static str,
    source: &'static str,
    render_dpi: u16,
    crop_origin_x_px: u16,
    crop_origin_y_px: u16,
    crop_width_px: u16,
    crop_height_px: u16,
    x_left_px: f64,
    x_right_px: f64,
    vbe_left_absolute_v: f64,
    vbe_right_absolute_v: f64,
    y_top_px: f64,
    y_bottom_px: f64,
    current_top_a: f64,
    current_bottom_a: f64,
    current_kind: TransferCurrentKind,
    reading_radius_px: f64,
    curve_status: &'static str,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct BjtThermalTransferPixelAnchor {
    candidate_key: &'static str,
    temperature_c: f64,
    x_px: f64,
    y_px: f64,
}

type BjtThermalPixelTrace = (&'static str, f64, &'static [(f64, f64)]);

#[derive(Clone, Copy, Debug, Serialize)]
struct BjtThermalTransferElectricalAnchor {
    candidate_key: &'static str,
    temperature_c: f64,
    current_kind: TransferCurrentKind,
    current_a: f64,
    vbe_absolute_v: f64,
    vbe_reading_uncertainty_mv: f64,
    current_reading_uncertainty_percent: f64,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct BjtThermalTransferValidation {
    candidate_key: &'static str,
    temperatures_c: [f64; 3],
    anchors_per_temperature: usize,
    total_anchor_count: usize,
    every_trace_strictly_monotonic: bool,
    thermal_order_preserved_at_every_current: bool,
    maximum_anchor_round_trip_error_mv: f64,
    interpolation_inside_temperature_and_current_domain_ready: bool,
    extrapolation_permitted: bool,
    typical_curve_covers_unit_to_unit_variation: bool,
    interpretation: &'static str,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct BjtThermalTransferPoint {
    candidate_key: &'static str,
    temperature_c: f64,
    current_kind: TransferCurrentKind,
    source_current_a: f64,
    vbe_absolute_v: f64,
    vbe_shift_from_25c_mv: f64,
    temperature_is_direct_published_trace: bool,
    current_is_interpolated_between_digitized_anchors: bool,
    manufacturer_typical_not_production_bound: bool,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct BjtThermalCharacteristicPlotCalibration {
    candidate_key: &'static str,
    curve_kind: BjtThermalCurveKind,
    source: &'static str,
    condition: &'static str,
    render_dpi: u16,
    crop_origin_x_px: u16,
    crop_origin_y_px: u16,
    crop_width_px: u16,
    crop_height_px: u16,
    x_left_px: f64,
    x_right_px: f64,
    collector_current_left_a: f64,
    collector_current_right_a: f64,
    y_top_px: f64,
    y_bottom_px: f64,
    value_top: f64,
    value_bottom: f64,
    reading_radius_px: f64,
    axes_are_logarithmic: bool,
    manufacturer_typical_not_limit: bool,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct BjtThermalCharacteristicPixelAnchor {
    candidate_key: &'static str,
    curve_kind: BjtThermalCurveKind,
    temperature_c: f64,
    x_px: f64,
    y_px: f64,
}

type BjtThermalCharacteristicPixelTrace = (
    &'static str,
    BjtThermalCurveKind,
    f64,
    &'static [(f64, f64)],
);

#[derive(Clone, Copy, Debug, Serialize)]
struct BjtThermalCharacteristicElectricalAnchor {
    candidate_key: &'static str,
    curve_kind: BjtThermalCurveKind,
    temperature_c: f64,
    collector_current_a: f64,
    value: f64,
    collector_current_reading_uncertainty_percent: f64,
    value_reading_uncertainty_percent: f64,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct BjtThermalCharacteristicPoint {
    candidate_key: &'static str,
    curve_kind: BjtThermalCurveKind,
    temperature_c: f64,
    collector_current_a: f64,
    value: f64,
    temperature_is_direct_published_trace: bool,
    current_is_interpolated_between_digitized_anchors: bool,
    manufacturer_typical_not_production_bound: bool,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct BjtThermalCharacteristicValidation {
    candidate_key: &'static str,
    curve_kind: BjtThermalCurveKind,
    temperatures_c: [f64; 3],
    anchors_per_temperature: usize,
    total_anchor_count: usize,
    every_anchor_positive_and_finite: bool,
    hot_above_room_above_cold_at_every_reference_current: bool,
    maximum_anchor_round_trip_relative_error_percent: f64,
    interpolation_inside_temperature_and_current_domain_ready: bool,
    extrapolation_permitted: bool,
    typical_curve_covers_unit_to_unit_variation: bool,
    interpretation: &'static str,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct BjtComparativeThermalShiftInterval {
    target_candidate_key: &'static str,
    temperature_c: f64,
    normalized_log_current_coordinate: f64,
    target_collector_current_a: f64,
    target_vbe_at_25c_v: f64,
    lower_vbe_v: f64,
    upper_vbe_v: f64,
    lower_shift_from_25c_mv: f64,
    upper_shift_from_25c_mv: f64,
    donor_candidate_keys: [&'static str; 2],
    donor_current_kind: TransferCurrentKind,
    target_current_kind: TransferCurrentKind,
    includes_digitization_reading_uncertainty: bool,
    direct_target_temperature_evidence: bool,
    comparable_family_hypothesis_not_device_measurement: bool,
    covers_target_unit_to_unit_variation: bool,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct BjtComparativeThermalEnvelopeValidation {
    target_candidate_key: &'static str,
    temperature_domain_c: [f64; 2],
    normalized_current_domain: [f64; 2],
    temperature_samples: usize,
    normalized_current_samples: usize,
    evaluated_points: usize,
    every_interval_is_ordered: bool,
    every_cold_interval_is_above_25c: bool,
    every_hot_interval_is_below_25c: bool,
    room_temperature_collapses_to_direct_target_curve: bool,
    rejects_temperature_extrapolation: bool,
    rejects_current_extrapolation: bool,
    direct_mitsubishi_temperature_curve_available: bool,
    usable_as_comparative_sweep_interval: bool,
    usable_as_confirmed_device_model: bool,
    interpretation: &'static str,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct BjtCollectorPowerPoint {
    candidate_key: &'static str,
    ambient_temperature_c: f64,
    collector_power_limit_mw: f64,
    inside_published_graph_domain: bool,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct BjtThermalEvidenceReadiness {
    candidate_count: usize,
    candidates_with_direct_power_derating_graph: usize,
    candidates_with_three_temperature_transfer_curves: usize,
    candidates_with_three_temperature_saturation_curves: usize,
    candidates_with_three_temperature_hfe_curves: usize,
    candidates_with_digitized_three_temperature_transfer_curves: usize,
    candidates_with_digitized_three_temperature_hfe_curves: usize,
    candidates_with_digitized_three_temperature_saturation_curves: usize,
    candidates_with_comparative_temperature_transfer_intervals: usize,
    power_derating_ready_for_all_candidates: bool,
    direct_toshiba_temperature_interpolation_ready: bool,
    direct_toshiba_thermal_gain_interpolation_ready: bool,
    direct_toshiba_thermal_saturation_interpolation_ready: bool,
    comparative_mitsubishi_temperature_sweep_ready: bool,
    direct_temperature_transfer_ready_for_all_candidates: bool,
    comparative_temperature_borrowing_required_for_mitsubishi: bool,
    continuous_temperature_dc_models_ready: bool,
    safe_scope: &'static str,
    interpretation: &'static str,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct BipolarModelParameterBoundary {
    published_candidate_records: usize,
    all_shortlist_candidates_have_a_record: bool,
    usable_for_absolute_limit_checks: bool,
    usable_for_rank_and_capacitance_corners: bool,
    complete_large_signal_model_available: bool,
    missing_model_parameter_groups: [&'static str; 10],
    next_evidence_step: &'static str,
    interpretation: &'static str,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct UncertaintyAcceptancePolicy {
    reference: &'static str,
    mean_relative_deviation_limit_percent: f64,
    maximum_relative_deviation_limit_percent: f64,
    minimum_hypotheses: usize,
    required_metrics: [&'static str; 5],
    interpretation: &'static str,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct UncertaintyComparison {
    hypothesis_count: usize,
    ensemble_mean: f64,
    mean_absolute_relative_deviation_percent: f64,
    maximum_absolute_relative_deviation_percent: f64,
    all_values_finite: bool,
    enough_hypotheses: bool,
    accepted: bool,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct UncertaintyGateFixture {
    name: &'static str,
    expected_acceptance: bool,
    comparison: UncertaintyComparison,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct ActiveDeviceUncertaintyStatus {
    expected_hypotheses: usize,
    attempted_hypotheses: usize,
    numerically_converged_hypotheses: usize,
    evaluated_hypotheses: usize,
    required_metric_count: usize,
    evaluated_metric_count: usize,
    accepted_as_behaviorally_equivalent: bool,
    exact_device_identity_solved: bool,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct ActiveNetworkSolverReadiness {
    explicit_connection_graph_ready: bool,
    published_bjt_dc_curve_sets: usize,
    digitized_bjt_typical_transfer_fits_ready: bool,
    bjt_forward_active_evidence_corners_ready: bool,
    bjt_cutoff_endpoint_limits_ready: bool,
    bjt_saturation_endpoint_limits_ready: bool,
    bjt_local_output_admittance_records: usize,
    bjt_published_power_derating_ready: bool,
    bjt_candidates_with_direct_temperature_transfer_curves: usize,
    bjt_candidates_with_comparative_temperature_transfer_intervals: usize,
    bjt_direct_toshiba_thermal_gain_curves_ready: bool,
    bjt_direct_toshiba_thermal_saturation_curves_ready: bool,
    bjt_continuous_dc_hypothesis_count: usize,
    bjt_preliminary_continuous_region_sweep_ready: bool,
    bjt_candidates_with_output_characteristic_base_drive_families: usize,
    bjt_output_characteristic_saturation_knee_domain_covered: bool,
    bjt_output_characteristic_surfaces_digitized: bool,
    bjt_output_characteristic_clamped_points_evaluated: usize,
    bjt_output_characteristic_clamped_points_directly_resolvable: usize,
    bjt_output_characteristic_low_vce_resolution_sufficient: bool,
    bjt_low_vce_base_drive_hypotheses_defined: usize,
    bjt_low_vce_base_drive_reference_survivors: usize,
    bjt_low_vce_base_drive_matrix_points_evaluated: usize,
    bjt_low_vce_base_drive_matrix_points_converged: usize,
    bjt_low_vce_base_drive_hypotheses_eliminating_all_clamps: usize,
    bjt_low_vce_base_drive_family_viable: bool,
    bjt_reciprocal_transport_hypotheses_defined: usize,
    bjt_reciprocal_transport_matrix_points_evaluated: usize,
    bjt_reciprocal_transport_matrix_points_converged: usize,
    bjt_reciprocal_transport_exact_reference_targets_reached: usize,
    bjt_reciprocal_transport_exact_reference_targets_inside_all_gates: usize,
    bjt_reciprocal_transport_reference_candidate_viable: bool,
    bjt_reciprocal_transport_assignment_endpoints_attempted: usize,
    bjt_reciprocal_transport_assignment_endpoints_converged: usize,
    bjt_reciprocal_transport_assignment_endpoints_inside_all_gates: usize,
    bjt_reciprocal_transport_assignment_endpoints_without_clamping: usize,
    bjt_reciprocal_transport_assignment_dc_metrics_accepted: bool,
    bjt_reciprocal_transport_dispersion_attributed: bool,
    bjt_reciprocal_transport_dominant_current_designator: &'static str,
    bjt_reciprocal_transport_dominant_power_designator: &'static str,
    bjt_reciprocal_transport_tr19_candidate_envelopes: usize,
    bjt_reciprocal_transport_tr19_candidates_inside_output_plot_voltage_domain: usize,
    bjt_reciprocal_transport_tr19_candidates_inside_25c_power_limit: usize,
    bjt_reciprocal_transport_tr19_existing_documents_sufficient: bool,
    bjt_reciprocal_transport_tr19_higher_voltage_output_evidence_required: bool,
    bjt_tr19_high_vce_conductance_hypotheses_screened: usize,
    bjt_tr19_high_vce_conductance_can_pass_power_gate: bool,
    bjt_tr19_high_vce_conductance_can_pass_current_gate: bool,
    bjt_tr19_high_vce_conductance_can_pass_both_gates: bool,
    bjt_reciprocal_transport_tr22_candidates_resolvable: usize,
    bjt_reciprocal_transport_tr22_candidates_inside_base_drive_domain: usize,
    bjt_reciprocal_transport_tr23_reverse_transport_candidates: usize,
    bjt_reciprocal_transport_tr23_reverse_transport_evidence_required: bool,
    bjt_reciprocal_transport_tr22_tr23_existing_documents_sufficient: bool,
    bjt_tr22_tr23_reverse_beta_pairs_screened: usize,
    bjt_tr22_tr23_reverse_beta_pairs_passing_frozen_gates: usize,
    bjt_tr22_tr23_reverse_beta_direct_evidence_available: bool,
    bjt_tr22_tr23_reverse_beta_ready_for_solver_substitution: bool,
    bjt_correlated_reverse_transport_frozen_assignments_exceeding_kcl: usize,
    bjt_correlated_reverse_transport_maximum_kcl_multiple: f64,
    bjt_correlated_reverse_transport_full_resolve_required: bool,
    bjt_correlated_reverse_transport_resolved_assignments: usize,
    bjt_correlated_reverse_transport_resolved_dc_metrics_accepted: bool,
    bjt_correlated_reverse_transport_ready_for_evidence_testing: bool,
    bjt_correlated_reverse_transport_dispersion_attributed: bool,
    bjt_correlated_reverse_transport_dominant_current_designator: &'static str,
    bjt_correlated_reverse_transport_dominant_power_designator: &'static str,
    bjt_correlated_reverse_transport_tr19_high_voltage_evidence_required: bool,
    bjt_correlated_reverse_transport_tr22_surface_evidence_required: bool,
    bjt_correlated_reverse_transport_tr23_reverse_evidence_required: bool,
    bjt_forced_beta_ten_base_drive_homotopy_reached: bool,
    bjt_saturation_transfer_clamps_eliminated: bool,
    coupled_dc_residual_assembly_ready: bool,
    preliminary_dc_fixed_point_count: usize,
    preliminary_dc_fixed_point_solver_ready: bool,
    bjt_cutoff_saturation_and_early_regions_ready: bool,
    bounded_bjt_dc_fits_ready: bool,
    digitized_diode_typical_forward_curve_ready: bool,
    bounded_diode_iv_model_ready: bool,
    mn3101_ox_logic_and_endpoint_bounds_available: bool,
    mn3101_ox_continuous_port_model_available: bool,
    mn3101_frozen_node_matrix_points_evaluated: usize,
    mn3101_macros_compatible_with_all_frozen_assignments: usize,
    mn3101_homotopy_exact_targets_reached: usize,
    mn3101_homotopy_all_targets_reached: bool,
    mn3101_full_nonlinear_macro_sweep_ready: bool,
    full_dc_problem_well_posed: bool,
    evaluated_assignment_hypotheses: usize,
    required_missing_models: [&'static str; 1],
    safe_intermediate_scope: &'static str,
    interpretation: &'static str,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct ActiveClockNetworkBoundary {
    channel: &'static str,
    triangle_input: &'static str,
    positive_supply: &'static str,
    negative_supply: &'static str,
    timing_and_driver_nodes: &'static str,
    clock_output: &'static str,
    bbd_destination: &'static str,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct FactoryAdjustmentEvidence {
    name: &'static str,
    test_instrument: &'static str,
    test_points: &'static str,
    setup: &'static str,
    injected_signal: &'static str,
    adjusted_components: &'static str,
    acceptance: &'static str,
    constrains_clock_rate_or_depth: bool,
}

#[derive(Clone, Debug, Serialize)]
struct Report {
    schema: &'static str,
    evidence_boundary: &'static str,
    component_sources: [&'static str; 6],
    firmware_sources: [&'static str; 2],
    source_render_dpi: u16,
    source_conflicts: [SourceConflict; 1],
    components: [ComponentValue; 10],
    semiconductors: [SemiconductorAudit; 3],
    fet_grades: [FetGrade; 2],
    device_bounds: [DeviceBounds; 1],
    control_paths: [ControlPath; 2],
    hardware_control_states: [HardwareControlState; 3],
    resistance_tolerance: f64,
    capacitance_tolerance: f64,
    rc_scales: [RcScale; 2],
    ideal_oscillator_estimates: [IdealOscillatorEstimate; 2],
    firmware_chorus_states: [FirmwareChorusState; 4],
    mn3101_pins: [Mn3101Pin; 8],
    mn3101_oscillator_examples: [Mn3101OscillatorExample; 3],
    mn3101_ox_electrical_bounds: Mn3101OxElectricalBounds,
    mn3101_ox_macro_hypotheses: Vec<Mn3101OxMacroHypothesis>,
    mn3101_ox_macro_reference_points: [Mn3101OxMacroPoint; 3],
    mn3101_ox_macro_readiness: Mn3101OxMacroReadiness,
    clock_channels: [ClockChannelNetwork; 2],
    current_clock_envelopes: [ClockDelayEnvelope; 2],
    mn3101_endpoint_fits: [Mn3101EndpointFit; 2],
    c150_equivalent_r2_envelopes: [EquivalentR2Envelope; 2],
    c150_equivalent_conductance_envelopes: [EquivalentConductanceEnvelope; 2],
    c150_interpolation_method: &'static str,
    c150_interpolation_has_assigned_device_tolerance: bool,
    mirrored_clock_components: [MirroredClockComponent; 17],
    mirrored_active_network_elements: [MirroredNetlistElement; 18],
    mirrored_bipolar_connections: [MirroredBipolarConnection; 5],
    mirrored_mn3101_connections: [MirroredMn3101PinConnection; 8],
    active_network_netlist_audit: ActiveNetworkNetlistAudit,
    mirrored_active_devices: [MirroredActiveDevice; 5],
    mirrored_diode_candidate: MirroredDiodeCandidate,
    diode_candidate_electrical_evidence: [DiodeCandidateElectricalEvidence; 2],
    diode_forward_plot_calibration: DiodeForwardPlotCalibration,
    diode_forward_anchors: Vec<DiodeForwardAnchor>,
    diode_piecewise_forward_validation: DiodePiecewiseForwardValidation,
    diode_shared_endpoint_envelope: DiodeSharedEndpointEnvelope,
    diode_forward_reference_intervals: [DiodeForwardVoltageInterval; 4],
    diode_model_readiness: DiodeModelReadiness,
    board_wide_bipolar_inventory: BoardWideBipolarInventory,
    related_family_active_device_rule: RelatedFamilyActiveDeviceRule,
    comparative_candidate_shortlist: ComparativeCandidateShortlist,
    active_device_assignment_space: ActiveDeviceAssignmentSpace,
    active_device_hypotheses: Vec<ActiveDeviceHypothesis>,
    active_device_model_sources: [&'static str; 4],
    candidate_bipolar_model_evidence: [CandidateBipolarModelEvidence; 4],
    bjt_output_characteristic_plot_evidence: [BjtOutputCharacteristicPlotEvidence; 4],
    bjt_output_characteristic_evidence_validation: BjtOutputCharacteristicEvidenceValidation,
    bjt_output_characteristic_clamped_point_resolutions:
        Vec<BjtOutputCharacteristicClampedPointResolution>,
    bjt_output_characteristic_clamped_resolution_validation:
        BjtOutputCharacteristicClampedResolutionValidation,
    bjt_low_vce_base_drive_hypotheses: Vec<BjtLowVceBaseDriveHypothesis>,
    bjt_low_vce_base_drive_reference_screen_points: Vec<BjtLowVceBaseDriveSweepPoint>,
    bjt_low_vce_base_drive_sweep_points: Vec<BjtLowVceBaseDriveSweepPoint>,
    bjt_low_vce_base_drive_sweep_validation: BjtLowVceBaseDriveSweepValidation,
    bjt_reciprocal_transport_hypotheses: Vec<BjtReciprocalTransportHypothesis>,
    bjt_reciprocal_transport_sweep_points: Vec<BjtReciprocalTransportSweepPoint>,
    bjt_reciprocal_transport_sweep_validation: BjtReciprocalTransportSweepValidation,
    bjt_reciprocal_transport_homotopy_stages: Vec<BjtReciprocalTransportHomotopyStage>,
    bjt_reciprocal_transport_homotopy_summaries: Vec<BjtReciprocalTransportHomotopySummary>,
    bjt_reciprocal_transport_homotopy_validation: BjtReciprocalTransportHomotopyValidation,
    bjt_reciprocal_transport_assignment_points: Vec<BjtReciprocalTransportAssignmentPoint>,
    bjt_reciprocal_transport_assignment_validation: BjtReciprocalTransportAssignmentValidation,
    bjt_reciprocal_transport_dispersion_attribution: BjtReciprocalTransportDispersionAttribution,
    bjt_reciprocal_transport_device_evidence_envelopes:
        Vec<BjtReciprocalTransportDeviceEvidenceEnvelope>,
    bjt_reciprocal_transport_tr19_evidence_audit: BjtReciprocalTransportTr19EvidenceAudit,
    bjt_tr19_high_vce_conductance_requirement: BjtTr19HighVceConductanceRequirement,
    bjt_reciprocal_transport_tr22_tr23_evidence_audit: BjtReciprocalTransportTr22Tr23EvidenceAudit,
    bjt_tr22_tr23_reverse_beta_requirement: BjtTr22Tr23ReverseBetaRequirement,
    bjt_correlated_reverse_transport_frozen_residual_points:
        Vec<BjtCorrelatedReverseTransportFrozenResidualPoint>,
    bjt_correlated_reverse_transport_frozen_residual_audit:
        BjtCorrelatedReverseTransportFrozenResidualAudit,
    bjt_correlated_reverse_transport_resolve_points: Vec<BjtCorrelatedReverseTransportResolvePoint>,
    bjt_correlated_reverse_transport_resolve_validation:
        BjtCorrelatedReverseTransportResolveValidation,
    bjt_correlated_reverse_transport_dispersion_attribution:
        BjtReciprocalTransportDispersionAttribution,
    bjt_correlated_reverse_transport_device_evidence_envelopes:
        Vec<BjtReciprocalTransportDeviceEvidenceEnvelope>,
    bjt_correlated_reverse_transport_tr19_evidence_audit: BjtReciprocalTransportTr19EvidenceAudit,
    bjt_correlated_reverse_transport_tr22_tr23_evidence_audit:
        BjtReciprocalTransportTr22Tr23EvidenceAudit,
    bjt_transfer_plot_calibrations: [BjtTransferPlotCalibration; 4],
    bjt_transfer_pixel_anchors: Vec<BjtTransferPixelAnchor>,
    bjt_transfer_electrical_anchors: Vec<BjtTransferElectricalAnchor>,
    bjt_typical_transfer_fits: [BjtTypicalTransferFit; 4],
    bjt_piecewise_transfer_validations: [BjtPiecewiseTransferValidation; 4],
    bjt_forward_active_envelopes: [BjtForwardActiveEnvelope; 4],
    bjt_evidence_corners: Vec<BjtEvidenceCorner>,
    bjt_forward_active_reference_points: [BjtForwardActivePoint; 4],
    bjt_dc_region_evidence_envelopes: [BjtDcRegionEvidenceEnvelope; 4],
    bjt_continuous_dc_hypotheses: Vec<BjtContinuousDcHypothesis>,
    bjt_continuous_dc_reference_points: Vec<BjtContinuousDcPoint>,
    bjt_continuous_dc_validations: [BjtContinuousDcValidation; 4],
    active_network_dc_coordinates: [ActiveNetworkDcCoordinate; 3],
    active_network_dc_operating_points: Vec<ActiveNetworkDcOperatingPoint>,
    active_network_dc_solver_validation: ActiveNetworkDcSolverValidation,
    active_device_dc_sweep_points: Vec<ActiveDeviceDcSweepPoint>,
    active_device_dc_sweep_validation: ActiveDeviceDcSweepValidation,
    mn3101_frozen_node_compatibility_points: Vec<Mn3101FrozenNodeCompatibilityPoint>,
    mn3101_frozen_node_compatibility_summaries: Vec<Mn3101FrozenNodeCompatibilitySummary>,
    mn3101_frozen_node_matrix_validation: Mn3101FrozenNodeMatrixValidation,
    mn3101_homotopy_stages: Vec<Mn3101HomotopyStage>,
    mn3101_homotopy_summaries: Vec<Mn3101HomotopySummary>,
    mn3101_homotopy_validation: Mn3101HomotopyValidation,
    bjt_saturation_base_drive_homotopy_stages: Vec<BjtSaturationBaseDriveHomotopyStage>,
    bjt_saturation_base_drive_homotopy_validation: BjtSaturationBaseDriveHomotopyValidation,
    bjt_published_thermal_evidence: [BjtPublishedThermalEvidence; 4],
    bjt_thermal_transfer_plot_calibrations: [BjtThermalTransferPlotCalibration; 2],
    bjt_thermal_transfer_pixel_anchors: Vec<BjtThermalTransferPixelAnchor>,
    bjt_thermal_transfer_electrical_anchors: Vec<BjtThermalTransferElectricalAnchor>,
    bjt_thermal_transfer_validations: [BjtThermalTransferValidation; 2],
    bjt_thermal_transfer_reference_points: Vec<BjtThermalTransferPoint>,
    bjt_thermal_characteristic_plot_calibrations: [BjtThermalCharacteristicPlotCalibration; 4],
    bjt_thermal_characteristic_pixel_anchors: Vec<BjtThermalCharacteristicPixelAnchor>,
    bjt_thermal_characteristic_electrical_anchors: Vec<BjtThermalCharacteristicElectricalAnchor>,
    bjt_thermal_characteristic_validations: [BjtThermalCharacteristicValidation; 4],
    bjt_thermal_characteristic_reference_points: Vec<BjtThermalCharacteristicPoint>,
    bjt_comparative_thermal_reference_intervals: Vec<BjtComparativeThermalShiftInterval>,
    bjt_comparative_thermal_envelope_validations: [BjtComparativeThermalEnvelopeValidation; 2],
    bjt_collector_power_reference_points: Vec<BjtCollectorPowerPoint>,
    bjt_thermal_evidence_readiness: BjtThermalEvidenceReadiness,
    bipolar_model_parameter_boundary: BipolarModelParameterBoundary,
    uncertainty_acceptance_policy: UncertaintyAcceptancePolicy,
    uncertainty_gate_fixtures: [UncertaintyGateFixture; 2],
    active_device_uncertainty_status: ActiveDeviceUncertaintyStatus,
    active_network_solver_readiness: ActiveNetworkSolverReadiness,
    module_board_device_legend_applies_to_clock_network: bool,
    active_clock_network_boundaries: [ActiveClockNetworkBoundary; 2],
    factory_adjustment: FactoryAdjustmentEvidence,
    factory_clock_target_present: bool,
    mn3101_oscillator_to_cp_divisor: f64,
    mn3009_stages: f64,
    clock_networks_are_nominally_mirrored: bool,
    clock_frequency_chain_solved: bool,
    mn3101_control_transfer_solved: bool,
    active_network_schematic_topology_captured: bool,
    active_network_dc_solver_topology_ready: bool,
    exact_active_device_assignments_solved: bool,
    active_network_operating_point_solved: bool,
    active_network_voltage_to_r2_solved: bool,
    provisional_coordinates: [ProvisionalCoordinate; 2],
    coordinate_comparisons: [CoordinateComparison; 2],
    unsupported_combined_coordinate_removed: bool,
    reduced_rate_model_solved: bool,
    steady_state_control_solved: bool,
    absolute_rate_solved: bool,
    production_promotion_ready: bool,
    blockers: [&'static str; 4],
}

const fn mn3101_pins() -> [Mn3101Pin; 8] {
    [
        Mn3101Pin {
            pin: 1,
            name: "GND",
            function: "ground",
        },
        Mn3101Pin {
            pin: 2,
            name: "CP1",
            function: "clock phase 1 output",
        },
        Mn3101Pin {
            pin: 3,
            name: "VDD",
            function: "negative supply",
        },
        Mn3101Pin {
            pin: 4,
            name: "CP2",
            function: "clock phase 2 output",
        },
        Mn3101Pin {
            pin: 5,
            name: "OX3",
            function: "external oscillator node 3",
        },
        Mn3101Pin {
            pin: 6,
            name: "OX2",
            function: "external oscillator node 2",
        },
        Mn3101Pin {
            pin: 7,
            name: "OX1",
            function: "external oscillator node 1",
        },
        Mn3101Pin {
            pin: 8,
            name: "VGG",
            function: "BBD gate-bias output",
        },
    ]
}

const fn mn3101_oscillator_examples() -> [Mn3101OscillatorExample; 3] {
    [
        Mn3101OscillatorExample {
            r1_ohms: 0.0,
            r2_min_ohms: 5_000.0,
            r2_max_ohms: 1_000_000.0,
            c1_picofarads: 33.0,
            oscillator_min_hz: 15_000.0,
            oscillator_max_hz: 1_500_000.0,
            cp_min_hz: 7_500.0,
            cp_max_hz: 750_000.0,
        },
        Mn3101OscillatorExample {
            r1_ohms: 22_000.0,
            r2_min_ohms: 5_000.0,
            r2_max_ohms: 1_000_000.0,
            c1_picofarads: 100.0,
            oscillator_min_hz: 5_200.0,
            oscillator_max_hz: 440_000.0,
            cp_min_hz: 2_600.0,
            cp_max_hz: 220_000.0,
        },
        Mn3101OscillatorExample {
            r1_ohms: 22_000.0,
            r2_min_ohms: 5_000.0,
            r2_max_ohms: 1_000_000.0,
            c1_picofarads: 200.0,
            oscillator_min_hz: 1_400.0,
            oscillator_max_hz: 280_000.0,
            cp_min_hz: 700.0,
            cp_max_hz: 140_000.0,
        },
    ]
}

const fn mn3101_ox_electrical_bounds() -> Mn3101OxElectricalBounds {
    Mn3101OxElectricalBounds {
        test_supply_v: -15.0,
        power_dissipation_absolute_max_mw: 200.0,
        ox1_guaranteed_high_range_v: [0.0, -1.0],
        ox1_guaranteed_low_range_v: [-14.0, -15.0],
        ox1_input_leakage_max_ua: 30.0,
        output_high_test_voltage_v: -1.0,
        output_low_test_voltage_v: -14.0,
        ox2_output_high_current_min_ma: 0.6,
        ox2_output_low_current_min_ma: 0.5,
        ox2_output_leakage_max_ua: 30.0,
        ox3_output_high_current_min_ma: 1.5,
        ox3_output_low_current_min_ma: 2.0,
        ox3_output_leakage_max_ua: 30.0,
        ox2_high_endpoint_resistance_max_ohms: 1.0 / 0.000_6,
        ox2_low_endpoint_resistance_max_ohms: 1.0 / 0.000_5,
        ox3_high_endpoint_resistance_max_ohms: 1.0 / 0.001_5,
        ox3_low_endpoint_resistance_max_ohms: 1.0 / 0.002,
        transition_region_is_published: false,
        continuous_output_iv_is_published: false,
        evidence: "MN3101 electrical-characteristics table at VDD=-15 V",
        interpretation: "the four resistance ceilings are conservative one-volt endpoint quotients, not continuous output-resistance models",
    }
}

fn mn3101_ox_macro_hypotheses() -> Vec<Mn3101OxMacroHypothesis> {
    let mut hypotheses = Vec::with_capacity(9);
    for transition_shape in [
        Mn3101TransitionShape::Early,
        Mn3101TransitionShape::Linear,
        Mn3101TransitionShape::Late,
    ] {
        for output_strength_corner in [
            Mn3101OutputStrengthCorner::Strong,
            Mn3101OutputStrengthCorner::Geometric,
            Mn3101OutputStrengthCorner::Weak,
        ] {
            hypotheses.push(Mn3101OxMacroHypothesis {
                index: hypotheses.len(),
                transition_shape,
                output_strength_corner,
                transition_shape_is_hypothesis: true,
                minimum_resistance_from_total_power_is_conservative_hypothesis: true,
                published_endpoint_currents_remain_hard_minima: true,
            });
        }
    }
    hypotheses
}

fn mn3101_output_resistance(
    strength: Mn3101OutputStrengthCorner,
    published_maximum_ohms: f64,
) -> f64 {
    const ONE_VOLT_ENDPOINT_DROP_V: f64 = 1.0;
    let bounds = mn3101_ox_electrical_bounds();
    let power_limited_minimum_ohms =
        ONE_VOLT_ENDPOINT_DROP_V.powi(2) / (bounds.power_dissipation_absolute_max_mw / 1_000.0);
    match strength {
        Mn3101OutputStrengthCorner::Strong => power_limited_minimum_ohms,
        Mn3101OutputStrengthCorner::Geometric => {
            (power_limited_minimum_ohms * published_maximum_ohms).sqrt()
        }
        Mn3101OutputStrengthCorner::Weak => published_maximum_ohms,
    }
}

fn evaluate_mn3101_ox_macro(
    hypothesis: Mn3101OxMacroHypothesis,
    ox1_input_v: f64,
    ox2_node_v: f64,
    ox3_node_v: f64,
) -> Option<Mn3101OxMacroPoint> {
    let bounds = mn3101_ox_electrical_bounds();
    let negative_rail_v = bounds.test_supply_v;
    if !(negative_rail_v..=0.0).contains(&ox1_input_v)
        || !(negative_rail_v..=0.0).contains(&ox2_node_v)
        || !(negative_rail_v..=0.0).contains(&ox3_node_v)
    {
        return None;
    }
    let (ox1_region, linear_high_fraction) = if ox1_input_v >= -1.0 {
        (Mn3101Ox1Region::GuaranteedHigh, 1.0)
    } else if ox1_input_v <= -14.0 {
        (Mn3101Ox1Region::GuaranteedLow, 0.0)
    } else {
        (
            Mn3101Ox1Region::UnpublishedTransition,
            (ox1_input_v + 14.0) / 13.0,
        )
    };
    let normalized_high_fraction = match hypothesis.transition_shape {
        Mn3101TransitionShape::Early => linear_high_fraction.sqrt(),
        Mn3101TransitionShape::Linear => linear_high_fraction,
        Mn3101TransitionShape::Late => linear_high_fraction.powi(2),
    };
    let ox2_high_fraction = 1.0 - normalized_high_fraction;
    let ox3_high_fraction = normalized_high_fraction;
    let ox2_target_v = negative_rail_v * normalized_high_fraction;
    let ox3_target_v = negative_rail_v * (1.0 - normalized_high_fraction);

    let ox2_high_resistance = mn3101_output_resistance(
        hypothesis.output_strength_corner,
        bounds.ox2_high_endpoint_resistance_max_ohms,
    );
    let ox2_low_resistance = mn3101_output_resistance(
        hypothesis.output_strength_corner,
        bounds.ox2_low_endpoint_resistance_max_ohms,
    );
    let ox3_high_resistance = mn3101_output_resistance(
        hypothesis.output_strength_corner,
        bounds.ox3_high_endpoint_resistance_max_ohms,
    );
    let ox3_low_resistance = mn3101_output_resistance(
        hypothesis.output_strength_corner,
        bounds.ox3_low_endpoint_resistance_max_ohms,
    );
    let ox2_resistance_ohms =
        ox2_low_resistance * (1.0 - ox2_high_fraction) + ox2_high_resistance * ox2_high_fraction;
    let ox3_resistance_ohms =
        ox3_low_resistance * (1.0 - ox3_high_fraction) + ox3_high_resistance * ox3_high_fraction;

    let ox2_drop_v = ox2_target_v - ox2_node_v;
    let ox3_drop_v = ox3_target_v - ox3_node_v;
    let mut ox2_output_current_a = ox2_drop_v / ox2_resistance_ohms;
    let mut ox3_output_current_a = ox3_drop_v / ox3_resistance_ohms;
    let raw_combined_power_w =
        (ox2_drop_v * ox2_output_current_a).abs() + (ox3_drop_v * ox3_output_current_a).abs();
    let power_limit_w = bounds.power_dissipation_absolute_max_mw / 1_000.0;
    if raw_combined_power_w > power_limit_w {
        let scale = power_limit_w / raw_combined_power_w;
        ox2_output_current_a *= scale;
        ox3_output_current_a *= scale;
    }
    let ox2_output_power_w = (ox2_drop_v * ox2_output_current_a).abs();
    let ox3_output_power_w = (ox3_drop_v * ox3_output_current_a).abs();
    let combined_output_power_w = ox2_output_power_w + ox3_output_power_w;
    Some(Mn3101OxMacroPoint {
        hypothesis_index: hypothesis.index,
        ox1_input_v,
        ox1_region,
        normalized_high_fraction,
        ox2_target_v,
        ox3_target_v,
        ox2_resistance_ohms,
        ox3_resistance_ohms,
        ox2_output_current_a,
        ox3_output_current_a,
        ox2_output_power_w,
        ox3_output_power_w,
        combined_output_power_w,
        combined_power_inside_absolute_limit: combined_output_power_w <= power_limit_w + 1.0e-12,
        transition_is_published: ox1_region != Mn3101Ox1Region::UnpublishedTransition,
    })
}

fn mn3101_ox_macro_reference_points() -> [Mn3101OxMacroPoint; 3] {
    let hypothesis = mn3101_ox_macro_hypotheses()
        .into_iter()
        .find(|candidate| {
            candidate.transition_shape == Mn3101TransitionShape::Linear
                && candidate.output_strength_corner == Mn3101OutputStrengthCorner::Geometric
        })
        .expect("the linear/geometric macro hypothesis is enumerated");
    [
        evaluate_mn3101_ox_macro(hypothesis, -14.0, -1.0, -14.0)
            .expect("the guaranteed-low reference is inside the supply"),
        evaluate_mn3101_ox_macro(hypothesis, -7.5, -7.5, -7.5)
            .expect("the transition reference is inside the supply"),
        evaluate_mn3101_ox_macro(hypothesis, -1.0, -14.0, -1.0)
            .expect("the guaranteed-high reference is inside the supply"),
    ]
}

fn mn3101_ox_macro_readiness() -> Mn3101OxMacroReadiness {
    let bounds = mn3101_ox_electrical_bounds();
    let hypotheses = mn3101_ox_macro_hypotheses();
    let endpoint_current_minima_preserved = hypotheses.iter().all(|hypothesis| {
        let low = evaluate_mn3101_ox_macro(*hypothesis, -14.0, -1.0, -14.0)
            .expect("low endpoint is inside the supply");
        let high = evaluate_mn3101_ox_macro(*hypothesis, -1.0, -14.0, -1.0)
            .expect("high endpoint is inside the supply");
        low.ox2_output_current_a.abs() >= bounds.ox2_output_high_current_min_ma / 1_000.0
            && low.ox3_output_current_a.abs() >= bounds.ox3_output_low_current_min_ma / 1_000.0
            && high.ox2_output_current_a.abs() >= bounds.ox2_output_low_current_min_ma / 1_000.0
            && high.ox3_output_current_a.abs() >= bounds.ox3_output_high_current_min_ma / 1_000.0
    });
    let combined_power_limit_enforced = hypotheses.iter().all(|hypothesis| {
        [-15.0, -14.0, -7.5, -1.0, 0.0].into_iter().all(|input| {
            evaluate_mn3101_ox_macro(*hypothesis, input, -7.5, -7.5)
                .is_some_and(|point| point.combined_power_inside_absolute_limit)
        })
    });
    Mn3101OxMacroReadiness {
        hypothesis_count: hypotheses.len(),
        guaranteed_logic_regions_preserved: true,
        endpoint_current_minima_preserved,
        combined_power_limit_enforced,
        complete_supply_domain_covered: true,
        actual_transition_transfer_solved: false,
        suitable_for_interval_sweep: hypotheses.len() == 9
            && endpoint_current_minima_preserved
            && combined_power_limit_enforced,
        suitable_as_identified_nominal_device_model: false,
        interpretation: "nine continuous endpoint-constrained hypotheses span early/linear/late OX1 transitions and strong/geometric/weak output stages; the family is sweepable but does not identify the unpublished internal transfer",
    }
}

const fn clock_channels() -> [ClockChannelNetwork; 2] {
    [
        ClockChannelNetwork {
            channel: "a",
            triangle_source: "TP3",
            modulation_input: "R128 1 kOhm",
            clock_driver: "IC9 MN3101",
            bbd: "IC7 MN3009",
            oscillator_capacitor: "C53 150 pF",
            oscillator_resistors: ["R134 22 kOhm", "R135 1.8 kOhm"],
            active_network: "Tr19-Tr23, D9, R123-R135",
        },
        ClockChannelNetwork {
            channel: "b",
            triangle_source: "TP4",
            modulation_input: "R148 1 kOhm",
            clock_driver: "IC11 MN3101",
            bbd: "IC10 MN3009",
            oscillator_capacitor: "C57 150 pF",
            oscillator_resistors: ["R140 22 kOhm", "R139 1.8 kOhm"],
            active_network: "Tr24-Tr28, D10, R136-R148",
        },
    ]
}

fn bbd_clock_hz_for_delay_ms(delay_ms: f64) -> f64 {
    MN3009_STAGES * 1_000.0 / (2.0 * delay_ms)
}

#[cfg(test)]
fn bbd_delay_ms_for_clock_hz(clock_hz: f64) -> f64 {
    MN3009_STAGES * 1_000.0 / (2.0 * clock_hz)
}

fn clock_delay_envelope(
    mode: &'static str,
    center_delay_ms: f64,
    depth_ms: f64,
) -> ClockDelayEnvelope {
    let delay_min_ms = center_delay_ms - depth_ms;
    let delay_max_ms = center_delay_ms + depth_ms;
    let cp_min_hz = bbd_clock_hz_for_delay_ms(delay_max_ms);
    let cp_max_hz = bbd_clock_hz_for_delay_ms(delay_min_ms);
    ClockDelayEnvelope {
        mode,
        delay_min_ms,
        delay_max_ms,
        cp_min_hz,
        cp_max_hz,
        mn3101_oscillator_min_hz: cp_min_hz * MN3101_OSCILLATOR_TO_CP_DIVISOR,
        mn3101_oscillator_max_hz: cp_max_hz * MN3101_OSCILLATOR_TO_CP_DIVISOR,
        inside_mn3009_clock_range: cp_min_hz >= MN3009_MIN_CLOCK_HZ
            && cp_max_hz <= MN3009_MAX_CLOCK_HZ,
    }
}

fn mn3101_endpoint_fit(
    c1_picofarads: f64,
    cp_at_r2_min_hz: f64,
    cp_at_r2_max_hz: f64,
) -> Mn3101EndpointFit {
    let r2_min_ohms: f64 = 5_000.0;
    let r2_max_ohms: f64 = 1_000_000.0;
    let log_log_exponent =
        (cp_at_r2_max_hz / cp_at_r2_min_hz).ln() / (r2_max_ohms / r2_min_ohms).ln();
    Mn3101EndpointFit {
        r1_ohms: 22_000.0,
        c1_picofarads,
        r2_min_ohms,
        r2_max_ohms,
        cp_at_r2_min_hz,
        cp_at_r2_max_hz,
        log_log_exponent,
        evidence: "power-law fit through the two exact manufacturer-table endpoints; not a digitized device curve",
    }
}

fn mn3101_endpoint_fits() -> [Mn3101EndpointFit; 2] {
    [
        mn3101_endpoint_fit(100.0, 220_000.0, 2_600.0),
        mn3101_endpoint_fit(200.0, 140_000.0, 700.0),
    ]
}

fn cp_hz_from_endpoint_fit(fit: Mn3101EndpointFit, r2_ohms: f64) -> f64 {
    fit.cp_at_r2_min_hz * (r2_ohms / fit.r2_min_ohms).powf(fit.log_log_exponent)
}

fn cp_hz_for_150pf_endpoint_interpolation(r2_ohms: f64) -> f64 {
    let fits = mn3101_endpoint_fits();
    let capacitor_fraction = (150.0_f64 / 100.0).ln() / (200.0_f64 / 100.0).ln();
    let log_cp_100 = cp_hz_from_endpoint_fit(fits[0], r2_ohms).ln();
    let log_cp_200 = cp_hz_from_endpoint_fit(fits[1], r2_ohms).ln();
    (log_cp_100 + capacitor_fraction * (log_cp_200 - log_cp_100)).exp()
}

fn equivalent_r2_for_150pf_cp_hz(cp_hz: f64) -> f64 {
    let r2_reference = 5_000.0;
    let cp_reference = cp_hz_for_150pf_endpoint_interpolation(r2_reference);
    let cp_at_max_r2 = cp_hz_for_150pf_endpoint_interpolation(1_000_000.0);
    let exponent = (cp_at_max_r2 / cp_reference).ln() / (1_000_000.0_f64 / r2_reference).ln();
    r2_reference * (cp_hz / cp_reference).powf(1.0 / exponent)
}

fn equivalent_r2_envelope(envelope: ClockDelayEnvelope) -> EquivalentR2Envelope {
    let r2_at_cp_max_ohms = equivalent_r2_for_150pf_cp_hz(envelope.cp_max_hz);
    let r2_at_cp_min_ohms = equivalent_r2_for_150pf_cp_hz(envelope.cp_min_hz);
    EquivalentR2Envelope {
        mode: envelope.mode,
        cp_min_hz: envelope.cp_min_hz,
        cp_max_hz: envelope.cp_max_hz,
        r2_at_cp_max_ohms,
        r2_at_cp_min_ohms,
        inside_manufacturer_sweep: r2_at_cp_max_ohms >= 5_000.0 && r2_at_cp_min_ohms <= 1_000_000.0,
        interpretation: "simple-network R2 equivalent only; the target active transistor network has not been reduced to this resistance",
    }
}

fn equivalent_conductance_envelope(
    resistance: EquivalentR2Envelope,
) -> EquivalentConductanceEnvelope {
    let minimum_microsiemens = 1_000_000.0 / resistance.r2_at_cp_min_ohms;
    let maximum_microsiemens = 1_000_000.0 / resistance.r2_at_cp_max_ohms;
    EquivalentConductanceEnvelope {
        mode: resistance.mode,
        minimum_microsiemens,
        maximum_microsiemens,
        span_ratio: maximum_microsiemens / minimum_microsiemens,
        interpretation: "required simple-network conductance envelope; it constrains but does not identify the Tr19-Tr28 device model",
    }
}

const fn mirrored_clock_components() -> [MirroredClockComponent; 17] {
    [
        MirroredClockComponent {
            channel_a_designator: "R128",
            channel_b_designator: "R148",
            nominal: 1_000.0,
            unit: "ohm",
        },
        MirroredClockComponent {
            channel_a_designator: "R123",
            channel_b_designator: "R136",
            nominal: 1_800.0,
            unit: "ohm",
        },
        MirroredClockComponent {
            channel_a_designator: "R124",
            channel_b_designator: "R137",
            nominal: 8_200.0,
            unit: "ohm",
        },
        MirroredClockComponent {
            channel_a_designator: "R125",
            channel_b_designator: "R138",
            nominal: 10_000.0,
            unit: "ohm",
        },
        MirroredClockComponent {
            channel_a_designator: "R126",
            channel_b_designator: "R142",
            nominal: 2_200.0,
            unit: "ohm",
        },
        MirroredClockComponent {
            channel_a_designator: "R127",
            channel_b_designator: "R143",
            nominal: 2_200.0,
            unit: "ohm",
        },
        MirroredClockComponent {
            channel_a_designator: "R129",
            channel_b_designator: "R147",
            nominal: 33_000.0,
            unit: "ohm",
        },
        MirroredClockComponent {
            channel_a_designator: "R130",
            channel_b_designator: "R146",
            nominal: 330_000.0,
            unit: "ohm",
        },
        MirroredClockComponent {
            channel_a_designator: "R131",
            channel_b_designator: "R145",
            nominal: 33_000.0,
            unit: "ohm",
        },
        MirroredClockComponent {
            channel_a_designator: "R132",
            channel_b_designator: "R144",
            nominal: 6_800.0,
            unit: "ohm",
        },
        MirroredClockComponent {
            channel_a_designator: "R133",
            channel_b_designator: "R141",
            nominal: 2_200.0,
            unit: "ohm",
        },
        MirroredClockComponent {
            channel_a_designator: "R134",
            channel_b_designator: "R140",
            nominal: 22_000.0,
            unit: "ohm",
        },
        MirroredClockComponent {
            channel_a_designator: "R135",
            channel_b_designator: "R139",
            nominal: 1_800.0,
            unit: "ohm",
        },
        MirroredClockComponent {
            channel_a_designator: "C53",
            channel_b_designator: "C57",
            nominal: 150.0,
            unit: "picofarad",
        },
        MirroredClockComponent {
            channel_a_designator: "C51",
            channel_b_designator: "C55",
            nominal: 10.0,
            unit: "microfarad",
        },
        MirroredClockComponent {
            channel_a_designator: "R99",
            channel_b_designator: "R108",
            nominal: 100.0,
            unit: "ohm",
        },
        MirroredClockComponent {
            channel_a_designator: "C46",
            channel_b_designator: "C49",
            nominal: 100.0,
            unit: "microfarad",
        },
    ]
}

const fn mirrored_active_network_elements() -> [MirroredNetlistElement; 18] {
    [
        MirroredNetlistElement {
            channel_a_designator: "R128",
            channel_b_designator: "R148",
            kind: NetlistElementKind::Resistor,
            terminal_1: ActiveNetworkNode::TriangleInput,
            terminal_2: ActiveNetworkNode::InputEmitterNode,
            terminal_1_meaning: "TP3 / TP4",
            terminal_2_meaning: "Tr21 / Tr26 emitter network",
            nominal: Some(1_000.0),
            unit: Some("ohm"),
        },
        MirroredNetlistElement {
            channel_a_designator: "R123",
            channel_b_designator: "R136",
            kind: NetlistElementKind::Resistor,
            terminal_1: ActiveNetworkNode::PositiveSupply,
            terminal_2: ActiveNetworkNode::PnpBiasBase,
            terminal_1_meaning: "+15 V",
            terminal_2_meaning: "Tr19 / Tr24 base",
            nominal: Some(1_800.0),
            unit: Some("ohm"),
        },
        MirroredNetlistElement {
            channel_a_designator: "R124",
            channel_b_designator: "R137",
            kind: NetlistElementKind::Resistor,
            terminal_1: ActiveNetworkNode::PositiveSupply,
            terminal_2: ActiveNetworkNode::PnpBiasEmitter,
            terminal_1_meaning: "+15 V",
            terminal_2_meaning: "Tr19 / Tr24 emitter",
            nominal: Some(8_200.0),
            unit: Some("ohm"),
        },
        MirroredNetlistElement {
            channel_a_designator: "R125",
            channel_b_designator: "R138",
            kind: NetlistElementKind::Resistor,
            terminal_1: ActiveNetworkNode::PnpBiasBase,
            terminal_2: ActiveNetworkNode::Ground,
            terminal_1_meaning: "Tr19 / Tr24 base",
            terminal_2_meaning: "ground",
            nominal: Some(10_000.0),
            unit: Some("ohm"),
        },
        MirroredNetlistElement {
            channel_a_designator: "R126",
            channel_b_designator: "R142",
            kind: NetlistElementKind::Resistor,
            terminal_1: ActiveNetworkNode::PnpSwitchCollector,
            terminal_2: ActiveNetworkNode::Control,
            terminal_1_meaning: "Tr20 / Tr25 collector",
            terminal_2_meaning: "OX3 control node",
            nominal: Some(2_200.0),
            unit: Some("ohm"),
        },
        MirroredNetlistElement {
            channel_a_designator: "R127",
            channel_b_designator: "R143",
            kind: NetlistElementKind::Resistor,
            terminal_1: ActiveNetworkNode::PositiveSupply,
            terminal_2: ActiveNetworkNode::PnpSwitchBase,
            terminal_1_meaning: "+15 V",
            terminal_2_meaning: "Tr20 / Tr25 base and Tr21 / Tr26 collector",
            nominal: Some(2_200.0),
            unit: Some("ohm"),
        },
        MirroredNetlistElement {
            channel_a_designator: "R129",
            channel_b_designator: "R147",
            kind: NetlistElementKind::Resistor,
            terminal_1: ActiveNetworkNode::DiodeBase,
            terminal_2: ActiveNetworkNode::InputEmitterNode,
            terminal_1_meaning: "Tr21 / Tr26 base",
            terminal_2_meaning: "Tr21 / Tr26 emitter network",
            nominal: Some(33_000.0),
            unit: Some("ohm"),
        },
        MirroredNetlistElement {
            channel_a_designator: "R130",
            channel_b_designator: "R146",
            kind: NetlistElementKind::Resistor,
            terminal_1: ActiveNetworkNode::InputEmitterNode,
            terminal_2: ActiveNetworkNode::LowerNpnBase,
            terminal_1_meaning: "triangle-conditioned input",
            terminal_2_meaning: "Tr23 / Tr28 base",
            nominal: Some(330_000.0),
            unit: Some("ohm"),
        },
        MirroredNetlistElement {
            channel_a_designator: "R131",
            channel_b_designator: "R145",
            kind: NetlistElementKind::Resistor,
            terminal_1: ActiveNetworkNode::LowerNpnBase,
            terminal_2: ActiveNetworkNode::LocalNegativeRail,
            terminal_1_meaning: "Tr23 / Tr28 base",
            terminal_2_meaning: "local negative rail",
            nominal: Some(33_000.0),
            unit: Some("ohm"),
        },
        MirroredNetlistElement {
            channel_a_designator: "R132",
            channel_b_designator: "R144",
            kind: NetlistElementKind::Resistor,
            terminal_1: ActiveNetworkNode::Control,
            terminal_2: ActiveNetworkNode::Timing,
            terminal_1_meaning: "OX3 control node",
            terminal_2_meaning: "150 pF timing node",
            nominal: Some(6_800.0),
            unit: Some("ohm"),
        },
        MirroredNetlistElement {
            channel_a_designator: "R133",
            channel_b_designator: "R141",
            kind: NetlistElementKind::Resistor,
            terminal_1: ActiveNetworkNode::FeedbackEmitter,
            terminal_2: ActiveNetworkNode::LocalNegativeRail,
            terminal_1_meaning: "Tr22 / Tr27 emitter",
            terminal_2_meaning: "local negative rail",
            nominal: Some(2_200.0),
            unit: Some("ohm"),
        },
        MirroredNetlistElement {
            channel_a_designator: "R134",
            channel_b_designator: "R140",
            kind: NetlistElementKind::Resistor,
            terminal_1: ActiveNetworkNode::OscillatorFeedback,
            terminal_2: ActiveNetworkNode::FeedbackBase,
            terminal_1_meaning: "MN3101 OX2 pin 6",
            terminal_2_meaning: "MN3101 OX1 pin 7 and Tr22 / Tr27 base",
            nominal: Some(22_000.0),
            unit: Some("ohm"),
        },
        MirroredNetlistElement {
            channel_a_designator: "R135",
            channel_b_designator: "R139",
            kind: NetlistElementKind::Resistor,
            terminal_1: ActiveNetworkNode::FeedbackBase,
            terminal_2: ActiveNetworkNode::LocalNegativeRail,
            terminal_1_meaning: "MN3101 OX1 pin 7 and Tr22 / Tr27 base",
            terminal_2_meaning: "local negative rail",
            nominal: Some(1_800.0),
            unit: Some("ohm"),
        },
        MirroredNetlistElement {
            channel_a_designator: "R99",
            channel_b_designator: "R108",
            kind: NetlistElementKind::Resistor,
            terminal_1: ActiveNetworkNode::LocalNegativeRail,
            terminal_2: ActiveNetworkNode::NegativeSupply,
            terminal_1_meaning: "MN3101 pin 3 local rail",
            terminal_2_meaning: "-15 V",
            nominal: Some(100.0),
            unit: Some("ohm"),
        },
        MirroredNetlistElement {
            channel_a_designator: "C53",
            channel_b_designator: "C57",
            kind: NetlistElementKind::Capacitor,
            terminal_1: ActiveNetworkNode::Timing,
            terminal_2: ActiveNetworkNode::LocalNegativeRail,
            terminal_1_meaning: "timing node",
            terminal_2_meaning: "local negative rail",
            nominal: Some(150.0),
            unit: Some("picofarad"),
        },
        MirroredNetlistElement {
            channel_a_designator: "C51",
            channel_b_designator: "C55",
            kind: NetlistElementKind::Capacitor,
            terminal_1: ActiveNetworkNode::Vgg,
            terminal_2: ActiveNetworkNode::Ground,
            terminal_1_meaning: "MN3101 VGG pin 8",
            terminal_2_meaning: "ground",
            nominal: Some(10.0),
            unit: Some("microfarad"),
        },
        MirroredNetlistElement {
            channel_a_designator: "C46",
            channel_b_designator: "C49",
            kind: NetlistElementKind::Capacitor,
            terminal_1: ActiveNetworkNode::Ground,
            terminal_2: ActiveNetworkNode::LocalNegativeRail,
            terminal_1_meaning: "ground / positive capacitor terminal",
            terminal_2_meaning: "local negative rail",
            nominal: Some(100.0),
            unit: Some("microfarad"),
        },
        MirroredNetlistElement {
            channel_a_designator: "D9",
            channel_b_designator: "D10",
            kind: NetlistElementKind::Diode,
            terminal_1: ActiveNetworkNode::DiodeBase,
            terminal_2: ActiveNetworkNode::Control,
            terminal_1_meaning: "cathode at Tr21 / Tr26 base",
            terminal_2_meaning: "anode at OX3 control node",
            nominal: None,
            unit: None,
        },
    ]
}

const fn mirrored_bipolar_connections() -> [MirroredBipolarConnection; 5] {
    [
        MirroredBipolarConnection {
            channel_a_designator: "Tr19",
            channel_b_designator: "Tr24",
            polarity: BipolarPolarity::Pnp,
            collector: ActiveNetworkNode::Control,
            base: ActiveNetworkNode::PnpBiasBase,
            emitter: ActiveNetworkNode::PnpBiasEmitter,
        },
        MirroredBipolarConnection {
            channel_a_designator: "Tr20",
            channel_b_designator: "Tr25",
            polarity: BipolarPolarity::Pnp,
            collector: ActiveNetworkNode::PnpSwitchCollector,
            base: ActiveNetworkNode::PnpSwitchBase,
            emitter: ActiveNetworkNode::PositiveSupply,
        },
        MirroredBipolarConnection {
            channel_a_designator: "Tr21",
            channel_b_designator: "Tr26",
            polarity: BipolarPolarity::Npn,
            collector: ActiveNetworkNode::PnpSwitchBase,
            base: ActiveNetworkNode::DiodeBase,
            emitter: ActiveNetworkNode::InputEmitterNode,
        },
        MirroredBipolarConnection {
            channel_a_designator: "Tr22",
            channel_b_designator: "Tr27",
            polarity: BipolarPolarity::Npn,
            collector: ActiveNetworkNode::Timing,
            base: ActiveNetworkNode::FeedbackBase,
            emitter: ActiveNetworkNode::FeedbackEmitter,
        },
        MirroredBipolarConnection {
            channel_a_designator: "Tr23",
            channel_b_designator: "Tr28",
            polarity: BipolarPolarity::Npn,
            collector: ActiveNetworkNode::Control,
            base: ActiveNetworkNode::LowerNpnBase,
            emitter: ActiveNetworkNode::LocalNegativeRail,
        },
    ]
}

const fn mirrored_mn3101_connections() -> [MirroredMn3101PinConnection; 8] {
    [
        MirroredMn3101PinConnection {
            channel_a_designator: "IC9.1",
            channel_b_designator: "IC11.1",
            pin: 1,
            function: "GND",
            node: ActiveNetworkNode::Ground,
        },
        MirroredMn3101PinConnection {
            channel_a_designator: "IC9.2",
            channel_b_designator: "IC11.2",
            pin: 2,
            function: "CP1",
            node: ActiveNetworkNode::ClockPhaseOne,
        },
        MirroredMn3101PinConnection {
            channel_a_designator: "IC9.3",
            channel_b_designator: "IC11.3",
            pin: 3,
            function: "VDD",
            node: ActiveNetworkNode::LocalNegativeRail,
        },
        MirroredMn3101PinConnection {
            channel_a_designator: "IC9.4",
            channel_b_designator: "IC11.4",
            pin: 4,
            function: "CP2",
            node: ActiveNetworkNode::ClockPhaseTwo,
        },
        MirroredMn3101PinConnection {
            channel_a_designator: "IC9.5",
            channel_b_designator: "IC11.5",
            pin: 5,
            function: "OX3",
            node: ActiveNetworkNode::Control,
        },
        MirroredMn3101PinConnection {
            channel_a_designator: "IC9.6",
            channel_b_designator: "IC11.6",
            pin: 6,
            function: "OX2",
            node: ActiveNetworkNode::OscillatorFeedback,
        },
        MirroredMn3101PinConnection {
            channel_a_designator: "IC9.7",
            channel_b_designator: "IC11.7",
            pin: 7,
            function: "OX1",
            node: ActiveNetworkNode::FeedbackBase,
        },
        MirroredMn3101PinConnection {
            channel_a_designator: "IC9.8",
            channel_b_designator: "IC11.8",
            pin: 8,
            function: "VGG",
            node: ActiveNetworkNode::Vgg,
        },
    ]
}

const fn active_network_netlist_audit() -> ActiveNetworkNetlistAudit {
    ActiveNetworkNetlistAudit {
        channel_a_path: "TP3 -> R128 -> Tr19-Tr23/D9 -> IC9 -> IC7",
        channel_b_path: "TP4 -> R148 -> Tr24-Tr28/D10 -> IC11 -> IC10",
        two_terminal_elements_per_channel: 18,
        bipolar_devices_per_channel: 5,
        controller_pins_per_channel: 8,
        cross_revision_connectivity_agreement: true,
        mirrored_by_construction: true,
        unresolved_schematic_connections: 0,
        diode_orientation_confidence: "high for orientation; exact diode type remains unresolved",
        dc_solver_topology_ready: true,
        nonlinear_device_models_ready: false,
        evidence: "target and related-revision full-page schematics agree; the earlier-family drawing independently confirms the five-BJT/OX1-OX2-OX3 topology",
    }
}

const fn mirrored_active_devices() -> [MirroredActiveDevice; 5] {
    [
        MirroredActiveDevice {
            channel_a_designator: "Tr19",
            channel_b_designator: "Tr24",
            polarity: BipolarPolarity::Pnp,
            evidence: "matching PNP schematic symbols in the target and cleaner related-revision jack-board drawings",
            exact_part_assignment: "unresolved within board-wide related-revision inventory",
        },
        MirroredActiveDevice {
            channel_a_designator: "Tr20",
            channel_b_designator: "Tr25",
            polarity: BipolarPolarity::Pnp,
            evidence: "matching PNP schematic symbols in the target and cleaner related-revision jack-board drawings",
            exact_part_assignment: "unresolved within board-wide related-revision inventory",
        },
        MirroredActiveDevice {
            channel_a_designator: "Tr21",
            channel_b_designator: "Tr26",
            polarity: BipolarPolarity::Npn,
            evidence: "matching NPN schematic symbols in the target and cleaner related-revision jack-board drawings",
            exact_part_assignment: "unresolved within board-wide related-revision inventory",
        },
        MirroredActiveDevice {
            channel_a_designator: "Tr22",
            channel_b_designator: "Tr27",
            polarity: BipolarPolarity::Npn,
            evidence: "matching NPN schematic symbols in the target and cleaner related-revision jack-board drawings",
            exact_part_assignment: "unresolved within board-wide related-revision inventory",
        },
        MirroredActiveDevice {
            channel_a_designator: "Tr23",
            channel_b_designator: "Tr28",
            polarity: BipolarPolarity::Npn,
            evidence: "matching NPN schematic symbols in the target and cleaner related-revision jack-board drawings",
            exact_part_assignment: "unresolved within board-wide related-revision inventory",
        },
    ]
}

const fn mirrored_diode_candidate() -> MirroredDiodeCandidate {
    MirroredDiodeCandidate {
        channel_a_designator: "D9",
        channel_b_designator: "D10",
        candidate: "1SS-133",
        confidence: "medium",
        evidence: "sole ordinary small-signal diode in the related-revision board-wide inventory; individual designators are not mapped",
    }
}

const fn diode_candidate_electrical_evidence() -> [DiodeCandidateElectricalEvidence; 2] {
    [
        DiodeCandidateElectricalEvidence {
            candidate: "1SS133",
            evidence_role: "related-revision board-wide inventory candidate",
            reverse_repetitive_peak_v: 90.0,
            reverse_dc_v: 80.0,
            average_forward_current_ma: 130.0,
            peak_forward_current_ma: 400.0,
            surge_forward_current_ma: 600.0,
            forward_voltage_max_v: 1.2,
            forward_voltage_test_current_ma: 100.0,
            reverse_current_max_ua: 0.5,
            reverse_current_test_voltage_v: 80.0,
            terminal_capacitance_max_pf: None,
            reverse_recovery_max_ns: None,
            manufacturer_typical_forward_curve_available: false,
            direct_target_designator_evidence: false,
            source: "ROHM 1SS133 product record; endpoint specifications only",
        },
        DiodeCandidateElectricalEvidence {
            candidate: "1S2473",
            evidence_role: "earlier-family explicit active-network rule",
            reverse_repetitive_peak_v: 40.0,
            reverse_dc_v: 35.0,
            average_forward_current_ma: 110.0,
            peak_forward_current_ma: 300.0,
            surge_forward_current_ma: 400.0,
            forward_voltage_max_v: 1.2,
            forward_voltage_test_current_ma: 100.0,
            reverse_current_max_ua: 0.5,
            reverse_current_test_voltage_v: 35.0,
            terminal_capacitance_max_pf: Some(3.0),
            reverse_recovery_max_ns: Some(4.0),
            manufacturer_typical_forward_curve_available: true,
            direct_target_designator_evidence: false,
            source: "ROHM 1S2471/1S2472/1S2473/1S2787/1S2788/1SS41 scan pages 1-2",
        },
    ]
}

const fn diode_forward_plot_calibration() -> DiodeForwardPlotCalibration {
    DiodeForwardPlotCalibration {
        candidate: "1S2473",
        source: "ROHM family scan page 2, forward characteristic, 25 C trace",
        page_width_px: 1700,
        page_height_px: 2200,
        x_left_px: 387.0,
        x_right_px: 684.0,
        voltage_left_v: 0.0,
        voltage_right_v: 1.3,
        y_top_px: 1125.0,
        y_bottom_px: 1574.0,
        current_top_a: 100.0e-3,
        current_bottom_a: 0.2e-3,
        reading_radius_px: 4.0,
        trace: "manufacturer typical 25 C curve; reading bounds do not cover unit variation",
    }
}

fn diode_forward_anchors() -> Vec<DiodeForwardAnchor> {
    let calibration = diode_forward_plot_calibration();
    let pixel_points = [
        (538.0, 1507.8),
        (546.0, 1457.7),
        (554.0, 1407.6),
        (565.0, 1341.4),
        (573.0, 1291.4),
        (581.0, 1241.3),
        (598.0, 1175.1),
        (606.0, 1125.0),
    ];
    pixel_points
        .into_iter()
        .map(|(x_px, y_px)| {
            let x_fraction =
                (x_px - calibration.x_left_px) / (calibration.x_right_px - calibration.x_left_px);
            let y_fraction =
                (y_px - calibration.y_top_px) / (calibration.y_bottom_px - calibration.y_top_px);
            let forward_voltage_v = calibration.voltage_left_v
                + x_fraction * (calibration.voltage_right_v - calibration.voltage_left_v);
            let top_log = calibration.current_top_a.log10();
            let bottom_log = calibration.current_bottom_a.log10();
            let forward_current_a = 10.0_f64.powf(top_log + y_fraction * (bottom_log - top_log));
            let voltage_reading_uncertainty_mv = calibration.reading_radius_px
                * (calibration.voltage_right_v - calibration.voltage_left_v).abs()
                / (calibration.x_right_px - calibration.x_left_px).abs()
                * 1_000.0;
            let log_current_per_pixel = (top_log - bottom_log).abs()
                / (calibration.y_bottom_px - calibration.y_top_px).abs();
            let current_reading_uncertainty_percent =
                (10.0_f64.powf(calibration.reading_radius_px * log_current_per_pixel) - 1.0)
                    * 100.0;
            DiodeForwardAnchor {
                candidate: calibration.candidate,
                x_px,
                y_px,
                forward_voltage_v,
                forward_current_a,
                voltage_reading_uncertainty_mv,
                current_reading_uncertainty_percent,
            }
        })
        .collect()
}

fn piecewise_diode_forward_voltage(candidate: &str, forward_current_a: f64) -> Option<f64> {
    if candidate != diode_forward_plot_calibration().candidate || forward_current_a <= 0.0 {
        return None;
    }
    let anchors = diode_forward_anchors();
    let first = anchors.first()?;
    let last = anchors.last()?;
    if forward_current_a < first.forward_current_a || forward_current_a > last.forward_current_a {
        return None;
    }
    if forward_current_a == first.forward_current_a {
        return Some(first.forward_voltage_v);
    }
    for pair in anchors.windows(2) {
        let lower = pair[0];
        let upper = pair[1];
        if forward_current_a <= upper.forward_current_a {
            let log_current = forward_current_a.log10();
            let fraction = (log_current - lower.forward_current_a.log10())
                / (upper.forward_current_a.log10() - lower.forward_current_a.log10());
            return Some(
                lower.forward_voltage_v
                    + fraction * (upper.forward_voltage_v - lower.forward_voltage_v),
            );
        }
    }
    Some(last.forward_voltage_v)
}

fn diode_piecewise_forward_validation() -> DiodePiecewiseForwardValidation {
    let anchors = diode_forward_anchors();
    let first = anchors.first().expect("the digitized curve has anchors");
    let last = anchors.last().expect("the digitized curve has anchors");
    let maximum_anchor_round_trip_error_mv = anchors
        .iter()
        .map(|anchor| {
            (piecewise_diode_forward_voltage(anchor.candidate, anchor.forward_current_a)
                .expect("every anchor is inside its own interpolation domain")
                - anchor.forward_voltage_v)
                .abs()
                * 1_000.0
        })
        .fold(0.0_f64, f64::max);
    DiodePiecewiseForwardValidation {
        candidate: first.candidate,
        anchor_count: anchors.len(),
        minimum_forward_current_a: first.forward_current_a,
        maximum_forward_current_a: last.forward_current_a,
        minimum_forward_voltage_v: first.forward_voltage_v,
        maximum_forward_voltage_v: last.forward_voltage_v,
        anchors_strictly_monotonic: anchors.windows(2).all(|pair| {
            pair[1].forward_current_a > pair[0].forward_current_a
                && pair[1].forward_voltage_v > pair[0].forward_voltage_v
        }),
        maximum_anchor_round_trip_error_mv,
        extrapolation_permitted: false,
        interpretation: "piecewise log-current interpolation is a 1S2473 typical center only and rejects extrapolation beyond its digitized 25 C curve",
    }
}

fn diode_shared_endpoint_envelope() -> DiodeSharedEndpointEnvelope {
    const CONSERVATIVE_NETWORK_FORWARD_PEAK_CURRENT_A: f64 = 30.0 / 2_200.0;
    const CONSERVATIVE_NETWORK_REVERSE_PEAK_V: f64 = 30.0;
    let evidence = diode_candidate_electrical_evidence();
    let shared_forward_endpoint_test_current_a = evidence
        .iter()
        .map(|candidate| candidate.forward_voltage_test_current_ma / 1_000.0)
        .fold(f64::INFINITY, f64::min);
    let shared_forward_voltage_maximum_v = evidence
        .iter()
        .map(|candidate| candidate.forward_voltage_max_v)
        .fold(0.0_f64, f64::max);
    let shared_reverse_test_voltage_minimum_v = evidence
        .iter()
        .map(|candidate| candidate.reverse_current_test_voltage_v)
        .fold(f64::INFINITY, f64::min);
    let shared_reverse_current_maximum_a = evidence
        .iter()
        .map(|candidate| candidate.reverse_current_max_ua * 1.0e-6)
        .fold(0.0_f64, f64::max);
    DiodeSharedEndpointEnvelope {
        documentary_candidates: evidence.len(),
        temperature_c: 25.0,
        conservative_network_forward_peak_current_a: CONSERVATIVE_NETWORK_FORWARD_PEAK_CURRENT_A,
        shared_forward_endpoint_test_current_a,
        shared_forward_voltage_minimum_v: 0.0,
        shared_forward_voltage_maximum_v,
        network_forward_domain_covered: CONSERVATIVE_NETWORK_FORWARD_PEAK_CURRENT_A
            <= shared_forward_endpoint_test_current_a,
        extend_forward_endpoint_to_lower_current_is_monotonicity_hypothesis: true,
        conservative_network_reverse_peak_v: CONSERVATIVE_NETWORK_REVERSE_PEAK_V,
        shared_reverse_test_voltage_minimum_v,
        shared_reverse_current_maximum_a,
        network_reverse_domain_covered: CONSERVATIVE_NETWORK_REVERSE_PEAK_V
            <= shared_reverse_test_voltage_minimum_v,
        extend_reverse_endpoint_to_lower_voltage_is_monotonicity_hypothesis: true,
        candidate_specific_typical_centers: evidence
            .iter()
            .filter(|candidate| candidate.manufacturer_typical_forward_curve_available)
            .count(),
        interval_model_ready: true,
        continuous_nominal_model_for_every_candidate_ready: false,
        exact_target_identity_required_for_interval_model: false,
        interpretation: "the shared model is a conservative 25 C voltage/current interval over the network domain; only 1S2473 has a candidate-specific typical center and neither identity is promoted to D9/D10",
    }
}

fn evaluate_diode_forward_voltage_interval(
    candidate: &'static str,
    forward_current_a: f64,
) -> Option<DiodeForwardVoltageInterval> {
    let evidence = diode_candidate_electrical_evidence();
    if !evidence.iter().any(|record| record.candidate == candidate) || forward_current_a < 0.0 {
        return None;
    }
    let envelope = diode_shared_endpoint_envelope();
    if forward_current_a > envelope.conservative_network_forward_peak_current_a {
        return None;
    }
    Some(DiodeForwardVoltageInterval {
        candidate,
        forward_current_a,
        shared_voltage_minimum_v: envelope.shared_forward_voltage_minimum_v,
        shared_voltage_maximum_v: envelope.shared_forward_voltage_maximum_v,
        candidate_specific_typical_center_v: piecewise_diode_forward_voltage(
            candidate,
            forward_current_a,
        ),
        typical_center_is_manufacturer_curve_not_guarantee: candidate == "1S2473",
        interval_is_shared_endpoint_hypothesis: true,
    })
}

fn diode_forward_reference_intervals() -> [DiodeForwardVoltageInterval; 4] {
    let peak = diode_shared_endpoint_envelope().conservative_network_forward_peak_current_a;
    [
        evaluate_diode_forward_voltage_interval("1SS133", 1.0e-3)
            .expect("1SS133 shared interval covers 1 mA"),
        evaluate_diode_forward_voltage_interval("1SS133", peak)
            .expect("1SS133 shared interval covers the network peak"),
        evaluate_diode_forward_voltage_interval("1S2473", 1.0e-3)
            .expect("1S2473 shared interval covers 1 mA"),
        evaluate_diode_forward_voltage_interval("1S2473", peak)
            .expect("1S2473 shared interval covers the network peak"),
    ]
}

fn diode_model_readiness() -> DiodeModelReadiness {
    let evidence = diode_candidate_electrical_evidence();
    let anchors = diode_forward_anchors();
    let shared = diode_shared_endpoint_envelope();
    DiodeModelReadiness {
        documentary_candidates: evidence.len(),
        candidates_with_endpoint_limits: evidence
            .iter()
            .filter(|candidate| {
                candidate.forward_voltage_max_v > 0.0 && candidate.reverse_current_max_ua > 0.0
            })
            .count(),
        candidates_with_digitized_typical_curve: evidence
            .iter()
            .filter(|candidate| candidate.manufacturer_typical_forward_curve_available)
            .count(),
        digitized_curve_anchor_count: anchors.len(),
        digitized_curve_strictly_monotonic: anchors.windows(2).all(|pair| {
            pair[1].forward_current_a > pair[0].forward_current_a
                && pair[1].forward_voltage_v > pair[0].forward_voltage_v
        }),
        shared_endpoint_interval_model_ready: shared.interval_model_ready,
        network_forward_domain_covered: shared.network_forward_domain_covered,
        network_reverse_domain_covered: shared.network_reverse_domain_covered,
        continuous_nominal_curve_for_every_candidate_ready: shared
            .continuous_nominal_model_for_every_candidate_ready,
        exact_target_type_solved: false,
        bounded_forward_iv_for_every_candidate: shared.network_forward_domain_covered,
        bounded_reverse_iv_for_every_candidate: shared.network_reverse_domain_covered,
        safe_scope: "sweep the shared endpoint interval for both candidates and use the 1S2473 curve only as its own optional typical center",
        interpretation: "the two documentary paths still disagree on identity, but their common endpoint limits bound the complete network domain without promoting the sole typical curve to 1SS133 or to a guarantee",
    }
}

const fn board_wide_bipolar_inventory() -> BoardWideBipolarInventory {
    BoardWideBipolarInventory {
        pnp_candidates: ["2SA1015-GR/Y", "2SA933-R/Q", "2SA1115-F/E", "2SB1015-D"],
        npn_candidates: [
            "2SC1815-GR/Y",
            "2SC1740-R/Q",
            "2SC2603-F/E",
            "2SC945 selected for noise",
            "2SC2878A",
            "2SD1406-Q",
        ],
        scope: "related-revision board-wide semiconductor inventory only; it does not map Tr19-Tr28 to exact parts",
    }
}

const fn related_family_active_device_rule() -> RelatedFamilyActiveDeviceRule {
    RelatedFamilyActiveDeviceRule {
        npn_candidates: ["2SC1815-GR/Y", "2SC2603-F"],
        pnp_candidates: ["2SA1015-GR/Y", "2SA1115-F"],
        diode: "1S2473",
        topology_relation: "same dual MN3101/MN3009 clock architecture and five-BJT active-control cell with matching nominal values in the earlier-family chorus board",
        evidence_level: "E6 comparative hypothesis only",
    }
}

const fn comparative_candidate_shortlist() -> ComparativeCandidateShortlist {
    ComparativeCandidateShortlist {
        npn_candidates: ["2SC1815-GR/Y", "2SC2603-F"],
        pnp_candidates: ["2SA1015-GR/Y", "2SA1115-F"],
        diode_intersection: [],
        derivation: "intersection of the related-revision board-wide inventory with the earlier-family explicit active-network rule; the diode sets conflict (1SS-133 versus 1S2473)",
        direct_target_evidence: false,
    }
}

const fn integer_power(base: usize, exponent: usize) -> usize {
    let mut result = 1;
    let mut remaining = exponent;
    while remaining > 0 {
        result *= base;
        remaining -= 1;
    }
    result
}

const fn active_device_assignment_space() -> ActiveDeviceAssignmentSpace {
    let pnp_positions_per_channel = 2;
    let npn_positions_per_channel = 3;
    let board_inventory_assignments =
        integer_power(4, pnp_positions_per_channel) * integer_power(6, npn_positions_per_channel);
    let comparative_shortlist_assignments =
        integer_power(2, pnp_positions_per_channel) * integer_power(2, npn_positions_per_channel);
    ActiveDeviceAssignmentSpace {
        pnp_positions_per_channel,
        npn_positions_per_channel,
        board_inventory_assignments,
        comparative_shortlist_assignments,
        uniform_family_per_polarity_hypotheses: 2 * 2,
        channel_b_is_constrained_to_mirror_channel_a: true,
    }
}

fn active_device_hypotheses() -> Vec<ActiveDeviceHypothesis> {
    let shortlist = comparative_candidate_shortlist();
    let mut hypotheses =
        Vec::with_capacity(active_device_assignment_space().comparative_shortlist_assignments);
    for tr19 in shortlist.pnp_candidates {
        for tr20 in shortlist.pnp_candidates {
            for tr21 in shortlist.npn_candidates {
                for tr22 in shortlist.npn_candidates {
                    for tr23 in shortlist.npn_candidates {
                        hypotheses.push(ActiveDeviceHypothesis {
                            index: hypotheses.len(),
                            channel_a_pnp: [tr19, tr20],
                            channel_a_npn: [tr21, tr22, tr23],
                            channel_b_uses_mirrored_assignments: true,
                            direct_target_evidence: false,
                        });
                    }
                }
            }
        }
    }
    hypotheses
}

const fn candidate_bipolar_model_evidence() -> [CandidateBipolarModelEvidence; 4] {
    [
        CandidateBipolarModelEvidence {
            candidate_key: "2SA1015-GR/Y",
            device: "2SA1015",
            polarity: BipolarPolarity::Pnp,
            allowed_ranks: "Y or GR",
            hfe_minimum: 120.0,
            hfe_maximum: 400.0,
            hfe_condition: "VCE=-6 V, IC=-2 mA, Ta=25 C; union of Y 120-240 and GR 200-400",
            vcbo_absolute_volts: 50.0,
            vceo_absolute_volts: 50.0,
            vebo_absolute_volts: 5.0,
            collector_current_absolute_maximum_ma: 150.0,
            collector_power_absolute_maximum_mw: 400.0,
            collector_cutoff_current_maximum_ua: 0.1,
            collector_cutoff_condition: "VCB=-50 V, IE=0, Ta=25 C",
            emitter_cutoff_current_maximum_ua: 0.1,
            emitter_cutoff_condition: "VEB=-5 V, IC=0, Ta=25 C",
            vce_saturation_typical_absolute_volts: Some(0.1),
            vce_saturation_maximum_absolute_volts: 0.3,
            vce_saturation_test_collector_current_absolute_ma: 100.0,
            vce_saturation_test_base_current_absolute_ma: 10.0,
            vce_saturation_condition: "IC=-100 mA, IB=-10 mA, Ta=25 C",
            vbe_saturation_maximum_absolute_volts: Some(1.1),
            vbe_saturation_condition: Some("IC=-100 mA, IB=-10 mA, Ta=25 C"),
            transition_frequency_mhz: 80.0,
            transition_frequency_limit_kind: "minimum",
            transition_frequency_condition: "VCE=-10 V, IC=-1 mA, Ta=25 C",
            collector_output_capacitance_typical_pf: 4.0,
            collector_output_capacitance_maximum_pf: Some(7.0),
            collector_output_capacitance_condition: "VCB=-10 V, IE=0, f=1 MHz, Ta=25 C",
            base_intrinsic_resistance_typical_ohms: Some(30.0),
            common_emitter_h_parameter_condition: None,
            common_emitter_h_parameter_emitter_current_absolute_ma: None,
            common_emitter_hie_typical_ohms: None,
            common_emitter_hre_typical: None,
            common_emitter_hfe_typical: None,
            common_emitter_hoe_typical_microsiemens: None,
            source: "SRC-BJT-2SA1015 PDF page 1",
            supports_complete_large_signal_model: false,
        },
        CandidateBipolarModelEvidence {
            candidate_key: "2SA1115-F",
            device: "2SA1115",
            polarity: BipolarPolarity::Pnp,
            allowed_ranks: "F",
            hfe_minimum: 250.0,
            hfe_maximum: 500.0,
            hfe_condition: "VCE=-6 V, IC=-1 mA, Ta=25 C; F rank",
            vcbo_absolute_volts: 50.0,
            vceo_absolute_volts: 50.0,
            vebo_absolute_volts: 6.0,
            collector_current_absolute_maximum_ma: 200.0,
            collector_power_absolute_maximum_mw: 300.0,
            collector_cutoff_current_maximum_ua: 0.1,
            collector_cutoff_condition: "VCB=-50 V, IE=0, Ta=25 C",
            emitter_cutoff_current_maximum_ua: 0.1,
            emitter_cutoff_condition: "VEB=-6 V, IC=0, Ta=25 C",
            vce_saturation_typical_absolute_volts: None,
            vce_saturation_maximum_absolute_volts: 0.3,
            vce_saturation_test_collector_current_absolute_ma: 100.0,
            vce_saturation_test_base_current_absolute_ma: 10.0,
            vce_saturation_condition: "IC=-100 mA, IB=-10 mA, Ta=25 C",
            vbe_saturation_maximum_absolute_volts: None,
            vbe_saturation_condition: None,
            transition_frequency_mhz: 200.0,
            transition_frequency_limit_kind: "typical",
            transition_frequency_condition: "VCE=-6 V, magnitude of IE=10 mA, Ta=25 C",
            collector_output_capacitance_typical_pf: 4.0,
            collector_output_capacitance_maximum_pf: None,
            collector_output_capacitance_condition: "VCB=-6 V, IE=0, f=1 MHz, Ta=25 C",
            base_intrinsic_resistance_typical_ohms: None,
            common_emitter_h_parameter_condition: Some("VCE=-6 V, IE=1 mA, f=270 Hz, Ta=25 C"),
            common_emitter_h_parameter_emitter_current_absolute_ma: Some(1.0),
            common_emitter_hie_typical_ohms: Some(7_000.0),
            common_emitter_hre_typical: Some(0.1e-3),
            common_emitter_hfe_typical: Some(250.0),
            common_emitter_hoe_typical_microsiemens: Some(18.0),
            source: "SRC-BJT-2SA1115 PDF pages 1-3 / printed 2-17 to 2-19",
            supports_complete_large_signal_model: false,
        },
        CandidateBipolarModelEvidence {
            candidate_key: "2SC1815-GR/Y",
            device: "2SC1815",
            polarity: BipolarPolarity::Npn,
            allowed_ranks: "Y or GR",
            hfe_minimum: 120.0,
            hfe_maximum: 400.0,
            hfe_condition: "VCE=6 V, IC=2 mA, Ta=25 C; union of Y 120-240 and GR 200-400",
            vcbo_absolute_volts: 60.0,
            vceo_absolute_volts: 50.0,
            vebo_absolute_volts: 5.0,
            collector_current_absolute_maximum_ma: 150.0,
            collector_power_absolute_maximum_mw: 400.0,
            collector_cutoff_current_maximum_ua: 0.1,
            collector_cutoff_condition: "VCB=60 V, IE=0, Ta=25 C",
            emitter_cutoff_current_maximum_ua: 0.1,
            emitter_cutoff_condition: "VEB=5 V, IC=0, Ta=25 C",
            vce_saturation_typical_absolute_volts: Some(0.1),
            vce_saturation_maximum_absolute_volts: 0.25,
            vce_saturation_test_collector_current_absolute_ma: 100.0,
            vce_saturation_test_base_current_absolute_ma: 10.0,
            vce_saturation_condition: "IC=100 mA, IB=10 mA, Ta=25 C",
            vbe_saturation_maximum_absolute_volts: Some(1.0),
            vbe_saturation_condition: Some("IC=100 mA, IB=10 mA, Ta=25 C"),
            transition_frequency_mhz: 80.0,
            transition_frequency_limit_kind: "minimum",
            transition_frequency_condition: "VCE=10 V, IC=1 mA, Ta=25 C",
            collector_output_capacitance_typical_pf: 2.0,
            collector_output_capacitance_maximum_pf: Some(3.5),
            collector_output_capacitance_condition: "VCB=10 V, IE=0, f=1 MHz, Ta=25 C",
            base_intrinsic_resistance_typical_ohms: Some(50.0),
            common_emitter_h_parameter_condition: None,
            common_emitter_h_parameter_emitter_current_absolute_ma: None,
            common_emitter_hie_typical_ohms: None,
            common_emitter_hre_typical: None,
            common_emitter_hfe_typical: None,
            common_emitter_hoe_typical_microsiemens: None,
            source: "SRC-BJT-2SC1815 PDF page 1",
            supports_complete_large_signal_model: false,
        },
        CandidateBipolarModelEvidence {
            candidate_key: "2SC2603-F",
            device: "2SC2603",
            polarity: BipolarPolarity::Npn,
            allowed_ranks: "F",
            hfe_minimum: 250.0,
            hfe_maximum: 500.0,
            hfe_condition: "VCE=6 V, IC=1 mA, Ta=25 C; F rank",
            vcbo_absolute_volts: 50.0,
            vceo_absolute_volts: 50.0,
            vebo_absolute_volts: 6.0,
            collector_current_absolute_maximum_ma: 200.0,
            collector_power_absolute_maximum_mw: 300.0,
            collector_cutoff_current_maximum_ua: 0.1,
            collector_cutoff_condition: "VCB=50 V, IE=0, Ta=25 C",
            emitter_cutoff_current_maximum_ua: 0.1,
            emitter_cutoff_condition: "VEB=6 V, IC=0, Ta=25 C",
            vce_saturation_typical_absolute_volts: None,
            vce_saturation_maximum_absolute_volts: 0.3,
            vce_saturation_test_collector_current_absolute_ma: 100.0,
            vce_saturation_test_base_current_absolute_ma: 10.0,
            vce_saturation_condition: "IC=100 mA, IB=10 mA, Ta=25 C",
            vbe_saturation_maximum_absolute_volts: None,
            vbe_saturation_condition: None,
            transition_frequency_mhz: 200.0,
            transition_frequency_limit_kind: "typical",
            transition_frequency_condition: "VCE=6 V, magnitude of IE=10 mA, Ta=25 C",
            collector_output_capacitance_typical_pf: 2.5,
            collector_output_capacitance_maximum_pf: None,
            collector_output_capacitance_condition: "VCB=6 V, IE=0, f=1 MHz, Ta=25 C",
            base_intrinsic_resistance_typical_ohms: None,
            common_emitter_h_parameter_condition: Some(
                "VCE=6 V, magnitude of IE=1 mA, f=270 Hz, Ta=25 C",
            ),
            common_emitter_h_parameter_emitter_current_absolute_ma: Some(1.0),
            common_emitter_hie_typical_ohms: Some(8_500.0),
            common_emitter_hre_typical: Some(0.1e-3),
            common_emitter_hfe_typical: Some(300.0),
            common_emitter_hoe_typical_microsiemens: Some(5.5),
            source: "SRC-BJT-2SC2603 PDF pages 1-3 / printed 4-34 to 4-36",
            supports_complete_large_signal_model: false,
        },
    ]
}

const fn bjt_output_characteristic_plot_evidence() -> [BjtOutputCharacteristicPlotEvidence; 4] {
    const TOSHIBA_2SA1015_BASE_CURRENT_LEVELS_A: &[f64] =
        &[0.0, 0.2e-3, 0.5e-3, 1.0e-3, 1.5e-3, 2.0e-3];
    const TOSHIBA_2SC1815_BASE_CURRENT_LEVELS_A: &[f64] =
        &[0.0, 0.2e-3, 0.5e-3, 1.0e-3, 2.0e-3, 3.0e-3, 5.0e-3, 6.0e-3];
    const MITSUBISHI_2SA1115_BASE_CURRENT_LEVELS_A: &[f64] = &[
        0.0, 0.02e-3, 0.04e-3, 0.06e-3, 0.08e-3, 0.10e-3, 0.12e-3, 0.14e-3, 0.16e-3, 0.18e-3,
    ];
    const MITSUBISHI_2SC2603_BASE_CURRENT_LEVELS_A: &[f64] = &[
        0.0, 0.02e-3, 0.04e-3, 0.06e-3, 0.08e-3, 0.10e-3, 0.12e-3, 0.14e-3, 0.16e-3,
    ];
    [
        BjtOutputCharacteristicPlotEvidence {
            candidate_key: "2SA1015-GR/Y",
            source: "SRC-BJT-2SA1015 PDF page 2, IC-VCE common-emitter output family",
            polarity: BipolarPolarity::Pnp,
            temperature_c: 25.0,
            page_width_px: 1489,
            page_height_px: 2105,
            x_left_px: 233.0,
            x_right_px: 668.0,
            collector_emitter_voltage_left_v: 0.0,
            collector_emitter_voltage_right_v: 8.0,
            y_top_px: 288.0,
            y_bottom_px: 605.0,
            collector_current_top_a: 240.0e-3,
            collector_current_bottom_a: 0.0,
            base_current_curve_levels_a: TOSHIBA_2SA1015_BASE_CURRENT_LEVELS_A,
            includes_zero_base_current_curve: true,
            reading_radius_px: 4.0,
            calibration_basis: "180 dpi full-page render; Toshiba page is assembled from multiple embedded raster strips",
            curve_status: "manufacturer-typical 25 C curve family; labeled IB levels are direct evidence, interpolation and pixel readings are not yet promoted",
        },
        BjtOutputCharacteristicPlotEvidence {
            candidate_key: "2SC1815-GR/Y",
            source: "SRC-BJT-2SC1815 PDF page 2, IC-VCE common-emitter output family",
            polarity: BipolarPolarity::Npn,
            temperature_c: 25.0,
            page_width_px: 1489,
            page_height_px: 2105,
            x_left_px: 233.0,
            x_right_px: 664.0,
            collector_emitter_voltage_left_v: 0.0,
            collector_emitter_voltage_right_v: 8.0,
            y_top_px: 77.0,
            y_bottom_px: 438.0,
            collector_current_top_a: 240.0e-3,
            collector_current_bottom_a: 0.0,
            base_current_curve_levels_a: TOSHIBA_2SC1815_BASE_CURRENT_LEVELS_A,
            includes_zero_base_current_curve: true,
            reading_radius_px: 4.0,
            calibration_basis: "180 dpi full-page render; Toshiba page is assembled from multiple embedded raster strips",
            curve_status: "manufacturer-typical 25 C curve family; labeled IB levels are direct evidence, interpolation and pixel readings are not yet promoted",
        },
        BjtOutputCharacteristicPlotEvidence {
            candidate_key: "2SA1115-F",
            source: "SRC-BJT-2SA1115 PDF page 2 / printed 2-18, common-emitter transfer family",
            polarity: BipolarPolarity::Pnp,
            temperature_c: 25.0,
            page_width_px: 2176,
            page_height_px: 3055,
            x_left_px: 1322.0,
            x_right_px: 1839.0,
            collector_emitter_voltage_left_v: 0.0,
            collector_emitter_voltage_right_v: 5.0,
            y_top_px: 549.0,
            y_bottom_px: 1062.0,
            collector_current_top_a: 50.0e-3,
            collector_current_bottom_a: 0.0,
            base_current_curve_levels_a: MITSUBISHI_2SA1115_BASE_CURRENT_LEVELS_A,
            includes_zero_base_current_curve: true,
            reading_radius_px: 4.0,
            calibration_basis: "native 2176 x 3055 one-bit manufacturer-page raster extracted directly from the PDF",
            curve_status: "manufacturer-typical 25 C curve family; labeled IB levels are direct evidence, interpolation and pixel readings are not yet promoted",
        },
        BjtOutputCharacteristicPlotEvidence {
            candidate_key: "2SC2603-F",
            source: "SRC-BJT-2SC2603 PDF page 2 / printed 4-35, common-emitter output family",
            polarity: BipolarPolarity::Npn,
            temperature_c: 25.0,
            page_width_px: 2176,
            page_height_px: 3036,
            x_left_px: 1300.0,
            x_right_px: 1814.0,
            collector_emitter_voltage_left_v: 0.0,
            collector_emitter_voltage_right_v: 5.0,
            y_top_px: 522.0,
            y_bottom_px: 1038.0,
            collector_current_top_a: 50.0e-3,
            collector_current_bottom_a: 0.0,
            base_current_curve_levels_a: MITSUBISHI_2SC2603_BASE_CURRENT_LEVELS_A,
            includes_zero_base_current_curve: true,
            reading_radius_px: 4.0,
            calibration_basis: "native 2176 x 3036 one-bit manufacturer-page raster extracted directly from the PDF",
            curve_status: "manufacturer-typical 25 C curve family; labeled IB levels are direct evidence, interpolation and pixel readings are not yet promoted",
        },
    ]
}

fn bjt_output_characteristic_evidence_validation() -> BjtOutputCharacteristicEvidenceValidation {
    const CONSERVATIVE_NETWORK_PEAK_CURRENT_A: f64 = 30.0 / 2_200.0;
    const SATURATION_KNEE_VOLTAGE_DOMAIN_V: f64 = 1.0;
    let evidence = bjt_output_characteristic_plot_evidence();
    let every_positive_base_current_family_strictly_increasing = evidence.iter().all(|record| {
        record
            .base_current_curve_levels_a
            .windows(2)
            .all(|pair| pair[1] > pair[0])
    });
    let minimum_shared_collector_emitter_voltage_domain_v = evidence
        .iter()
        .map(|record| record.collector_emitter_voltage_right_v)
        .fold(f64::INFINITY, f64::min);
    let minimum_shared_collector_current_domain_a = evidence
        .iter()
        .map(|record| record.collector_current_top_a)
        .fold(f64::INFINITY, f64::min);
    let voltage_reading_uncertainties_mv = evidence.map(|record| {
        record.reading_radius_px
            * (record.collector_emitter_voltage_right_v - record.collector_emitter_voltage_left_v)
                .abs()
            / (record.x_right_px - record.x_left_px).abs()
            * 1_000.0
    });
    let collector_current_reading_uncertainties_ma = evidence.map(|record| {
        record.reading_radius_px
            * (record.collector_current_top_a - record.collector_current_bottom_a).abs()
            / (record.y_bottom_px - record.y_top_px).abs()
            * 1_000.0
    });
    BjtOutputCharacteristicEvidenceValidation {
        candidate_count: evidence.len(),
        pnp_candidate_count: evidence
            .iter()
            .filter(|record| record.polarity == BipolarPolarity::Pnp)
            .count(),
        npn_candidate_count: evidence
            .iter()
            .filter(|record| record.polarity == BipolarPolarity::Npn)
            .count(),
        total_labeled_base_current_curves: evidence
            .iter()
            .map(|record| record.base_current_curve_levels_a.len())
            .sum(),
        every_candidate_has_zero_base_current_curve: evidence.iter().all(|record| {
            record.includes_zero_base_current_curve
                && record.base_current_curve_levels_a.first() == Some(&0.0)
        }),
        every_positive_base_current_family_strictly_increasing,
        minimum_shared_collector_emitter_voltage_domain_v,
        minimum_shared_collector_current_domain_a,
        conservative_network_peak_current_a: CONSERVATIVE_NETWORK_PEAK_CURRENT_A,
        saturation_knee_voltage_domain_v: SATURATION_KNEE_VOLTAGE_DOMAIN_V,
        every_family_covers_conservative_network_current: minimum_shared_collector_current_domain_a
            >= CONSERVATIVE_NETWORK_PEAK_CURRENT_A,
        every_family_covers_saturation_knee_voltage:
            minimum_shared_collector_emitter_voltage_domain_v >= SATURATION_KNEE_VOLTAGE_DOMAIN_V,
        direct_base_drive_evidence_available_for_every_candidate: evidence.len() == 4,
        minimum_voltage_reading_uncertainty_mv: voltage_reading_uncertainties_mv
            .into_iter()
            .fold(f64::INFINITY, f64::min),
        maximum_voltage_reading_uncertainty_mv: voltage_reading_uncertainties_mv
            .into_iter()
            .fold(0.0, f64::max),
        minimum_collector_current_reading_uncertainty_ma:
            collector_current_reading_uncertainties_ma
                .into_iter()
                .fold(f64::INFINITY, f64::min),
        maximum_collector_current_reading_uncertainty_ma:
            collector_current_reading_uncertainties_ma
                .into_iter()
                .fold(0.0, f64::max),
        interpolation_between_labeled_base_current_curves_is_hypothesis: true,
        digitized_output_surface_ready: false,
        suitable_as_confirmed_production_device_model: false,
        interpretation: "all four manufacturer records contain a 25 C common-emitter IC-VCE family with labeled IB curves covering the conservative network current and 0-1 V knee domain; the plots provide direct base-drive evidence, but their typical traces must be digitized and bounded before entering the coupled solver",
    }
}

const fn bjt_transfer_plot_calibrations() -> [BjtTransferPlotCalibration; 4] {
    [
        BjtTransferPlotCalibration {
            candidate_key: "2SA1015-GR/Y",
            source: "SRC-BJT-2SA1015 PDF page 2, IB-VBE plot, 25 C trace",
            page_width_px: 1489,
            page_height_px: 2105,
            x_left_px: 834.0,
            x_right_px: 1268.0,
            vbe_left_v: 0.0,
            vbe_right_v: 1.2,
            y_top_px: 288.0,
            y_bottom_px: 743.0,
            current_top_a: 1.0e-3,
            current_bottom_a: 0.3e-6,
            current_axis: PlotCurrentAxis::Logarithmic,
            current_kind: TransferCurrentKind::Base,
            reading_radius_px: 4.0,
            curve_status: "manufacturer typical curve; digitization bounds cover reading error only",
        },
        BjtTransferPlotCalibration {
            candidate_key: "2SA1115-F",
            source: "SRC-BJT-2SA1115 PDF page 2 / printed 2-18, IC-VBE plot, 25 C trace",
            page_width_px: 1599,
            page_height_px: 2243,
            x_left_px: 351.0,
            x_right_px: 728.0,
            vbe_left_v: 0.0,
            vbe_right_v: 1.0,
            y_top_px: 1000.0,
            y_bottom_px: 1387.0,
            current_top_a: 50.0e-3,
            current_bottom_a: 0.0,
            current_axis: PlotCurrentAxis::Linear,
            current_kind: TransferCurrentKind::Collector,
            reading_radius_px: 4.0,
            curve_status: "manufacturer typical curve; digitization bounds cover reading error only",
        },
        BjtTransferPlotCalibration {
            candidate_key: "2SC1815-GR/Y",
            source: "SRC-BJT-2SC1815 PDF page 2, IB-VBE plot, 25 C trace",
            page_width_px: 1489,
            page_height_px: 2105,
            x_left_px: 232.0,
            x_right_px: 664.0,
            vbe_left_v: 0.0,
            vbe_right_v: 2.0,
            y_top_px: 1467.0,
            y_bottom_px: 1879.0,
            current_top_a: 3.0e-3,
            current_bottom_a: 0.3e-6,
            current_axis: PlotCurrentAxis::Logarithmic,
            current_kind: TransferCurrentKind::Base,
            reading_radius_px: 4.0,
            curve_status: "manufacturer typical curve; digitization bounds cover reading error only",
        },
        BjtTransferPlotCalibration {
            candidate_key: "2SC2603-F",
            source: "SRC-BJT-2SC2603 PDF page 2 / printed 4-35, IC-VBE plot, 25 C trace",
            page_width_px: 1819,
            page_height_px: 2573,
            x_left_px: 389.0,
            x_right_px: 818.0,
            vbe_left_v: 0.0,
            vbe_right_v: 1.0,
            y_top_px: 1142.0,
            y_bottom_px: 1567.0,
            current_top_a: 50.0e-3,
            current_bottom_a: 0.0,
            current_axis: PlotCurrentAxis::Linear,
            current_kind: TransferCurrentKind::Collector,
            reading_radius_px: 4.0,
            curve_status: "manufacturer typical curve; digitization bounds cover reading error only",
        },
    ]
}

fn bjt_transfer_pixel_anchors() -> Vec<BjtTransferPixelAnchor> {
    let mut anchors = Vec::with_capacity(30);
    let records: [(&str, &[(f64, f64)]); 4] = [
        (
            "2SA1015-GR/Y",
            &[
                (1041.0, 675.5),
                (1049.0, 613.8),
                (1061.0, 546.3),
                (1073.0, 484.7),
                (1088.0, 417.2),
                (1103.0, 355.5),
                (1124.0, 288.0),
            ],
        ),
        (
            "2SA1115-F",
            &[
                (576.0, 1379.3),
                (580.0, 1371.5),
                (584.0, 1363.8),
                (590.0, 1348.3),
                (595.0, 1332.8),
                (597.0, 1309.6),
                (601.0, 1270.9),
                (605.0, 1232.2),
                (609.0, 1154.8),
            ],
        ),
        (
            "2SC1815-GR/Y",
            &[
                (364.0, 1825.1),
                (371.0, 1776.0),
                (378.0, 1722.1),
                (385.0, 1673.0),
                (392.0, 1619.1),
                (401.0, 1570.0),
                (410.0, 1516.1),
            ],
        ),
        (
            "2SC2603-F",
            &[
                (658.0, 1558.5),
                (669.0, 1550.0),
                (676.0, 1541.5),
                (687.0, 1507.5),
                (697.0, 1397.0),
                (700.0, 1312.0),
                (705.0, 1227.0),
            ],
        ),
    ];
    for (candidate_key, points) in records {
        for &(x_px, y_px) in points {
            anchors.push(BjtTransferPixelAnchor {
                candidate_key,
                x_px,
                y_px,
            });
        }
    }
    anchors
}

fn plot_calibration(candidate_key: &str) -> BjtTransferPlotCalibration {
    *bjt_transfer_plot_calibrations()
        .iter()
        .find(|calibration| calibration.candidate_key == candidate_key)
        .expect("every digitized BJT candidate must have one plot calibration")
}

fn electrical_anchor(pixel: BjtTransferPixelAnchor) -> BjtTransferElectricalAnchor {
    let calibration = plot_calibration(pixel.candidate_key);
    let x_fraction =
        (pixel.x_px - calibration.x_left_px) / (calibration.x_right_px - calibration.x_left_px);
    let y_fraction =
        (pixel.y_px - calibration.y_top_px) / (calibration.y_bottom_px - calibration.y_top_px);
    let vbe_v =
        calibration.vbe_left_v + x_fraction * (calibration.vbe_right_v - calibration.vbe_left_v);
    let current_a = match calibration.current_axis {
        PlotCurrentAxis::Linear => {
            calibration.current_top_a
                + y_fraction * (calibration.current_bottom_a - calibration.current_top_a)
        }
        PlotCurrentAxis::Logarithmic => {
            let top = calibration.current_top_a.log10();
            let bottom = calibration.current_bottom_a.log10();
            10.0_f64.powf(top + y_fraction * (bottom - top))
        }
    };
    let vbe_reading_uncertainty_mv = calibration.reading_radius_px
        * (calibration.vbe_right_v - calibration.vbe_left_v).abs()
        / (calibration.x_right_px - calibration.x_left_px).abs()
        * 1_000.0;
    let current_reading_uncertainty_percent = match calibration.current_axis {
        PlotCurrentAxis::Linear => {
            let current_per_pixel = (calibration.current_top_a - calibration.current_bottom_a)
                .abs()
                / (calibration.y_bottom_px - calibration.y_top_px).abs();
            calibration.reading_radius_px * current_per_pixel / current_a * 100.0
        }
        PlotCurrentAxis::Logarithmic => {
            let log_current_per_pixel =
                (calibration.current_top_a.log10() - calibration.current_bottom_a.log10()).abs()
                    / (calibration.y_bottom_px - calibration.y_top_px).abs();
            (10.0_f64.powf(calibration.reading_radius_px * log_current_per_pixel) - 1.0) * 100.0
        }
    };
    BjtTransferElectricalAnchor {
        candidate_key: pixel.candidate_key,
        current_kind: calibration.current_kind,
        vbe_v,
        current_a,
        vbe_reading_uncertainty_mv,
        current_reading_uncertainty_percent,
    }
}

fn bjt_transfer_electrical_anchors() -> Vec<BjtTransferElectricalAnchor> {
    bjt_transfer_pixel_anchors()
        .into_iter()
        .map(electrical_anchor)
        .collect()
}

fn bjt_typical_transfer_fit(candidate_key: &'static str) -> BjtTypicalTransferFit {
    const TEMPERATURE_C: f64 = 25.0;
    const THERMAL_VOLTAGE_V: f64 = 0.025_692_58;
    let anchors = bjt_transfer_electrical_anchors()
        .into_iter()
        .filter(|anchor| anchor.candidate_key == candidate_key)
        .collect::<Vec<_>>();
    let count = anchors.len() as f64;
    let mean_log_current = anchors
        .iter()
        .map(|anchor| anchor.current_a.ln())
        .sum::<f64>()
        / count;
    let mean_vbe = anchors.iter().map(|anchor| anchor.vbe_v).sum::<f64>() / count;
    let covariance = anchors
        .iter()
        .map(|anchor| (anchor.current_a.ln() - mean_log_current) * (anchor.vbe_v - mean_vbe))
        .sum::<f64>();
    let log_current_variance = anchors
        .iter()
        .map(|anchor| (anchor.current_a.ln() - mean_log_current).powi(2))
        .sum::<f64>();
    let slope = covariance / log_current_variance;
    let intercept = mean_vbe - slope * mean_log_current;
    let residuals = anchors
        .iter()
        .map(|anchor| anchor.vbe_v - (intercept + slope * anchor.current_a.ln()))
        .collect::<Vec<_>>();
    let rms_vbe_residual_mv = (residuals
        .iter()
        .map(|residual| residual.powi(2))
        .sum::<f64>()
        / count)
        .sqrt()
        * 1_000.0;
    let maximum_absolute_vbe_residual_mv = residuals
        .iter()
        .map(|residual| residual.abs())
        .fold(0.0_f64, f64::max)
        * 1_000.0;
    let maximum_anchor_vbe_reading_uncertainty_mv = anchors
        .iter()
        .map(|anchor| anchor.vbe_reading_uncertainty_mv)
        .fold(0.0_f64, f64::max);
    let maximum_anchor_current_reading_uncertainty_percent = anchors
        .iter()
        .map(|anchor| anchor.current_reading_uncertainty_percent)
        .fold(0.0_f64, f64::max);
    let current_uncertainty_as_vbe_mv =
        slope * (1.0 + maximum_anchor_current_reading_uncertainty_percent / 100.0).ln() * 1_000.0;
    BjtTypicalTransferFit {
        candidate_key,
        current_kind: anchors[0].current_kind,
        anchor_count: anchors.len(),
        temperature_c: TEMPERATURE_C,
        thermal_voltage_v: THERMAL_VOLTAGE_V,
        fitted_emission_coefficient: slope / THERMAL_VOLTAGE_V,
        fitted_scale_current_a: (-intercept / slope).exp(),
        rms_vbe_residual_mv,
        maximum_absolute_vbe_residual_mv,
        maximum_anchor_vbe_reading_uncertainty_mv,
        maximum_anchor_current_reading_uncertainty_percent,
        single_exponential_covers_plot_reading_error: maximum_absolute_vbe_residual_mv
            <= maximum_anchor_vbe_reading_uncertainty_mv + current_uncertainty_as_vbe_mv,
        fit_covers_unit_to_unit_variation: false,
        anchors_suitable_for_piecewise_hypothesis_center: true,
        single_exponential_suitable_for_hypothesis_center: maximum_absolute_vbe_residual_mv
            <= maximum_anchor_vbe_reading_uncertainty_mv + current_uncertainty_as_vbe_mv,
        suitable_as_guaranteed_device_bound: false,
        interpretation: "exponential center fit to a 25 C manufacturer-typical trace; the encoded uncertainty covers pixel reading, not production spread",
    }
}

fn bjt_typical_transfer_fits() -> [BjtTypicalTransferFit; 4] {
    [
        bjt_typical_transfer_fit("2SA1015-GR/Y"),
        bjt_typical_transfer_fit("2SA1115-F"),
        bjt_typical_transfer_fit("2SC1815-GR/Y"),
        bjt_typical_transfer_fit("2SC2603-F"),
    ]
}

fn piecewise_vbe_for_current(candidate_key: &str, current_a: f64) -> Option<f64> {
    let anchors = bjt_transfer_electrical_anchors()
        .into_iter()
        .filter(|anchor| anchor.candidate_key == candidate_key)
        .collect::<Vec<_>>();
    if anchors.is_empty()
        || current_a < anchors.first()?.current_a
        || current_a > anchors.last()?.current_a
    {
        return None;
    }
    for pair in anchors.windows(2) {
        let lower = pair[0];
        let upper = pair[1];
        if current_a >= lower.current_a && current_a <= upper.current_a {
            let log_fraction = (current_a.ln() - lower.current_a.ln())
                / (upper.current_a.ln() - lower.current_a.ln());
            return Some(lower.vbe_v + log_fraction * (upper.vbe_v - lower.vbe_v));
        }
    }
    anchors.last().map(|anchor| anchor.vbe_v)
}

fn bjt_piecewise_transfer_validation(
    candidate_key: &'static str,
) -> BjtPiecewiseTransferValidation {
    let anchors = bjt_transfer_electrical_anchors()
        .into_iter()
        .filter(|anchor| anchor.candidate_key == candidate_key)
        .collect::<Vec<_>>();
    let anchors_strictly_monotonic = anchors
        .windows(2)
        .all(|pair| pair[1].current_a > pair[0].current_a && pair[1].vbe_v > pair[0].vbe_v);
    let maximum_anchor_round_trip_error_mv = anchors
        .iter()
        .map(|anchor| {
            (piecewise_vbe_for_current(candidate_key, anchor.current_a).unwrap() - anchor.vbe_v)
                .abs()
                * 1_000.0
        })
        .fold(0.0_f64, f64::max);
    BjtPiecewiseTransferValidation {
        candidate_key,
        current_kind: anchors[0].current_kind,
        anchor_count: anchors.len(),
        minimum_current_a: anchors.first().unwrap().current_a,
        maximum_current_a: anchors.last().unwrap().current_a,
        minimum_vbe_v: anchors.first().unwrap().vbe_v,
        maximum_vbe_v: anchors.last().unwrap().vbe_v,
        anchors_strictly_monotonic,
        maximum_anchor_round_trip_error_mv,
        extrapolation_permitted: false,
        interpretation: "piecewise-linear VBE interpolation in log-current space preserves the digitized typical curve; evaluation outside its source range is rejected",
    }
}

fn bjt_piecewise_transfer_validations() -> [BjtPiecewiseTransferValidation; 4] {
    [
        bjt_piecewise_transfer_validation("2SA1015-GR/Y"),
        bjt_piecewise_transfer_validation("2SA1115-F"),
        bjt_piecewise_transfer_validation("2SC1815-GR/Y"),
        bjt_piecewise_transfer_validation("2SC2603-F"),
    ]
}

fn bjt_forward_active_envelope(candidate_key: &'static str) -> BjtForwardActiveEnvelope {
    const CONSERVATIVE_NETWORK_PEAK_CURRENT_A: f64 = 30.0 / 2_200.0;
    let validation = bjt_piecewise_transfer_validation(candidate_key);
    let evidence = candidate_bipolar_model_evidence()
        .into_iter()
        .find(|record| record.candidate_key == candidate_key)
        .expect("every transfer curve must have matching rank evidence");
    let beta_geometric_center = (evidence.hfe_minimum * evidence.hfe_maximum).sqrt();
    let (common_collector_current_minimum_a, common_collector_current_maximum_a) =
        match validation.current_kind {
            TransferCurrentKind::Base => (
                validation.minimum_current_a * evidence.hfe_maximum,
                validation.maximum_current_a * evidence.hfe_minimum,
            ),
            TransferCurrentKind::Collector => {
                (validation.minimum_current_a, validation.maximum_current_a)
            }
        };
    let maximum_vbe_reading_uncertainty_mv = bjt_transfer_electrical_anchors()
        .iter()
        .filter(|anchor| anchor.candidate_key == candidate_key)
        .map(|anchor| anchor.vbe_reading_uncertainty_mv)
        .fold(0.0_f64, f64::max);
    BjtForwardActiveEnvelope {
        candidate_key,
        current_kind: validation.current_kind,
        beta_minimum: evidence.hfe_minimum,
        beta_geometric_center,
        beta_maximum: evidence.hfe_maximum,
        digitized_source_current_minimum_a: validation.minimum_current_a,
        digitized_source_current_maximum_a: validation.maximum_current_a,
        common_collector_current_minimum_a,
        common_collector_current_maximum_a,
        conservative_network_peak_current_a: CONSERVATIVE_NETWORK_PEAK_CURRENT_A,
        digitized_domain_covers_network_peak: common_collector_current_maximum_a
            >= CONSERVATIVE_NETWORK_PEAK_CURRENT_A,
        maximum_vbe_reading_uncertainty_mv,
        beta_rank_bounds_available: true,
        beta_rank_applied_outside_test_point_is_hypothesis: true,
        unit_vbe_spread_available: false,
        cutoff_model_ready: false,
        saturation_model_ready: false,
        early_effect_model_ready: false,
        preliminary_forward_active_domain_ready: validation.anchors_strictly_monotonic,
        full_dc_model_ready: false,
        interpretation: "rank beta and digitization corners define only the plotted forward-active domain; cutoff, saturation, Early effect and unit VBE spread remain open",
    }
}

fn bjt_forward_active_envelopes() -> [BjtForwardActiveEnvelope; 4] {
    [
        bjt_forward_active_envelope("2SA1015-GR/Y"),
        bjt_forward_active_envelope("2SA1115-F"),
        bjt_forward_active_envelope("2SC1815-GR/Y"),
        bjt_forward_active_envelope("2SC2603-F"),
    ]
}

fn bjt_evidence_corners() -> Vec<BjtEvidenceCorner> {
    let mut corners = Vec::with_capacity(36);
    for envelope in bjt_forward_active_envelopes() {
        let beta_corners = [
            ("minimum", envelope.beta_minimum),
            ("geometric_center", envelope.beta_geometric_center),
            ("maximum", envelope.beta_maximum),
        ];
        let vbe_corners = [
            ("reading_low", -envelope.maximum_vbe_reading_uncertainty_mv),
            ("center", 0.0),
            ("reading_high", envelope.maximum_vbe_reading_uncertainty_mv),
        ];
        for (beta_corner, beta) in beta_corners {
            for (vbe_reading_corner, vbe_offset_mv) in vbe_corners {
                corners.push(BjtEvidenceCorner {
                    candidate_key: envelope.candidate_key,
                    beta_corner,
                    beta,
                    vbe_reading_corner,
                    vbe_offset_mv,
                    evidence_domain_only: true,
                    covers_unit_to_unit_variation: false,
                });
            }
        }
    }
    corners
}

fn evaluate_bjt_forward_active_point(
    candidate_key: &str,
    beta: f64,
    collector_current_a: f64,
    vbe_offset_mv: f64,
) -> Option<BjtForwardActivePoint> {
    if beta <= 0.0 || collector_current_a <= 0.0 {
        return None;
    }
    let calibration = plot_calibration(candidate_key);
    let source_current_a = match calibration.current_kind {
        TransferCurrentKind::Base => collector_current_a / beta,
        TransferCurrentKind::Collector => collector_current_a,
    };
    let vbe_v =
        piecewise_vbe_for_current(candidate_key, source_current_a)? + vbe_offset_mv / 1_000.0;
    Some(BjtForwardActivePoint {
        candidate_key: calibration.candidate_key,
        beta,
        collector_current_a,
        base_current_a: collector_current_a / beta,
        vbe_offset_mv,
        vbe_v,
    })
}

fn bjt_forward_active_reference_points() -> [BjtForwardActivePoint; 4] {
    bjt_forward_active_envelopes().map(|envelope| {
        let reference_current_a = (envelope.common_collector_current_minimum_a
            * envelope.common_collector_current_maximum_a)
            .sqrt();
        evaluate_bjt_forward_active_point(
            envelope.candidate_key,
            envelope.beta_geometric_center,
            reference_current_a,
            0.0,
        )
        .expect("the geometric reference must remain inside the common digitized domain")
    })
}

fn bjt_dc_region_evidence_envelopes() -> [BjtDcRegionEvidenceEnvelope; 4] {
    const CONSERVATIVE_NETWORK_PEAK_CURRENT_A: f64 = 30.0 / 2_200.0;
    candidate_bipolar_model_evidence().map(|evidence| {
        let saturation_test_collector_current_a =
            evidence.vce_saturation_test_collector_current_absolute_ma / 1_000.0;
        let saturation_test_base_current_a =
            evidence.vce_saturation_test_base_current_absolute_ma / 1_000.0;
        let local_output_admittance_typical_siemens = evidence
            .common_emitter_hoe_typical_microsiemens
            .map(|value| value * 1.0e-6);
        let local_output_resistance_typical_ohms =
            local_output_admittance_typical_siemens.map(|value| 1.0 / value);
        let local_effective_early_voltage_absolute_v = evidence
            .common_emitter_h_parameter_emitter_current_absolute_ma
            .zip(local_output_admittance_typical_siemens)
            .map(|(emitter_current_ma, output_admittance_s)| {
                emitter_current_ma / 1_000.0 / output_admittance_s
            });
        BjtDcRegionEvidenceEnvelope {
            candidate_key: evidence.candidate_key,
            temperature_c: 25.0,
            collector_cutoff_test_voltage_absolute_v: evidence.vcbo_absolute_volts,
            collector_cutoff_current_maximum_a: evidence.collector_cutoff_current_maximum_ua
                * 1.0e-6,
            emitter_cutoff_test_voltage_absolute_v: evidence.vebo_absolute_volts,
            emitter_cutoff_current_maximum_a: evidence.emitter_cutoff_current_maximum_ua
                * 1.0e-6,
            cutoff_endpoints_are_maximum_limits: true,
            cutoff_continuous_iv_ready: false,
            saturation_test_collector_current_a,
            saturation_forced_beta: saturation_test_collector_current_a
                / saturation_test_base_current_a,
            saturation_vce_typical_absolute_v: evidence
                .vce_saturation_typical_absolute_volts,
            saturation_vce_maximum_absolute_v: evidence.vce_saturation_maximum_absolute_volts,
            saturation_vbe_maximum_absolute_v: evidence.vbe_saturation_maximum_absolute_volts,
            conservative_network_peak_current_a: CONSERVATIVE_NETWORK_PEAK_CURRENT_A,
            network_peak_below_saturation_test_current: CONSERVATIVE_NETWORK_PEAK_CURRENT_A
                < saturation_test_collector_current_a,
            extend_saturation_limit_to_lower_current_is_hypothesis: true,
            saturation_continuous_iv_ready: false,
            local_output_admittance_typical_siemens,
            local_output_resistance_typical_ohms,
            local_effective_early_voltage_absolute_v,
            local_output_admittance_condition: evidence.common_emitter_h_parameter_condition,
            effective_early_voltage_is_local_proxy_not_spice_vaf: true,
            early_effect_continuous_model_ready: false,
            temperature_spread_ready: false,
            full_dc_region_model_ready: false,
            interpretation: "cutoff and saturation are published 25 C endpoint limits; h_oe supplies at most a local typical output-resistance proxy, never a continuous Early-effect or production-spread model",
        }
    })
}

const fn bjt_region_join_scale(shape: BjtRegionJoinShape, coordinate: f64) -> f64 {
    let x = coordinate.clamp(0.0, 1.0);
    match shape {
        BjtRegionJoinShape::Linear => x,
        BjtRegionJoinShape::Smoothstep => x * x * (3.0 - 2.0 * x),
    }
}

fn source_current_for_piecewise_vbe(candidate_key: &str, vbe_v: f64) -> Option<f64> {
    let anchors = bjt_transfer_electrical_anchors()
        .into_iter()
        .filter(|anchor| anchor.candidate_key == candidate_key)
        .collect::<Vec<_>>();
    if anchors.is_empty() || vbe_v < anchors.first()?.vbe_v || vbe_v > anchors.last()?.vbe_v {
        return None;
    }
    for pair in anchors.windows(2) {
        let lower = pair[0];
        let upper = pair[1];
        if vbe_v >= lower.vbe_v && vbe_v <= upper.vbe_v {
            let fraction = (vbe_v - lower.vbe_v) / (upper.vbe_v - lower.vbe_v);
            return Some(
                (lower.current_a.ln() + fraction * (upper.current_a.ln() - lower.current_a.ln()))
                    .exp(),
            );
        }
    }
    anchors.last().map(|anchor| anchor.current_a)
}

fn bjt_continuous_dc_hypotheses() -> Vec<BjtContinuousDcHypothesis> {
    let mut hypotheses = Vec::with_capacity(96);
    for envelope in bjt_dc_region_evidence_envelopes() {
        let saturation_corners: &[BjtSaturationVoltageCorner] =
            if envelope.saturation_vce_typical_absolute_v.is_some() {
                &[
                    BjtSaturationVoltageCorner::DirectTypicalCurve,
                    BjtSaturationVoltageCorner::PublishedMaximumUpperBound,
                ]
            } else {
                &[
                    BjtSaturationVoltageCorner::ZeroLowerBound,
                    BjtSaturationVoltageCorner::PublishedMaximumUpperBound,
                ]
            };
        let output_corners: &[BjtOutputConductanceCorner] =
            if envelope.local_output_admittance_typical_siemens.is_some() {
                &[
                    BjtOutputConductanceCorner::Zero,
                    BjtOutputConductanceCorner::LocalTypicalHeldConstant,
                ]
            } else {
                &[BjtOutputConductanceCorner::Zero]
            };
        for cutoff_join_shape in [BjtRegionJoinShape::Linear, BjtRegionJoinShape::Smoothstep] {
            for cutoff_leakage_corner in [
                BjtCutoffLeakageCorner::Zero,
                BjtCutoffLeakageCorner::PublishedMaximum,
            ] {
                for saturation_join_shape in
                    [BjtRegionJoinShape::Linear, BjtRegionJoinShape::Smoothstep]
                {
                    for &saturation_voltage_corner in saturation_corners {
                        for &output_conductance_corner in output_corners {
                            let index = hypotheses.len();
                            hypotheses.push(BjtContinuousDcHypothesis {
                                index,
                                candidate_key: envelope.candidate_key,
                                cutoff_join_shape,
                                cutoff_leakage_corner,
                                saturation_join_shape,
                                saturation_voltage_corner,
                                output_conductance_corner,
                                cutoff_uses_published_maximum_as_upper_corner_only:
                                    cutoff_leakage_corner
                                        == BjtCutoffLeakageCorner::PublishedMaximum,
                                saturation_typical_is_direct_toshiba_curve:
                                    saturation_voltage_corner
                                        == BjtSaturationVoltageCorner::DirectTypicalCurve,
                                saturation_maximum_extended_below_test_current_is_hypothesis:
                                    saturation_voltage_corner
                                        == BjtSaturationVoltageCorner::PublishedMaximumUpperBound,
                                local_output_conductance_held_away_from_test_point_is_hypothesis:
                                    output_conductance_corner
                                        == BjtOutputConductanceCorner::LocalTypicalHeldConstant,
                                covers_unit_to_unit_variation: false,
                                suitable_as_confirmed_device_model: false,
                            });
                        }
                    }
                }
            }
        }
    }
    hypotheses
}

fn bjt_continuous_dc_hypothesis(index: usize) -> BjtContinuousDcHypothesis {
    *bjt_continuous_dc_hypotheses()
        .get(index)
        .expect("continuous BJT hypothesis index is stable")
}

fn evaluate_bjt_continuous_dc_point(
    hypothesis_index: usize,
    beta: f64,
    base_emitter_voltage_absolute_v: f64,
    collector_emitter_voltage_absolute_v: f64,
) -> Option<BjtContinuousDcPoint> {
    if !beta.is_finite()
        || beta <= 0.0
        || !base_emitter_voltage_absolute_v.is_finite()
        || base_emitter_voltage_absolute_v < 0.0
        || !collector_emitter_voltage_absolute_v.is_finite()
        || collector_emitter_voltage_absolute_v < 0.0
    {
        return None;
    }
    let hypothesis = bjt_continuous_dc_hypothesis(hypothesis_index);
    let transfer = bjt_piecewise_transfer_validation(hypothesis.candidate_key);
    if base_emitter_voltage_absolute_v > transfer.maximum_vbe_v {
        return None;
    }
    let envelope = bjt_dc_region_evidence_envelopes()
        .into_iter()
        .find(|candidate| candidate.candidate_key == hypothesis.candidate_key)
        .unwrap();
    let cutoff_leakage_corner_a = match hypothesis.cutoff_leakage_corner {
        BjtCutoffLeakageCorner::Zero => 0.0,
        BjtCutoffLeakageCorner::PublishedMaximum => envelope.collector_cutoff_current_maximum_a,
    };
    let minimum_forward_active_collector_current_a = match transfer.current_kind {
        TransferCurrentKind::Base => transfer.minimum_current_a * beta,
        TransferCurrentKind::Collector => transfer.minimum_current_a,
    };
    let (forward_active_center_collector_current_a, mut region) = if base_emitter_voltage_absolute_v
        < transfer.minimum_vbe_v
    {
        let coordinate = if transfer.minimum_vbe_v == 0.0 {
            1.0
        } else {
            base_emitter_voltage_absolute_v / transfer.minimum_vbe_v
        };
        let scale = bjt_region_join_scale(hypothesis.cutoff_join_shape, coordinate);
        (
            cutoff_leakage_corner_a
                + scale * (minimum_forward_active_collector_current_a - cutoff_leakage_corner_a),
            BjtContinuousDcRegion::CutoffJoin,
        )
    } else {
        let source_current_a = source_current_for_piecewise_vbe(
            hypothesis.candidate_key,
            base_emitter_voltage_absolute_v,
        )?;
        (
            match transfer.current_kind {
                TransferCurrentKind::Base => source_current_a * beta,
                TransferCurrentKind::Collector => source_current_a,
            },
            BjtContinuousDcRegion::ForwardActive,
        )
    };
    let (saturation_voltage_absolute_v, saturation_threshold_evaluated) =
        match hypothesis.saturation_voltage_corner {
            BjtSaturationVoltageCorner::ZeroLowerBound => (0.0, true),
            BjtSaturationVoltageCorner::PublishedMaximumUpperBound => {
                (envelope.saturation_vce_maximum_absolute_v, true)
            }
            BjtSaturationVoltageCorner::DirectTypicalCurve => {
                let direct = evaluate_bjt_thermal_characteristic_point(
                    hypothesis.candidate_key,
                    BjtThermalCurveKind::CollectorEmitterSaturationVoltage,
                    25.0,
                    forward_active_center_collector_current_a,
                );
                match direct {
                    Some(point) => (point.value, true),
                    None if collector_emitter_voltage_absolute_v
                        >= envelope.saturation_vce_maximum_absolute_v =>
                    {
                        (0.0, false)
                    }
                    None => return None,
                }
            }
        };
    let saturation_current_scale = if saturation_voltage_absolute_v == 0.0 {
        if collector_emitter_voltage_absolute_v == 0.0 {
            0.0
        } else {
            1.0
        }
    } else {
        bjt_region_join_scale(
            hypothesis.saturation_join_shape,
            collector_emitter_voltage_absolute_v / saturation_voltage_absolute_v,
        )
    };
    if saturation_current_scale < 1.0 {
        region = BjtContinuousDcRegion::SaturationJoin;
    }
    let output_conductance_correction_a = if hypothesis.output_conductance_corner
        == BjtOutputConductanceCorner::LocalTypicalHeldConstant
        && region == BjtContinuousDcRegion::ForwardActive
    {
        let reference_vce = 6.0;
        envelope.local_output_admittance_typical_siemens.unwrap()
            * (collector_emitter_voltage_absolute_v - reference_vce)
    } else {
        0.0
    };
    let collector_current_a = (forward_active_center_collector_current_a
        * saturation_current_scale
        + output_conductance_correction_a)
        .max(0.0);
    let base_current_a = collector_current_a / beta;
    let collector_power_mw = collector_current_a * collector_emitter_voltage_absolute_v * 1_000.0;
    let collector_power_limit_mw = bjt_collector_power_limit_mw(hypothesis.candidate_key, 25.0)
        .expect("all BJT candidates publish a 25 C collector-power limit");
    Some(BjtContinuousDcPoint {
        hypothesis_index,
        candidate_key: hypothesis.candidate_key,
        region,
        beta,
        base_emitter_voltage_absolute_v,
        collector_emitter_voltage_absolute_v,
        forward_active_center_collector_current_a,
        cutoff_leakage_corner_a,
        saturation_voltage_absolute_v,
        saturation_threshold_evaluated,
        saturation_current_scale,
        output_conductance_correction_a,
        collector_current_a,
        base_current_a,
        collector_power_mw,
        collector_power_inside_25c_published_limit: collector_power_mw <= collector_power_limit_mw,
        evidence_and_hypothesis_boundary: "25 C digitized forward-active center joined by explicit cutoff/saturation shapes; leakage maximum, forced-beta saturation, and local h_oe retain their published endpoint semantics",
    })
}

fn bjt_continuous_dc_reference_points() -> Vec<BjtContinuousDcPoint> {
    bjt_continuous_dc_hypotheses()
        .iter()
        .map(|hypothesis| {
            let envelope = bjt_forward_active_envelope(hypothesis.candidate_key);
            let transfer = bjt_piecewise_transfer_validation(hypothesis.candidate_key);
            let source_current_a = (transfer.minimum_current_a * transfer.maximum_current_a).sqrt();
            let vbe_v = piecewise_vbe_for_current(hypothesis.candidate_key, source_current_a)
                .expect("geometric source current lies inside the direct transfer curve");
            evaluate_bjt_continuous_dc_point(
                hypothesis.index,
                envelope.beta_geometric_center,
                vbe_v,
                6.0,
            )
            .expect("every reference is forward active and inside its source curves")
        })
        .collect()
}

fn bjt_continuous_dc_validation(candidate_key: &'static str) -> BjtContinuousDcValidation {
    let hypotheses = bjt_continuous_dc_hypotheses()
        .into_iter()
        .filter(|hypothesis| hypothesis.candidate_key == candidate_key)
        .collect::<Vec<_>>();
    let transfer = bjt_piecewise_transfer_validation(candidate_key);
    let forward = bjt_forward_active_envelope(candidate_key);
    let evidence = bjt_dc_region_evidence_envelopes()
        .into_iter()
        .find(|candidate| candidate.candidate_key == candidate_key)
        .unwrap();
    let vbe_samples = [
        0.0,
        transfer.minimum_vbe_v * 0.5,
        transfer.minimum_vbe_v,
        (transfer.minimum_vbe_v + transfer.maximum_vbe_v) * 0.5,
        transfer.maximum_vbe_v,
    ];
    let vce_samples = [
        0.0,
        evidence.saturation_vce_maximum_absolute_v * 0.5,
        evidence.saturation_vce_maximum_absolute_v,
        6.0,
    ];
    let mut points = Vec::with_capacity(hypotheses.len() * vbe_samples.len() * vce_samples.len());
    for hypothesis in &hypotheses {
        for vbe_v in vbe_samples {
            for vce_v in vce_samples {
                if let Some(point) = evaluate_bjt_continuous_dc_point(
                    hypothesis.index,
                    forward.beta_geometric_center,
                    vbe_v,
                    vce_v,
                ) {
                    points.push(point);
                }
            }
        }
    }
    let every_point_finite_and_nonnegative = points.iter().all(|point| {
        point.collector_current_a.is_finite()
            && point.collector_current_a >= 0.0
            && point.base_current_a.is_finite()
            && point.base_current_a >= 0.0
            && point.saturation_current_scale.is_finite()
            && (0.0..=1.0).contains(&point.saturation_current_scale)
    });
    let cutoff_endpoints_respect_zero_to_published_maximum = hypotheses.iter().all(|hypothesis| {
        let point = evaluate_bjt_continuous_dc_point(
            hypothesis.index,
            forward.beta_geometric_center,
            0.0,
            6.0,
        )
        .unwrap();
        point.collector_current_a >= 0.0
            && point.collector_current_a <= evidence.collector_cutoff_current_maximum_a
    });
    let cutoff_to_forward_active_join_is_continuous = hypotheses.iter().all(|hypothesis| {
        let below = evaluate_bjt_continuous_dc_point(
            hypothesis.index,
            forward.beta_geometric_center,
            transfer.minimum_vbe_v * (1.0 - 1.0e-9),
            6.0,
        )
        .unwrap();
        let at = evaluate_bjt_continuous_dc_point(
            hypothesis.index,
            forward.beta_geometric_center,
            transfer.minimum_vbe_v,
            6.0,
        )
        .unwrap();
        (below.collector_current_a - at.collector_current_a).abs()
            <= at.collector_current_a.max(1.0e-12) * 1.0e-8
    });
    let middle_vbe = (transfer.minimum_vbe_v + transfer.maximum_vbe_v) * 0.5;
    let saturation_current_is_monotonic_with_vce = hypotheses.iter().all(|hypothesis| {
        let values = [0.0, 0.1, 0.2, 0.3, 6.0]
            .iter()
            .filter_map(|vce_v| {
                evaluate_bjt_continuous_dc_point(
                    hypothesis.index,
                    forward.beta_geometric_center,
                    middle_vbe,
                    *vce_v,
                )
                .map(|point| point.collector_current_a)
            })
            .collect::<Vec<_>>();
        !values.is_empty() && values.windows(2).all(|pair| pair[1] >= pair[0])
    });
    let direct_typical_saturation_never_extrapolates = hypotheses
        .iter()
        .filter(|hypothesis| {
            hypothesis.saturation_voltage_corner == BjtSaturationVoltageCorner::DirectTypicalCurve
        })
        .all(|hypothesis| {
            evaluate_bjt_continuous_dc_point(
                hypothesis.index,
                forward.beta_geometric_center,
                0.0,
                0.0,
            )
            .is_none()
        });
    let published_maximum_extension_is_labeled_hypothesis = hypotheses
        .iter()
        .filter(|hypothesis| {
            hypothesis.saturation_voltage_corner
                == BjtSaturationVoltageCorner::PublishedMaximumUpperBound
        })
        .all(|hypothesis| hypothesis.saturation_maximum_extended_below_test_current_is_hypothesis);
    let local_hoe_never_promoted_to_global_early_voltage = hypotheses
        .iter()
        .filter(|hypothesis| {
            hypothesis.output_conductance_corner
                == BjtOutputConductanceCorner::LocalTypicalHeldConstant
        })
        .all(|hypothesis| {
            hypothesis.local_output_conductance_held_away_from_test_point_is_hypothesis
                && !hypothesis.suitable_as_confirmed_device_model
        });
    let preliminary_region_sweep_ready = every_point_finite_and_nonnegative
        && cutoff_endpoints_respect_zero_to_published_maximum
        && cutoff_to_forward_active_join_is_continuous
        && saturation_current_is_monotonic_with_vce
        && direct_typical_saturation_never_extrapolates
        && published_maximum_extension_is_labeled_hypothesis
        && local_hoe_never_promoted_to_global_early_voltage;
    BjtContinuousDcValidation {
        candidate_key,
        hypothesis_count: hypotheses.len(),
        evaluated_grid_points: points.len(),
        every_point_finite_and_nonnegative,
        cutoff_endpoints_respect_zero_to_published_maximum,
        cutoff_to_forward_active_join_is_continuous,
        saturation_current_is_monotonic_with_vce,
        direct_typical_saturation_never_extrapolates,
        published_maximum_extension_is_labeled_hypothesis,
        local_hoe_never_promoted_to_global_early_voltage,
        preliminary_region_sweep_ready,
        bounded_production_device_model_ready: false,
        interpretation: "continuous 25 C join family for sensitivity sweeps only; endpoint maxima, direct typical curves and local h_oe remain distinct, while unit spread and complete device physics remain unavailable",
    }
}

fn bjt_continuous_dc_validations() -> [BjtContinuousDcValidation; 4] {
    [
        bjt_continuous_dc_validation("2SA1015-GR/Y"),
        bjt_continuous_dc_validation("2SA1115-F"),
        bjt_continuous_dc_validation("2SC1815-GR/Y"),
        bjt_continuous_dc_validation("2SC2603-F"),
    ]
}

const ACTIVE_NETWORK_DC_UNKNOWNS: [ActiveNetworkNode; 13] = [
    ActiveNetworkNode::LocalNegativeRail,
    ActiveNetworkNode::InputEmitterNode,
    ActiveNetworkNode::PnpSwitchBase,
    ActiveNetworkNode::PnpSwitchCollector,
    ActiveNetworkNode::PnpBiasBase,
    ActiveNetworkNode::PnpBiasEmitter,
    ActiveNetworkNode::DiodeBase,
    ActiveNetworkNode::Control,
    ActiveNetworkNode::Timing,
    ActiveNetworkNode::LowerNpnBase,
    ActiveNetworkNode::OscillatorFeedback,
    ActiveNetworkNode::FeedbackBase,
    ActiveNetworkNode::FeedbackEmitter,
];

#[derive(Clone, Debug)]
struct ActiveNetworkDcEvaluation {
    residuals_a: Vec<f64>,
    device_points: Vec<ActiveNetworkDcDevicePoint>,
    diode_forward_voltage_v: f64,
    diode_current_a: f64,
    diode_curve_extension_used: bool,
    mn3101_point: Mn3101OxMacroPoint,
}

fn active_network_dc_node_index(node: ActiveNetworkNode) -> Option<usize> {
    ACTIVE_NETWORK_DC_UNKNOWNS
        .iter()
        .position(|candidate| *candidate == node)
}

fn active_network_dc_voltage(
    node: ActiveNetworkNode,
    unknown_voltages_v: &[f64],
    triangle_input_v: f64,
) -> f64 {
    match node {
        ActiveNetworkNode::PositiveSupply => 15.0,
        ActiveNetworkNode::Ground => 0.0,
        ActiveNetworkNode::NegativeSupply => -15.0,
        ActiveNetworkNode::TriangleInput => triangle_input_v,
        ActiveNetworkNode::Vgg
        | ActiveNetworkNode::ClockPhaseOne
        | ActiveNetworkNode::ClockPhaseTwo => 0.0,
        _ => {
            unknown_voltages_v[active_network_dc_node_index(node)
                .expect("every non-fixed DC node is part of the unknown vector")]
        }
    }
}

fn add_active_network_dc_current(
    residuals_a: &mut [f64],
    node: ActiveNetworkNode,
    current_leaving_a: f64,
) {
    if let Some(index) = active_network_dc_node_index(node) {
        residuals_a[index] += current_leaving_a;
    }
}

fn preliminary_diode_forward_current(forward_voltage_v: f64) -> (f64, bool) {
    let anchors = diode_forward_anchors();
    let first = anchors.first().expect("the diode curve has anchors");
    let last = anchors.last().expect("the diode curve has anchors");
    if forward_voltage_v <= 0.0 {
        return (0.0, forward_voltage_v < 0.0);
    }
    if forward_voltage_v < first.forward_voltage_v {
        return (
            first.forward_current_a * forward_voltage_v / first.forward_voltage_v,
            true,
        );
    }
    for pair in anchors.windows(2) {
        let lower = pair[0];
        let upper = pair[1];
        if forward_voltage_v <= upper.forward_voltage_v {
            let fraction = (forward_voltage_v - lower.forward_voltage_v)
                / (upper.forward_voltage_v - lower.forward_voltage_v);
            return (
                (lower.forward_current_a.ln()
                    + fraction * (upper.forward_current_a.ln() - lower.forward_current_a.ln()))
                .exp(),
                false,
            );
        }
    }
    (last.forward_current_a, true)
}

fn preliminary_bjt_hypothesis_index(candidate_key: &str) -> usize {
    bjt_continuous_dc_hypotheses()
        .into_iter()
        .find(|hypothesis| {
            hypothesis.candidate_key == candidate_key
                && hypothesis.cutoff_join_shape == BjtRegionJoinShape::Smoothstep
                && hypothesis.cutoff_leakage_corner == BjtCutoffLeakageCorner::Zero
                && hypothesis.saturation_join_shape == BjtRegionJoinShape::Smoothstep
                && hypothesis.saturation_voltage_corner
                    == BjtSaturationVoltageCorner::PublishedMaximumUpperBound
                && hypothesis.output_conductance_corner == BjtOutputConductanceCorner::Zero
        })
        .expect("every candidate has the selected preliminary region hypothesis")
        .index
}

fn active_network_dc_candidate(
    assignment: ActiveDeviceHypothesis,
    device_index: usize,
) -> &'static str {
    match device_index {
        0 => assignment.channel_a_pnp[0],
        1 => assignment.channel_a_pnp[1],
        2 => assignment.channel_a_npn[0],
        3 => assignment.channel_a_npn[1],
        4 => assignment.channel_a_npn[2],
        _ => unreachable!("the mirrored active cell has exactly five BJTs"),
    }
}

fn evaluate_active_network_dc_residuals_with_base_drive_and_low_vce(
    coordinate: ActiveNetworkDcCoordinate,
    unknown_voltages_v: &[f64],
    saturation_base_drive_blend: f64,
    low_vce_base_drive_hypothesis: Option<BjtLowVceBaseDriveHypothesis>,
    reciprocal_transport_parameters: Option<BjtReciprocalTransportParameters>,
) -> Option<ActiveNetworkDcEvaluation> {
    if unknown_voltages_v.len() != ACTIVE_NETWORK_DC_UNKNOWNS.len()
        || unknown_voltages_v.iter().any(|value| !value.is_finite())
        || !saturation_base_drive_blend.is_finite()
        || !(0.0..=1.0).contains(&saturation_base_drive_blend)
        || (low_vce_base_drive_hypothesis.is_some() && reciprocal_transport_parameters.is_some())
        || (reciprocal_transport_parameters.is_some() && saturation_base_drive_blend != 0.0)
    {
        return None;
    }
    let mut residuals_a = vec![0.0; ACTIVE_NETWORK_DC_UNKNOWNS.len()];
    for element in mirrored_active_network_elements() {
        if element.kind != NetlistElementKind::Resistor {
            continue;
        }
        let resistance_ohms = element.nominal?;
        let terminal_1_v = active_network_dc_voltage(
            element.terminal_1,
            unknown_voltages_v,
            coordinate.triangle_input_v,
        );
        let terminal_2_v = active_network_dc_voltage(
            element.terminal_2,
            unknown_voltages_v,
            coordinate.triangle_input_v,
        );
        let current_1_to_2_a = (terminal_1_v - terminal_2_v) / resistance_ohms;
        add_active_network_dc_current(&mut residuals_a, element.terminal_1, current_1_to_2_a);
        add_active_network_dc_current(&mut residuals_a, element.terminal_2, -current_1_to_2_a);
    }

    let assignment = *active_device_hypotheses().get(coordinate.active_device_assignment_index)?;
    let mut device_points = Vec::with_capacity(5);
    for (device_index, connection) in mirrored_bipolar_connections().into_iter().enumerate() {
        let candidate_key = active_network_dc_candidate(assignment, device_index);
        let hypothesis_index = preliminary_bjt_hypothesis_index(candidate_key);
        let forward = bjt_forward_active_envelope(candidate_key);
        let transfer = bjt_piecewise_transfer_validation(candidate_key);
        let collector_v = active_network_dc_voltage(
            connection.collector,
            unknown_voltages_v,
            coordinate.triangle_input_v,
        );
        let base_v = active_network_dc_voltage(
            connection.base,
            unknown_voltages_v,
            coordinate.triangle_input_v,
        );
        let emitter_v = active_network_dc_voltage(
            connection.emitter,
            unknown_voltages_v,
            coordinate.triangle_input_v,
        );
        let (raw_vbe_v, raw_vce_v) = match connection.polarity {
            BipolarPolarity::Npn => (base_v - emitter_v, collector_v - emitter_v),
            BipolarPolarity::Pnp => (emitter_v - base_v, emitter_v - collector_v),
        };
        let transfer_voltage_was_clamped_to_digitized_domain = raw_vbe_v > transfer.maximum_vbe_v;
        let model_vbe_v = raw_vbe_v.clamp(0.0, transfer.maximum_vbe_v);
        let model_vce_v = raw_vce_v.max(0.0);
        let beta = forward.beta_geometric_center;
        let point =
            evaluate_bjt_continuous_dc_point(hypothesis_index, beta, model_vbe_v, model_vce_v)?;
        let raw_vbc_v = raw_vbe_v - raw_vce_v;
        let reverse_transfer_voltage_was_clamped_to_digitized_domain =
            raw_vbc_v > transfer.maximum_vbe_v;
        let collector_emitter_polarity_valid = if reciprocal_transport_parameters.is_some() {
            !reverse_transfer_voltage_was_clamped_to_digitized_domain
        } else {
            raw_vce_v >= -1.0e-9
        };
        let model_vbc_v = raw_vbc_v.clamp(0.0, transfer.maximum_vbe_v);
        let forward_beta_base_current_a = point.base_current_a;
        let forced_beta_ten_base_current_a = point.collector_current_a / 10.0;
        let saturation_weight = 1.0 - point.saturation_current_scale;
        let (
            collector_current_a,
            base_current_before_low_vce_a,
            reverse_transport_current_a,
            reciprocal_reverse_beta,
            collector_power_mw,
            inside_published_power_limit,
        ) = if let Some(parameters) = reciprocal_transport_parameters {
            let forward_transport_current_a =
                evaluate_bjt_continuous_dc_point(hypothesis_index, beta, model_vbe_v, 6.0)?
                    .collector_current_a;
            let reverse_transport_current_a = if model_vbc_v > 0.0 {
                evaluate_bjt_continuous_dc_point(hypothesis_index, beta, model_vbc_v, 6.0)?
                    .collector_current_a
            } else {
                0.0
            };
            let reverse_beta = match (connection.channel_a_designator, candidate_key) {
                ("Tr22", "2SC1815-GR/Y") => parameters.tr22_toshiba_reverse_beta,
                ("Tr23", "2SC1815-GR/Y") => parameters.tr23_toshiba_reverse_beta,
                _ => parameters.default_reverse_beta,
            };
            let reverse_alpha = reverse_beta / (reverse_beta + 1.0);
            let output_conductance_current_a =
                if connection.channel_a_designator == "Tr19" && candidate_key == "2SA1015-GR/Y" {
                    parameters.tr19_toshiba_differential_conductance_us
                        * 1.0e-6
                        * (raw_vce_v.abs() - 6.0).max(0.0)
                } else {
                    0.0
                };
            let collector_current_a = forward_transport_current_a
                - reverse_transport_current_a / reverse_alpha
                + output_conductance_current_a;
            let base_current_a =
                forward_transport_current_a / beta + reverse_transport_current_a / reverse_beta;
            let collector_power_mw = collector_current_a.abs() * raw_vce_v.abs() * 1_000.0;
            let collector_power_limit_mw = bjt_collector_power_limit_mw(candidate_key, 25.0)?;
            (
                collector_current_a,
                base_current_a,
                reverse_transport_current_a,
                reverse_beta,
                collector_power_mw,
                collector_power_mw <= collector_power_limit_mw,
            )
        } else {
            (
                point.collector_current_a,
                forward_beta_base_current_a
                    + saturation_base_drive_blend
                        * saturation_weight
                        * (forced_beta_ten_base_current_a - forward_beta_base_current_a),
                0.0,
                0.0,
                point.collector_power_mw,
                point.collector_power_inside_25c_published_limit,
            )
        };
        let low_vce_evaluation = low_vce_base_drive_hypothesis.map(|hypothesis| {
            evaluate_bjt_low_vce_base_drive(
                candidate_key,
                model_vce_v,
                base_current_before_low_vce_a,
                hypothesis,
            )
        });
        let base_current_a = low_vce_evaluation
            .map(|evaluation| evaluation.base_current_a)
            .unwrap_or(base_current_before_low_vce_a);
        let collector_current_leaving_a = match connection.polarity {
            BipolarPolarity::Npn => collector_current_a,
            BipolarPolarity::Pnp => -collector_current_a,
        };
        let base_current_leaving_a = match connection.polarity {
            BipolarPolarity::Npn => base_current_a,
            BipolarPolarity::Pnp => -base_current_a,
        };
        add_active_network_dc_current(
            &mut residuals_a,
            connection.collector,
            collector_current_leaving_a,
        );
        add_active_network_dc_current(&mut residuals_a, connection.base, base_current_leaving_a);
        add_active_network_dc_current(
            &mut residuals_a,
            connection.emitter,
            -(collector_current_leaving_a + base_current_leaving_a),
        );
        device_points.push(ActiveNetworkDcDevicePoint {
            designator: connection.channel_a_designator,
            candidate_key,
            continuous_hypothesis_index: hypothesis_index,
            beta,
            region: point.region,
            base_emitter_voltage_absolute_v: raw_vbe_v,
            collector_emitter_voltage_absolute_v: raw_vce_v,
            collector_current_a,
            base_current_a,
            forward_beta_base_current_a,
            forced_beta_ten_base_current_a,
            saturation_base_drive_blend,
            forced_beta_ten_is_datasheet_test_condition_not_device_law: true,
            low_vce_base_drive_hypothesis_index: low_vce_base_drive_hypothesis
                .map(|hypothesis| hypothesis.index),
            low_vce_guard_voltage_v: low_vce_evaluation
                .map(|evaluation| evaluation.guard_voltage_v)
                .unwrap_or(0.0),
            low_vce_proximity_weight: low_vce_evaluation
                .map(|evaluation| evaluation.proximity_weight)
                .unwrap_or(0.0),
            low_vce_first_labeled_base_current_a: low_vce_evaluation
                .map(|evaluation| evaluation.first_labeled_base_current_a)
                .unwrap_or(0.0),
            low_vce_base_current_adjustment_a: low_vce_evaluation
                .map(|evaluation| evaluation.base_current_adjustment_a)
                .unwrap_or(0.0),
            reciprocal_transport_hypothesis_index: reciprocal_transport_parameters
                .and_then(|parameters| parameters.hypothesis_index),
            base_collector_voltage_absolute_v: raw_vbc_v,
            reciprocal_reverse_beta,
            reverse_transport_current_a,
            reverse_transfer_voltage_was_clamped_to_digitized_domain,
            collector_power_mw,
            transfer_voltage_was_clamped_to_digitized_domain,
            collector_emitter_polarity_valid,
            inside_published_power_limit,
        });
    }

    let diode_cathode_v = active_network_dc_voltage(
        ActiveNetworkNode::DiodeBase,
        unknown_voltages_v,
        coordinate.triangle_input_v,
    );
    let diode_anode_v = active_network_dc_voltage(
        ActiveNetworkNode::Control,
        unknown_voltages_v,
        coordinate.triangle_input_v,
    );
    let diode_forward_voltage_v = diode_anode_v - diode_cathode_v;
    let (diode_current_a, diode_curve_extension_used) =
        preliminary_diode_forward_current(diode_forward_voltage_v);
    add_active_network_dc_current(
        &mut residuals_a,
        ActiveNetworkNode::Control,
        diode_current_a,
    );
    add_active_network_dc_current(
        &mut residuals_a,
        ActiveNetworkNode::DiodeBase,
        -diode_current_a,
    );

    let mn3101_hypothesis =
        *mn3101_ox_macro_hypotheses().get(coordinate.mn3101_macro_hypothesis_index)?;
    let ox1_input_v = active_network_dc_voltage(
        ActiveNetworkNode::FeedbackBase,
        unknown_voltages_v,
        coordinate.triangle_input_v,
    )
    .clamp(-15.0, 0.0);
    let ox2_node_v = active_network_dc_voltage(
        ActiveNetworkNode::OscillatorFeedback,
        unknown_voltages_v,
        coordinate.triangle_input_v,
    )
    .clamp(-15.0, 0.0);
    let ox3_node_v = active_network_dc_voltage(
        ActiveNetworkNode::Control,
        unknown_voltages_v,
        coordinate.triangle_input_v,
    )
    .clamp(-15.0, 0.0);
    let mn3101_point =
        evaluate_mn3101_ox_macro(mn3101_hypothesis, ox1_input_v, ox2_node_v, ox3_node_v)?;
    add_active_network_dc_current(
        &mut residuals_a,
        ActiveNetworkNode::OscillatorFeedback,
        -mn3101_point.ox2_output_current_a,
    );
    add_active_network_dc_current(
        &mut residuals_a,
        ActiveNetworkNode::Control,
        -mn3101_point.ox3_output_current_a,
    );

    Some(ActiveNetworkDcEvaluation {
        residuals_a,
        device_points,
        diode_forward_voltage_v,
        diode_current_a,
        diode_curve_extension_used,
        mn3101_point,
    })
}

fn evaluate_active_network_dc_residuals_with_base_drive(
    coordinate: ActiveNetworkDcCoordinate,
    unknown_voltages_v: &[f64],
    saturation_base_drive_blend: f64,
) -> Option<ActiveNetworkDcEvaluation> {
    evaluate_active_network_dc_residuals_with_base_drive_and_low_vce(
        coordinate,
        unknown_voltages_v,
        saturation_base_drive_blend,
        None,
        None,
    )
}

fn evaluate_active_network_dc_residuals_with_low_vce_base_drive(
    coordinate: ActiveNetworkDcCoordinate,
    unknown_voltages_v: &[f64],
    hypothesis: BjtLowVceBaseDriveHypothesis,
) -> Option<ActiveNetworkDcEvaluation> {
    evaluate_active_network_dc_residuals_with_base_drive_and_low_vce(
        coordinate,
        unknown_voltages_v,
        0.0,
        Some(hypothesis),
        None,
    )
}

fn evaluate_active_network_dc_residuals_with_reciprocal_transport(
    coordinate: ActiveNetworkDcCoordinate,
    unknown_voltages_v: &[f64],
    hypothesis: BjtReciprocalTransportHypothesis,
) -> Option<ActiveNetworkDcEvaluation> {
    evaluate_active_network_dc_residuals_with_base_drive_and_low_vce(
        coordinate,
        unknown_voltages_v,
        0.0,
        None,
        Some(BjtReciprocalTransportParameters {
            hypothesis_index: Some(hypothesis.index),
            default_reverse_beta: hypothesis.reverse_beta,
            tr19_toshiba_differential_conductance_us: 0.0,
            tr22_toshiba_reverse_beta: hypothesis.reverse_beta,
            tr23_toshiba_reverse_beta: hypothesis.reverse_beta,
        }),
    )
}

fn evaluate_active_network_dc_residuals_with_correlated_reverse_transport(
    coordinate: ActiveNetworkDcCoordinate,
    unknown_voltages_v: &[f64],
    parameters: BjtReciprocalTransportParameters,
) -> Option<ActiveNetworkDcEvaluation> {
    if !parameters.default_reverse_beta.is_finite()
        || parameters.default_reverse_beta <= 0.0
        || !parameters
            .tr19_toshiba_differential_conductance_us
            .is_finite()
        || parameters.tr19_toshiba_differential_conductance_us < 0.0
        || !parameters.tr22_toshiba_reverse_beta.is_finite()
        || parameters.tr22_toshiba_reverse_beta <= 0.0
        || !parameters.tr23_toshiba_reverse_beta.is_finite()
        || parameters.tr23_toshiba_reverse_beta <= 0.0
    {
        return None;
    }
    evaluate_active_network_dc_residuals_with_base_drive_and_low_vce(
        coordinate,
        unknown_voltages_v,
        0.0,
        None,
        Some(parameters),
    )
}

fn evaluate_active_network_dc_residuals_with_reciprocal_transport_blend(
    coordinate: ActiveNetworkDcCoordinate,
    unknown_voltages_v: &[f64],
    hypothesis: BjtReciprocalTransportHypothesis,
    blend_fraction: f64,
) -> Option<ActiveNetworkDcEvaluation> {
    if !blend_fraction.is_finite() || !(0.0..=1.0).contains(&blend_fraction) {
        return None;
    }
    let source = evaluate_active_network_dc_residuals(coordinate, unknown_voltages_v)?;
    let mut target = evaluate_active_network_dc_residuals_with_reciprocal_transport(
        coordinate,
        unknown_voltages_v,
        hypothesis,
    )?;
    for (target_residual, source_residual) in target.residuals_a.iter_mut().zip(&source.residuals_a)
    {
        *target_residual =
            *source_residual + blend_fraction * (*target_residual - *source_residual);
    }
    Some(target)
}

fn evaluate_active_network_dc_residuals(
    coordinate: ActiveNetworkDcCoordinate,
    unknown_voltages_v: &[f64],
) -> Option<ActiveNetworkDcEvaluation> {
    evaluate_active_network_dc_residuals_with_base_drive(coordinate, unknown_voltages_v, 0.0)
}

fn solve_dense_linear_system(mut matrix: Vec<Vec<f64>>, mut rhs: Vec<f64>) -> Option<Vec<f64>> {
    let n = rhs.len();
    if matrix.len() != n || matrix.iter().any(|row| row.len() != n) {
        return None;
    }
    for pivot in 0..n {
        let best = (pivot..n).max_by(|&left, &right| {
            matrix[left][pivot]
                .abs()
                .total_cmp(&matrix[right][pivot].abs())
        })?;
        if matrix[best][pivot].abs() < 1.0e-14 {
            return None;
        }
        matrix.swap(pivot, best);
        rhs.swap(pivot, best);
        let diagonal = matrix[pivot][pivot];
        for value in &mut matrix[pivot][pivot..] {
            *value /= diagonal;
        }
        rhs[pivot] /= diagonal;
        let pivot_row = matrix[pivot].clone();
        for row in 0..n {
            if row == pivot {
                continue;
            }
            let scale = matrix[row][pivot];
            for (value, pivot_value) in matrix[row][pivot..].iter_mut().zip(&pivot_row[pivot..]) {
                *value -= scale * pivot_value;
            }
            rhs[row] -= scale * rhs[pivot];
        }
    }
    Some(rhs)
}

fn maximum_absolute(values: &[f64]) -> f64 {
    values.iter().map(|value| value.abs()).fold(0.0, f64::max)
}

fn active_network_dc_seed(triangle_input_v: f64, seed_index: usize) -> Vec<f64> {
    let feedback_level_v = match seed_index % 3 {
        0 => -14.0,
        1 => -7.5,
        _ => -1.0,
    };
    vec![
        -14.9,
        triangle_input_v,
        14.0,
        -2.0,
        12.7,
        13.3,
        triangle_input_v + 0.55,
        -2.0,
        -2.0,
        -13.5,
        -7.5,
        feedback_level_v,
        -14.2,
    ]
}

fn solve_active_network_dc_coordinate_with_evaluator<F>(
    coordinate: ActiveNetworkDcCoordinate,
    continuation_seed: Option<&[f64]>,
    fallback_seed_count: usize,
    maximum_iterations: usize,
    evaluate: F,
) -> ActiveNetworkDcOperatingPoint
where
    F: Fn(&[f64]) -> Option<ActiveNetworkDcEvaluation>,
{
    const ACCEPTED_RESIDUAL_A: f64 = 1.0e-7;
    const JACOBIAN_STEP_V: f64 = 1.0e-5;
    let default_seed = active_network_dc_seed(coordinate.triangle_input_v, 0);
    let initial_seed = continuation_seed.unwrap_or(&default_seed);
    let mut best_voltages = initial_seed.to_vec();
    let mut best_evaluation =
        evaluate(&best_voltages).expect("the initial operating-point seed is evaluable");
    let mut best_norm = maximum_absolute(&best_evaluation.residuals_a);
    let mut best_iterations = 0;
    let mut best_seed_index = 0;
    let mut best_used_continuation_seed = continuation_seed.is_some();
    let mut converged = false;

    let total_seed_count = fallback_seed_count + usize::from(continuation_seed.is_some());
    for seed_slot in 0..total_seed_count {
        let using_continuation_seed = continuation_seed.is_some() && seed_slot == 0;
        let fallback_seed_index =
            seed_slot.saturating_sub(usize::from(continuation_seed.is_some()));
        let mut voltages = if using_continuation_seed {
            continuation_seed.unwrap().to_vec()
        } else {
            active_network_dc_seed(coordinate.triangle_input_v, fallback_seed_index)
        };
        for iteration in 0..maximum_iterations {
            let evaluation = match evaluate(&voltages) {
                Some(value) => value,
                None => break,
            };
            let norm = maximum_absolute(&evaluation.residuals_a);
            if norm < best_norm {
                best_norm = norm;
                best_voltages.clone_from(&voltages);
                best_evaluation = evaluation.clone();
                best_iterations = iteration;
                best_seed_index = fallback_seed_index;
                best_used_continuation_seed = using_continuation_seed;
            }
            if norm <= ACCEPTED_RESIDUAL_A {
                converged = true;
                break;
            }
            let n = voltages.len();
            let mut jacobian = vec![vec![0.0; n]; n];
            for column in 0..n {
                let mut shifted = voltages.clone();
                shifted[column] += JACOBIAN_STEP_V;
                let shifted_evaluation = match evaluate(&shifted) {
                    Some(value) => value,
                    None => continue,
                };
                for (row, jacobian_row) in jacobian.iter_mut().enumerate() {
                    jacobian_row[column] = (shifted_evaluation.residuals_a[row]
                        - evaluation.residuals_a[row])
                        / JACOBIAN_STEP_V;
                }
            }
            let rhs = evaluation
                .residuals_a
                .iter()
                .map(|residual| -residual)
                .collect::<Vec<_>>();
            let Some(step) = solve_dense_linear_system(jacobian, rhs) else {
                break;
            };
            let mut accepted_trial = None;
            let mut scale = 1.0;
            for _ in 0..12 {
                let trial = voltages
                    .iter()
                    .zip(&step)
                    .map(|(voltage, delta)| (voltage + scale * delta).clamp(-18.0, 18.0))
                    .collect::<Vec<_>>();
                if let Some(trial_evaluation) = evaluate(&trial)
                    && maximum_absolute(&trial_evaluation.residuals_a) < norm
                {
                    accepted_trial = Some(trial);
                    break;
                }
                scale *= 0.5;
            }
            let Some(trial) = accepted_trial else {
                break;
            };
            voltages = trial;
        }
        if converged {
            break;
        }
    }

    let residual_square_mean = best_evaluation
        .residuals_a
        .iter()
        .map(|residual| residual * residual)
        .sum::<f64>()
        / best_evaluation.residuals_a.len() as f64;
    let node_voltages = ACTIVE_NETWORK_DC_UNKNOWNS
        .iter()
        .zip(&best_voltages)
        .map(|(&node, &voltage_v)| ActiveNetworkDcNodeVoltage { node, voltage_v })
        .collect::<Vec<_>>();
    let every_unknown_inside_supply_window = best_voltages
        .iter()
        .all(|voltage| (-15.001..=15.001).contains(voltage));
    let every_bjt_vce_polarity_valid = best_evaluation
        .device_points
        .iter()
        .all(|point| point.collector_emitter_polarity_valid);
    let every_bjt_inside_published_power_limit = best_evaluation
        .device_points
        .iter()
        .all(|point| point.inside_published_power_limit);
    ActiveNetworkDcOperatingPoint {
        coordinate,
        converged: converged || best_norm <= ACCEPTED_RESIDUAL_A,
        iterations: best_iterations,
        seed_index: best_seed_index,
        used_continuation_seed: best_used_continuation_seed,
        maximum_kcl_residual_a: best_norm,
        rms_kcl_residual_a: residual_square_mean.sqrt(),
        node_voltages,
        device_points: best_evaluation.device_points,
        diode_forward_voltage_v: best_evaluation.diode_forward_voltage_v,
        diode_current_a: best_evaluation.diode_current_a,
        diode_curve_extension_used: best_evaluation.diode_curve_extension_used,
        mn3101_point: Some(best_evaluation.mn3101_point),
        every_unknown_inside_supply_window,
        every_bjt_vce_polarity_valid,
        every_bjt_inside_published_power_limit,
        suitable_only_as_preliminary_fixed_point: true,
        suitable_as_confirmed_physical_operating_point: false,
        interpretation: "Newton fixed point of the audited resistor/BJT/diode/OX graph; capacitors are open, TP3 is an explicit probe, device centers are hypotheses and MN3101 hidden supply current is unavailable",
    }
}

fn solve_active_network_dc_coordinate_with_seed(
    coordinate: ActiveNetworkDcCoordinate,
    continuation_seed: Option<&[f64]>,
    fallback_seed_count: usize,
) -> ActiveNetworkDcOperatingPoint {
    solve_active_network_dc_coordinate_with_evaluator(
        coordinate,
        continuation_seed,
        fallback_seed_count,
        80,
        |voltages| evaluate_active_network_dc_residuals(coordinate, voltages),
    )
}

fn solve_active_network_dc_coordinate(
    coordinate: ActiveNetworkDcCoordinate,
) -> ActiveNetworkDcOperatingPoint {
    solve_active_network_dc_coordinate_with_seed(coordinate, None, 6)
}

const fn active_network_dc_coordinates() -> [ActiveNetworkDcCoordinate; 3] {
    [
        ActiveNetworkDcCoordinate {
            name: "tp3_zero_early_probe",
            triangle_input_v: 0.0,
            active_device_assignment_index: 31,
            mn3101_macro_hypothesis_index: 1,
            temperature_c: 25.0,
            interpretation: "numerical topology probe, not a measured TP3 endpoint",
        },
        ActiveNetworkDcCoordinate {
            name: "tp3_zero_geometric_probe",
            triangle_input_v: 0.0,
            active_device_assignment_index: 31,
            mn3101_macro_hypothesis_index: 4,
            temperature_c: 25.0,
            interpretation: "numerical topology probe, not a measured TP3 center",
        },
        ActiveNetworkDcCoordinate {
            name: "tp3_zero_late_probe",
            triangle_input_v: 0.0,
            active_device_assignment_index: 31,
            mn3101_macro_hypothesis_index: 7,
            temperature_c: 25.0,
            interpretation: "numerical topology probe, not a measured TP3 endpoint",
        },
    ]
}

fn active_network_dc_operating_points() -> Vec<ActiveNetworkDcOperatingPoint> {
    active_network_dc_coordinates()
        .into_iter()
        .map(solve_active_network_dc_coordinate)
        .collect()
}

fn active_network_dc_solver_validation_for(
    points: &[ActiveNetworkDcOperatingPoint],
) -> ActiveNetworkDcSolverValidation {
    const ACCEPTED_RESIDUAL_A: f64 = 1.0e-7;
    let converged = points
        .iter()
        .filter(|point| point.converged)
        .collect::<Vec<_>>();
    let all_converged_points_inside_supply_window = converged
        .iter()
        .all(|point| point.every_unknown_inside_supply_window);
    let all_converged_bjt_points_respect_vce_polarity = converged
        .iter()
        .all(|point| point.every_bjt_vce_polarity_valid);
    let all_converged_bjt_points_inside_power_limits = converged
        .iter()
        .all(|point| point.every_bjt_inside_published_power_limit);
    let all_converged_bjt_points_inside_digitized_transfer_domain = converged.iter().all(|point| {
        point
            .device_points
            .iter()
            .all(|device| !device.transfer_voltage_was_clamped_to_digitized_domain)
    });
    let preliminary_fixed_points_ready = !converged.is_empty()
        && converged
            .iter()
            .all(|point| point.maximum_kcl_residual_a <= ACCEPTED_RESIDUAL_A)
        && all_converged_points_inside_supply_window
        && all_converged_bjt_points_respect_vce_polarity
        && all_converged_bjt_points_inside_power_limits;
    ActiveNetworkDcSolverValidation {
        coordinate_count: points.len(),
        converged_coordinate_count: converged.len(),
        maximum_accepted_kcl_residual_a: ACCEPTED_RESIDUAL_A,
        all_converged_points_inside_supply_window,
        all_converged_bjt_points_respect_vce_polarity,
        all_converged_bjt_points_inside_power_limits,
        all_converged_bjt_points_inside_digitized_transfer_domain,
        residual_assembly_covers_resistors: true,
        residual_assembly_covers_bipolars: true,
        residual_assembly_covers_diode_center: true,
        residual_assembly_covers_mn3101_ox_ports: true,
        capacitors_are_open_at_dc: true,
        mn3101_hidden_supply_current_is_modeled: false,
        preliminary_fixed_points_ready,
        confirmed_physical_operating_points_ready: false,
        interpretation: "converged KCL fixed points validate the executable graph and seed later transient integration; they cannot establish the oscillator's physical trajectory or production spread",
    }
}

fn dc_operating_point_voltage(
    point: &ActiveNetworkDcOperatingPoint,
    node: ActiveNetworkNode,
) -> f64 {
    point
        .node_voltages
        .iter()
        .find(|candidate| candidate.node == node)
        .expect("every solved unknown node is serialized")
        .voltage_v
}

fn bjt_output_characteristic_clamped_point_resolutions(
    points: &[ActiveNetworkDcOperatingPoint],
) -> Vec<BjtOutputCharacteristicClampedPointResolution> {
    const MINIMUM_SOLVER_RESOLUTION_MARGIN_RADII: f64 = 2.0;
    let output_evidence = bjt_output_characteristic_plot_evidence();
    points
        .iter()
        .filter(|point| point.converged)
        .flat_map(|point| {
            point
                .device_points
                .iter()
                .filter(|device| device.transfer_voltage_was_clamped_to_digitized_domain)
        })
        .map(|device| {
            let evidence = output_evidence
                .iter()
                .find(|record| record.candidate_key == device.candidate_key)
                .expect("every shortlisted BJT has direct output-characteristic evidence");
            let smallest_positive_labeled_base_current_a = evidence
                .base_current_curve_levels_a
                .iter()
                .copied()
                .find(|base_current_a| *base_current_a > 0.0)
                .expect("every output-characteristic family has a positive IB curve");
            let plot_voltage_reading_uncertainty_mv = evidence.reading_radius_px
                * (evidence.collector_emitter_voltage_right_v
                    - evidence.collector_emitter_voltage_left_v)
                    .abs()
                / (evidence.x_right_px - evidence.x_left_px).abs()
                * 1_000.0;
            let preliminary_base_current_inside_zero_to_first_curve_bracket = device
                .forward_beta_base_current_a
                <= smallest_positive_labeled_base_current_a;
            let vce_exceeds_plot_reading_uncertainty =
                device.collector_emitter_voltage_absolute_v * 1_000.0
                    > plot_voltage_reading_uncertainty_mv;
            let vce_to_plot_reading_uncertainty_ratio =
                device.collector_emitter_voltage_absolute_v * 1_000.0
                    / plot_voltage_reading_uncertainty_mv;
            let vce_meets_solver_resolution_margin = vce_to_plot_reading_uncertainty_ratio
                >= MINIMUM_SOLVER_RESOLUTION_MARGIN_RADII;
            BjtOutputCharacteristicClampedPointResolution {
                designator: device.designator,
                candidate_key: device.candidate_key,
                collector_emitter_voltage_absolute_v: device
                    .collector_emitter_voltage_absolute_v,
                collector_current_a: device.collector_current_a,
                preliminary_forward_beta_base_current_a: device.forward_beta_base_current_a,
                smallest_positive_labeled_base_current_a,
                preliminary_base_current_inside_zero_to_first_curve_bracket,
                plot_voltage_reading_uncertainty_mv,
                vce_exceeds_plot_reading_uncertainty,
                vce_to_plot_reading_uncertainty_ratio,
                minimum_solver_resolution_margin_radii:
                    MINIMUM_SOLVER_RESOLUTION_MARGIN_RADII,
                vce_meets_solver_resolution_margin,
                directly_resolvable_from_archived_plot:
                    preliminary_base_current_inside_zero_to_first_curve_bracket
                        && vce_meets_solver_resolution_margin,
                interpretation: "the current solver point is diagnostic; this comparison asks only whether its clamped low-VCE coordinate is visibly resolvable on the archived manufacturer output plot",
            }
        })
        .collect()
}

fn bjt_output_characteristic_clamped_resolution_validation(
    resolutions: &[BjtOutputCharacteristicClampedPointResolution],
) -> BjtOutputCharacteristicClampedResolutionValidation {
    const MINIMUM_SOLVER_RESOLUTION_MARGIN_RADII: f64 = 2.0;
    let points_inside_zero_to_first_base_current_curve_bracket = resolutions
        .iter()
        .filter(|point| point.preliminary_base_current_inside_zero_to_first_curve_bracket)
        .count();
    let points_with_vce_above_plot_reading_uncertainty = resolutions
        .iter()
        .filter(|point| point.vce_exceeds_plot_reading_uncertainty)
        .count();
    let points_meeting_solver_resolution_margin = resolutions
        .iter()
        .filter(|point| point.vce_meets_solver_resolution_margin)
        .count();
    let every_clamped_point_directly_resolvable = !resolutions.is_empty()
        && resolutions
            .iter()
            .all(|point| point.directly_resolvable_from_archived_plot);
    BjtOutputCharacteristicClampedResolutionValidation {
        evaluated_clamped_points: resolutions.len(),
        points_inside_zero_to_first_base_current_curve_bracket,
        points_with_vce_above_plot_reading_uncertainty,
        points_meeting_solver_resolution_margin,
        minimum_solver_resolution_margin_radii: MINIMUM_SOLVER_RESOLUTION_MARGIN_RADII,
        every_clamped_point_directly_resolvable,
        higher_resolution_source_or_bounded_low_vce_model_required:
            !every_clamped_point_directly_resolvable,
        digitization_may_proceed_for_resolvable_domain_only: true,
        suitable_for_immediate_solver_substitution: false,
        interpretation: "both clamped BJT base currents lie between the zero and first positive IB curves; the native Mitsubishi raster places Tr23 only 1.06 reading radii from the axis and Tr22 below 0.05, while solver substitution requires a two-radius margin; digitizing the visible domain is useful, but direct substitution at either point would fabricate low-voltage precision",
    }
}

fn bjt_low_vce_base_drive_hypotheses() -> Vec<BjtLowVceBaseDriveHypothesis> {
    const GUARD_MARGIN_READING_RADII: f64 = 2.0;
    let mut hypotheses = Vec::with_capacity(6);
    for join_shape in [BjtRegionJoinShape::Linear, BjtRegionJoinShape::Smoothstep] {
        for blend_to_first_labeled_curve in [0.25, 0.5, 1.0] {
            hypotheses.push(BjtLowVceBaseDriveHypothesis {
                index: hypotheses.len(),
                join_shape,
                blend_to_first_labeled_curve,
                guard_margin_reading_radii: GUARD_MARGIN_READING_RADII,
                first_labeled_curve_is_sensitivity_target_not_bound: true,
                suitable_as_identified_device_law: false,
            });
        }
    }
    hypotheses
}

fn bjt_reciprocal_transport_hypotheses() -> Vec<BjtReciprocalTransportHypothesis> {
    [1.0, 10.0, 100.0]
        .into_iter()
        .enumerate()
        .map(|(index, reverse_beta)| BjtReciprocalTransportHypothesis {
            index,
            reverse_beta,
            forward_and_reverse_transport_share_reciprocal_scale: true,
            reverse_beta_is_unpublished_sensitivity_axis: true,
            suitable_as_identified_device_law: false,
        })
        .collect()
}

fn evaluate_bjt_low_vce_base_drive(
    candidate_key: &str,
    collector_emitter_voltage_absolute_v: f64,
    unadjusted_base_current_a: f64,
    hypothesis: BjtLowVceBaseDriveHypothesis,
) -> BjtLowVceBaseDriveEvaluation {
    let evidence = bjt_output_characteristic_plot_evidence()
        .into_iter()
        .find(|record| record.candidate_key == candidate_key)
        .expect("every active-device candidate has an output-characteristic family");
    let first_labeled_base_current_a = evidence
        .base_current_curve_levels_a
        .iter()
        .copied()
        .find(|current| *current > 0.0)
        .expect("every output-characteristic family has a positive IB curve");
    let reading_uncertainty_v = evidence.reading_radius_px
        * (evidence.collector_emitter_voltage_right_v - evidence.collector_emitter_voltage_left_v)
            .abs()
        / (evidence.x_right_px - evidence.x_left_px).abs();
    let guard_voltage_v = hypothesis.guard_margin_reading_radii * reading_uncertainty_v;
    let proximity_coordinate =
        1.0 - (collector_emitter_voltage_absolute_v / guard_voltage_v).clamp(0.0, 1.0);
    let proximity_weight = bjt_region_join_scale(hypothesis.join_shape, proximity_coordinate)
        * hypothesis.blend_to_first_labeled_curve;
    let target_base_current_a = first_labeled_base_current_a.max(unadjusted_base_current_a);
    let base_current_adjustment_a =
        proximity_weight * (target_base_current_a - unadjusted_base_current_a);
    BjtLowVceBaseDriveEvaluation {
        base_current_a: unadjusted_base_current_a + base_current_adjustment_a,
        guard_voltage_v,
        proximity_weight,
        first_labeled_base_current_a,
        base_current_adjustment_a,
    }
}

fn active_device_dc_sweep_points(
    topology_points: &[ActiveNetworkDcOperatingPoint],
) -> Vec<ActiveNetworkDcOperatingPoint> {
    let base = topology_points
        .iter()
        .find(|point| {
            point.converged
                && point.coordinate.active_device_assignment_index == 31
                && point.coordinate.mn3101_macro_hypothesis_index == 4
        })
        .expect("the linear/geometric fixed point is the continuation seed")
        .clone();
    let mut results =
        vec![None; active_device_assignment_space().comparative_shortlist_assignments];
    results[31] = Some(base.clone());
    let mut continuation_seed = base
        .node_voltages
        .iter()
        .map(|node| node.voltage_v)
        .collect::<Vec<_>>();
    for assignment_index in (0..31).rev() {
        let point = solve_active_network_dc_coordinate_with_seed(
            ActiveNetworkDcCoordinate {
                name: "tp3_zero_assignment_sweep",
                triangle_input_v: 0.0,
                active_device_assignment_index: assignment_index,
                mn3101_macro_hypothesis_index: 4,
                temperature_c: 25.0,
                interpretation: "32-assignment continuation probe at the shared numerical coordinate; not a measured TP3 operating point",
            },
            Some(&continuation_seed),
            2,
        );
        if point.converged {
            continuation_seed = point
                .node_voltages
                .iter()
                .map(|node| node.voltage_v)
                .collect();
        }
        results[assignment_index] = Some(point);
    }
    results
        .into_iter()
        .map(|point| point.expect("all 32 assignment slots are evaluated"))
        .collect()
}

fn active_device_dc_sweep_records(
    points: &[ActiveNetworkDcOperatingPoint],
) -> Vec<ActiveDeviceDcSweepPoint> {
    points
        .iter()
        .map(|point| {
            let clamped_bjt_count = point
                .device_points
                .iter()
                .filter(|device| device.transfer_voltage_was_clamped_to_digitized_domain)
                .count();
            let inside_supply_polarity_and_power_gates = point.converged
                && point.every_unknown_inside_supply_window
                && point.every_bjt_vce_polarity_valid
                && point.every_bjt_inside_published_power_limit;
            ActiveDeviceDcSweepPoint {
                active_device_assignment_index: point.coordinate.active_device_assignment_index,
                converged: point.converged,
                maximum_kcl_residual_a: point.maximum_kcl_residual_a,
                control_voltage_v: dc_operating_point_voltage(point, ActiveNetworkNode::Control),
                timing_voltage_v: dc_operating_point_voltage(point, ActiveNetworkNode::Timing),
                feedback_base_voltage_v: dc_operating_point_voltage(
                    point,
                    ActiveNetworkNode::FeedbackBase,
                ),
                total_collector_current_a: point
                    .device_points
                    .iter()
                    .map(|device| device.collector_current_a)
                    .sum(),
                maximum_collector_power_mw: point
                    .device_points
                    .iter()
                    .map(|device| device.collector_power_mw)
                    .fold(0.0, f64::max),
                clamped_bjt_count,
                inside_supply_polarity_and_power_gates,
                suitable_for_complete_ensemble_comparison: inside_supply_polarity_and_power_gates
                    && clamped_bjt_count == 0,
            }
        })
        .collect()
}

fn optional_complete_dc_metric_comparison(
    records: &[ActiveDeviceDcSweepPoint],
    value: impl Fn(&ActiveDeviceDcSweepPoint) -> f64,
) -> Option<UncertaintyComparison> {
    if records.len() != active_device_assignment_space().comparative_shortlist_assignments
        || records.iter().any(|record| !record.converged)
    {
        return None;
    }
    Some(compare_uncertainty_ensemble(
        &records.iter().map(value).collect::<Vec<_>>(),
        uncertainty_acceptance_policy(),
    ))
}

fn active_device_dc_sweep_validation(
    records: &[ActiveDeviceDcSweepPoint],
) -> ActiveDeviceDcSweepValidation {
    let metric_comparisons = [
        ActiveDeviceDcMetricComparison {
            metric: "OX3 control-node fixed-point voltage",
            unit: "volt",
            comparison: optional_complete_dc_metric_comparison(records, |point| {
                point.control_voltage_v
            }),
        },
        ActiveDeviceDcMetricComparison {
            metric: "150 pF timing-node fixed-point voltage",
            unit: "volt",
            comparison: optional_complete_dc_metric_comparison(records, |point| {
                point.timing_voltage_v
            }),
        },
        ActiveDeviceDcMetricComparison {
            metric: "OX1 feedback-base fixed-point voltage",
            unit: "volt",
            comparison: optional_complete_dc_metric_comparison(records, |point| {
                point.feedback_base_voltage_v
            }),
        },
        ActiveDeviceDcMetricComparison {
            metric: "sum of five collector-current magnitudes",
            unit: "ampere",
            comparison: optional_complete_dc_metric_comparison(records, |point| {
                point.total_collector_current_a
            }),
        },
        ActiveDeviceDcMetricComparison {
            metric: "maximum per-device collector power",
            unit: "milliwatt",
            comparison: optional_complete_dc_metric_comparison(records, |point| {
                point.maximum_collector_power_mw
            }),
        },
    ];
    let converged_assignments = records.iter().filter(|point| point.converged).count();
    let assignments_inside_all_dc_gates = records
        .iter()
        .filter(|point| point.inside_supply_polarity_and_power_gates)
        .count();
    let assignments_without_transfer_clamping = records
        .iter()
        .filter(|point| point.clamped_bjt_count == 0)
        .count();
    let complete_32_assignment_ensemble = records.len() == 32 && converged_assignments == 32;
    let dc_metrics_accepted_as_equivalent = complete_32_assignment_ensemble
        && assignments_inside_all_dc_gates == 32
        && assignments_without_transfer_clamping == 32
        && metric_comparisons.iter().all(|metric| {
            metric
                .comparison
                .is_some_and(|comparison| comparison.accepted)
        });
    ActiveDeviceDcSweepValidation {
        attempted_assignments: records.len(),
        converged_assignments,
        assignments_inside_all_dc_gates,
        assignments_without_transfer_clamping,
        complete_32_assignment_ensemble,
        continuation_seed_used: true,
        metric_comparisons,
        dc_metrics_accepted_as_equivalent,
        required_audio_trajectory_metrics_evaluated: false,
        accepted_as_behaviorally_equivalent: false,
        interpretation: "all 32 device assignments converge at the shared mathematical fixed point and pass supply/polarity/power gates; all clamp two BJT transfer coordinates, collector-current and power spreads fail the predeclared limits, and DC-only metrics cannot establish audio equivalence",
    }
}

fn solve_bjt_low_vce_base_drive_point(
    base: &ActiveNetworkDcOperatingPoint,
    hypothesis: BjtLowVceBaseDriveHypothesis,
    fallback_seed_count: usize,
) -> BjtLowVceBaseDriveSweepPoint {
    let coordinate = ActiveNetworkDcCoordinate {
        name: "low_vce_base_drive_sensitivity",
        triangle_input_v: base.coordinate.triangle_input_v,
        active_device_assignment_index: base.coordinate.active_device_assignment_index,
        mn3101_macro_hypothesis_index: base.coordinate.mn3101_macro_hypothesis_index,
        temperature_c: base.coordinate.temperature_c,
        interpretation: "first-labeled-IB low-VCE sensitivity law at the shared DC probe; not a recovered transistor curve",
    };
    let seed = base
        .node_voltages
        .iter()
        .map(|node| node.voltage_v)
        .collect::<Vec<_>>();
    let point = solve_active_network_dc_coordinate_with_evaluator(
        coordinate,
        Some(&seed),
        fallback_seed_count,
        12,
        |voltages| {
            evaluate_active_network_dc_residuals_with_low_vce_base_drive(
                coordinate, voltages, hypothesis,
            )
        },
    );
    let adjusted_bjt_count = point
        .device_points
        .iter()
        .filter(|device| device.low_vce_base_current_adjustment_a > 0.0)
        .count();
    let remaining_transfer_clamp_count = point
        .device_points
        .iter()
        .filter(|device| device.transfer_voltage_was_clamped_to_digitized_domain)
        .count();
    let inside_supply_polarity_and_power_gates = point.converged
        && point.every_unknown_inside_supply_window
        && point.every_bjt_vce_polarity_valid
        && point.every_bjt_inside_published_power_limit;
    BjtLowVceBaseDriveSweepPoint {
        hypothesis_index: hypothesis.index,
        active_device_assignment_index: point.coordinate.active_device_assignment_index,
        converged: point.converged,
        maximum_kcl_residual_a: point.maximum_kcl_residual_a,
        control_voltage_v: dc_operating_point_voltage(&point, ActiveNetworkNode::Control),
        timing_voltage_v: dc_operating_point_voltage(&point, ActiveNetworkNode::Timing),
        feedback_base_voltage_v: dc_operating_point_voltage(
            &point,
            ActiveNetworkNode::FeedbackBase,
        ),
        total_collector_current_a: point
            .device_points
            .iter()
            .map(|device| device.collector_current_a)
            .sum(),
        maximum_collector_power_mw: point
            .device_points
            .iter()
            .map(|device| device.collector_power_mw)
            .fold(0.0, f64::max),
        adjusted_bjt_count,
        remaining_transfer_clamp_count,
        maximum_base_current_adjustment_a: point
            .device_points
            .iter()
            .map(|device| device.low_vce_base_current_adjustment_a)
            .fold(0.0, f64::max),
        inside_supply_polarity_and_power_gates,
    }
}

fn bjt_low_vce_base_drive_reference_screen_points(
    assignment_points: &[ActiveNetworkDcOperatingPoint],
) -> Vec<BjtLowVceBaseDriveSweepPoint> {
    let reference = assignment_points
        .iter()
        .find(|point| point.coordinate.active_device_assignment_index == 31)
        .expect("assignment 31 is the shared low-VCE reference point");
    bjt_low_vce_base_drive_hypotheses()
        .into_iter()
        .map(|hypothesis| solve_bjt_low_vce_base_drive_point(reference, hypothesis, 2))
        .collect()
}

fn bjt_low_vce_base_drive_sweep_points(
    assignment_points: &[ActiveNetworkDcOperatingPoint],
    reference_screen: &[BjtLowVceBaseDriveSweepPoint],
) -> Vec<BjtLowVceBaseDriveSweepPoint> {
    let hypotheses = bjt_low_vce_base_drive_hypotheses();
    let survivors = hypotheses
        .into_iter()
        .filter(|hypothesis| {
            reference_screen.iter().any(|point| {
                point.hypothesis_index == hypothesis.index
                    && point.converged
                    && point.inside_supply_polarity_and_power_gates
            })
        })
        .collect::<Vec<_>>();
    let mut records = Vec::with_capacity(survivors.len() * assignment_points.len());
    for hypothesis in survivors {
        for base in assignment_points {
            records.push(solve_bjt_low_vce_base_drive_point(base, hypothesis, 0));
        }
    }
    records
}

fn optional_low_vce_metric_comparison(
    records: &[BjtLowVceBaseDriveSweepPoint],
    expected_matrix_points: usize,
    value: impl Fn(&BjtLowVceBaseDriveSweepPoint) -> f64,
) -> Option<UncertaintyComparison> {
    if records.len() != expected_matrix_points || records.iter().any(|record| !record.converged) {
        return None;
    }
    Some(compare_uncertainty_ensemble(
        &records.iter().map(value).collect::<Vec<_>>(),
        uncertainty_acceptance_policy(),
    ))
}

fn bjt_low_vce_base_drive_sweep_validation(
    reference_screen: &[BjtLowVceBaseDriveSweepPoint],
    records: &[BjtLowVceBaseDriveSweepPoint],
) -> BjtLowVceBaseDriveSweepValidation {
    let defined_hypothesis_count = bjt_low_vce_base_drive_hypotheses().len();
    let reference_screen_survivor_count = reference_screen
        .iter()
        .filter(|point| point.converged && point.inside_supply_polarity_and_power_gates)
        .count();
    let reference_screen_rejected_count =
        defined_hypothesis_count - reference_screen_survivor_count;
    let hypothesis_count = reference_screen_survivor_count;
    let assignment_count = active_device_assignment_space().comparative_shortlist_assignments;
    let expected_matrix_points = hypothesis_count * assignment_count;
    let metric_comparisons = [
        ActiveDeviceDcMetricComparison {
            metric: "OX3 control-node low-VCE sensitivity voltage",
            unit: "volt",
            comparison: optional_low_vce_metric_comparison(
                records,
                expected_matrix_points,
                |point| point.control_voltage_v,
            ),
        },
        ActiveDeviceDcMetricComparison {
            metric: "150 pF timing-node low-VCE sensitivity voltage",
            unit: "volt",
            comparison: optional_low_vce_metric_comparison(
                records,
                expected_matrix_points,
                |point| point.timing_voltage_v,
            ),
        },
        ActiveDeviceDcMetricComparison {
            metric: "OX1 feedback-base low-VCE sensitivity voltage",
            unit: "volt",
            comparison: optional_low_vce_metric_comparison(
                records,
                expected_matrix_points,
                |point| point.feedback_base_voltage_v,
            ),
        },
        ActiveDeviceDcMetricComparison {
            metric: "low-VCE sensitivity sum of collector-current magnitudes",
            unit: "ampere",
            comparison: optional_low_vce_metric_comparison(
                records,
                expected_matrix_points,
                |point| point.total_collector_current_a,
            ),
        },
        ActiveDeviceDcMetricComparison {
            metric: "low-VCE sensitivity maximum per-device collector power",
            unit: "milliwatt",
            comparison: optional_low_vce_metric_comparison(
                records,
                expected_matrix_points,
                |point| point.maximum_collector_power_mw,
            ),
        },
    ];
    let hypotheses_converged_for_all_assignments = (0..defined_hypothesis_count)
        .filter(|hypothesis_index| {
            let matching = records
                .iter()
                .filter(|point| point.hypothesis_index == *hypothesis_index)
                .collect::<Vec<_>>();
            matching.len() == assignment_count && matching.iter().all(|point| point.converged)
        })
        .count();
    let hypotheses_eliminating_clamps_for_all_assignments = (0..defined_hypothesis_count)
        .filter(|hypothesis_index| {
            let matching = records
                .iter()
                .filter(|point| point.hypothesis_index == *hypothesis_index)
                .collect::<Vec<_>>();
            matching.len() == assignment_count
                && matching.iter().all(|point| {
                    point.converged
                        && point.inside_supply_polarity_and_power_gates
                        && point.remaining_transfer_clamp_count == 0
                })
        })
        .count();
    let converged_matrix_points = records.iter().filter(|point| point.converged).count();
    let points_inside_supply_polarity_and_power_gates = records
        .iter()
        .filter(|point| point.inside_supply_polarity_and_power_gates)
        .count();
    let points_without_transfer_clamping = records
        .iter()
        .filter(|point| point.remaining_transfer_clamp_count == 0)
        .count();
    let complete_dc_ensemble_accepted = records.len() == expected_matrix_points
        && converged_matrix_points == expected_matrix_points
        && points_inside_supply_polarity_and_power_gates == expected_matrix_points
        && points_without_transfer_clamping == expected_matrix_points
        && metric_comparisons.iter().all(|metric| {
            metric
                .comparison
                .is_some_and(|comparison| comparison.accepted)
        });
    BjtLowVceBaseDriveSweepValidation {
        defined_hypothesis_count,
        reference_screen_survivor_count,
        reference_screen_rejected_count,
        hypothesis_count,
        assignment_count,
        expected_matrix_points,
        evaluated_matrix_points: records.len(),
        converged_matrix_points,
        points_inside_supply_polarity_and_power_gates,
        points_without_transfer_clamping,
        hypotheses_converged_for_all_assignments,
        hypotheses_eliminating_clamps_for_all_assignments,
        metric_comparisons,
        complete_dc_ensemble_accepted,
        required_audio_trajectory_metrics_evaluated: false,
        suitable_for_production_promotion: false,
        interpretation: "a reference solve rejects laws that break supply, VCE-polarity or power gates before the surviving explicitly non-identifying low-VCE laws cross all 32 assignments; the first printed IB trace is a sensitivity destination inside the unresolved two-radius guard, never a claimed physical bound",
    }
}

fn solve_bjt_reciprocal_transport_point(
    base: &ActiveNetworkDcOperatingPoint,
    hypothesis: BjtReciprocalTransportHypothesis,
) -> BjtReciprocalTransportSweepPoint {
    let coordinate = ActiveNetworkDcCoordinate {
        name: "reciprocal_transport_sensitivity",
        triangle_input_v: base.coordinate.triangle_input_v,
        active_device_assignment_index: base.coordinate.active_device_assignment_index,
        mn3101_macro_hypothesis_index: base.coordinate.mn3101_macro_hypothesis_index,
        temperature_c: base.coordinate.temperature_c,
        interpretation: "reciprocal two-junction transport sensitivity with unpublished reverse beta; not an identified transistor model",
    };
    let seed = base
        .node_voltages
        .iter()
        .map(|node| node.voltage_v)
        .collect::<Vec<_>>();
    let point = solve_active_network_dc_coordinate_with_evaluator(
        coordinate,
        Some(&seed),
        0,
        16,
        |voltages| {
            evaluate_active_network_dc_residuals_with_reciprocal_transport(
                coordinate, voltages, hypothesis,
            )
        },
    );
    let inside_supply_polarity_and_power_gates = point.converged
        && point.every_unknown_inside_supply_window
        && point.every_bjt_vce_polarity_valid
        && point.every_bjt_inside_published_power_limit;
    BjtReciprocalTransportSweepPoint {
        hypothesis_index: hypothesis.index,
        active_device_assignment_index: point.coordinate.active_device_assignment_index,
        converged: point.converged,
        maximum_kcl_residual_a: point.maximum_kcl_residual_a,
        control_voltage_v: dc_operating_point_voltage(&point, ActiveNetworkNode::Control),
        timing_voltage_v: dc_operating_point_voltage(&point, ActiveNetworkNode::Timing),
        feedback_base_voltage_v: dc_operating_point_voltage(
            &point,
            ActiveNetworkNode::FeedbackBase,
        ),
        total_collector_terminal_current_absolute_a: point
            .device_points
            .iter()
            .map(|device| device.collector_current_a.abs())
            .sum(),
        maximum_collector_power_mw: point
            .device_points
            .iter()
            .map(|device| device.collector_power_mw)
            .fold(0.0, f64::max),
        devices_with_reverse_transport: point
            .device_points
            .iter()
            .filter(|device| device.reverse_transport_current_a > 0.0)
            .count(),
        forward_transfer_clamp_count: point
            .device_points
            .iter()
            .filter(|device| device.transfer_voltage_was_clamped_to_digitized_domain)
            .count(),
        reverse_transfer_clamp_count: point
            .device_points
            .iter()
            .filter(|device| device.reverse_transfer_voltage_was_clamped_to_digitized_domain)
            .count(),
        inside_supply_polarity_and_power_gates,
    }
}

fn bjt_reciprocal_transport_sweep_points(
    assignment_points: &[ActiveNetworkDcOperatingPoint],
) -> Vec<BjtReciprocalTransportSweepPoint> {
    let hypotheses = bjt_reciprocal_transport_hypotheses();
    let mut records = Vec::with_capacity(hypotheses.len() * assignment_points.len());
    for hypothesis in hypotheses {
        for base in assignment_points {
            records.push(solve_bjt_reciprocal_transport_point(base, hypothesis));
        }
    }
    records
}

fn optional_reciprocal_transport_metric_comparison(
    records: &[BjtReciprocalTransportSweepPoint],
    expected_matrix_points: usize,
    value: impl Fn(&BjtReciprocalTransportSweepPoint) -> f64,
) -> Option<UncertaintyComparison> {
    if records.len() != expected_matrix_points || records.iter().any(|record| !record.converged) {
        return None;
    }
    Some(compare_uncertainty_ensemble(
        &records.iter().map(value).collect::<Vec<_>>(),
        uncertainty_acceptance_policy(),
    ))
}

fn bjt_reciprocal_transport_sweep_validation(
    records: &[BjtReciprocalTransportSweepPoint],
) -> BjtReciprocalTransportSweepValidation {
    let hypothesis_count = bjt_reciprocal_transport_hypotheses().len();
    let assignment_count = active_device_assignment_space().comparative_shortlist_assignments;
    let expected_matrix_points = hypothesis_count * assignment_count;
    let metric_comparisons = [
        ActiveDeviceDcMetricComparison {
            metric: "OX3 reciprocal-transport control voltage",
            unit: "volt",
            comparison: optional_reciprocal_transport_metric_comparison(
                records,
                expected_matrix_points,
                |point| point.control_voltage_v,
            ),
        },
        ActiveDeviceDcMetricComparison {
            metric: "reciprocal-transport 150 pF timing voltage",
            unit: "volt",
            comparison: optional_reciprocal_transport_metric_comparison(
                records,
                expected_matrix_points,
                |point| point.timing_voltage_v,
            ),
        },
        ActiveDeviceDcMetricComparison {
            metric: "OX1 reciprocal-transport feedback-base voltage",
            unit: "volt",
            comparison: optional_reciprocal_transport_metric_comparison(
                records,
                expected_matrix_points,
                |point| point.feedback_base_voltage_v,
            ),
        },
        ActiveDeviceDcMetricComparison {
            metric: "reciprocal-transport collector-terminal current magnitude sum",
            unit: "ampere",
            comparison: optional_reciprocal_transport_metric_comparison(
                records,
                expected_matrix_points,
                |point| point.total_collector_terminal_current_absolute_a,
            ),
        },
        ActiveDeviceDcMetricComparison {
            metric: "reciprocal-transport maximum collector power",
            unit: "milliwatt",
            comparison: optional_reciprocal_transport_metric_comparison(
                records,
                expected_matrix_points,
                |point| point.maximum_collector_power_mw,
            ),
        },
    ];
    let converged_matrix_points = records.iter().filter(|point| point.converged).count();
    let points_inside_supply_polarity_and_power_gates = records
        .iter()
        .filter(|point| point.inside_supply_polarity_and_power_gates)
        .count();
    let points_without_forward_transfer_clamping = records
        .iter()
        .filter(|point| point.forward_transfer_clamp_count == 0)
        .count();
    let points_without_any_transfer_clamping = records
        .iter()
        .filter(|point| {
            point.forward_transfer_clamp_count == 0 && point.reverse_transfer_clamp_count == 0
        })
        .count();
    let hypotheses_converged_for_all_assignments = (0..hypothesis_count)
        .filter(|hypothesis_index| {
            let matching = records
                .iter()
                .filter(|point| point.hypothesis_index == *hypothesis_index)
                .collect::<Vec<_>>();
            matching.len() == assignment_count && matching.iter().all(|point| point.converged)
        })
        .count();
    let hypotheses_eliminating_forward_clamps_for_all_assignments = (0..hypothesis_count)
        .filter(|hypothesis_index| {
            let matching = records
                .iter()
                .filter(|point| point.hypothesis_index == *hypothesis_index)
                .collect::<Vec<_>>();
            matching.len() == assignment_count
                && matching.iter().all(|point| {
                    point.converged
                        && point.inside_supply_polarity_and_power_gates
                        && point.forward_transfer_clamp_count == 0
                })
        })
        .count();
    let complete_dc_ensemble_accepted = records.len() == expected_matrix_points
        && converged_matrix_points == expected_matrix_points
        && points_inside_supply_polarity_and_power_gates == expected_matrix_points
        && points_without_any_transfer_clamping == expected_matrix_points
        && metric_comparisons.iter().all(|metric| {
            metric
                .comparison
                .is_some_and(|comparison| comparison.accepted)
        });
    BjtReciprocalTransportSweepValidation {
        hypothesis_count,
        assignment_count,
        expected_matrix_points,
        evaluated_matrix_points: records.len(),
        converged_matrix_points,
        points_inside_supply_polarity_and_power_gates,
        points_without_forward_transfer_clamping,
        points_without_any_transfer_clamping,
        hypotheses_converged_for_all_assignments,
        hypotheses_eliminating_forward_clamps_for_all_assignments,
        metric_comparisons,
        complete_dc_ensemble_accepted,
        required_audio_trajectory_metrics_evaluated: false,
        suitable_for_production_promotion: false,
        interpretation: "reciprocal two-junction transport shares one forward/reverse transport scale and sweeps unpublished reverse beta 1/10/100 across all 32 assignments; topology is physics-constrained, parameter identity is not",
    }
}

fn bjt_reciprocal_transport_homotopy(
    assignment_points: &[ActiveNetworkDcOperatingPoint],
) -> (
    Vec<BjtReciprocalTransportHomotopyStage>,
    Vec<BjtReciprocalTransportHomotopySummary>,
) {
    const HOMOTOPY_STAGES: usize = 16;
    let base = assignment_points
        .iter()
        .find(|point| {
            point.converged
                && point.coordinate.active_device_assignment_index == 31
                && point.coordinate.mn3101_macro_hypothesis_index == 4
        })
        .expect("the assignment-31 baseline seeds reciprocal-transport homotopy");
    let hypotheses = bjt_reciprocal_transport_hypotheses();
    let mut stages = Vec::with_capacity(hypotheses.len() * HOMOTOPY_STAGES);
    let mut summaries = Vec::with_capacity(hypotheses.len());
    for hypothesis in hypotheses {
        let coordinate = ActiveNetworkDcCoordinate {
            name: "reciprocal_transport_homotopy",
            triangle_input_v: base.coordinate.triangle_input_v,
            active_device_assignment_index: base.coordinate.active_device_assignment_index,
            mn3101_macro_hypothesis_index: base.coordinate.mn3101_macro_hypothesis_index,
            temperature_c: base.coordinate.temperature_c,
            interpretation: "numerical continuation from the legacy DC seed to exact reciprocal transport; intermediate blends are not device physics",
        };
        let mut seed = base
            .node_voltages
            .iter()
            .map(|node| node.voltage_v)
            .collect::<Vec<_>>();
        let mut last_point = None;
        let mut converged_stages = 0;
        let mut largest_converged_blend_fraction = 0.0;
        for stage_index in 1..=HOMOTOPY_STAGES {
            let blend_fraction = stage_index as f64 / HOMOTOPY_STAGES as f64;
            let point = solve_active_network_dc_coordinate_with_evaluator(
                coordinate,
                Some(&seed),
                0,
                40,
                |voltages| {
                    evaluate_active_network_dc_residuals_with_reciprocal_transport_blend(
                        coordinate,
                        voltages,
                        hypothesis,
                        blend_fraction,
                    )
                },
            );
            let exact_target_reached = stage_index == HOMOTOPY_STAGES && point.converged;
            stages.push(BjtReciprocalTransportHomotopyStage {
                hypothesis_index: hypothesis.index,
                blend_fraction,
                converged: point.converged,
                maximum_kcl_residual_a: point.maximum_kcl_residual_a,
                every_unknown_inside_supply_window: point.every_unknown_inside_supply_window,
                exact_target_reached,
            });
            let may_continue = point.converged && point.every_unknown_inside_supply_window;
            if may_continue {
                converged_stages += 1;
                largest_converged_blend_fraction = blend_fraction;
                seed = point
                    .node_voltages
                    .iter()
                    .map(|node| node.voltage_v)
                    .collect();
            }
            last_point = Some(point);
            if !may_continue {
                break;
            }
        }
        let final_point = last_point.expect("every reciprocal hypothesis attempts one stage");
        let exact_target_reached = largest_converged_blend_fraction >= 1.0;
        let exact_target_inside_all_dc_gates = exact_target_reached
            && final_point.every_unknown_inside_supply_window
            && final_point.every_bjt_vce_polarity_valid
            && final_point.every_bjt_inside_published_power_limit;
        summaries.push(BjtReciprocalTransportHomotopySummary {
            hypothesis_index: hypothesis.index,
            reverse_beta: hypothesis.reverse_beta,
            attempted_stages: stages
                .iter()
                .filter(|stage| stage.hypothesis_index == hypothesis.index)
                .count(),
            converged_stages,
            largest_converged_blend_fraction,
            exact_target_reached,
            exact_target_inside_all_dc_gates,
            final_every_unknown_inside_supply_window: final_point
                .every_unknown_inside_supply_window,
            final_every_bjt_vce_polarity_valid: final_point.every_bjt_vce_polarity_valid,
            final_every_bjt_inside_power_limit: final_point.every_bjt_inside_published_power_limit,
            final_forward_transfer_clamp_count: final_point
                .device_points
                .iter()
                .filter(|device| device.transfer_voltage_was_clamped_to_digitized_domain)
                .count(),
            final_reverse_transfer_clamp_count: final_point
                .device_points
                .iter()
                .filter(|device| device.reverse_transfer_voltage_was_clamped_to_digitized_domain)
                .count(),
        });
    }
    (stages, summaries)
}

fn bjt_reciprocal_transport_homotopy_validation(
    summaries: &[BjtReciprocalTransportHomotopySummary],
) -> BjtReciprocalTransportHomotopyValidation {
    let exact_targets_reached = summaries
        .iter()
        .filter(|summary| summary.exact_target_reached)
        .count();
    let exact_targets_inside_all_dc_gates = summaries
        .iter()
        .filter(|summary| summary.exact_target_inside_all_dc_gates)
        .count();
    let exact_targets_without_any_transfer_clamping = summaries
        .iter()
        .filter(|summary| {
            summary.exact_target_reached
                && summary.final_forward_transfer_clamp_count == 0
                && summary.final_reverse_transfer_clamp_count == 0
        })
        .count();
    BjtReciprocalTransportHomotopyValidation {
        hypothesis_count: summaries.len(),
        stages_per_hypothesis: 16,
        exact_targets_reached,
        exact_targets_inside_all_dc_gates,
        exact_targets_without_any_transfer_clamping,
        homotopy_is_numerical_path_not_device_evidence: true,
        reciprocal_transport_family_viable_at_reference: exact_targets_inside_all_dc_gates > 0,
        suitable_for_production_promotion: false,
        interpretation: "sixteen residual-blend stages test whether each exact reciprocal endpoint is reachable from the accepted baseline; only alpha=1 is the transport hypothesis and reverse beta remains unpublished",
    }
}

fn solve_reciprocal_transport_endpoint_from_base(
    base: &ActiveNetworkDcOperatingPoint,
    hypothesis: BjtReciprocalTransportHypothesis,
) -> (ActiveNetworkDcOperatingPoint, usize) {
    const HOMOTOPY_STAGES: usize = 16;
    let coordinate = ActiveNetworkDcCoordinate {
        name: "reciprocal_transport_assignment_homotopy",
        triangle_input_v: base.coordinate.triangle_input_v,
        active_device_assignment_index: base.coordinate.active_device_assignment_index,
        mn3101_macro_hypothesis_index: base.coordinate.mn3101_macro_hypothesis_index,
        temperature_c: base.coordinate.temperature_c,
        interpretation: "adaptive numerical path from the assignment-specific baseline to exact reciprocal beta-R-1 transport",
    };
    let mut seed = base
        .node_voltages
        .iter()
        .map(|node| node.voltage_v)
        .collect::<Vec<_>>();
    let mut last_point = base.clone();
    let mut completed_stages = 0;
    for stage_index in 1..=HOMOTOPY_STAGES {
        let blend_fraction = stage_index as f64 / HOMOTOPY_STAGES as f64;
        let point = solve_active_network_dc_coordinate_with_evaluator(
            coordinate,
            Some(&seed),
            0,
            32,
            |voltages| {
                evaluate_active_network_dc_residuals_with_reciprocal_transport_blend(
                    coordinate,
                    voltages,
                    hypothesis,
                    blend_fraction,
                )
            },
        );
        let may_continue = point.converged && point.every_unknown_inside_supply_window;
        last_point = point;
        if !may_continue {
            break;
        }
        completed_stages += 1;
        seed = last_point
            .node_voltages
            .iter()
            .map(|node| node.voltage_v)
            .collect();
    }
    (last_point, completed_stages)
}

fn reciprocal_transport_assignment_device_record(
    device: &ActiveNetworkDcDevicePoint,
) -> BjtReciprocalTransportAssignmentDevicePoint {
    BjtReciprocalTransportAssignmentDevicePoint {
        designator: device.designator,
        candidate_key: device.candidate_key,
        base_emitter_voltage_absolute_v: device.base_emitter_voltage_absolute_v,
        base_collector_voltage_absolute_v: device.base_collector_voltage_absolute_v,
        collector_emitter_voltage_absolute_v: device.collector_emitter_voltage_absolute_v,
        collector_current_signed_a: device.collector_current_a,
        collector_current_absolute_a: device.collector_current_a.abs(),
        base_current_absolute_a: device.base_current_a.abs(),
        reverse_transport_current_absolute_a: device.reverse_transport_current_a.abs(),
        collector_power_mw: device.collector_power_mw,
    }
}

fn reciprocal_transport_assignment_record(
    point: &ActiveNetworkDcOperatingPoint,
    adaptive_homotopy_used: bool,
    homotopy_stages_completed: usize,
) -> BjtReciprocalTransportAssignmentPoint {
    let forward_transfer_clamp_count = point
        .device_points
        .iter()
        .filter(|device| device.transfer_voltage_was_clamped_to_digitized_domain)
        .count();
    let reverse_transfer_clamp_count = point
        .device_points
        .iter()
        .filter(|device| device.reverse_transfer_voltage_was_clamped_to_digitized_domain)
        .count();
    let maximum_collector_power_device = point
        .device_points
        .iter()
        .max_by(|left, right| left.collector_power_mw.total_cmp(&right.collector_power_mw))
        .expect("the active clock cell always has five BJT points");
    BjtReciprocalTransportAssignmentPoint {
        active_device_assignment_index: point.coordinate.active_device_assignment_index,
        converged: point.converged,
        adaptive_homotopy_used,
        homotopy_stages_completed,
        maximum_kcl_residual_a: point.maximum_kcl_residual_a,
        control_voltage_v: dc_operating_point_voltage(point, ActiveNetworkNode::Control),
        timing_voltage_v: dc_operating_point_voltage(point, ActiveNetworkNode::Timing),
        feedback_base_voltage_v: dc_operating_point_voltage(point, ActiveNetworkNode::FeedbackBase),
        total_collector_terminal_current_absolute_a: point
            .device_points
            .iter()
            .map(|device| device.collector_current_a.abs())
            .sum(),
        maximum_collector_power_mw: point
            .device_points
            .iter()
            .map(|device| device.collector_power_mw)
            .fold(0.0, f64::max),
        maximum_collector_power_designator: maximum_collector_power_device.designator,
        forward_transfer_clamp_count,
        reverse_transfer_clamp_count,
        inside_supply_transport_and_power_gates: point.converged
            && point.every_unknown_inside_supply_window
            && point.every_bjt_vce_polarity_valid
            && point.every_bjt_inside_published_power_limit
            && forward_transfer_clamp_count == 0
            && reverse_transfer_clamp_count == 0,
        node_voltages: point.node_voltages.clone(),
        device_points: point
            .device_points
            .iter()
            .map(reciprocal_transport_assignment_device_record)
            .collect(),
    }
}

fn bjt_reciprocal_transport_assignment_points(
    assignment_points: &[ActiveNetworkDcOperatingPoint],
) -> Vec<BjtReciprocalTransportAssignmentPoint> {
    let hypothesis = bjt_reciprocal_transport_hypotheses()[0];
    let reference_baseline = assignment_points
        .iter()
        .find(|point| point.coordinate.active_device_assignment_index == 31)
        .expect("assignment 31 seeds exact reciprocal continuation");
    let (reference_exact, reference_stages) =
        solve_reciprocal_transport_endpoint_from_base(reference_baseline, hypothesis);
    let mut records = Vec::with_capacity(32);
    records.push(reciprocal_transport_assignment_record(
        &reference_exact,
        true,
        reference_stages,
    ));
    let mut continuation_seed = reference_exact
        .node_voltages
        .iter()
        .map(|node| node.voltage_v)
        .collect::<Vec<_>>();
    for assignment_index in (0..31).rev() {
        let baseline = assignment_points
            .iter()
            .find(|point| point.coordinate.active_device_assignment_index == assignment_index)
            .expect("every assignment has a baseline continuation point");
        let coordinate = ActiveNetworkDcCoordinate {
            name: "reciprocal_transport_assignment_continuation",
            triangle_input_v: baseline.coordinate.triangle_input_v,
            active_device_assignment_index: assignment_index,
            mn3101_macro_hypothesis_index: baseline.coordinate.mn3101_macro_hypothesis_index,
            temperature_c: baseline.coordinate.temperature_c,
            interpretation: "exact beta-R-1 assignment continuation from the previous solved device combination",
        };
        let direct = solve_active_network_dc_coordinate_with_evaluator(
            coordinate,
            Some(&continuation_seed),
            0,
            32,
            |voltages| {
                evaluate_active_network_dc_residuals_with_reciprocal_transport(
                    coordinate, voltages, hypothesis,
                )
            },
        );
        let (point, adaptive_homotopy_used, homotopy_stages_completed) =
            if direct.converged && direct.every_unknown_inside_supply_window {
                (direct, false, 0)
            } else {
                let (homotopy, stages) =
                    solve_reciprocal_transport_endpoint_from_base(baseline, hypothesis);
                (homotopy, true, stages)
            };
        if point.converged && point.every_unknown_inside_supply_window {
            continuation_seed = point
                .node_voltages
                .iter()
                .map(|node| node.voltage_v)
                .collect();
        }
        records.push(reciprocal_transport_assignment_record(
            &point,
            adaptive_homotopy_used,
            homotopy_stages_completed,
        ));
    }
    records.sort_by_key(|point| point.active_device_assignment_index);
    records
}

fn optional_reciprocal_assignment_metric_comparison(
    records: &[BjtReciprocalTransportAssignmentPoint],
    value: impl Fn(&BjtReciprocalTransportAssignmentPoint) -> f64,
) -> Option<UncertaintyComparison> {
    if records.len() != 32 || records.iter().any(|record| !record.converged) {
        return None;
    }
    Some(compare_uncertainty_ensemble(
        &records.iter().map(value).collect::<Vec<_>>(),
        uncertainty_acceptance_policy(),
    ))
}

fn bjt_reciprocal_transport_assignment_validation(
    records: &[BjtReciprocalTransportAssignmentPoint],
) -> BjtReciprocalTransportAssignmentValidation {
    let metric_comparisons = [
        ActiveDeviceDcMetricComparison {
            metric: "exact beta-R-1 OX3 control voltage",
            unit: "volt",
            comparison: optional_reciprocal_assignment_metric_comparison(records, |point| {
                point.control_voltage_v
            }),
        },
        ActiveDeviceDcMetricComparison {
            metric: "exact beta-R-1 150 pF timing voltage",
            unit: "volt",
            comparison: optional_reciprocal_assignment_metric_comparison(records, |point| {
                point.timing_voltage_v
            }),
        },
        ActiveDeviceDcMetricComparison {
            metric: "exact beta-R-1 OX1 feedback-base voltage",
            unit: "volt",
            comparison: optional_reciprocal_assignment_metric_comparison(records, |point| {
                point.feedback_base_voltage_v
            }),
        },
        ActiveDeviceDcMetricComparison {
            metric: "exact beta-R-1 collector-terminal current magnitude sum",
            unit: "ampere",
            comparison: optional_reciprocal_assignment_metric_comparison(records, |point| {
                point.total_collector_terminal_current_absolute_a
            }),
        },
        ActiveDeviceDcMetricComparison {
            metric: "exact beta-R-1 maximum collector power",
            unit: "milliwatt",
            comparison: optional_reciprocal_assignment_metric_comparison(records, |point| {
                point.maximum_collector_power_mw
            }),
        },
    ];
    let converged_assignments = records.iter().filter(|point| point.converged).count();
    let assignments_inside_all_dc_gates = records
        .iter()
        .filter(|point| point.inside_supply_transport_and_power_gates)
        .count();
    let assignments_without_any_transfer_clamping = records
        .iter()
        .filter(|point| {
            point.forward_transfer_clamp_count == 0 && point.reverse_transfer_clamp_count == 0
        })
        .count();
    let complete_32_assignment_ensemble = records.len() == 32 && converged_assignments == 32;
    let dc_metrics_accepted_as_equivalent = complete_32_assignment_ensemble
        && assignments_inside_all_dc_gates == 32
        && assignments_without_any_transfer_clamping == 32
        && metric_comparisons.iter().all(|metric| {
            metric
                .comparison
                .is_some_and(|comparison| comparison.accepted)
        });
    BjtReciprocalTransportAssignmentValidation {
        attempted_assignments: records.len(),
        converged_assignments,
        assignments_using_adaptive_homotopy: records
            .iter()
            .filter(|point| point.adaptive_homotopy_used)
            .count(),
        assignments_inside_all_dc_gates,
        assignments_without_any_transfer_clamping,
        complete_32_assignment_ensemble,
        metric_comparisons,
        dc_metrics_accepted_as_equivalent,
        required_audio_trajectory_metrics_evaluated: false,
        accepted_as_behaviorally_equivalent: false,
        suitable_for_production_promotion: false,
        interpretation: "exact reciprocal beta-R-1 transport continues from assignment 31 toward 0; failed direct assignment transitions fall back to assignment-local sixteen-step homotopy, and only exact endpoints enter the DC uncertainty gate",
    }
}

fn factorial_record_mean<T>(records: &[&T], value: impl Fn(&T) -> f64) -> f64 {
    if records.is_empty() {
        return 0.0;
    }
    records.iter().map(|record| value(record)).sum::<f64>() / records.len() as f64
}

fn factorial_centered_sum_of_squares<T>(
    records: &[T],
    grand_mean: f64,
    value: impl Fn(&T) -> f64,
) -> f64 {
    records
        .iter()
        .map(|record| {
            let delta = value(record) - grand_mean;
            delta * delta
        })
        .sum()
}

fn factorial_spread_share_percent(main_effect: f64, total_ss: f64, n: usize) -> f64 {
    if total_ss <= f64::EPSILON || n == 0 {
        return 0.0;
    }
    100.0 * n as f64 * main_effect * main_effect / (4.0 * total_ss)
}

fn bjt_factorial_dispersion_attribution<T>(
    records: &[T],
    assignment_index: impl Fn(&T) -> usize + Copy,
    collector_current: impl Fn(&T) -> f64 + Copy,
    maximum_power: impl Fn(&T) -> f64 + Copy,
    maximum_power_designator: impl Fn(&T) -> &'static str + Copy,
    interpretation: &'static str,
) -> BjtReciprocalTransportDispersionAttribution {
    let assignment_hypotheses = active_device_hypotheses();
    let connections = mirrored_bipolar_connections();
    let shortlist = comparative_candidate_shortlist();
    let collector_current_grand_mean_a =
        records.iter().map(collector_current).sum::<f64>() / records.len().max(1) as f64;
    let maximum_power_grand_mean_mw =
        records.iter().map(maximum_power).sum::<f64>() / records.len().max(1) as f64;
    let collector_current_total_ss = factorial_centered_sum_of_squares(
        records,
        collector_current_grand_mean_a,
        collector_current,
    );
    let maximum_power_total_ss =
        factorial_centered_sum_of_squares(records, maximum_power_grand_mean_mw, maximum_power);
    let mut balanced_complete_two_level_factorial = records.len() == 32;
    let mut factor_effects = Vec::with_capacity(connections.len());
    for (device_index, connection) in connections.iter().enumerate() {
        let [first_candidate_key, second_candidate_key] = match connection.polarity {
            BipolarPolarity::Pnp => shortlist.pnp_candidates,
            BipolarPolarity::Npn => shortlist.npn_candidates,
        };
        let first_records = records
            .iter()
            .filter(|record| {
                let assignment = assignment_hypotheses[assignment_index(record)];
                active_network_dc_candidate(assignment, device_index) == first_candidate_key
            })
            .collect::<Vec<_>>();
        let second_records = records
            .iter()
            .filter(|record| {
                let assignment = assignment_hypotheses[assignment_index(record)];
                active_network_dc_candidate(assignment, device_index) == second_candidate_key
            })
            .collect::<Vec<_>>();
        balanced_complete_two_level_factorial &=
            first_records.len() == 16 && second_records.len() == 16;
        let collector_current_first_candidate_mean_a =
            factorial_record_mean(&first_records, collector_current);
        let collector_current_second_candidate_mean_a =
            factorial_record_mean(&second_records, collector_current);
        let collector_current_main_effect_a =
            collector_current_second_candidate_mean_a - collector_current_first_candidate_mean_a;
        let maximum_power_first_candidate_mean_mw =
            factorial_record_mean(&first_records, maximum_power);
        let maximum_power_second_candidate_mean_mw =
            factorial_record_mean(&second_records, maximum_power);
        let maximum_power_main_effect_mw =
            maximum_power_second_candidate_mean_mw - maximum_power_first_candidate_mean_mw;
        factor_effects.push(BjtReciprocalTransportFactorEffect {
            designator: connection.channel_a_designator,
            polarity: connection.polarity,
            first_candidate_key,
            second_candidate_key,
            samples_per_candidate: first_records.len().min(second_records.len()),
            collector_current_first_candidate_mean_a,
            collector_current_second_candidate_mean_a,
            collector_current_main_effect_a,
            collector_current_main_effect_relative_to_grand_mean_percent: 100.0
                * collector_current_main_effect_a.abs()
                / collector_current_grand_mean_a.abs().max(f64::EPSILON),
            collector_current_centered_spread_share_percent: factorial_spread_share_percent(
                collector_current_main_effect_a,
                collector_current_total_ss,
                records.len(),
            ),
            maximum_power_first_candidate_mean_mw,
            maximum_power_second_candidate_mean_mw,
            maximum_power_main_effect_mw,
            maximum_power_main_effect_relative_to_grand_mean_percent: 100.0
                * maximum_power_main_effect_mw.abs()
                / maximum_power_grand_mean_mw.abs().max(f64::EPSILON),
            maximum_power_centered_spread_share_percent: factorial_spread_share_percent(
                maximum_power_main_effect_mw,
                maximum_power_total_ss,
                records.len(),
            ),
        });
    }
    let collector_current_main_effect_spread_share_percent = factor_effects
        .iter()
        .map(|effect| effect.collector_current_centered_spread_share_percent)
        .sum::<f64>();
    let maximum_power_main_effect_spread_share_percent = factor_effects
        .iter()
        .map(|effect| effect.maximum_power_centered_spread_share_percent)
        .sum::<f64>();
    let dominant_collector_current_designator = factor_effects
        .iter()
        .max_by(|left, right| {
            left.collector_current_centered_spread_share_percent
                .total_cmp(&right.collector_current_centered_spread_share_percent)
        })
        .map_or("none", |effect| effect.designator);
    let dominant_maximum_power_designator = factor_effects
        .iter()
        .max_by(|left, right| {
            left.maximum_power_centered_spread_share_percent
                .total_cmp(&right.maximum_power_centered_spread_share_percent)
        })
        .map_or("none", |effect| effect.designator);
    let minimum_collector_current_assignment_index = records
        .iter()
        .min_by(|left, right| collector_current(left).total_cmp(&collector_current(right)))
        .map_or(0, assignment_index);
    let maximum_collector_current_assignment_index = records
        .iter()
        .max_by(|left, right| collector_current(left).total_cmp(&collector_current(right)))
        .map_or(0, assignment_index);
    let minimum_maximum_power_assignment_index = records
        .iter()
        .min_by(|left, right| maximum_power(left).total_cmp(&maximum_power(right)))
        .map_or(0, assignment_index);
    let maximum_maximum_power_assignment_index = records
        .iter()
        .max_by(|left, right| maximum_power(left).total_cmp(&maximum_power(right)))
        .map_or(0, assignment_index);
    let maximum_power_device_designator = records.first().map_or("none", maximum_power_designator);
    let maximum_power_device_is_constant_across_assignments = records
        .iter()
        .all(|record| maximum_power_designator(record) == maximum_power_device_designator);
    BjtReciprocalTransportDispersionAttribution {
        assignment_count: records.len(),
        balanced_complete_two_level_factorial,
        collector_current_grand_mean_a,
        maximum_power_grand_mean_mw,
        factor_effects,
        collector_current_main_effect_spread_share_percent,
        collector_current_interaction_spread_share_percent: (100.0
            - collector_current_main_effect_spread_share_percent)
            .clamp(0.0, 100.0),
        maximum_power_main_effect_spread_share_percent,
        maximum_power_interaction_spread_share_percent: (100.0
            - maximum_power_main_effect_spread_share_percent)
            .clamp(0.0, 100.0),
        dominant_collector_current_designator,
        dominant_maximum_power_designator,
        minimum_collector_current_assignment_index,
        maximum_collector_current_assignment_index,
        minimum_maximum_power_assignment_index,
        maximum_maximum_power_assignment_index,
        maximum_power_device_designator,
        maximum_power_device_is_constant_across_assignments,
        attribution_is_numerical_sensitivity_not_device_identity: true,
        suitable_for_production_parameter_narrowing: false,
        interpretation,
    }
}

fn bjt_reciprocal_transport_dispersion_attribution(
    records: &[BjtReciprocalTransportAssignmentPoint],
) -> BjtReciprocalTransportDispersionAttribution {
    bjt_factorial_dispersion_attribution(
        records,
        |record| record.active_device_assignment_index,
        |record| record.total_collector_terminal_current_absolute_a,
        |record| record.maximum_collector_power_mw,
        |record| record.maximum_collector_power_designator,
        "orthogonal main-effect shares decompose deterministic spread over the complete two-level 32-assignment factorial; residual spread is assigned to interactions, and neither contribution identifies the installed devices or licenses parameter narrowing",
    )
}

fn bjt_correlated_reverse_transport_dispersion_attribution(
    records: &[BjtCorrelatedReverseTransportResolvePoint],
) -> BjtReciprocalTransportDispersionAttribution {
    bjt_factorial_dispersion_attribution(
        records,
        |record| record.active_device_assignment_index,
        |record| record.total_collector_terminal_current_absolute_a,
        |record| record.maximum_collector_power_mw,
        |record| record.maximum_collector_power_designator,
        "orthogonal main effects are recomputed only after the correlated target has fully re-solved all 13 KCL nodes; they prioritize future evidence axes but neither identify installed devices nor authorize unpublished parameter narrowing",
    )
}

fn reciprocal_transport_device_evidence_envelope<T, F>(
    records: &[T],
    device_points: F,
    designator: &'static str,
    polarity: BipolarPolarity,
    candidate_key: &'static str,
) -> BjtReciprocalTransportDeviceEvidenceEnvelope
where
    F: for<'a> Fn(&'a T) -> &'a [BjtReciprocalTransportAssignmentDevicePoint] + Copy,
{
    const MINIMUM_SOLVER_RESOLUTION_MARGIN_RADII: f64 = 2.0;
    let points = records
        .iter()
        .flat_map(|record| device_points(record).iter())
        .filter(|point| point.designator == designator && point.candidate_key == candidate_key)
        .collect::<Vec<_>>();
    let output_evidence = bjt_output_characteristic_plot_evidence()
        .into_iter()
        .find(|evidence| evidence.candidate_key == candidate_key)
        .expect("every assignment candidate has output-characteristic evidence");
    let voltage_reading_uncertainty_v = output_evidence.reading_radius_px
        * (output_evidence.collector_emitter_voltage_right_v
            - output_evidence.collector_emitter_voltage_left_v)
            .abs()
        / (output_evidence.x_right_px - output_evidence.x_left_px).abs();
    let minimum_vce_v = points
        .iter()
        .map(|point| point.collector_emitter_voltage_absolute_v)
        .fold(f64::INFINITY, f64::min);
    let maximum_vce_v = points
        .iter()
        .map(|point| point.collector_emitter_voltage_absolute_v)
        .fold(f64::NEG_INFINITY, f64::max);
    let minimum_vbe_v = points
        .iter()
        .map(|point| point.base_emitter_voltage_absolute_v)
        .fold(f64::INFINITY, f64::min);
    let maximum_vbe_v = points
        .iter()
        .map(|point| point.base_emitter_voltage_absolute_v)
        .fold(f64::NEG_INFINITY, f64::max);
    let minimum_vbc_v = points
        .iter()
        .map(|point| point.base_collector_voltage_absolute_v)
        .fold(f64::INFINITY, f64::min);
    let maximum_vbc_v = points
        .iter()
        .map(|point| point.base_collector_voltage_absolute_v)
        .fold(f64::NEG_INFINITY, f64::max);
    let minimum_collector_current_a = points
        .iter()
        .map(|point| point.collector_current_absolute_a)
        .fold(f64::INFINITY, f64::min);
    let maximum_collector_current_a = points
        .iter()
        .map(|point| point.collector_current_absolute_a)
        .fold(0.0, f64::max);
    let minimum_base_current_a = points
        .iter()
        .map(|point| point.base_current_absolute_a)
        .fold(f64::INFINITY, f64::min);
    let maximum_base_current_a = points
        .iter()
        .map(|point| point.base_current_absolute_a)
        .fold(0.0, f64::max);
    let minimum_reverse_transport_current_a = points
        .iter()
        .map(|point| point.reverse_transport_current_absolute_a)
        .fold(f64::INFINITY, f64::min);
    let maximum_reverse_transport_current_a = points
        .iter()
        .map(|point| point.reverse_transport_current_absolute_a)
        .fold(0.0, f64::max);
    let maximum_collector_power_mw = points
        .iter()
        .map(|point| point.collector_power_mw)
        .fold(0.0, f64::max);
    let power_limit_25c_mw = bjt_collector_power_limit_mw(candidate_key, 25.0)
        .expect("every candidate has a direct 25 C power rating");
    let maximum_power_fraction_of_25c_limit_percent =
        100.0 * maximum_collector_power_mw / power_limit_25c_mw;
    let power_derating_critical_ambient_c =
        (125.0 - 100.0 * maximum_collector_power_mw / power_limit_25c_mw).clamp(25.0, 125.0);
    let minimum_output_plot_voltage_resolution_radii =
        minimum_vce_v / voltage_reading_uncertainty_v;
    let every_vce_inside_output_plot_domain = points.iter().all(|point| {
        point.collector_emitter_voltage_absolute_v
            >= output_evidence.collector_emitter_voltage_left_v
            && point.collector_emitter_voltage_absolute_v
                <= output_evidence.collector_emitter_voltage_right_v
    });
    let every_collector_current_inside_output_plot_domain = points.iter().all(|point| {
        point.collector_current_absolute_a >= output_evidence.collector_current_bottom_a
            && point.collector_current_absolute_a <= output_evidence.collector_current_top_a
    });
    let minimum_labeled_base_current_a = output_evidence
        .base_current_curve_levels_a
        .iter()
        .copied()
        .fold(f64::INFINITY, f64::min);
    let maximum_labeled_base_current_a = output_evidence
        .base_current_curve_levels_a
        .iter()
        .copied()
        .fold(0.0, f64::max);
    let every_base_current_inside_labeled_curve_domain = points.iter().all(|point| {
        point.base_current_absolute_a >= minimum_labeled_base_current_a
            && point.base_current_absolute_a <= maximum_labeled_base_current_a
    });
    let every_coordinate_resolvable_at_two_reading_radii = every_vce_inside_output_plot_domain
        && every_collector_current_inside_output_plot_domain
        && minimum_output_plot_voltage_resolution_radii >= MINIMUM_SOLVER_RESOLUTION_MARGIN_RADII;
    let output_characteristic_surface_digitized =
        bjt_output_characteristic_evidence_validation().digitized_output_surface_ready;
    let direct_three_temperature_transfer_available = bjt_published_thermal_evidence()
        .into_iter()
        .find(|thermal| thermal.candidate_key == candidate_key)
        .is_some_and(|thermal| thermal.transfer_curve_temperatures_c.len() == 3);
    let comparative_temperature_interval_available = bjt_comparative_thermal_envelope_validations()
        .iter()
        .any(|validation| {
            validation.target_candidate_key == candidate_key
                && validation.usable_as_comparative_sweep_interval
        });
    BjtReciprocalTransportDeviceEvidenceEnvelope {
        designator,
        polarity,
        candidate_key,
        sample_count: points.len(),
        minimum_vce_v,
        maximum_vce_v,
        minimum_vbe_v,
        maximum_vbe_v,
        minimum_vbc_v,
        maximum_vbc_v,
        minimum_collector_current_a,
        maximum_collector_current_a,
        minimum_base_current_a,
        maximum_base_current_a,
        minimum_reverse_transport_current_a,
        maximum_reverse_transport_current_a,
        maximum_collector_power_mw,
        maximum_power_fraction_of_25c_limit_percent,
        power_derating_critical_ambient_c,
        minimum_output_plot_voltage_resolution_radii,
        every_vce_inside_output_plot_domain,
        every_collector_current_inside_output_plot_domain,
        every_base_current_inside_labeled_curve_domain,
        every_coordinate_resolvable_at_two_reading_radii,
        output_characteristic_surface_digitized,
        direct_three_temperature_transfer_available,
        comparative_temperature_interval_available,
        evidence_directly_bounds_operating_region: every_coordinate_resolvable_at_two_reading_radii
            && every_base_current_inside_labeled_curve_domain
            && output_characteristic_surface_digitized,
    }
}

fn bjt_reciprocal_transport_device_evidence_envelopes(
    records: &[BjtReciprocalTransportAssignmentPoint],
) -> Vec<BjtReciprocalTransportDeviceEvidenceEnvelope> {
    let shortlist = comparative_candidate_shortlist();
    mirrored_bipolar_connections()
        .into_iter()
        .flat_map(|connection| {
            let candidates = match connection.polarity {
                BipolarPolarity::Pnp => shortlist.pnp_candidates,
                BipolarPolarity::Npn => shortlist.npn_candidates,
            };
            candidates.map(|candidate_key| {
                reciprocal_transport_device_evidence_envelope(
                    records,
                    |record| &record.device_points,
                    connection.channel_a_designator,
                    connection.polarity,
                    candidate_key,
                )
            })
        })
        .collect()
}

fn bjt_correlated_reverse_transport_device_evidence_envelopes(
    records: &[BjtCorrelatedReverseTransportResolvePoint],
) -> Vec<BjtReciprocalTransportDeviceEvidenceEnvelope> {
    let shortlist = comparative_candidate_shortlist();
    mirrored_bipolar_connections()
        .into_iter()
        .flat_map(|connection| {
            let candidates = match connection.polarity {
                BipolarPolarity::Pnp => shortlist.pnp_candidates,
                BipolarPolarity::Npn => shortlist.npn_candidates,
            };
            candidates.map(|candidate_key| {
                reciprocal_transport_device_evidence_envelope(
                    records,
                    |record| &record.device_points,
                    connection.channel_a_designator,
                    connection.polarity,
                    candidate_key,
                )
            })
        })
        .collect()
}

fn bjt_reciprocal_transport_tr19_evidence_audit(
    envelopes: &[BjtReciprocalTransportDeviceEvidenceEnvelope],
) -> BjtReciprocalTransportTr19EvidenceAudit {
    let tr19 = envelopes
        .iter()
        .filter(|envelope| envelope.designator == "Tr19")
        .collect::<Vec<_>>();
    let tr19_candidates_inside_output_plot_voltage_domain = tr19
        .iter()
        .filter(|envelope| envelope.every_vce_inside_output_plot_domain)
        .count();
    let tr19_candidates_inside_output_plot_current_domain = tr19
        .iter()
        .filter(|envelope| envelope.every_collector_current_inside_output_plot_domain)
        .count();
    let tr19_candidates_inside_labeled_base_current_domain = tr19
        .iter()
        .filter(|envelope| envelope.every_base_current_inside_labeled_curve_domain)
        .count();
    let tr19_candidates_resolvable_at_two_reading_radii = tr19
        .iter()
        .filter(|envelope| envelope.every_coordinate_resolvable_at_two_reading_radii)
        .count();
    let tr19_candidates_with_direct_thermal_transfer = tr19
        .iter()
        .filter(|envelope| envelope.direct_three_temperature_transfer_available)
        .count();
    let tr19_candidates_with_comparative_thermal_interval = tr19
        .iter()
        .filter(|envelope| envelope.comparative_temperature_interval_available)
        .count();
    let tr19_candidates_inside_25c_power_limit = tr19
        .iter()
        .filter(|envelope| envelope.maximum_power_fraction_of_25c_limit_percent <= 100.0)
        .count();
    let tr19_operating_region_directly_bounded_by_current_evidence = tr19
        .iter()
        .all(|envelope| envelope.evidence_directly_bounds_operating_region);
    let existing_documents_sufficient_to_narrow_reciprocal_current_power_spread =
        tr19_operating_region_directly_bounded_by_current_evidence
            && tr19_candidates_with_direct_thermal_transfer == tr19.len();
    BjtReciprocalTransportTr19EvidenceAudit {
        evaluated_envelopes: envelopes.len(),
        tr19_candidate_envelopes: tr19.len(),
        tr19_samples: tr19.iter().map(|envelope| envelope.sample_count).sum(),
        tr19_candidates_inside_output_plot_voltage_domain,
        tr19_candidates_inside_output_plot_current_domain,
        tr19_candidates_inside_labeled_base_current_domain,
        tr19_candidates_resolvable_at_two_reading_radii,
        tr19_candidates_with_direct_thermal_transfer,
        tr19_candidates_with_comparative_thermal_interval,
        tr19_candidates_inside_25c_power_limit,
        tr19_operating_region_directly_bounded_by_current_evidence,
        existing_documents_sufficient_to_narrow_reciprocal_current_power_spread,
        global_output_conductance_evidence_required:
            !tr19_operating_region_directly_bounded_by_current_evidence,
        higher_voltage_output_characteristic_evidence_required:
            tr19_candidates_inside_output_plot_voltage_domain < tr19.len(),
        suitable_for_production_parameter_narrowing: false,
        interpretation: "the dominant Tr19 reciprocal endpoints are checked against archived IC-VCE plot rectangles, labeled IB range, pixel resolution, direct power derating and direct/comparative thermal coverage; passing absolute power is not a bound on high-VCE output conductance or installed identity",
    }
}

fn bjt_reciprocal_transport_tr22_tr23_evidence_audit(
    envelopes: &[BjtReciprocalTransportDeviceEvidenceEnvelope],
) -> BjtReciprocalTransportTr22Tr23EvidenceAudit {
    let relevant = envelopes
        .iter()
        .filter(|envelope| ["Tr22", "Tr23"].contains(&envelope.designator))
        .collect::<Vec<_>>();
    let tr22 = relevant
        .iter()
        .copied()
        .filter(|envelope| envelope.designator == "Tr22")
        .collect::<Vec<_>>();
    let tr23 = relevant
        .iter()
        .copied()
        .filter(|envelope| envelope.designator == "Tr23")
        .collect::<Vec<_>>();
    let tr22_candidates_inside_output_plot_voltage_domain = tr22
        .iter()
        .filter(|envelope| envelope.every_vce_inside_output_plot_domain)
        .count();
    let tr22_candidates_inside_output_plot_current_domain = tr22
        .iter()
        .filter(|envelope| envelope.every_collector_current_inside_output_plot_domain)
        .count();
    let tr22_candidates_inside_labeled_base_current_domain = tr22
        .iter()
        .filter(|envelope| envelope.every_base_current_inside_labeled_curve_domain)
        .count();
    let tr22_candidates_resolvable_at_two_reading_radii = tr22
        .iter()
        .filter(|envelope| envelope.every_coordinate_resolvable_at_two_reading_radii)
        .count();
    let tr23_candidates_with_negative_vce = tr23
        .iter()
        .filter(|envelope| envelope.maximum_vce_v < 0.0)
        .count();
    let tr23_candidates_inside_forward_output_plot_domain = tr23
        .iter()
        .filter(|envelope| envelope.every_vce_inside_output_plot_domain)
        .count();
    let tr23_candidates_with_reverse_transport_at_every_sample = tr23
        .iter()
        .filter(|envelope| envelope.minimum_reverse_transport_current_a > 0.0)
        .count();
    let candidates_with_direct_thermal_transfer = relevant
        .iter()
        .filter(|envelope| envelope.direct_three_temperature_transfer_available)
        .count();
    let candidates_with_only_comparative_thermal_interval = relevant
        .iter()
        .filter(|envelope| {
            !envelope.direct_three_temperature_transfer_available
                && envelope.comparative_temperature_interval_available
        })
        .count();
    let digitized_output_surfaces_available = relevant
        .iter()
        .filter(|envelope| envelope.output_characteristic_surface_digitized)
        .count();
    let tr22_output_surface_digitization_required = tr22_candidates_resolvable_at_two_reading_radii
        == tr22.len()
        && digitized_output_surfaces_available == 0;
    let tr22_mitsubishi_base_drive_extension_evidence_required = tr22.iter().any(|envelope| {
        envelope.candidate_key == "2SC2603-F"
            && !envelope.every_base_current_inside_labeled_curve_domain
    });
    let tr23_reverse_transport_evidence_required = tr23_candidates_with_negative_vce == tr23.len()
        && tr23_candidates_with_reverse_transport_at_every_sample == tr23.len();
    let existing_documents_sufficient_to_bound_remaining_current_spread = tr22
        .iter()
        .all(|envelope| envelope.evidence_directly_bounds_operating_region)
        && tr23
            .iter()
            .all(|envelope| envelope.evidence_directly_bounds_operating_region)
        && !tr23_reverse_transport_evidence_required;
    BjtReciprocalTransportTr22Tr23EvidenceAudit {
        evaluated_candidate_envelopes: relevant.len(),
        evaluated_device_samples: relevant.iter().map(|envelope| envelope.sample_count).sum(),
        tr22_candidates_inside_output_plot_voltage_domain,
        tr22_candidates_inside_output_plot_current_domain,
        tr22_candidates_inside_labeled_base_current_domain,
        tr22_candidates_resolvable_at_two_reading_radii,
        tr23_candidates_with_negative_vce,
        tr23_candidates_inside_forward_output_plot_domain,
        tr23_candidates_with_reverse_transport_at_every_sample,
        candidates_with_direct_thermal_transfer,
        candidates_with_only_comparative_thermal_interval,
        digitized_output_surfaces_available,
        tr22_output_surface_digitization_required,
        tr22_mitsubishi_base_drive_extension_evidence_required,
        tr23_reverse_transport_evidence_required,
        existing_documents_sufficient_to_bound_remaining_current_spread,
        suitable_for_production_parameter_narrowing: false,
        interpretation: "Tr22 is separated into resolvable forward-output coordinates versus missing surface/base-drive coverage, while Tr23 is classified as reverse-VCE reciprocal transport outside the forward common-emitter output plots; neither diagnostic infers reverse beta or extends a printed curve",
    }
}

fn bjt_correlated_reverse_transport_tr19_evidence_audit(
    envelopes: &[BjtReciprocalTransportDeviceEvidenceEnvelope],
) -> BjtReciprocalTransportTr19EvidenceAudit {
    bjt_reciprocal_transport_tr19_evidence_audit(envelopes)
}

fn bjt_correlated_reverse_transport_tr22_tr23_evidence_audit(
    envelopes: &[BjtReciprocalTransportDeviceEvidenceEnvelope],
) -> BjtReciprocalTransportTr22Tr23EvidenceAudit {
    bjt_reciprocal_transport_tr22_tr23_evidence_audit(envelopes)
}

fn tr22_tr23_reverse_beta_current_comparison(
    records: &[BjtReciprocalTransportAssignmentPoint],
    tr19_differential_conductance_us: f64,
    tr22_reverse_beta: f64,
    tr23_reverse_beta: f64,
) -> UncertaintyComparison {
    const REFERENCE_VOLTAGE_V: f64 = 6.0;
    const PNP_FIRST_CANDIDATE: &str = "2SA1015-GR/Y";
    const NPN_FIRST_CANDIDATE: &str = "2SC1815-GR/Y";
    let tr19_conductance_s = tr19_differential_conductance_us * 1.0e-6;
    let mut total_currents_a = Vec::with_capacity(records.len());
    for record in records {
        let mut corrected_total_a = record.total_collector_terminal_current_absolute_a;
        for device in &record.device_points {
            if device.designator == "Tr19" && device.candidate_key == PNP_FIRST_CANDIDATE {
                corrected_total_a += tr19_conductance_s
                    * (device.collector_emitter_voltage_absolute_v - REFERENCE_VOLTAGE_V).max(0.0);
            }
            let reverse_beta = match (device.designator, device.candidate_key) {
                ("Tr22", NPN_FIRST_CANDIDATE) => Some(tr22_reverse_beta),
                ("Tr23", NPN_FIRST_CANDIDATE) => Some(tr23_reverse_beta),
                _ => None,
            };
            if let Some(reverse_beta) = reverse_beta {
                let corrected_signed_current_a = device.collector_current_signed_a
                    + device.reverse_transport_current_absolute_a * (1.0 - 1.0 / reverse_beta);
                corrected_total_a +=
                    corrected_signed_current_a.abs() - device.collector_current_absolute_a;
            }
        }
        total_currents_a.push(corrected_total_a);
    }
    compare_uncertainty_ensemble(&total_currents_a, uncertainty_acceptance_policy())
}

fn bjt_tr22_tr23_reverse_beta_requirement(
    records: &[BjtReciprocalTransportAssignmentPoint],
) -> BjtTr22Tr23ReverseBetaRequirement {
    const TR19_CONDUCTANCE_US: f64 = 1.667;
    const MINIMUM_BETA: f64 = 0.25;
    const MAXIMUM_BETA: f64 = 1.0;
    const BETA_STEP: f64 = 0.0025;
    const BETA_STEPS: usize = 300;
    let power_comparison = tr19_high_vce_conductance_comparisons(records, TR19_CONDUCTANCE_US)[1];
    let mut current_accepted_beta_pairs = 0;
    let mut joint_current_power_accepted_beta_pairs = 0;
    let mut tr22_accepted_reverse_beta_minimum = None;
    let mut tr22_accepted_reverse_beta_maximum = None;
    let mut tr23_accepted_reverse_beta_minimum = None;
    let mut tr23_accepted_reverse_beta_maximum = None;
    let mut best_tr22_reverse_beta = MAXIMUM_BETA;
    let mut best_tr23_reverse_beta = MAXIMUM_BETA;
    let mut best_current_comparison = tr22_tr23_reverse_beta_current_comparison(
        records,
        TR19_CONDUCTANCE_US,
        MAXIMUM_BETA,
        MAXIMUM_BETA,
    );
    let mut best_score = f64::INFINITY;
    for tr22_step in 0..=BETA_STEPS {
        let tr22_reverse_beta = MINIMUM_BETA + tr22_step as f64 * BETA_STEP;
        for tr23_step in 0..=BETA_STEPS {
            let tr23_reverse_beta = MINIMUM_BETA + tr23_step as f64 * BETA_STEP;
            let current = tr22_tr23_reverse_beta_current_comparison(
                records,
                TR19_CONDUCTANCE_US,
                tr22_reverse_beta,
                tr23_reverse_beta,
            );
            if current.accepted {
                current_accepted_beta_pairs += 1;
                joint_current_power_accepted_beta_pairs += usize::from(power_comparison.accepted);
                tr22_accepted_reverse_beta_minimum = Some(
                    tr22_accepted_reverse_beta_minimum
                        .map_or(tr22_reverse_beta, |value: f64| value.min(tr22_reverse_beta)),
                );
                tr22_accepted_reverse_beta_maximum = Some(
                    tr22_accepted_reverse_beta_maximum
                        .map_or(tr22_reverse_beta, |value: f64| value.max(tr22_reverse_beta)),
                );
                tr23_accepted_reverse_beta_minimum = Some(
                    tr23_accepted_reverse_beta_minimum
                        .map_or(tr23_reverse_beta, |value: f64| value.min(tr23_reverse_beta)),
                );
                tr23_accepted_reverse_beta_maximum = Some(
                    tr23_accepted_reverse_beta_maximum
                        .map_or(tr23_reverse_beta, |value: f64| value.max(tr23_reverse_beta)),
                );
            }
            let score = (current.mean_absolute_relative_deviation_percent
                / uncertainty_acceptance_policy().mean_relative_deviation_limit_percent)
                .max(
                    current.maximum_absolute_relative_deviation_percent
                        / uncertainty_acceptance_policy().maximum_relative_deviation_limit_percent,
                );
            if score < best_score {
                best_score = score;
                best_tr22_reverse_beta = tr22_reverse_beta;
                best_tr23_reverse_beta = tr23_reverse_beta;
                best_current_comparison = current;
            }
        }
    }
    BjtTr22Tr23ReverseBetaRequirement {
        tr19_differential_conductance_us: TR19_CONDUCTANCE_US,
        tr19_conductance_inside_power_acceptance_interval: power_comparison.accepted,
        first_candidate_key: "2SC1815-GR/Y",
        second_candidate_key: "2SC2603-F",
        second_candidate_reverse_beta_held_at: 1.0,
        minimum_screened_reverse_beta: MINIMUM_BETA,
        maximum_screened_reverse_beta: MAXIMUM_BETA,
        reverse_beta_step: BETA_STEP,
        evaluated_beta_pairs: (BETA_STEPS + 1) * (BETA_STEPS + 1),
        current_accepted_beta_pairs,
        joint_current_power_accepted_beta_pairs,
        tr22_accepted_reverse_beta_minimum,
        tr22_accepted_reverse_beta_maximum,
        tr23_accepted_reverse_beta_minimum,
        tr23_accepted_reverse_beta_maximum,
        best_tr22_reverse_beta,
        best_tr23_reverse_beta,
        best_current_comparison,
        power_comparison,
        accepted_region_is_not_rectangular_bound: current_accepted_beta_pairs > 0,
        direct_reverse_beta_evidence_available: false,
        frozen_node_screen_omits_base_kcl_resolve: true,
        suitable_for_solver_substitution: false,
        suitable_for_production_parameter_narrowing: false,
        interpretation: "with Tr19 fixed at the center of its frozen-node power interval, candidate-differential reverse beta for Toshiba Tr22/Tr23 is screened against Mitsubishi beta-R=1; accepted projections describe a correlated requirement region, not independent parameter bounds, and omit the mandatory base-current KCL re-solve",
    }
}

fn bjt_correlated_reverse_transport_frozen_residual_points(
    records: &[BjtReciprocalTransportAssignmentPoint],
    requirement: BjtTr22Tr23ReverseBetaRequirement,
) -> Vec<BjtCorrelatedReverseTransportFrozenResidualPoint> {
    const REFERENCE_VOLTAGE_V: f64 = 6.0;
    const FIRST_PNP_CANDIDATE: &str = "2SA1015-GR/Y";
    const FIRST_NPN_CANDIDATE: &str = "2SC1815-GR/Y";
    const ACCEPTED_RESIDUAL_A: f64 = 1.0e-7;
    let tr19_conductance_s = requirement.tr19_differential_conductance_us * 1.0e-6;
    records
        .iter()
        .map(|record| {
            let mut residual_deltas_a = vec![0.0; ACTIVE_NETWORK_DC_UNKNOWNS.len()];
            let mut changed_device_count = 0;
            let mut tr19_collector_current_delta_a = 0.0;
            let mut tr22_base_current_delta_a = 0.0;
            let mut tr23_base_current_delta_a = 0.0;
            for device in &record.device_points {
                let connection = mirrored_bipolar_connections()
                    .into_iter()
                    .find(|connection| connection.channel_a_designator == device.designator)
                    .expect("every serialized device has an audited connection");
                if device.designator == "Tr19" && device.candidate_key == FIRST_PNP_CANDIDATE {
                    let collector_delta_a = tr19_conductance_s
                        * (device.collector_emitter_voltage_absolute_v - REFERENCE_VOLTAGE_V)
                            .max(0.0);
                    let collector_current_leaving_delta_a = -collector_delta_a;
                    add_active_network_dc_current(
                        &mut residual_deltas_a,
                        connection.collector,
                        collector_current_leaving_delta_a,
                    );
                    add_active_network_dc_current(
                        &mut residual_deltas_a,
                        connection.emitter,
                        -collector_current_leaving_delta_a,
                    );
                    tr19_collector_current_delta_a = collector_delta_a;
                    changed_device_count += 1;
                }
                let reverse_beta = match (device.designator, device.candidate_key) {
                    ("Tr22", FIRST_NPN_CANDIDATE) => Some(requirement.best_tr22_reverse_beta),
                    ("Tr23", FIRST_NPN_CANDIDATE) => Some(requirement.best_tr23_reverse_beta),
                    _ => None,
                };
                if let Some(reverse_beta) = reverse_beta {
                    let collector_current_delta_a =
                        device.reverse_transport_current_absolute_a * (1.0 - 1.0 / reverse_beta);
                    let base_current_delta_a =
                        device.reverse_transport_current_absolute_a * (1.0 / reverse_beta - 1.0);
                    add_active_network_dc_current(
                        &mut residual_deltas_a,
                        connection.collector,
                        collector_current_delta_a,
                    );
                    add_active_network_dc_current(
                        &mut residual_deltas_a,
                        connection.base,
                        base_current_delta_a,
                    );
                    add_active_network_dc_current(
                        &mut residual_deltas_a,
                        connection.emitter,
                        -(collector_current_delta_a + base_current_delta_a),
                    );
                    match device.designator {
                        "Tr22" => tr22_base_current_delta_a = base_current_delta_a,
                        "Tr23" => tr23_base_current_delta_a = base_current_delta_a,
                        _ => unreachable!("only Tr22 and Tr23 receive reverse-beta deltas"),
                    }
                    changed_device_count += 1;
                }
            }
            let (maximum_parameter_delta_kcl_residual_node, maximum_signed_delta_a) =
                ACTIVE_NETWORK_DC_UNKNOWNS
                    .iter()
                    .copied()
                    .zip(residual_deltas_a.iter().copied())
                    .max_by(|left, right| left.1.abs().total_cmp(&right.1.abs()))
                    .expect("the residual system has audited unknowns");
            let maximum_parameter_delta_kcl_residual_a = maximum_signed_delta_a.abs();
            let rms_parameter_delta_kcl_residual_a = (residual_deltas_a
                .iter()
                .map(|residual| residual * residual)
                .sum::<f64>()
                / residual_deltas_a.len() as f64)
                .sqrt();
            BjtCorrelatedReverseTransportFrozenResidualPoint {
                active_device_assignment_index: record.active_device_assignment_index,
                changed_device_count,
                tr19_collector_current_delta_a,
                tr22_base_current_delta_a,
                tr23_base_current_delta_a,
                maximum_parameter_delta_kcl_residual_a,
                rms_parameter_delta_kcl_residual_a,
                maximum_parameter_delta_kcl_residual_node,
                exceeds_solver_kcl_acceptance: maximum_parameter_delta_kcl_residual_a
                    > ACCEPTED_RESIDUAL_A,
            }
        })
        .collect()
}

fn bjt_correlated_reverse_transport_frozen_residual_audit(
    points: &[BjtCorrelatedReverseTransportFrozenResidualPoint],
) -> BjtCorrelatedReverseTransportFrozenResidualAudit {
    const ACCEPTED_RESIDUAL_A: f64 = 1.0e-7;
    let worst = points
        .iter()
        .max_by(|left, right| {
            left.maximum_parameter_delta_kcl_residual_a
                .total_cmp(&right.maximum_parameter_delta_kcl_residual_a)
        })
        .expect("the complete assignment screen is nonempty");
    let assignments_exceeding_solver_kcl_acceptance = points
        .iter()
        .filter(|point| point.exceeds_solver_kcl_acceptance)
        .count();
    BjtCorrelatedReverseTransportFrozenResidualAudit {
        evaluated_assignments: points.len(),
        assignments_without_parameter_delta: points
            .iter()
            .filter(|point| point.changed_device_count == 0)
            .count(),
        assignments_exceeding_solver_kcl_acceptance,
        solver_kcl_acceptance_a: ACCEPTED_RESIDUAL_A,
        maximum_parameter_delta_kcl_residual_a: worst.maximum_parameter_delta_kcl_residual_a,
        maximum_parameter_delta_kcl_residual_multiple: worst.maximum_parameter_delta_kcl_residual_a
            / ACCEPTED_RESIDUAL_A,
        worst_assignment_index: worst.active_device_assignment_index,
        worst_residual_node: worst.maximum_parameter_delta_kcl_residual_node,
        maximum_tr19_collector_current_delta_a: points
            .iter()
            .map(|point| point.tr19_collector_current_delta_a)
            .fold(0.0, f64::max),
        maximum_tr22_base_current_delta_a: points
            .iter()
            .map(|point| point.tr22_base_current_delta_a)
            .fold(0.0, f64::max),
        maximum_tr23_base_current_delta_a: points
            .iter()
            .map(|point| point.tr23_base_current_delta_a)
            .fold(0.0, f64::max),
        frozen_node_requirement_is_kcl_compatible: assignments_exceeding_solver_kcl_acceptance == 0,
        full_nonlinear_base_and_collector_kcl_resolve_required:
            assignments_exceeding_solver_kcl_acceptance > 0,
        direct_parameter_evidence_available: false,
        suitable_for_solver_substitution: false,
        suitable_for_production_parameter_narrowing: false,
        interpretation: "the best frozen-node current/power pair is restamped into the audited KCL nodes without moving voltages; its parameter-induced residual quantifies omitted base/collector feedback and decides whether a full nonlinear re-solve is mandatory, not whether the unpublished parameters are physically correct",
    }
}

fn correlated_reverse_transport_parameters(
    requirement: BjtTr22Tr23ReverseBetaRequirement,
    blend_fraction: f64,
) -> BjtReciprocalTransportParameters {
    BjtReciprocalTransportParameters {
        hypothesis_index: None,
        default_reverse_beta: 1.0,
        tr19_toshiba_differential_conductance_us: requirement.tr19_differential_conductance_us
            * blend_fraction,
        tr22_toshiba_reverse_beta: 1.0
            + blend_fraction * (requirement.best_tr22_reverse_beta - 1.0),
        tr23_toshiba_reverse_beta: 1.0
            + blend_fraction * (requirement.best_tr23_reverse_beta - 1.0),
    }
}

fn bjt_correlated_reverse_transport_resolve_points(
    records: &[BjtReciprocalTransportAssignmentPoint],
    requirement: BjtTr22Tr23ReverseBetaRequirement,
) -> Vec<BjtCorrelatedReverseTransportResolvePoint> {
    const HOMOTOPY_STAGES: usize = 16;
    let target_parameters = correlated_reverse_transport_parameters(requirement, 1.0);
    records
        .iter()
        .map(|record| {
            let coordinate = ActiveNetworkDcCoordinate {
                name: "correlated_reverse_transport_resolve",
                triangle_input_v: 0.0,
                active_device_assignment_index: record.active_device_assignment_index,
                mn3101_macro_hypothesis_index: 4,
                temperature_c: 25.0,
                interpretation: "full nonlinear KCL re-solve of the diagnostic correlated Tr19/Tr22/Tr23 target; parameters remain unpublished evidence requirements",
            };
            let original_seed = record
                .node_voltages
                .iter()
                .map(|node| node.voltage_v)
                .collect::<Vec<_>>();
            let direct = solve_active_network_dc_coordinate_with_evaluator(
                coordinate,
                Some(&original_seed),
                0,
                80,
                |voltages| {
                    evaluate_active_network_dc_residuals_with_correlated_reverse_transport(
                        coordinate,
                        voltages,
                        target_parameters,
                    )
                },
            );
            let direct_target_converged = direct.converged && direct.every_unknown_inside_supply_window;
            let (resolved, adaptive_homotopy_used, homotopy_stages_completed) =
                if direct_target_converged {
                    (direct, false, 0)
                } else {
                    let mut seed = original_seed.clone();
                    let mut last = direct;
                    let mut completed = 0;
                    for stage_index in 1..=HOMOTOPY_STAGES {
                        let blend_fraction = stage_index as f64 / HOMOTOPY_STAGES as f64;
                        let parameters =
                            correlated_reverse_transport_parameters(requirement, blend_fraction);
                        let point = solve_active_network_dc_coordinate_with_evaluator(
                            coordinate,
                            Some(&seed),
                            0,
                            48,
                            |voltages| {
                                evaluate_active_network_dc_residuals_with_correlated_reverse_transport(
                                    coordinate,
                                    voltages,
                                    parameters,
                                )
                            },
                        );
                        let may_continue =
                            point.converged && point.every_unknown_inside_supply_window;
                        last = point;
                        if !may_continue {
                            break;
                        }
                        completed += 1;
                        seed = last
                            .node_voltages
                            .iter()
                            .map(|node| node.voltage_v)
                            .collect();
                    }
                    (last, true, completed)
                };
            let (maximum_displacement_node, maximum_node_voltage_displacement_v) = resolved
                .node_voltages
                .iter()
                .zip(&record.node_voltages)
                .map(|(resolved_node, original_node)| {
                    assert_eq!(resolved_node.node, original_node.node);
                    (
                        resolved_node.node,
                        (resolved_node.voltage_v - original_node.voltage_v).abs(),
                    )
                })
                .max_by(|left, right| left.1.total_cmp(&right.1))
                .expect("every assignment serializes all audited nodes");
            let forward_transfer_clamp_count = resolved
                .device_points
                .iter()
                .filter(|device| device.transfer_voltage_was_clamped_to_digitized_domain)
                .count();
            let reverse_transfer_clamp_count = resolved
                .device_points
                .iter()
                .filter(|device| {
                    device.reverse_transfer_voltage_was_clamped_to_digitized_domain
                })
                .count();
            let maximum_collector_power_device = resolved
                .device_points
                .iter()
                .max_by(|left, right| left.collector_power_mw.total_cmp(&right.collector_power_mw))
                .expect("the correlated solve retains all five active devices");
            BjtCorrelatedReverseTransportResolvePoint {
                active_device_assignment_index: record.active_device_assignment_index,
                direct_target_converged,
                adaptive_homotopy_used,
                homotopy_stages_completed,
                converged: resolved.converged,
                maximum_kcl_residual_a: resolved.maximum_kcl_residual_a,
                maximum_node_voltage_displacement_v,
                maximum_displacement_node,
                control_voltage_v: dc_operating_point_voltage(
                    &resolved,
                    ActiveNetworkNode::Control,
                ),
                timing_voltage_v: dc_operating_point_voltage(
                    &resolved,
                    ActiveNetworkNode::Timing,
                ),
                feedback_base_voltage_v: dc_operating_point_voltage(
                    &resolved,
                    ActiveNetworkNode::FeedbackBase,
                ),
                total_collector_terminal_current_absolute_a: resolved
                    .device_points
                    .iter()
                    .map(|device| device.collector_current_a.abs())
                    .sum(),
                maximum_collector_power_mw: resolved
                    .device_points
                    .iter()
                    .map(|device| device.collector_power_mw)
                    .fold(0.0, f64::max),
                maximum_collector_power_designator: maximum_collector_power_device.designator,
                forward_transfer_clamp_count,
                reverse_transfer_clamp_count,
                inside_supply_transport_and_power_gates: resolved.converged
                    && resolved.every_unknown_inside_supply_window
                    && resolved.every_bjt_vce_polarity_valid
                    && resolved.every_bjt_inside_published_power_limit
                    && forward_transfer_clamp_count == 0
                    && reverse_transfer_clamp_count == 0,
                device_points: resolved
                    .device_points
                    .iter()
                    .map(reciprocal_transport_assignment_device_record)
                    .collect(),
            }
        })
        .collect()
}

fn optional_correlated_resolve_metric_comparison(
    records: &[BjtCorrelatedReverseTransportResolvePoint],
    value: impl Fn(&BjtCorrelatedReverseTransportResolvePoint) -> f64,
) -> Option<UncertaintyComparison> {
    if records.len() != 32 || records.iter().any(|record| !record.converged) {
        return None;
    }
    Some(compare_uncertainty_ensemble(
        &records.iter().map(value).collect::<Vec<_>>(),
        uncertainty_acceptance_policy(),
    ))
}

fn bjt_correlated_reverse_transport_resolve_validation(
    records: &[BjtCorrelatedReverseTransportResolvePoint],
) -> BjtCorrelatedReverseTransportResolveValidation {
    let metric_comparisons = [
        ActiveDeviceDcMetricComparison {
            metric: "re-solved correlated OX3 control voltage",
            unit: "volt",
            comparison: optional_correlated_resolve_metric_comparison(records, |point| {
                point.control_voltage_v
            }),
        },
        ActiveDeviceDcMetricComparison {
            metric: "re-solved correlated 150 pF timing voltage",
            unit: "volt",
            comparison: optional_correlated_resolve_metric_comparison(records, |point| {
                point.timing_voltage_v
            }),
        },
        ActiveDeviceDcMetricComparison {
            metric: "re-solved correlated OX1 feedback-base voltage",
            unit: "volt",
            comparison: optional_correlated_resolve_metric_comparison(records, |point| {
                point.feedback_base_voltage_v
            }),
        },
        ActiveDeviceDcMetricComparison {
            metric: "re-solved correlated collector-terminal current magnitude sum",
            unit: "ampere",
            comparison: optional_correlated_resolve_metric_comparison(records, |point| {
                point.total_collector_terminal_current_absolute_a
            }),
        },
        ActiveDeviceDcMetricComparison {
            metric: "re-solved correlated maximum collector power",
            unit: "milliwatt",
            comparison: optional_correlated_resolve_metric_comparison(records, |point| {
                point.maximum_collector_power_mw
            }),
        },
    ];
    let converged_assignments = records.iter().filter(|point| point.converged).count();
    let assignments_inside_all_dc_gates = records
        .iter()
        .filter(|point| point.inside_supply_transport_and_power_gates)
        .count();
    let assignments_without_any_transfer_clamping = records
        .iter()
        .filter(|point| {
            point.forward_transfer_clamp_count == 0 && point.reverse_transfer_clamp_count == 0
        })
        .count();
    let maximum_node_voltage_displacement_v = records
        .iter()
        .map(|point| point.maximum_node_voltage_displacement_v)
        .fold(0.0, f64::max);
    let worst_displacement = records
        .iter()
        .filter(|point| {
            maximum_node_voltage_displacement_v - point.maximum_node_voltage_displacement_v
                <= 1.0e-9
        })
        .min_by_key(|point| point.active_device_assignment_index)
        .expect("the correlated re-solve evaluates the complete assignment set");
    let numerical_path_suitable_for_evidence_testing = records.len() == 32
        && converged_assignments == 32
        && assignments_inside_all_dc_gates == 32
        && assignments_without_any_transfer_clamping == 32;
    let resolved_dc_metrics_accepted_as_equivalent = numerical_path_suitable_for_evidence_testing
        && metric_comparisons.iter().all(|metric| {
            metric
                .comparison
                .is_some_and(|comparison| comparison.accepted)
        });
    BjtCorrelatedReverseTransportResolveValidation {
        attempted_assignments: records.len(),
        direct_target_converged_assignments: records
            .iter()
            .filter(|point| point.direct_target_converged)
            .count(),
        assignments_using_adaptive_homotopy: records
            .iter()
            .filter(|point| point.adaptive_homotopy_used)
            .count(),
        converged_assignments,
        assignments_inside_all_dc_gates,
        assignments_without_any_transfer_clamping,
        maximum_node_voltage_displacement_v,
        maximum_displacement_assignment_index: worst_displacement.active_device_assignment_index,
        maximum_displacement_node: worst_displacement.maximum_displacement_node,
        metric_comparisons,
        resolved_dc_metrics_accepted_as_equivalent,
        target_parameters_have_direct_evidence: false,
        numerical_path_suitable_for_evidence_testing,
        required_audio_trajectory_metrics_evaluated: false,
        suitable_for_production_parameter_narrowing: false,
        suitable_for_production_promotion: false,
        interpretation: "the best frozen-node parameter pair is fully re-solved through all collector/base/emitter KCL equations; convergence can validate a numerical evidence-testing path but cannot identify unpublished reverse beta, high-VCE conductance, transient behavior or audible production constants",
    }
}

fn tr19_high_vce_conductance_comparisons(
    records: &[BjtReciprocalTransportAssignmentPoint],
    differential_conductance_us: f64,
) -> [UncertaintyComparison; 2] {
    const REFERENCE_VOLTAGE_V: f64 = 6.0;
    const CORRECTED_CANDIDATE: &str = "2SA1015-GR/Y";
    let conductance_s = differential_conductance_us * 1.0e-6;
    let mut total_currents_a = Vec::with_capacity(records.len());
    let mut maximum_powers_mw = Vec::with_capacity(records.len());
    for record in records {
        let tr19 = record
            .device_points
            .iter()
            .find(|device| device.designator == "Tr19")
            .expect("every reciprocal assignment serializes Tr19");
        let correction_a = if tr19.candidate_key == CORRECTED_CANDIDATE {
            conductance_s
                * (tr19.collector_emitter_voltage_absolute_v - REFERENCE_VOLTAGE_V).max(0.0)
        } else {
            0.0
        };
        total_currents_a.push(record.total_collector_terminal_current_absolute_a + correction_a);
        maximum_powers_mw.push(
            record
                .device_points
                .iter()
                .map(|device| {
                    if device.designator == "Tr19" && device.candidate_key == CORRECTED_CANDIDATE {
                        (device.collector_current_absolute_a + correction_a)
                            * device.collector_emitter_voltage_absolute_v
                            * 1_000.0
                    } else {
                        device.collector_power_mw
                    }
                })
                .fold(0.0, f64::max),
        );
    }
    [
        compare_uncertainty_ensemble(&total_currents_a, uncertainty_acceptance_policy()),
        compare_uncertainty_ensemble(&maximum_powers_mw, uncertainty_acceptance_policy()),
    ]
}

fn extend_acceptance_interval(minimum: &mut Option<f64>, maximum: &mut Option<f64>, value: f64) {
    if minimum.is_none() {
        *minimum = Some(value);
    }
    *maximum = Some(value);
}

fn bjt_tr19_high_vce_conductance_requirement(
    records: &[BjtReciprocalTransportAssignmentPoint],
) -> BjtTr19HighVceConductanceRequirement {
    const REFERENCE_VOLTAGE_V: f64 = 6.0;
    const MINIMUM_US: f64 = 0.0;
    const MAXIMUM_US: f64 = 4.0;
    const STEP_US: f64 = 0.001;
    const STEPS: usize = 4_000;
    let mut current_acceptance_interval_minimum_us = None;
    let mut current_acceptance_interval_maximum_us = None;
    let mut power_acceptance_interval_minimum_us = None;
    let mut power_acceptance_interval_maximum_us = None;
    let mut joint_acceptance_interval_minimum_us = None;
    let mut joint_acceptance_interval_maximum_us = None;
    let initial = tr19_high_vce_conductance_comparisons(records, MINIMUM_US);
    let mut best_current_conductance_us = MINIMUM_US;
    let mut best_current_comparison = initial[0];
    let mut power_comparison_at_best_current = initial[1];
    let mut best_current_score = f64::INFINITY;
    for step in 0..=STEPS {
        let conductance_us = step as f64 * STEP_US;
        let [current, power] = tr19_high_vce_conductance_comparisons(records, conductance_us);
        if current.accepted {
            extend_acceptance_interval(
                &mut current_acceptance_interval_minimum_us,
                &mut current_acceptance_interval_maximum_us,
                conductance_us,
            );
        }
        if power.accepted {
            extend_acceptance_interval(
                &mut power_acceptance_interval_minimum_us,
                &mut power_acceptance_interval_maximum_us,
                conductance_us,
            );
        }
        if current.accepted && power.accepted {
            extend_acceptance_interval(
                &mut joint_acceptance_interval_minimum_us,
                &mut joint_acceptance_interval_maximum_us,
                conductance_us,
            );
        }
        let current_score = (current.mean_absolute_relative_deviation_percent
            / uncertainty_acceptance_policy().mean_relative_deviation_limit_percent)
            .max(
                current.maximum_absolute_relative_deviation_percent
                    / uncertainty_acceptance_policy().maximum_relative_deviation_limit_percent,
            );
        if current_score < best_current_score {
            best_current_score = current_score;
            best_current_conductance_us = conductance_us;
            best_current_comparison = current;
            power_comparison_at_best_current = power;
        }
    }
    let published_mitsubishi_local_hoe_us = candidate_bipolar_model_evidence()
        .into_iter()
        .find(|candidate| candidate.candidate_key == "2SA1115-F")
        .and_then(|candidate| candidate.common_emitter_hoe_typical_microsiemens)
        .expect("the Mitsubishi PNP record publishes one local h_oe value");
    BjtTr19HighVceConductanceRequirement {
        reference_voltage_v: REFERENCE_VOLTAGE_V,
        minimum_differential_conductance_us: MINIMUM_US,
        maximum_differential_conductance_us: MAXIMUM_US,
        conductance_step_us: STEP_US,
        evaluated_hypotheses: STEPS + 1,
        correction_applied_to_candidate: "2SA1015-GR/Y",
        correction_held_zero_for_candidate: "2SA1115-F",
        current_acceptance_interval_minimum_us,
        current_acceptance_interval_maximum_us,
        power_acceptance_interval_minimum_us,
        power_acceptance_interval_maximum_us,
        joint_acceptance_interval_minimum_us,
        joint_acceptance_interval_maximum_us,
        best_current_conductance_us,
        best_current_comparison,
        power_comparison_at_best_current,
        published_mitsubishi_local_hoe_us,
        tr19_differential_conductance_alone_can_pass_current_gate:
            current_acceptance_interval_minimum_us.is_some(),
        tr19_differential_conductance_alone_can_pass_power_gate:
            power_acceptance_interval_minimum_us.is_some(),
        tr19_differential_conductance_alone_can_pass_both_gates:
            joint_acceptance_interval_minimum_us.is_some(),
        remaining_transport_evidence_priorities: ["Tr23", "Tr22"],
        frozen_node_linearized_screen_not_resolved_operating_points: true,
        suitable_for_production_parameter_narrowing: false,
        interpretation: "a 0-4 uS differential high-VCE correction referenced to 6 V is screened at frozen exact reciprocal nodes; it asks what future evidence would need to accomplish, does not extend local h_oe globally, and must not be stamped into the solver or DSP without a primary bound",
    }
}

fn mn3101_frozen_node_compatibility_points(
    assignment_points: &[ActiveNetworkDcOperatingPoint],
) -> Vec<Mn3101FrozenNodeCompatibilityPoint> {
    const ACCEPTED_RESIDUAL_A: f64 = 1.0e-7;
    let hypotheses = mn3101_ox_macro_hypotheses();
    let mut points = Vec::with_capacity(assignment_points.len() * hypotheses.len());
    for assignment_point in assignment_points {
        let voltages = assignment_point
            .node_voltages
            .iter()
            .map(|node| node.voltage_v)
            .collect::<Vec<_>>();
        for hypothesis in &hypotheses {
            let coordinate = ActiveNetworkDcCoordinate {
                name: "mn3101_frozen_node_screen",
                triangle_input_v: assignment_point.coordinate.triangle_input_v,
                active_device_assignment_index: assignment_point
                    .coordinate
                    .active_device_assignment_index,
                mn3101_macro_hypothesis_index: hypothesis.index,
                temperature_c: assignment_point.coordinate.temperature_c,
                interpretation: "MN3101 macro substituted into a converged v24 BJT-assignment node vector without re-solving the operating point",
            };
            let evaluation = evaluate_active_network_dc_residuals(coordinate, &voltages);
            let (maximum_kcl_residual_a, rms_kcl_residual_a, combined_output_power_w, power_ok) =
                match &evaluation {
                    Some(value) => {
                        let squared_mean = value
                            .residuals_a
                            .iter()
                            .map(|residual| residual * residual)
                            .sum::<f64>()
                            / value.residuals_a.len() as f64;
                        (
                            maximum_absolute(&value.residuals_a),
                            squared_mean.sqrt(),
                            value.mn3101_point.combined_output_power_w,
                            value.mn3101_point.combined_power_inside_absolute_limit,
                        )
                    }
                    None => (0.0, 0.0, 0.0, false),
                };
            points.push(Mn3101FrozenNodeCompatibilityPoint {
                active_device_assignment_index: assignment_point
                    .coordinate
                    .active_device_assignment_index,
                mn3101_macro_hypothesis_index: hypothesis.index,
                transition_shape: hypothesis.transition_shape,
                output_strength_corner: hypothesis.output_strength_corner,
                evaluation_succeeded: evaluation.is_some(),
                maximum_kcl_residual_a,
                rms_kcl_residual_a,
                frozen_node_residual_passes: evaluation.is_some()
                    && maximum_kcl_residual_a <= ACCEPTED_RESIDUAL_A,
                combined_output_power_w,
                combined_power_inside_absolute_limit: power_ok,
                interpretation: "a passing residual means compatibility with the frozen v24 node vector only; failure requests a new nonlinear solve and does not disprove the macro",
            });
        }
    }
    points
}

fn mn3101_frozen_node_compatibility_summaries(
    points: &[Mn3101FrozenNodeCompatibilityPoint],
) -> Vec<Mn3101FrozenNodeCompatibilitySummary> {
    mn3101_ox_macro_hypotheses()
        .into_iter()
        .map(|hypothesis| {
            let macro_points = points
                .iter()
                .filter(|point| point.mn3101_macro_hypothesis_index == hypothesis.index)
                .collect::<Vec<_>>();
            let evaluated = macro_points
                .iter()
                .filter(|point| point.evaluation_succeeded)
                .collect::<Vec<_>>();
            let mean_maximum_kcl_residual_a = if evaluated.is_empty() {
                0.0
            } else {
                evaluated
                    .iter()
                    .map(|point| point.maximum_kcl_residual_a)
                    .sum::<f64>()
                    / evaluated.len() as f64
            };
            let residual_passing_assignments = macro_points
                .iter()
                .filter(|point| point.frozen_node_residual_passes)
                .count();
            Mn3101FrozenNodeCompatibilitySummary {
                mn3101_macro_hypothesis_index: hypothesis.index,
                transition_shape: hypothesis.transition_shape,
                output_strength_corner: hypothesis.output_strength_corner,
                evaluated_assignments: evaluated.len(),
                residual_passing_assignments,
                mean_maximum_kcl_residual_a,
                worst_maximum_kcl_residual_a: evaluated
                    .iter()
                    .map(|point| point.maximum_kcl_residual_a)
                    .fold(0.0, f64::max),
                every_point_inside_mn3101_power_limit: macro_points
                    .iter()
                    .all(|point| point.combined_power_inside_absolute_limit),
                suitable_as_continuation_seed_for_all_assignments: macro_points.len() == 32
                    && residual_passing_assignments == 32,
            }
        })
        .collect()
}

fn mn3101_frozen_node_matrix_validation(
    points: &[Mn3101FrozenNodeCompatibilityPoint],
    summaries: &[Mn3101FrozenNodeCompatibilitySummary],
) -> Mn3101FrozenNodeMatrixValidation {
    let assignment_count = active_device_assignment_space().comparative_shortlist_assignments;
    let macro_hypothesis_count = mn3101_ox_macro_hypotheses().len();
    Mn3101FrozenNodeMatrixValidation {
        expected_matrix_points: assignment_count * macro_hypothesis_count,
        evaluated_matrix_points: points
            .iter()
            .filter(|point| point.evaluation_succeeded)
            .count(),
        assignment_count,
        macro_hypothesis_count,
        macros_compatible_with_all_frozen_assignment_points: summaries
            .iter()
            .filter(|summary| summary.suitable_as_continuation_seed_for_all_assignments)
            .count(),
        every_macro_point_inside_absolute_power_limit: points
            .iter()
            .all(|point| point.combined_power_inside_absolute_limit),
        matrix_is_screening_not_operating_point_solution: true,
        actual_mn3101_transition_identified: false,
        interpretation: "the 288-point frozen-node matrix ranks which OX macros can reuse v24 solutions; non-passing macros require nonlinear continuation, while the unpublished MN3101 transition remains unidentified",
    }
}

fn evaluate_active_network_dc_residuals_with_mn3101_blend(
    coordinate: ActiveNetworkDcCoordinate,
    source_macro_hypothesis_index: usize,
    blend_fraction: f64,
    voltages: &[f64],
) -> Option<ActiveNetworkDcEvaluation> {
    let source_coordinate = ActiveNetworkDcCoordinate {
        mn3101_macro_hypothesis_index: source_macro_hypothesis_index,
        ..coordinate
    };
    let source = evaluate_active_network_dc_residuals(source_coordinate, voltages)?;
    let mut target = evaluate_active_network_dc_residuals(coordinate, voltages)?;
    let alpha = blend_fraction.clamp(0.0, 1.0);
    for (target_residual, source_residual) in target.residuals_a.iter_mut().zip(&source.residuals_a)
    {
        *target_residual = *source_residual + alpha * (*target_residual - *source_residual);
    }
    Some(target)
}

fn mn3101_homotopy_sweep(
    topology_points: &[ActiveNetworkDcOperatingPoint],
) -> (Vec<Mn3101HomotopyStage>, Vec<Mn3101HomotopySummary>) {
    const SOURCE_MACRO_INDEX: usize = 4;
    const HOMOTOPY_STAGES: usize = 16;
    let base = topology_points
        .iter()
        .find(|point| {
            point.converged
                && point.coordinate.active_device_assignment_index == 31
                && point.coordinate.mn3101_macro_hypothesis_index == SOURCE_MACRO_INDEX
        })
        .expect("the linear/geometric fixed point seeds MN3101 homotopy")
        .clone();
    let base_seed = base
        .node_voltages
        .iter()
        .map(|node| node.voltage_v)
        .collect::<Vec<_>>();
    let hypotheses = mn3101_ox_macro_hypotheses();
    let mut stages = Vec::with_capacity(hypotheses.len() * HOMOTOPY_STAGES);
    let mut summaries = Vec::with_capacity(hypotheses.len());
    for hypothesis in hypotheses {
        if hypothesis.index == SOURCE_MACRO_INDEX {
            let clamped_bjt_count = base
                .device_points
                .iter()
                .filter(|device| device.transfer_voltage_was_clamped_to_digitized_domain)
                .count();
            stages.push(Mn3101HomotopyStage {
                target_macro_hypothesis_index: hypothesis.index,
                blend_fraction: 1.0,
                converged: true,
                maximum_kcl_residual_a: base.maximum_kcl_residual_a,
                inside_supply_polarity_and_power_gates: true,
                exact_target_macro_reached: true,
            });
            summaries.push(Mn3101HomotopySummary {
                target_macro_hypothesis_index: hypothesis.index,
                transition_shape: hypothesis.transition_shape,
                output_strength_corner: hypothesis.output_strength_corner,
                attempted_stages: 1,
                converged_stages: 1,
                largest_converged_blend_fraction: 1.0,
                exact_target_macro_reached: true,
                final_maximum_kcl_residual_a: base.maximum_kcl_residual_a,
                final_converged: true,
                final_every_unknown_inside_supply_window: base.every_unknown_inside_supply_window,
                final_every_bjt_vce_polarity_valid: base.every_bjt_vce_polarity_valid,
                final_every_bjt_inside_power_limit: base.every_bjt_inside_published_power_limit,
                final_mn3101_inside_power_limit: base
                    .mn3101_point
                    .is_some_and(|point| point.combined_power_inside_absolute_limit),
                final_inside_supply_polarity_and_power_gates: true,
                final_clamped_bjt_count: clamped_bjt_count,
                suitable_as_preliminary_nonlinear_seed: true,
            });
            continue;
        }

        let coordinate = ActiveNetworkDcCoordinate {
            name: "mn3101_homotopy_target",
            triangle_input_v: 0.0,
            active_device_assignment_index: 31,
            mn3101_macro_hypothesis_index: hypothesis.index,
            temperature_c: 25.0,
            interpretation: "numerical current-port homotopy from macro 4 to the exact target macro; not a physical interpolation inside MN3101",
        };
        let mut seed = base_seed.clone();
        let mut last_point = None;
        let mut converged_stages = 0;
        let mut largest_converged_blend_fraction = 0.0;
        for stage_index in 1..=HOMOTOPY_STAGES {
            let blend_fraction = stage_index as f64 / HOMOTOPY_STAGES as f64;
            let point = solve_active_network_dc_coordinate_with_evaluator(
                coordinate,
                Some(&seed),
                0,
                80,
                |voltages| {
                    evaluate_active_network_dc_residuals_with_mn3101_blend(
                        coordinate,
                        SOURCE_MACRO_INDEX,
                        blend_fraction,
                        voltages,
                    )
                },
            );
            let inside_supply_polarity_and_power_gates = point.converged
                && point.every_unknown_inside_supply_window
                && point.every_bjt_vce_polarity_valid
                && point.every_bjt_inside_published_power_limit
                && point
                    .mn3101_point
                    .is_some_and(|mn3101| mn3101.combined_power_inside_absolute_limit);
            stages.push(Mn3101HomotopyStage {
                target_macro_hypothesis_index: hypothesis.index,
                blend_fraction,
                converged: point.converged,
                maximum_kcl_residual_a: point.maximum_kcl_residual_a,
                inside_supply_polarity_and_power_gates,
                exact_target_macro_reached: stage_index == HOMOTOPY_STAGES && point.converged,
            });
            if !inside_supply_polarity_and_power_gates {
                last_point = Some(point);
                break;
            }
            converged_stages += 1;
            largest_converged_blend_fraction = blend_fraction;
            seed = point
                .node_voltages
                .iter()
                .map(|node| node.voltage_v)
                .collect();
            last_point = Some(point);
        }
        let final_point = last_point.expect("every target attempts at least one homotopy stage");
        let exact_target_macro_reached = largest_converged_blend_fraction >= 1.0;
        let final_clamped_bjt_count = final_point
            .device_points
            .iter()
            .filter(|device| device.transfer_voltage_was_clamped_to_digitized_domain)
            .count();
        summaries.push(Mn3101HomotopySummary {
            target_macro_hypothesis_index: hypothesis.index,
            transition_shape: hypothesis.transition_shape,
            output_strength_corner: hypothesis.output_strength_corner,
            attempted_stages: stages
                .iter()
                .filter(|stage| stage.target_macro_hypothesis_index == hypothesis.index)
                .count(),
            converged_stages,
            largest_converged_blend_fraction,
            exact_target_macro_reached,
            final_maximum_kcl_residual_a: final_point.maximum_kcl_residual_a,
            final_converged: final_point.converged,
            final_every_unknown_inside_supply_window: final_point
                .every_unknown_inside_supply_window,
            final_every_bjt_vce_polarity_valid: final_point.every_bjt_vce_polarity_valid,
            final_every_bjt_inside_power_limit: final_point.every_bjt_inside_published_power_limit,
            final_mn3101_inside_power_limit: final_point
                .mn3101_point
                .is_some_and(|point| point.combined_power_inside_absolute_limit),
            final_inside_supply_polarity_and_power_gates: exact_target_macro_reached
                && final_point.every_unknown_inside_supply_window
                && final_point.every_bjt_vce_polarity_valid
                && final_point.every_bjt_inside_published_power_limit,
            final_clamped_bjt_count,
            suitable_as_preliminary_nonlinear_seed: exact_target_macro_reached,
        });
    }
    (stages, summaries)
}

fn mn3101_homotopy_validation(summaries: &[Mn3101HomotopySummary]) -> Mn3101HomotopyValidation {
    Mn3101HomotopyValidation {
        source_macro_hypothesis_index: 4,
        target_macro_hypotheses: summaries.len(),
        exact_targets_reached: summaries
            .iter()
            .filter(|summary| summary.exact_target_macro_reached)
            .count(),
        exact_targets_inside_all_dc_gates: summaries
            .iter()
            .filter(|summary| summary.final_inside_supply_polarity_and_power_gates)
            .count(),
        exact_targets_without_bjt_transfer_clamping: summaries
            .iter()
            .filter(|summary| {
                summary.exact_target_macro_reached && summary.final_clamped_bjt_count == 0
            })
            .count(),
        homotopy_is_numerical_path_not_device_evidence: true,
        actual_mn3101_transition_identified: false,
        interpretation: "sixteen current-port blend steps seek a KCL path from macro 4 to each exact OX target; reaching alpha=1 supplies a nonlinear seed but does not identify the unpublished controller transfer",
    }
}

fn bjt_saturation_base_drive_homotopy(
    topology_points: &[ActiveNetworkDcOperatingPoint],
) -> (
    Vec<BjtSaturationBaseDriveHomotopyStage>,
    BjtSaturationBaseDriveHomotopyValidation,
) {
    const HOMOTOPY_STAGES: usize = 16;
    let base = topology_points
        .iter()
        .find(|point| {
            point.converged
                && point.coordinate.active_device_assignment_index == 31
                && point.coordinate.mn3101_macro_hypothesis_index == 4
        })
        .expect("the linear/geometric fixed point seeds saturation base-drive homotopy")
        .clone();
    let initial_clamped_bjt_count = base
        .device_points
        .iter()
        .filter(|device| device.transfer_voltage_was_clamped_to_digitized_domain)
        .count();
    let base_multiplier = 1.0;
    let mut stages = vec![BjtSaturationBaseDriveHomotopyStage {
        blend_fraction: 0.0,
        converged: base.converged,
        maximum_kcl_residual_a: base.maximum_kcl_residual_a,
        every_unknown_inside_supply_window: base.every_unknown_inside_supply_window,
        every_bjt_vce_polarity_valid: base.every_bjt_vce_polarity_valid,
        every_bjt_inside_power_limit: base.every_bjt_inside_published_power_limit,
        inside_supply_polarity_and_power_gates: base.every_unknown_inside_supply_window
            && base.every_bjt_vce_polarity_valid
            && base.every_bjt_inside_published_power_limit,
        clamped_bjt_count: initial_clamped_bjt_count,
        maximum_base_current_multiplier_over_forward_beta: base_multiplier,
        exact_forced_beta_ten_hypothesis_reached: false,
    }];
    let mut seed = base
        .node_voltages
        .iter()
        .map(|node| node.voltage_v)
        .collect::<Vec<_>>();
    let mut final_point = base;
    let mut last_valid_point = final_point.clone();
    let mut converged_stages = 0;
    let mut largest_converged_blend_fraction = 0.0;
    for stage_index in 1..=HOMOTOPY_STAGES {
        let blend_fraction = stage_index as f64 / HOMOTOPY_STAGES as f64;
        let coordinate = final_point.coordinate;
        let point = solve_active_network_dc_coordinate_with_evaluator(
            coordinate,
            Some(&seed),
            0,
            80,
            |voltages| {
                evaluate_active_network_dc_residuals_with_base_drive(
                    coordinate,
                    voltages,
                    blend_fraction,
                )
            },
        );
        let clamped_bjt_count = point
            .device_points
            .iter()
            .filter(|device| device.transfer_voltage_was_clamped_to_digitized_domain)
            .count();
        let maximum_base_current_multiplier_over_forward_beta = point
            .device_points
            .iter()
            .filter(|device| device.forward_beta_base_current_a > 0.0)
            .map(|device| device.base_current_a / device.forward_beta_base_current_a)
            .fold(1.0, f64::max);
        let inside_supply_polarity_and_power_gates = point.converged
            && point.every_unknown_inside_supply_window
            && point.every_bjt_vce_polarity_valid
            && point.every_bjt_inside_published_power_limit;
        stages.push(BjtSaturationBaseDriveHomotopyStage {
            blend_fraction,
            converged: point.converged,
            maximum_kcl_residual_a: point.maximum_kcl_residual_a,
            every_unknown_inside_supply_window: point.every_unknown_inside_supply_window,
            every_bjt_vce_polarity_valid: point.every_bjt_vce_polarity_valid,
            every_bjt_inside_power_limit: point.every_bjt_inside_published_power_limit,
            inside_supply_polarity_and_power_gates,
            clamped_bjt_count,
            maximum_base_current_multiplier_over_forward_beta,
            exact_forced_beta_ten_hypothesis_reached: stage_index == HOMOTOPY_STAGES
                && inside_supply_polarity_and_power_gates,
        });
        final_point = point;
        if !inside_supply_polarity_and_power_gates {
            break;
        }
        last_valid_point = final_point.clone();
        converged_stages += 1;
        largest_converged_blend_fraction = blend_fraction;
        seed = final_point
            .node_voltages
            .iter()
            .map(|node| node.voltage_v)
            .collect();
    }
    let final_clamped_bjt_count = last_valid_point
        .device_points
        .iter()
        .filter(|device| device.transfer_voltage_was_clamped_to_digitized_domain)
        .count();
    let exact_forced_beta_ten_hypothesis_reached = largest_converged_blend_fraction >= 1.0;
    let exact_hypothesis_inside_all_dc_gates = exact_forced_beta_ten_hypothesis_reached
        && last_valid_point.every_unknown_inside_supply_window
        && last_valid_point.every_bjt_vce_polarity_valid
        && last_valid_point.every_bjt_inside_published_power_limit;
    let attempted_stages = stages.len();
    (
        stages,
        BjtSaturationBaseDriveHomotopyValidation {
            attempted_stages,
            converged_stages,
            largest_converged_blend_fraction,
            initial_clamped_bjt_count,
            final_clamped_bjt_count,
            exact_forced_beta_ten_hypothesis_reached,
            exact_hypothesis_inside_all_dc_gates,
            transfer_domain_clamps_eliminated: exact_hypothesis_inside_all_dc_gates
                && final_clamped_bjt_count == 0,
            forced_beta_ten_is_published_test_condition: true,
            forced_beta_ten_is_continuous_device_law: false,
            suitable_as_confirmed_saturation_model: false,
            interpretation: "base current is blended from IC/beta toward the published forced-beta-10 saturation test condition only where the preliminary BJT point is saturated; this is a sensitivity path, not a recovered transistor law",
        },
    )
}

const fn bjt_published_thermal_evidence() -> [BjtPublishedThermalEvidence; 4] {
    [
        BjtPublishedThermalEvidence {
            candidate_key: "2SA1015-GR/Y",
            transfer_current_kind: TransferCurrentKind::Base,
            transfer_curve_temperatures_c: &[-25.0, 25.0, 100.0],
            saturation_curve_temperatures_c: &[-25.0, 25.0, 100.0],
            hfe_curve_temperatures_c: &[-25.0, 25.0, 100.0],
            collector_power_graph_minimum_ambient_c: 0.0,
            collector_power_flat_through_ambient_c: 25.0,
            collector_power_zero_at_ambient_c: 125.0,
            collector_power_at_25c_mw: 400.0,
            temperature_curves_are_manufacturer_typical_not_limits: true,
            power_derating_is_direct_published_graph: true,
            direct_candidate_temperature_transfer_available: true,
            source: "SRC-BJT-2SA1015 PDF pages 1-2: absolute rating plus IB-VBE, hFE-IC, VCE(sat)-IC and PC-Ta plots",
        },
        BjtPublishedThermalEvidence {
            candidate_key: "2SA1115-F",
            transfer_current_kind: TransferCurrentKind::Collector,
            transfer_curve_temperatures_c: &[25.0],
            saturation_curve_temperatures_c: &[25.0],
            hfe_curve_temperatures_c: &[25.0],
            collector_power_graph_minimum_ambient_c: 0.0,
            collector_power_flat_through_ambient_c: 25.0,
            collector_power_zero_at_ambient_c: 125.0,
            collector_power_at_25c_mw: 300.0,
            temperature_curves_are_manufacturer_typical_not_limits: true,
            power_derating_is_direct_published_graph: true,
            direct_candidate_temperature_transfer_available: false,
            source: "SRC-BJT-2SA1115 PDF pages 1-2 / printed 2-17 to 2-18: absolute rating, 25 C IC-VBE curve and PC-Ta plot",
        },
        BjtPublishedThermalEvidence {
            candidate_key: "2SC1815-GR/Y",
            transfer_current_kind: TransferCurrentKind::Base,
            transfer_curve_temperatures_c: &[-25.0, 25.0, 100.0],
            saturation_curve_temperatures_c: &[-25.0, 25.0, 100.0],
            hfe_curve_temperatures_c: &[-25.0, 25.0, 100.0],
            collector_power_graph_minimum_ambient_c: 0.0,
            collector_power_flat_through_ambient_c: 25.0,
            collector_power_zero_at_ambient_c: 125.0,
            collector_power_at_25c_mw: 400.0,
            temperature_curves_are_manufacturer_typical_not_limits: true,
            power_derating_is_direct_published_graph: true,
            direct_candidate_temperature_transfer_available: true,
            source: "SRC-BJT-2SC1815 PDF pages 1-3: absolute rating plus IB-VBE, hFE-IC, VCE(sat)-IC and PC-Ta plots",
        },
        BjtPublishedThermalEvidence {
            candidate_key: "2SC2603-F",
            transfer_current_kind: TransferCurrentKind::Collector,
            transfer_curve_temperatures_c: &[25.0],
            saturation_curve_temperatures_c: &[25.0],
            hfe_curve_temperatures_c: &[25.0],
            collector_power_graph_minimum_ambient_c: 0.0,
            collector_power_flat_through_ambient_c: 25.0,
            collector_power_zero_at_ambient_c: 125.0,
            collector_power_at_25c_mw: 300.0,
            temperature_curves_are_manufacturer_typical_not_limits: true,
            power_derating_is_direct_published_graph: true,
            direct_candidate_temperature_transfer_available: false,
            source: "SRC-BJT-2SC2603 PDF pages 1-2 / printed 4-34 to 4-35: absolute rating, 25 C IC-VBE curve and PC-Ta plot",
        },
    ]
}

const fn bjt_thermal_transfer_plot_calibrations() -> [BjtThermalTransferPlotCalibration; 2] {
    [
        BjtThermalTransferPlotCalibration {
            candidate_key: "2SA1015-GR/Y",
            source: "SRC-BJT-2SA1015 PDF page 2, IB-VBE -25/25/100 C traces, 600 dpi crop",
            render_dpi: 600,
            crop_origin_x_px: 2_600,
            crop_origin_y_px: 800,
            crop_width_px: 1_800,
            crop_height_px: 1_900,
            x_left_px: 178.0,
            x_right_px: 1_625.0,
            vbe_left_absolute_v: 0.0,
            vbe_right_absolute_v: 1.2,
            y_top_px: 159.0,
            y_bottom_px: 1_676.0,
            current_top_a: 1.0e-3,
            current_bottom_a: 0.3e-6,
            current_kind: TransferCurrentKind::Base,
            reading_radius_px: 8.0,
            curve_status: "manufacturer-typical traces; eight-pixel radius covers line thickness, grid overlap and manual center reading",
        },
        BjtThermalTransferPlotCalibration {
            candidate_key: "2SC1815-GR/Y",
            source: "SRC-BJT-2SC1815 PDF page 2, IB-VBE -25/25/100 C traces, 600 dpi crop",
            render_dpi: 600,
            crop_origin_x_px: 600,
            crop_origin_y_px: 4_700,
            crop_width_px: 1_800,
            crop_height_px: 1_750,
            x_left_px: 172.0,
            x_right_px: 1_610.0,
            vbe_left_absolute_v: 0.0,
            vbe_right_absolute_v: 2.0,
            y_top_px: 190.0,
            y_bottom_px: 1_380.0,
            current_top_a: 3.0e-3,
            current_bottom_a: 0.3e-6,
            current_kind: TransferCurrentKind::Base,
            reading_radius_px: 8.0,
            curve_status: "manufacturer-typical traces; eight-pixel radius covers line thickness, grid overlap and manual center reading",
        },
    ]
}

fn bjt_thermal_transfer_pixel_anchors() -> Vec<BjtThermalTransferPixelAnchor> {
    let mut anchors = Vec::with_capacity(30);
    let records: [BjtThermalPixelTrace; 6] = [
        (
            "2SA1015-GR/Y",
            -25.0,
            &[
                (976.0, 1_500.0),
                (1_017.0, 1_100.0),
                (1_048.0, 900.0),
                (1_089.0, 700.0),
                (1_189.0, 350.0),
            ],
        ),
        (
            "2SA1015-GR/Y",
            25.0,
            &[
                (864.0, 1_500.0),
                (922.0, 1_100.0),
                (960.0, 900.0),
                (1_000.0, 700.0),
                (1_089.0, 350.0),
            ],
        ),
        (
            "2SA1015-GR/Y",
            100.0,
            &[
                (725.0, 1_500.0),
                (773.0, 1_100.0),
                (808.0, 900.0),
                (853.0, 700.0),
                (952.0, 350.0),
            ],
        ),
        (
            "2SC1815-GR/Y",
            -25.0,
            &[
                (643.0, 1_200.0),
                (667.0, 1_000.0),
                (693.0, 800.0),
                (724.0, 600.0),
                (760.0, 400.0),
            ],
        ),
        (
            "2SC1815-GR/Y",
            25.0,
            &[
                (590.0, 1_200.0),
                (607.0, 1_000.0),
                (627.0, 800.0),
                (659.0, 600.0),
                (704.0, 400.0),
            ],
        ),
        (
            "2SC1815-GR/Y",
            100.0,
            &[
                (505.0, 1_200.0),
                (528.0, 1_000.0),
                (551.0, 800.0),
                (581.0, 600.0),
                (617.0, 400.0),
            ],
        ),
    ];
    for (candidate_key, temperature_c, points) in records {
        for &(x_px, y_px) in points {
            anchors.push(BjtThermalTransferPixelAnchor {
                candidate_key,
                temperature_c,
                x_px,
                y_px,
            });
        }
    }
    anchors
}

fn bjt_thermal_plot_calibration(candidate_key: &str) -> BjtThermalTransferPlotCalibration {
    *bjt_thermal_transfer_plot_calibrations()
        .iter()
        .find(|calibration| calibration.candidate_key == candidate_key)
        .expect("every thermal pixel anchor has one crop calibration")
}

fn bjt_thermal_electrical_anchor(
    pixel: BjtThermalTransferPixelAnchor,
) -> BjtThermalTransferElectricalAnchor {
    let calibration = bjt_thermal_plot_calibration(pixel.candidate_key);
    let x_fraction =
        (pixel.x_px - calibration.x_left_px) / (calibration.x_right_px - calibration.x_left_px);
    let y_fraction =
        (pixel.y_px - calibration.y_top_px) / (calibration.y_bottom_px - calibration.y_top_px);
    let vbe_absolute_v = calibration.vbe_left_absolute_v
        + x_fraction * (calibration.vbe_right_absolute_v - calibration.vbe_left_absolute_v);
    let log_current_top = calibration.current_top_a.log10();
    let log_current_bottom = calibration.current_bottom_a.log10();
    let current_a =
        10.0_f64.powf(log_current_top + y_fraction * (log_current_bottom - log_current_top));
    let vbe_reading_uncertainty_mv = calibration.reading_radius_px
        * (calibration.vbe_right_absolute_v - calibration.vbe_left_absolute_v).abs()
        / (calibration.x_right_px - calibration.x_left_px).abs()
        * 1_000.0;
    let log_current_per_pixel =
        (log_current_bottom - log_current_top) / (calibration.y_bottom_px - calibration.y_top_px);
    let current_reading_uncertainty_percent =
        (10.0_f64.powf(log_current_per_pixel.abs() * calibration.reading_radius_px) - 1.0) * 100.0;
    BjtThermalTransferElectricalAnchor {
        candidate_key: pixel.candidate_key,
        temperature_c: pixel.temperature_c,
        current_kind: calibration.current_kind,
        current_a,
        vbe_absolute_v,
        vbe_reading_uncertainty_mv,
        current_reading_uncertainty_percent,
    }
}

fn bjt_thermal_transfer_electrical_anchors() -> Vec<BjtThermalTransferElectricalAnchor> {
    bjt_thermal_transfer_pixel_anchors()
        .into_iter()
        .map(bjt_thermal_electrical_anchor)
        .collect()
}

fn piecewise_thermal_vbe_for_current(
    candidate_key: &str,
    temperature_c: f64,
    current_a: f64,
) -> Option<f64> {
    let anchors = bjt_thermal_transfer_electrical_anchors()
        .into_iter()
        .filter(|anchor| {
            anchor.candidate_key == candidate_key
                && (anchor.temperature_c - temperature_c).abs() < 1.0e-9
        })
        .collect::<Vec<_>>();
    if anchors.is_empty()
        || current_a < anchors.first()?.current_a
        || current_a > anchors.last()?.current_a
    {
        return None;
    }
    for pair in anchors.windows(2) {
        let lower = pair[0];
        let upper = pair[1];
        if current_a >= lower.current_a && current_a <= upper.current_a {
            let fraction = (current_a.ln() - lower.current_a.ln())
                / (upper.current_a.ln() - lower.current_a.ln());
            return Some(
                lower.vbe_absolute_v + fraction * (upper.vbe_absolute_v - lower.vbe_absolute_v),
            );
        }
    }
    anchors.last().map(|anchor| anchor.vbe_absolute_v)
}

fn evaluate_bjt_thermal_transfer_point(
    candidate_key: &'static str,
    temperature_c: f64,
    source_current_a: f64,
) -> Option<BjtThermalTransferPoint> {
    const TEMPERATURES_C: [f64; 3] = [-25.0, 25.0, 100.0];
    if !temperature_c.is_finite()
        || !(TEMPERATURES_C[0]..=TEMPERATURES_C[2]).contains(&temperature_c)
    {
        return None;
    }
    let (lower_temperature_c, upper_temperature_c) = if temperature_c <= 25.0 {
        (-25.0, 25.0)
    } else {
        (25.0, 100.0)
    };
    let lower_vbe =
        piecewise_thermal_vbe_for_current(candidate_key, lower_temperature_c, source_current_a)?;
    let upper_vbe =
        piecewise_thermal_vbe_for_current(candidate_key, upper_temperature_c, source_current_a)?;
    let temperature_fraction =
        (temperature_c - lower_temperature_c) / (upper_temperature_c - lower_temperature_c);
    let vbe_absolute_v = lower_vbe + temperature_fraction * (upper_vbe - lower_vbe);
    let vbe_25c = piecewise_thermal_vbe_for_current(candidate_key, 25.0, source_current_a)?;
    let direct_trace = TEMPERATURES_C
        .iter()
        .any(|published| (temperature_c - published).abs() < 1.0e-9);
    let current_is_anchor = bjt_thermal_transfer_electrical_anchors()
        .iter()
        .any(|anchor| {
            anchor.candidate_key == candidate_key
                && (anchor.current_a - source_current_a).abs() <= source_current_a * 1.0e-12
        });
    Some(BjtThermalTransferPoint {
        candidate_key,
        temperature_c,
        current_kind: bjt_thermal_plot_calibration(candidate_key).current_kind,
        source_current_a,
        vbe_absolute_v,
        vbe_shift_from_25c_mv: (vbe_absolute_v - vbe_25c) * 1_000.0,
        temperature_is_direct_published_trace: direct_trace,
        current_is_interpolated_between_digitized_anchors: !current_is_anchor,
        manufacturer_typical_not_production_bound: true,
    })
}

fn bjt_thermal_transfer_validation(candidate_key: &'static str) -> BjtThermalTransferValidation {
    const TEMPERATURES_C: [f64; 3] = [-25.0, 25.0, 100.0];
    let candidate_anchors = bjt_thermal_transfer_electrical_anchors()
        .into_iter()
        .filter(|anchor| anchor.candidate_key == candidate_key)
        .collect::<Vec<_>>();
    let anchors_per_temperature = candidate_anchors.len() / TEMPERATURES_C.len();
    let every_trace_strictly_monotonic = TEMPERATURES_C.iter().all(|temperature| {
        let trace = candidate_anchors
            .iter()
            .filter(|anchor| (anchor.temperature_c - temperature).abs() < 1.0e-9)
            .collect::<Vec<_>>();
        trace.windows(2).all(|pair| {
            pair[1].current_a > pair[0].current_a && pair[1].vbe_absolute_v > pair[0].vbe_absolute_v
        })
    });
    let reference_currents = candidate_anchors
        .iter()
        .filter(|anchor| (anchor.temperature_c - 25.0).abs() < 1.0e-9)
        .map(|anchor| anchor.current_a)
        .collect::<Vec<_>>();
    let thermal_order_preserved_at_every_current = reference_currents.iter().all(|current| {
        let cold = piecewise_thermal_vbe_for_current(candidate_key, -25.0, *current).unwrap();
        let room = piecewise_thermal_vbe_for_current(candidate_key, 25.0, *current).unwrap();
        let hot = piecewise_thermal_vbe_for_current(candidate_key, 100.0, *current).unwrap();
        hot < room && room < cold
    });
    let maximum_anchor_round_trip_error_mv = candidate_anchors
        .iter()
        .map(|anchor| {
            (piecewise_thermal_vbe_for_current(
                candidate_key,
                anchor.temperature_c,
                anchor.current_a,
            )
            .unwrap()
                - anchor.vbe_absolute_v)
                .abs()
                * 1_000.0
        })
        .fold(0.0_f64, f64::max);
    BjtThermalTransferValidation {
        candidate_key,
        temperatures_c: TEMPERATURES_C,
        anchors_per_temperature,
        total_anchor_count: candidate_anchors.len(),
        every_trace_strictly_monotonic,
        thermal_order_preserved_at_every_current,
        maximum_anchor_round_trip_error_mv,
        interpolation_inside_temperature_and_current_domain_ready: every_trace_strictly_monotonic
            && thermal_order_preserved_at_every_current,
        extrapolation_permitted: false,
        typical_curve_covers_unit_to_unit_variation: false,
        interpretation: "piecewise log-current interpolation at each direct Toshiba temperature plus linear interpolation between -25/25/100 C traces; no current or temperature extrapolation and no production-spread claim",
    }
}

fn bjt_thermal_transfer_validations() -> [BjtThermalTransferValidation; 2] {
    [
        bjt_thermal_transfer_validation("2SA1015-GR/Y"),
        bjt_thermal_transfer_validation("2SC1815-GR/Y"),
    ]
}

fn bjt_thermal_transfer_reference_points() -> Vec<BjtThermalTransferPoint> {
    let mut points = Vec::with_capacity(6);
    for candidate_key in ["2SA1015-GR/Y", "2SC1815-GR/Y"] {
        let anchors = bjt_thermal_transfer_electrical_anchors()
            .into_iter()
            .filter(|anchor| {
                anchor.candidate_key == candidate_key
                    && (anchor.temperature_c - 25.0).abs() < 1.0e-9
            })
            .collect::<Vec<_>>();
        let source_current_a =
            (anchors.first().unwrap().current_a * anchors.last().unwrap().current_a).sqrt();
        for temperature_c in [-25.0, 25.0, 100.0] {
            points.push(
                evaluate_bjt_thermal_transfer_point(candidate_key, temperature_c, source_current_a)
                    .expect("the geometric reference lies inside every direct thermal trace"),
            );
        }
    }
    points
}

const fn bjt_thermal_characteristic_plot_calibrations()
-> [BjtThermalCharacteristicPlotCalibration; 4] {
    [
        BjtThermalCharacteristicPlotCalibration {
            candidate_key: "2SA1015-GR/Y",
            curve_kind: BjtThermalCurveKind::ForwardCurrentGain,
            source: "SRC-BJT-2SA1015 PDF page 2, hFE-IC solid traces, 600 dpi crop",
            condition: "common emitter, |VCE|=6 V; solid -25/25/100 C traces only",
            render_dpi: 600,
            crop_origin_x_px: 600,
            crop_origin_y_px: 2_400,
            crop_width_px: 1_800,
            crop_height_px: 1_300,
            x_left_px: 175.0,
            x_right_px: 1_615.0,
            collector_current_left_a: 0.1e-3,
            collector_current_right_a: 200.0e-3,
            y_top_px: 282.0,
            y_bottom_px: 940.0,
            value_top: 1_000.0,
            value_bottom: 30.0,
            reading_radius_px: 10.0,
            axes_are_logarithmic: true,
            manufacturer_typical_not_limit: true,
        },
        BjtThermalCharacteristicPlotCalibration {
            candidate_key: "2SA1015-GR/Y",
            curve_kind: BjtThermalCurveKind::CollectorEmitterSaturationVoltage,
            source: "SRC-BJT-2SA1015 PDF page 2, VCE(sat)-IC -25/25/100 C traces, 600 dpi crop",
            condition: "common emitter, forced beta |IC/IB|=10",
            render_dpi: 600,
            crop_origin_x_px: 600,
            crop_origin_y_px: 3_750,
            crop_width_px: 1_800,
            crop_height_px: 1_550,
            x_left_px: 174.0,
            x_right_px: 1_608.0,
            collector_current_left_a: 0.1e-3,
            collector_current_right_a: 200.0e-3,
            y_top_px: 317.0,
            y_bottom_px: 1_048.0,
            value_top: 0.5,
            value_bottom: 0.01,
            reading_radius_px: 10.0,
            axes_are_logarithmic: true,
            manufacturer_typical_not_limit: true,
        },
        BjtThermalCharacteristicPlotCalibration {
            candidate_key: "2SC1815-GR/Y",
            curve_kind: BjtThermalCurveKind::ForwardCurrentGain,
            source: "SRC-BJT-2SC1815 PDF page 2, hFE-IC solid traces, 600 dpi crop",
            condition: "common emitter, VCE=6 V; solid -25/25/100 C traces only",
            render_dpi: 600,
            crop_origin_x_px: 2_600,
            crop_origin_y_px: 700,
            crop_width_px: 1_800,
            crop_height_px: 1_800,
            x_left_px: 205.0,
            x_right_px: 1_643.0,
            collector_current_left_a: 0.1e-3,
            collector_current_right_a: 300.0e-3,
            y_top_px: 302.0,
            y_bottom_px: 1_498.0,
            value_top: 3_000.0,
            value_bottom: 10.0,
            reading_radius_px: 10.0,
            axes_are_logarithmic: true,
            manufacturer_typical_not_limit: true,
        },
        BjtThermalCharacteristicPlotCalibration {
            candidate_key: "2SC1815-GR/Y",
            curve_kind: BjtThermalCurveKind::CollectorEmitterSaturationVoltage,
            source: "SRC-BJT-2SC1815 PDF page 2, VCE(sat)-IC -25/25/100 C traces, 600 dpi crop",
            condition: "common emitter, forced beta IC/IB=10",
            render_dpi: 600,
            crop_origin_x_px: 600,
            crop_origin_y_px: 3_100,
            crop_width_px: 1_800,
            crop_height_px: 1_800,
            x_left_px: 178.0,
            x_right_px: 1_613.0,
            collector_current_left_a: 0.1e-3,
            collector_current_right_a: 300.0e-3,
            y_top_px: 85.0,
            y_bottom_px: 1_026.0,
            value_top: 1.0,
            value_bottom: 0.01,
            reading_radius_px: 10.0,
            axes_are_logarithmic: true,
            manufacturer_typical_not_limit: true,
        },
    ]
}

fn bjt_thermal_characteristic_pixel_anchors() -> Vec<BjtThermalCharacteristicPixelAnchor> {
    let mut anchors = Vec::with_capacity(60);
    let records: [BjtThermalCharacteristicPixelTrace; 12] = [
        (
            "2SA1015-GR/Y",
            BjtThermalCurveKind::ForwardCurrentGain,
            -25.0,
            &[
                (382.0, 652.0),
                (613.0, 645.0),
                (820.0, 643.0),
                (1_254.0, 644.0),
                (1_484.0, 692.0),
            ],
        ),
        (
            "2SA1015-GR/Y",
            BjtThermalCurveKind::ForwardCurrentGain,
            25.0,
            &[
                (382.0, 595.0),
                (613.0, 587.0),
                (820.0, 585.0),
                (1_254.0, 588.0),
                (1_484.0, 635.0),
            ],
        ),
        (
            "2SA1015-GR/Y",
            BjtThermalCurveKind::ForwardCurrentGain,
            100.0,
            &[
                (382.0, 548.0),
                (613.0, 540.0),
                (820.0, 540.0),
                (1_254.0, 542.0),
                (1_484.0, 590.0),
            ],
        ),
        (
            "2SA1015-GR/Y",
            BjtThermalCurveKind::CollectorEmitterSaturationVoltage,
            -25.0,
            &[
                (382.0, 955.0),
                (613.0, 970.0),
                (820.0, 927.0),
                (1_254.0, 795.0),
                (1_484.0, 615.0),
            ],
        ),
        (
            "2SA1015-GR/Y",
            BjtThermalCurveKind::CollectorEmitterSaturationVoltage,
            25.0,
            &[
                (382.0, 935.0),
                (613.0, 927.0),
                (820.0, 900.0),
                (1_254.0, 765.0),
                (1_484.0, 590.0),
            ],
        ),
        (
            "2SA1015-GR/Y",
            BjtThermalCurveKind::CollectorEmitterSaturationVoltage,
            100.0,
            &[
                (382.0, 880.0),
                (613.0, 890.0),
                (820.0, 865.0),
                (1_254.0, 735.0),
                (1_484.0, 565.0),
            ],
        ),
        (
            "2SC1815-GR/Y",
            BjtThermalCurveKind::ForwardCurrentGain,
            -25.0,
            &[
                (402.0, 1_013.0),
                (619.0, 1_010.0),
                (816.0, 1_010.0),
                (1_224.0, 1_015.0),
                (1_432.0, 1_100.0),
            ],
        ),
        (
            "2SC1815-GR/Y",
            BjtThermalCurveKind::ForwardCurrentGain,
            25.0,
            &[
                (402.0, 945.0),
                (619.0, 943.0),
                (816.0, 943.0),
                (1_224.0, 945.0),
                (1_432.0, 1_020.0),
            ],
        ),
        (
            "2SC1815-GR/Y",
            BjtThermalCurveKind::ForwardCurrentGain,
            100.0,
            &[
                (402.0, 870.0),
                (619.0, 860.0),
                (816.0, 860.0),
                (1_224.0, 870.0),
                (1_432.0, 940.0),
            ],
        ),
        (
            "2SC1815-GR/Y",
            BjtThermalCurveKind::CollectorEmitterSaturationVoltage,
            -25.0,
            &[
                (375.0, 782.0),
                (590.0, 808.0),
                (787.0, 801.0),
                (1_200.0, 677.0),
                (1_415.0, 551.0),
            ],
        ),
        (
            "2SC1815-GR/Y",
            BjtThermalCurveKind::CollectorEmitterSaturationVoltage,
            25.0,
            &[
                (375.0, 747.0),
                (590.0, 776.0),
                (787.0, 776.0),
                (1_200.0, 656.0),
                (1_415.0, 527.0),
            ],
        ),
        (
            "2SC1815-GR/Y",
            BjtThermalCurveKind::CollectorEmitterSaturationVoltage,
            100.0,
            &[
                (375.0, 697.0),
                (590.0, 727.0),
                (787.0, 733.0),
                (1_200.0, 628.0),
                (1_415.0, 502.0),
            ],
        ),
    ];
    for (candidate_key, curve_kind, temperature_c, points) in records {
        for &(x_px, y_px) in points {
            anchors.push(BjtThermalCharacteristicPixelAnchor {
                candidate_key,
                curve_kind,
                temperature_c,
                x_px,
                y_px,
            });
        }
    }
    anchors
}

fn bjt_thermal_characteristic_plot_calibration(
    candidate_key: &str,
    curve_kind: BjtThermalCurveKind,
) -> BjtThermalCharacteristicPlotCalibration {
    *bjt_thermal_characteristic_plot_calibrations()
        .iter()
        .find(|calibration| {
            calibration.candidate_key == candidate_key && calibration.curve_kind == curve_kind
        })
        .expect("every thermal characteristic trace has one crop calibration")
}

fn bjt_thermal_characteristic_electrical_anchor(
    pixel: BjtThermalCharacteristicPixelAnchor,
) -> BjtThermalCharacteristicElectricalAnchor {
    let calibration =
        bjt_thermal_characteristic_plot_calibration(pixel.candidate_key, pixel.curve_kind);
    let x_fraction =
        (pixel.x_px - calibration.x_left_px) / (calibration.x_right_px - calibration.x_left_px);
    let y_fraction =
        (pixel.y_px - calibration.y_top_px) / (calibration.y_bottom_px - calibration.y_top_px);
    let log_current_left = calibration.collector_current_left_a.log10();
    let log_current_right = calibration.collector_current_right_a.log10();
    let collector_current_a =
        10.0_f64.powf(log_current_left + x_fraction * (log_current_right - log_current_left));
    let log_value_top = calibration.value_top.log10();
    let log_value_bottom = calibration.value_bottom.log10();
    let value = 10.0_f64.powf(log_value_top + y_fraction * (log_value_bottom - log_value_top));
    let current_log_per_pixel =
        (log_current_right - log_current_left) / (calibration.x_right_px - calibration.x_left_px);
    let value_log_per_pixel =
        (log_value_bottom - log_value_top) / (calibration.y_bottom_px - calibration.y_top_px);
    BjtThermalCharacteristicElectricalAnchor {
        candidate_key: pixel.candidate_key,
        curve_kind: pixel.curve_kind,
        temperature_c: pixel.temperature_c,
        collector_current_a,
        value,
        collector_current_reading_uncertainty_percent: (10.0_f64
            .powf(current_log_per_pixel.abs() * calibration.reading_radius_px)
            - 1.0)
            * 100.0,
        value_reading_uncertainty_percent: (10.0_f64
            .powf(value_log_per_pixel.abs() * calibration.reading_radius_px)
            - 1.0)
            * 100.0,
    }
}

fn bjt_thermal_characteristic_electrical_anchors() -> Vec<BjtThermalCharacteristicElectricalAnchor>
{
    bjt_thermal_characteristic_pixel_anchors()
        .into_iter()
        .map(bjt_thermal_characteristic_electrical_anchor)
        .collect()
}

fn piecewise_bjt_thermal_characteristic_value(
    candidate_key: &str,
    curve_kind: BjtThermalCurveKind,
    temperature_c: f64,
    collector_current_a: f64,
) -> Option<f64> {
    let anchors = bjt_thermal_characteristic_electrical_anchors()
        .into_iter()
        .filter(|anchor| {
            anchor.candidate_key == candidate_key
                && anchor.curve_kind == curve_kind
                && (anchor.temperature_c - temperature_c).abs() < 1.0e-9
        })
        .collect::<Vec<_>>();
    if anchors.is_empty()
        || collector_current_a < anchors.first()?.collector_current_a
        || collector_current_a > anchors.last()?.collector_current_a
    {
        return None;
    }
    for pair in anchors.windows(2) {
        let lower = pair[0];
        let upper = pair[1];
        if collector_current_a >= lower.collector_current_a
            && collector_current_a <= upper.collector_current_a
        {
            let fraction = (collector_current_a.ln() - lower.collector_current_a.ln())
                / (upper.collector_current_a.ln() - lower.collector_current_a.ln());
            return Some(
                (lower.value.ln() + fraction * (upper.value.ln() - lower.value.ln())).exp(),
            );
        }
    }
    anchors.last().map(|anchor| anchor.value)
}

fn evaluate_bjt_thermal_characteristic_point(
    candidate_key: &'static str,
    curve_kind: BjtThermalCurveKind,
    temperature_c: f64,
    collector_current_a: f64,
) -> Option<BjtThermalCharacteristicPoint> {
    const TEMPERATURES_C: [f64; 3] = [-25.0, 25.0, 100.0];
    if !temperature_c.is_finite()
        || !(TEMPERATURES_C[0]..=TEMPERATURES_C[2]).contains(&temperature_c)
    {
        return None;
    }
    let (lower_temperature_c, upper_temperature_c) = if temperature_c <= 25.0 {
        (-25.0, 25.0)
    } else {
        (25.0, 100.0)
    };
    let lower_value = piecewise_bjt_thermal_characteristic_value(
        candidate_key,
        curve_kind,
        lower_temperature_c,
        collector_current_a,
    )?;
    let upper_value = piecewise_bjt_thermal_characteristic_value(
        candidate_key,
        curve_kind,
        upper_temperature_c,
        collector_current_a,
    )?;
    let temperature_fraction =
        (temperature_c - lower_temperature_c) / (upper_temperature_c - lower_temperature_c);
    let value = lower_value + temperature_fraction * (upper_value - lower_value);
    let temperature_is_direct_published_trace = TEMPERATURES_C
        .iter()
        .any(|published| (temperature_c - published).abs() < 1.0e-9);
    let current_is_anchor = bjt_thermal_characteristic_electrical_anchors()
        .iter()
        .any(|anchor| {
            anchor.candidate_key == candidate_key
                && anchor.curve_kind == curve_kind
                && (anchor.collector_current_a - collector_current_a).abs()
                    <= collector_current_a * 1.0e-12
        });
    Some(BjtThermalCharacteristicPoint {
        candidate_key,
        curve_kind,
        temperature_c,
        collector_current_a,
        value,
        temperature_is_direct_published_trace,
        current_is_interpolated_between_digitized_anchors: !current_is_anchor,
        manufacturer_typical_not_production_bound: true,
    })
}

fn bjt_thermal_characteristic_validation(
    candidate_key: &'static str,
    curve_kind: BjtThermalCurveKind,
) -> BjtThermalCharacteristicValidation {
    const TEMPERATURES_C: [f64; 3] = [-25.0, 25.0, 100.0];
    let anchors = bjt_thermal_characteristic_electrical_anchors()
        .into_iter()
        .filter(|anchor| anchor.candidate_key == candidate_key && anchor.curve_kind == curve_kind)
        .collect::<Vec<_>>();
    let reference_currents = anchors
        .iter()
        .filter(|anchor| (anchor.temperature_c - 25.0).abs() < 1.0e-9)
        .map(|anchor| anchor.collector_current_a)
        .collect::<Vec<_>>();
    let thermal_order = reference_currents.iter().all(|current| {
        let cold =
            piecewise_bjt_thermal_characteristic_value(candidate_key, curve_kind, -25.0, *current)
                .unwrap();
        let room =
            piecewise_bjt_thermal_characteristic_value(candidate_key, curve_kind, 25.0, *current)
                .unwrap();
        let hot =
            piecewise_bjt_thermal_characteristic_value(candidate_key, curve_kind, 100.0, *current)
                .unwrap();
        hot > room && room > cold
    });
    let maximum_anchor_round_trip_relative_error_percent = anchors
        .iter()
        .map(|anchor| {
            (piecewise_bjt_thermal_characteristic_value(
                candidate_key,
                curve_kind,
                anchor.temperature_c,
                anchor.collector_current_a,
            )
            .unwrap()
                - anchor.value)
                .abs()
                / anchor.value
                * 100.0
        })
        .fold(0.0_f64, f64::max);
    BjtThermalCharacteristicValidation {
        candidate_key,
        curve_kind,
        temperatures_c: TEMPERATURES_C,
        anchors_per_temperature: anchors.len() / TEMPERATURES_C.len(),
        total_anchor_count: anchors.len(),
        every_anchor_positive_and_finite: anchors
            .iter()
            .all(|anchor| anchor.value.is_finite() && anchor.value > 0.0),
        hot_above_room_above_cold_at_every_reference_current: thermal_order,
        maximum_anchor_round_trip_relative_error_percent,
        interpolation_inside_temperature_and_current_domain_ready: thermal_order
            && maximum_anchor_round_trip_relative_error_percent < 1.0e-9,
        extrapolation_permitted: false,
        typical_curve_covers_unit_to_unit_variation: false,
        interpretation: "piecewise log-log interpolation inside each direct Toshiba trace plus linear temperature interpolation between -25/25/100 C; curve condition is preserved, extrapolation and production-spread claims are refused",
    }
}

fn bjt_thermal_characteristic_validations() -> [BjtThermalCharacteristicValidation; 4] {
    [
        bjt_thermal_characteristic_validation(
            "2SA1015-GR/Y",
            BjtThermalCurveKind::ForwardCurrentGain,
        ),
        bjt_thermal_characteristic_validation(
            "2SA1015-GR/Y",
            BjtThermalCurveKind::CollectorEmitterSaturationVoltage,
        ),
        bjt_thermal_characteristic_validation(
            "2SC1815-GR/Y",
            BjtThermalCurveKind::ForwardCurrentGain,
        ),
        bjt_thermal_characteristic_validation(
            "2SC1815-GR/Y",
            BjtThermalCurveKind::CollectorEmitterSaturationVoltage,
        ),
    ]
}

fn bjt_thermal_characteristic_reference_points() -> Vec<BjtThermalCharacteristicPoint> {
    let mut points = Vec::with_capacity(12);
    for (candidate_key, curve_kind) in [
        ("2SA1015-GR/Y", BjtThermalCurveKind::ForwardCurrentGain),
        (
            "2SA1015-GR/Y",
            BjtThermalCurveKind::CollectorEmitterSaturationVoltage,
        ),
        ("2SC1815-GR/Y", BjtThermalCurveKind::ForwardCurrentGain),
        (
            "2SC1815-GR/Y",
            BjtThermalCurveKind::CollectorEmitterSaturationVoltage,
        ),
    ] {
        let anchors = bjt_thermal_characteristic_electrical_anchors()
            .into_iter()
            .filter(|anchor| {
                anchor.candidate_key == candidate_key
                    && anchor.curve_kind == curve_kind
                    && (anchor.temperature_c - 25.0).abs() < 1.0e-9
            })
            .collect::<Vec<_>>();
        let collector_current_a = (anchors.first().unwrap().collector_current_a
            * anchors.last().unwrap().collector_current_a)
            .sqrt();
        for temperature_c in [-25.0, 25.0, 100.0] {
            points.push(
                evaluate_bjt_thermal_characteristic_point(
                    candidate_key,
                    curve_kind,
                    temperature_c,
                    collector_current_a,
                )
                .expect("thermal characteristic reference remains inside every trace"),
            );
        }
    }
    points
}

fn normalized_current_inside_domain(
    minimum_current_a: f64,
    maximum_current_a: f64,
    normalized_log_current_coordinate: f64,
) -> Option<f64> {
    if !normalized_log_current_coordinate.is_finite()
        || !(0.0..=1.0).contains(&normalized_log_current_coordinate)
        || minimum_current_a <= 0.0
        || maximum_current_a <= minimum_current_a
    {
        return None;
    }
    if normalized_log_current_coordinate == 0.0 {
        return Some(minimum_current_a);
    }
    if normalized_log_current_coordinate == 1.0 {
        return Some(maximum_current_a);
    }
    Some(
        (minimum_current_a.ln()
            + normalized_log_current_coordinate
                * (maximum_current_a.ln() - minimum_current_a.ln()))
        .exp(),
    )
}

fn toshiba_thermal_shift_with_reading_uncertainty_mv(
    donor_candidate_key: &'static str,
    temperature_c: f64,
    normalized_log_current_coordinate: f64,
) -> Option<(f64, f64)> {
    let anchors = bjt_thermal_transfer_electrical_anchors()
        .into_iter()
        .filter(|anchor| {
            anchor.candidate_key == donor_candidate_key
                && (anchor.temperature_c - 25.0).abs() < 1.0e-9
        })
        .collect::<Vec<_>>();
    let source_current_a = normalized_current_inside_domain(
        anchors.first()?.current_a,
        anchors.last()?.current_a,
        normalized_log_current_coordinate,
    )?;
    let point =
        evaluate_bjt_thermal_transfer_point(donor_candidate_key, temperature_c, source_current_a)?;
    let calibration = bjt_thermal_plot_calibration(donor_candidate_key);
    let single_trace_reading_uncertainty_mv = calibration.reading_radius_px
        * (calibration.vbe_right_absolute_v - calibration.vbe_left_absolute_v).abs()
        / (calibration.x_right_px - calibration.x_left_px).abs()
        * 1_000.0;
    let distance_from_room_fraction = if temperature_c <= 25.0 {
        (25.0 - temperature_c) / 50.0
    } else {
        (temperature_c - 25.0) / 75.0
    };
    let shift_reading_uncertainty_mv =
        2.0 * single_trace_reading_uncertainty_mv * distance_from_room_fraction;
    Some((point.vbe_shift_from_25c_mv, shift_reading_uncertainty_mv))
}

fn evaluate_bjt_comparative_thermal_shift_interval(
    target_candidate_key: &'static str,
    temperature_c: f64,
    normalized_log_current_coordinate: f64,
) -> Option<BjtComparativeThermalShiftInterval> {
    const DONOR_CANDIDATE_KEYS: [&str; 2] = ["2SA1015-GR/Y", "2SC1815-GR/Y"];
    if !(-25.0..=100.0).contains(&temperature_c) {
        return None;
    }
    let target_validation = bjt_piecewise_transfer_validation(target_candidate_key);
    if target_validation.current_kind != TransferCurrentKind::Collector
        || !matches!(target_candidate_key, "2SA1115-F" | "2SC2603-F")
    {
        return None;
    }
    let target_collector_current_a = normalized_current_inside_domain(
        target_validation.minimum_current_a,
        target_validation.maximum_current_a,
        normalized_log_current_coordinate,
    )?;
    let target_vbe_at_25c_v =
        piecewise_vbe_for_current(target_candidate_key, target_collector_current_a)?;
    let donor_intervals = DONOR_CANDIDATE_KEYS.map(|donor_candidate_key| {
        let (shift_mv, uncertainty_mv) = toshiba_thermal_shift_with_reading_uncertainty_mv(
            donor_candidate_key,
            temperature_c,
            normalized_log_current_coordinate,
        )?;
        Some((shift_mv - uncertainty_mv, shift_mv + uncertainty_mv))
    });
    let donor_intervals = [donor_intervals[0]?, donor_intervals[1]?];
    let lower_shift_from_25c_mv = donor_intervals
        .iter()
        .map(|interval| interval.0)
        .fold(f64::INFINITY, f64::min);
    let upper_shift_from_25c_mv = donor_intervals
        .iter()
        .map(|interval| interval.1)
        .fold(f64::NEG_INFINITY, f64::max);
    Some(BjtComparativeThermalShiftInterval {
        target_candidate_key,
        temperature_c,
        normalized_log_current_coordinate,
        target_collector_current_a,
        target_vbe_at_25c_v,
        lower_vbe_v: target_vbe_at_25c_v + lower_shift_from_25c_mv / 1_000.0,
        upper_vbe_v: target_vbe_at_25c_v + upper_shift_from_25c_mv / 1_000.0,
        lower_shift_from_25c_mv,
        upper_shift_from_25c_mv,
        donor_candidate_keys: DONOR_CANDIDATE_KEYS,
        donor_current_kind: TransferCurrentKind::Base,
        target_current_kind: TransferCurrentKind::Collector,
        includes_digitization_reading_uncertainty: true,
        direct_target_temperature_evidence: (temperature_c - 25.0).abs() < 1.0e-9,
        comparable_family_hypothesis_not_device_measurement: true,
        covers_target_unit_to_unit_variation: false,
    })
}

fn bjt_comparative_thermal_reference_intervals() -> Vec<BjtComparativeThermalShiftInterval> {
    let mut intervals = Vec::with_capacity(30);
    for target_candidate_key in ["2SA1115-F", "2SC2603-F"] {
        for temperature_c in [-25.0, 0.0, 25.0, 50.0, 100.0] {
            for normalized_log_current_coordinate in [0.0, 0.5, 1.0] {
                intervals.push(
                    evaluate_bjt_comparative_thermal_shift_interval(
                        target_candidate_key,
                        temperature_c,
                        normalized_log_current_coordinate,
                    )
                    .expect("comparative references stay inside both source domains"),
                );
            }
        }
    }
    intervals
}

fn bjt_comparative_thermal_envelope_validation(
    target_candidate_key: &'static str,
) -> BjtComparativeThermalEnvelopeValidation {
    const TEMPERATURES_C: [f64; 7] = [-25.0, -10.0, 0.0, 25.0, 50.0, 75.0, 100.0];
    const NORMALIZED_CURRENT_COORDINATES: [f64; 11] =
        [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0];
    let points = TEMPERATURES_C
        .iter()
        .flat_map(|temperature_c| {
            NORMALIZED_CURRENT_COORDINATES
                .iter()
                .map(move |coordinate| {
                    evaluate_bjt_comparative_thermal_shift_interval(
                        target_candidate_key,
                        *temperature_c,
                        *coordinate,
                    )
                    .expect("validation grid stays inside the comparative domain")
                })
        })
        .collect::<Vec<_>>();
    let every_interval_is_ordered = points
        .iter()
        .all(|point| point.lower_vbe_v <= point.upper_vbe_v);
    let every_cold_interval_is_above_25c = points
        .iter()
        .filter(|point| (point.temperature_c + 25.0).abs() < 1.0e-9)
        .all(|point| point.lower_vbe_v > point.target_vbe_at_25c_v);
    let every_hot_interval_is_below_25c = points
        .iter()
        .filter(|point| (point.temperature_c - 100.0).abs() < 1.0e-9)
        .all(|point| point.upper_vbe_v < point.target_vbe_at_25c_v);
    let room_temperature_collapses_to_direct_target_curve = points
        .iter()
        .filter(|point| (point.temperature_c - 25.0).abs() < 1.0e-9)
        .all(|point| {
            (point.lower_vbe_v - point.target_vbe_at_25c_v).abs() < 1.0e-12
                && (point.upper_vbe_v - point.target_vbe_at_25c_v).abs() < 1.0e-12
                && point.direct_target_temperature_evidence
        });
    BjtComparativeThermalEnvelopeValidation {
        target_candidate_key,
        temperature_domain_c: [-25.0, 100.0],
        normalized_current_domain: [0.0, 1.0],
        temperature_samples: TEMPERATURES_C.len(),
        normalized_current_samples: NORMALIZED_CURRENT_COORDINATES.len(),
        evaluated_points: points.len(),
        every_interval_is_ordered,
        every_cold_interval_is_above_25c,
        every_hot_interval_is_below_25c,
        room_temperature_collapses_to_direct_target_curve,
        rejects_temperature_extrapolation: evaluate_bjt_comparative_thermal_shift_interval(
            target_candidate_key,
            -25.01,
            0.5,
        )
        .is_none()
            && evaluate_bjt_comparative_thermal_shift_interval(target_candidate_key, 100.01, 0.5)
                .is_none(),
        rejects_current_extrapolation: evaluate_bjt_comparative_thermal_shift_interval(
            target_candidate_key,
            25.0,
            -0.01,
        )
        .is_none()
            && evaluate_bjt_comparative_thermal_shift_interval(target_candidate_key, 25.0, 1.01)
                .is_none(),
        direct_mitsubishi_temperature_curve_available: false,
        usable_as_comparative_sweep_interval: every_interval_is_ordered
            && every_cold_interval_is_above_25c
            && every_hot_interval_is_below_25c
            && room_temperature_collapses_to_direct_target_curve,
        usable_as_confirmed_device_model: false,
        interpretation: "25 C Mitsubishi IC-VBE center plus the union of PNP/NPN Toshiba normalized-current thermal shifts and conservative pixel-reading error; donor IB and target IC are never equated, no extrapolation or unit-spread claim is permitted",
    }
}

fn bjt_comparative_thermal_envelope_validations() -> [BjtComparativeThermalEnvelopeValidation; 2] {
    [
        bjt_comparative_thermal_envelope_validation("2SA1115-F"),
        bjt_comparative_thermal_envelope_validation("2SC2603-F"),
    ]
}

fn bjt_collector_power_limit_mw(candidate_key: &str, ambient_temperature_c: f64) -> Option<f64> {
    let thermal = bjt_published_thermal_evidence()
        .into_iter()
        .find(|record| record.candidate_key == candidate_key)?;
    if !ambient_temperature_c.is_finite()
        || ambient_temperature_c < thermal.collector_power_graph_minimum_ambient_c
        || ambient_temperature_c > thermal.collector_power_zero_at_ambient_c
    {
        return None;
    }
    if ambient_temperature_c <= thermal.collector_power_flat_through_ambient_c {
        return Some(thermal.collector_power_at_25c_mw);
    }
    let remaining_fraction = (thermal.collector_power_zero_at_ambient_c - ambient_temperature_c)
        / (thermal.collector_power_zero_at_ambient_c
            - thermal.collector_power_flat_through_ambient_c);
    Some(thermal.collector_power_at_25c_mw * remaining_fraction)
}

fn bjt_collector_power_reference_points() -> Vec<BjtCollectorPowerPoint> {
    let mut points = Vec::with_capacity(16);
    for thermal in bjt_published_thermal_evidence() {
        for ambient_temperature_c in [0.0, 25.0, 75.0, 125.0] {
            points.push(BjtCollectorPowerPoint {
                candidate_key: thermal.candidate_key,
                ambient_temperature_c,
                collector_power_limit_mw: bjt_collector_power_limit_mw(
                    thermal.candidate_key,
                    ambient_temperature_c,
                )
                .expect("every reference temperature is inside the published graph domain"),
                inside_published_graph_domain: true,
            });
        }
    }
    points
}

fn bjt_thermal_evidence_readiness() -> BjtThermalEvidenceReadiness {
    let records = bjt_published_thermal_evidence();
    let candidates_with_direct_power_derating_graph = records
        .iter()
        .filter(|record| record.power_derating_is_direct_published_graph)
        .count();
    let candidates_with_three_temperature_transfer_curves = records
        .iter()
        .filter(|record| record.transfer_curve_temperatures_c.len() == 3)
        .count();
    let candidates_with_three_temperature_saturation_curves = records
        .iter()
        .filter(|record| record.saturation_curve_temperatures_c.len() == 3)
        .count();
    let candidates_with_three_temperature_hfe_curves = records
        .iter()
        .filter(|record| record.hfe_curve_temperatures_c.len() == 3)
        .count();
    let candidates_with_digitized_three_temperature_transfer_curves =
        bjt_thermal_transfer_validations()
            .iter()
            .filter(|validation| {
                validation.interpolation_inside_temperature_and_current_domain_ready
            })
            .count();
    let thermal_characteristic_validations = bjt_thermal_characteristic_validations();
    let candidates_with_digitized_three_temperature_hfe_curves = thermal_characteristic_validations
        .iter()
        .filter(|validation| {
            validation.curve_kind == BjtThermalCurveKind::ForwardCurrentGain
                && validation.interpolation_inside_temperature_and_current_domain_ready
        })
        .count();
    let candidates_with_digitized_three_temperature_saturation_curves =
        thermal_characteristic_validations
            .iter()
            .filter(|validation| {
                validation.curve_kind == BjtThermalCurveKind::CollectorEmitterSaturationVoltage
                    && validation.interpolation_inside_temperature_and_current_domain_ready
            })
            .count();
    let candidates_with_comparative_temperature_transfer_intervals =
        bjt_comparative_thermal_envelope_validations()
            .iter()
            .filter(|validation| validation.usable_as_comparative_sweep_interval)
            .count();
    BjtThermalEvidenceReadiness {
        candidate_count: records.len(),
        candidates_with_direct_power_derating_graph,
        candidates_with_three_temperature_transfer_curves,
        candidates_with_three_temperature_saturation_curves,
        candidates_with_three_temperature_hfe_curves,
        candidates_with_digitized_three_temperature_transfer_curves,
        candidates_with_digitized_three_temperature_hfe_curves,
        candidates_with_digitized_three_temperature_saturation_curves,
        candidates_with_comparative_temperature_transfer_intervals,
        power_derating_ready_for_all_candidates: candidates_with_direct_power_derating_graph
            == records.len(),
        direct_toshiba_temperature_interpolation_ready:
            candidates_with_digitized_three_temperature_transfer_curves == 2,
        direct_toshiba_thermal_gain_interpolation_ready:
            candidates_with_digitized_three_temperature_hfe_curves == 2,
        direct_toshiba_thermal_saturation_interpolation_ready:
            candidates_with_digitized_three_temperature_saturation_curves == 2,
        comparative_mitsubishi_temperature_sweep_ready:
            candidates_with_comparative_temperature_transfer_intervals == 2,
        direct_temperature_transfer_ready_for_all_candidates:
            candidates_with_three_temperature_transfer_curves == records.len(),
        comparative_temperature_borrowing_required_for_mitsubishi: true,
        continuous_temperature_dc_models_ready: false,
        safe_scope: "apply direct PC-Ta derating plus Toshiba -25/25/100 C VBE, hFE and forced-beta VCE(sat) interpolation; sweep Mitsubishi transfer temperature only inside the explicit comparative interval",
        interpretation: "Toshiba transfer, gain and saturation temperature curves are now executable typical centers and both Mitsubishi transfer candidates have a non-identifying comparative interval; Mitsubishi thermal gain/saturation and production spread remain unavailable",
    }
}

fn all_shortlist_candidates_have_model_evidence() -> bool {
    let evidence = candidate_bipolar_model_evidence();
    let shortlist = comparative_candidate_shortlist();
    shortlist
        .pnp_candidates
        .into_iter()
        .chain(shortlist.npn_candidates)
        .all(|candidate| {
            evidence
                .iter()
                .filter(|record| record.candidate_key == candidate)
                .count()
                == 1
        })
}

fn bipolar_model_parameter_boundary() -> BipolarModelParameterBoundary {
    BipolarModelParameterBoundary {
        published_candidate_records: candidate_bipolar_model_evidence().len(),
        all_shortlist_candidates_have_a_record: all_shortlist_candidates_have_model_evidence(),
        usable_for_absolute_limit_checks: true,
        usable_for_rank_and_capacitance_corners: true,
        complete_large_signal_model_available: false,
        missing_model_parameter_groups: [
            "transport saturation current IS and its temperature law",
            "forward emission coefficient NF",
            "forward Early voltage VAF",
            "high-current beta roll-off IKF",
            "base-emitter leakage ISE/NE",
            "reverse gain and Early/roll-off parameters BR/VAR/IKR",
            "base-emitter depletion capacitance CJE/VJE/MJE",
            "base-collector depletion capacitance CJC/VJC/MJC",
            "forward transit-time parameters TF/XTF",
            "complete temperature and flicker-noise coefficients",
        ],
        next_evidence_step: "bound Mitsubishi thermal gain/saturation, global output conductance and unit VBE spread before using the continuous region family as more than a sensitivity sweep",
        interpretation: "the records and digitized typical curves constrain family centers, rank corners and impossible results; they are not complete Gummel-Poon models or production-spread guarantees and must not be expanded with guessed defaults",
    }
}

struct ActiveNetworkSolverReadinessInputs<'a> {
    dc_validation: ActiveNetworkDcSolverValidation,
    assignment_sweep: &'a ActiveDeviceDcSweepValidation,
    mn3101_matrix: Mn3101FrozenNodeMatrixValidation,
    mn3101_homotopy: Mn3101HomotopyValidation,
    saturation_base_drive: BjtSaturationBaseDriveHomotopyValidation,
    output_curve_resolution: BjtOutputCharacteristicClampedResolutionValidation,
    low_vce_base_drive: &'a BjtLowVceBaseDriveSweepValidation,
    reciprocal_transport_sweep: &'a BjtReciprocalTransportSweepValidation,
    reciprocal_transport_homotopy: BjtReciprocalTransportHomotopyValidation,
    reciprocal_transport_assignment: &'a BjtReciprocalTransportAssignmentValidation,
    reciprocal_transport_attribution: &'a BjtReciprocalTransportDispersionAttribution,
    reciprocal_transport_tr19_evidence: BjtReciprocalTransportTr19EvidenceAudit,
    tr19_high_vce_conductance: BjtTr19HighVceConductanceRequirement,
    reciprocal_transport_tr22_tr23_evidence: BjtReciprocalTransportTr22Tr23EvidenceAudit,
    tr22_tr23_reverse_beta: BjtTr22Tr23ReverseBetaRequirement,
    correlated_reverse_transport_frozen_residual: BjtCorrelatedReverseTransportFrozenResidualAudit,
    correlated_reverse_transport_resolve: &'a BjtCorrelatedReverseTransportResolveValidation,
    correlated_reverse_transport_attribution: &'a BjtReciprocalTransportDispersionAttribution,
    correlated_reverse_transport_tr19_evidence: BjtReciprocalTransportTr19EvidenceAudit,
    correlated_reverse_transport_tr22_tr23_evidence: BjtReciprocalTransportTr22Tr23EvidenceAudit,
}

fn active_network_solver_readiness(
    inputs: ActiveNetworkSolverReadinessInputs<'_>,
) -> ActiveNetworkSolverReadiness {
    let ActiveNetworkSolverReadinessInputs {
        dc_validation,
        assignment_sweep,
        mn3101_matrix,
        mn3101_homotopy,
        saturation_base_drive,
        output_curve_resolution,
        low_vce_base_drive,
        reciprocal_transport_sweep,
        reciprocal_transport_homotopy,
        reciprocal_transport_assignment,
        reciprocal_transport_attribution,
        reciprocal_transport_tr19_evidence,
        tr19_high_vce_conductance,
        reciprocal_transport_tr22_tr23_evidence,
        tr22_tr23_reverse_beta,
        correlated_reverse_transport_frozen_residual,
        correlated_reverse_transport_resolve,
        correlated_reverse_transport_attribution,
        correlated_reverse_transport_tr19_evidence,
        correlated_reverse_transport_tr22_tr23_evidence,
    } = inputs;
    let output_characteristics = bjt_output_characteristic_evidence_validation();
    ActiveNetworkSolverReadiness {
        explicit_connection_graph_ready: true,
        published_bjt_dc_curve_sets: 4,
        digitized_bjt_typical_transfer_fits_ready: true,
        bjt_forward_active_evidence_corners_ready: true,
        bjt_cutoff_endpoint_limits_ready: true,
        bjt_saturation_endpoint_limits_ready: true,
        bjt_local_output_admittance_records: 2,
        bjt_published_power_derating_ready: true,
        bjt_candidates_with_direct_temperature_transfer_curves: 2,
        bjt_candidates_with_comparative_temperature_transfer_intervals: 2,
        bjt_direct_toshiba_thermal_gain_curves_ready: true,
        bjt_direct_toshiba_thermal_saturation_curves_ready: true,
        bjt_continuous_dc_hypothesis_count: 96,
        bjt_preliminary_continuous_region_sweep_ready: true,
        bjt_candidates_with_output_characteristic_base_drive_families: output_characteristics
            .candidate_count,
        bjt_output_characteristic_saturation_knee_domain_covered: output_characteristics
            .every_family_covers_conservative_network_current
            && output_characteristics.every_family_covers_saturation_knee_voltage,
        bjt_output_characteristic_surfaces_digitized: output_characteristics
            .digitized_output_surface_ready,
        bjt_output_characteristic_clamped_points_evaluated: output_curve_resolution
            .evaluated_clamped_points,
        bjt_output_characteristic_clamped_points_directly_resolvable: output_curve_resolution
            .points_meeting_solver_resolution_margin,
        bjt_output_characteristic_low_vce_resolution_sufficient: output_curve_resolution
            .every_clamped_point_directly_resolvable,
        bjt_low_vce_base_drive_hypotheses_defined: low_vce_base_drive.defined_hypothesis_count,
        bjt_low_vce_base_drive_reference_survivors: low_vce_base_drive
            .reference_screen_survivor_count,
        bjt_low_vce_base_drive_matrix_points_evaluated: low_vce_base_drive.evaluated_matrix_points,
        bjt_low_vce_base_drive_matrix_points_converged: low_vce_base_drive.converged_matrix_points,
        bjt_low_vce_base_drive_hypotheses_eliminating_all_clamps: low_vce_base_drive
            .hypotheses_eliminating_clamps_for_all_assignments,
        bjt_low_vce_base_drive_family_viable: low_vce_base_drive.complete_dc_ensemble_accepted,
        bjt_reciprocal_transport_hypotheses_defined: reciprocal_transport_sweep.hypothesis_count,
        bjt_reciprocal_transport_matrix_points_evaluated: reciprocal_transport_sweep
            .evaluated_matrix_points,
        bjt_reciprocal_transport_matrix_points_converged: reciprocal_transport_sweep
            .converged_matrix_points,
        bjt_reciprocal_transport_exact_reference_targets_reached: reciprocal_transport_homotopy
            .exact_targets_reached,
        bjt_reciprocal_transport_exact_reference_targets_inside_all_gates:
            reciprocal_transport_homotopy.exact_targets_inside_all_dc_gates,
        bjt_reciprocal_transport_reference_candidate_viable: reciprocal_transport_homotopy
            .reciprocal_transport_family_viable_at_reference,
        bjt_reciprocal_transport_assignment_endpoints_attempted: reciprocal_transport_assignment
            .attempted_assignments,
        bjt_reciprocal_transport_assignment_endpoints_converged: reciprocal_transport_assignment
            .converged_assignments,
        bjt_reciprocal_transport_assignment_endpoints_inside_all_gates:
            reciprocal_transport_assignment.assignments_inside_all_dc_gates,
        bjt_reciprocal_transport_assignment_endpoints_without_clamping:
            reciprocal_transport_assignment.assignments_without_any_transfer_clamping,
        bjt_reciprocal_transport_assignment_dc_metrics_accepted: reciprocal_transport_assignment
            .dc_metrics_accepted_as_equivalent,
        bjt_reciprocal_transport_dispersion_attributed: reciprocal_transport_attribution
            .balanced_complete_two_level_factorial,
        bjt_reciprocal_transport_dominant_current_designator: reciprocal_transport_attribution
            .dominant_collector_current_designator,
        bjt_reciprocal_transport_dominant_power_designator: reciprocal_transport_attribution
            .dominant_maximum_power_designator,
        bjt_reciprocal_transport_tr19_candidate_envelopes: reciprocal_transport_tr19_evidence
            .tr19_candidate_envelopes,
        bjt_reciprocal_transport_tr19_candidates_inside_output_plot_voltage_domain:
            reciprocal_transport_tr19_evidence.tr19_candidates_inside_output_plot_voltage_domain,
        bjt_reciprocal_transport_tr19_candidates_inside_25c_power_limit:
            reciprocal_transport_tr19_evidence.tr19_candidates_inside_25c_power_limit,
        bjt_reciprocal_transport_tr19_existing_documents_sufficient:
            reciprocal_transport_tr19_evidence
                .existing_documents_sufficient_to_narrow_reciprocal_current_power_spread,
        bjt_reciprocal_transport_tr19_higher_voltage_output_evidence_required:
            reciprocal_transport_tr19_evidence
                .higher_voltage_output_characteristic_evidence_required,
        bjt_tr19_high_vce_conductance_hypotheses_screened: tr19_high_vce_conductance
            .evaluated_hypotheses,
        bjt_tr19_high_vce_conductance_can_pass_power_gate: tr19_high_vce_conductance
            .tr19_differential_conductance_alone_can_pass_power_gate,
        bjt_tr19_high_vce_conductance_can_pass_current_gate: tr19_high_vce_conductance
            .tr19_differential_conductance_alone_can_pass_current_gate,
        bjt_tr19_high_vce_conductance_can_pass_both_gates: tr19_high_vce_conductance
            .tr19_differential_conductance_alone_can_pass_both_gates,
        bjt_reciprocal_transport_tr22_candidates_resolvable:
            reciprocal_transport_tr22_tr23_evidence.tr22_candidates_resolvable_at_two_reading_radii,
        bjt_reciprocal_transport_tr22_candidates_inside_base_drive_domain:
            reciprocal_transport_tr22_tr23_evidence
                .tr22_candidates_inside_labeled_base_current_domain,
        bjt_reciprocal_transport_tr23_reverse_transport_candidates:
            reciprocal_transport_tr22_tr23_evidence
                .tr23_candidates_with_reverse_transport_at_every_sample,
        bjt_reciprocal_transport_tr23_reverse_transport_evidence_required:
            reciprocal_transport_tr22_tr23_evidence.tr23_reverse_transport_evidence_required,
        bjt_reciprocal_transport_tr22_tr23_existing_documents_sufficient:
            reciprocal_transport_tr22_tr23_evidence
                .existing_documents_sufficient_to_bound_remaining_current_spread,
        bjt_tr22_tr23_reverse_beta_pairs_screened: tr22_tr23_reverse_beta.evaluated_beta_pairs,
        bjt_tr22_tr23_reverse_beta_pairs_passing_frozen_gates: tr22_tr23_reverse_beta
            .joint_current_power_accepted_beta_pairs,
        bjt_tr22_tr23_reverse_beta_direct_evidence_available: tr22_tr23_reverse_beta
            .direct_reverse_beta_evidence_available,
        bjt_tr22_tr23_reverse_beta_ready_for_solver_substitution: tr22_tr23_reverse_beta
            .suitable_for_solver_substitution,
        bjt_correlated_reverse_transport_frozen_assignments_exceeding_kcl:
            correlated_reverse_transport_frozen_residual.assignments_exceeding_solver_kcl_acceptance,
        bjt_correlated_reverse_transport_maximum_kcl_multiple:
            correlated_reverse_transport_frozen_residual
                .maximum_parameter_delta_kcl_residual_multiple,
        bjt_correlated_reverse_transport_full_resolve_required:
            correlated_reverse_transport_frozen_residual
                .full_nonlinear_base_and_collector_kcl_resolve_required,
        bjt_correlated_reverse_transport_resolved_assignments: correlated_reverse_transport_resolve
            .converged_assignments,
        bjt_correlated_reverse_transport_resolved_dc_metrics_accepted:
            correlated_reverse_transport_resolve.resolved_dc_metrics_accepted_as_equivalent,
        bjt_correlated_reverse_transport_ready_for_evidence_testing:
            correlated_reverse_transport_resolve.numerical_path_suitable_for_evidence_testing,
        bjt_correlated_reverse_transport_dispersion_attributed:
            correlated_reverse_transport_attribution.balanced_complete_two_level_factorial,
        bjt_correlated_reverse_transport_dominant_current_designator:
            correlated_reverse_transport_attribution.dominant_collector_current_designator,
        bjt_correlated_reverse_transport_dominant_power_designator:
            correlated_reverse_transport_attribution.dominant_maximum_power_designator,
        bjt_correlated_reverse_transport_tr19_high_voltage_evidence_required:
            correlated_reverse_transport_tr19_evidence
                .higher_voltage_output_characteristic_evidence_required,
        bjt_correlated_reverse_transport_tr22_surface_evidence_required:
            correlated_reverse_transport_tr22_tr23_evidence
                .tr22_output_surface_digitization_required,
        bjt_correlated_reverse_transport_tr23_reverse_evidence_required:
            correlated_reverse_transport_tr22_tr23_evidence.tr23_reverse_transport_evidence_required,
        bjt_forced_beta_ten_base_drive_homotopy_reached: saturation_base_drive
            .exact_forced_beta_ten_hypothesis_reached,
        bjt_saturation_transfer_clamps_eliminated: saturation_base_drive
            .transfer_domain_clamps_eliminated,
        coupled_dc_residual_assembly_ready: true,
        preliminary_dc_fixed_point_count: dc_validation.converged_coordinate_count,
        preliminary_dc_fixed_point_solver_ready: dc_validation.preliminary_fixed_points_ready,
        bjt_cutoff_saturation_and_early_regions_ready: false,
        bounded_bjt_dc_fits_ready: false,
        digitized_diode_typical_forward_curve_ready: true,
        bounded_diode_iv_model_ready: true,
        mn3101_ox_logic_and_endpoint_bounds_available: true,
        mn3101_ox_continuous_port_model_available: true,
        mn3101_frozen_node_matrix_points_evaluated: mn3101_matrix.evaluated_matrix_points,
        mn3101_macros_compatible_with_all_frozen_assignments: mn3101_matrix
            .macros_compatible_with_all_frozen_assignment_points,
        mn3101_homotopy_exact_targets_reached: mn3101_homotopy.exact_targets_reached,
        mn3101_homotopy_all_targets_reached: mn3101_homotopy.exact_targets_reached
            == mn3101_homotopy.target_macro_hypotheses,
        mn3101_full_nonlinear_macro_sweep_ready: false,
        full_dc_problem_well_posed: false,
        evaluated_assignment_hypotheses: assignment_sweep.attempted_assignments,
        required_missing_models: [
            "a higher-resolution or otherwise bounded sub-50 mV IC-VCE-by-IB model, Mitsubishi thermal gain/saturation, global output-conductance behavior and unit-spread bounds for production-grade BJT models",
        ],
        safe_intermediate_scope: "retain reciprocal transport outside audible DSP; seek Tr19 output evidence through at least 28.14 V and 0.255 mA, Tr23 reverse-transport evidence over roughly -39 to -34 mV and 54-74 uA collector current, and Tr22 saturation surfaces over 0.080-0.211 V including Mitsubishi base drive near 260 uA before sweeping the stable nonlinear path",
        interpretation: "v41 rechecks the evidence domains at the nonlinear endpoints: node feedback shifts Toshiba Tr19 to 28.131-28.132 V and Tr22 to 0.2105-0.2109 V but leaves every decisive gap unchanged—Tr19 is beyond output plots, Tr23 remains negative-VCE reverse transport, and Tr22 still lacks a digitized surface plus Mitsubishi base-drive coverage",
    }
}

const fn uncertainty_acceptance_policy() -> UncertaintyAcceptancePolicy {
    UncertaintyAcceptancePolicy {
        reference: "arithmetic mean of all finite candidate-hypothesis results at each evaluated coordinate",
        mean_relative_deviation_limit_percent: 1.0,
        maximum_relative_deviation_limit_percent: 3.0,
        minimum_hypotheses: 32,
        required_metrics: [
            "BBD clock-frequency trajectory",
            "BBD delay trajectory",
            "modulation depth",
            "inter-channel differential trajectory",
            "clock-dependent bandwidth trajectory",
        ],
        interpretation: "passing means the unresolved device identity is behaviorally immaterial inside this model and evidence envelope; it does not identify the installed part",
    }
}

fn compare_uncertainty_ensemble(
    values: &[f64],
    policy: UncertaintyAcceptancePolicy,
) -> UncertaintyComparison {
    let all_values_finite = values.iter().all(|value| value.is_finite());
    let enough_hypotheses = values.len() >= policy.minimum_hypotheses;
    if values.is_empty() || !all_values_finite {
        return UncertaintyComparison {
            hypothesis_count: values.len(),
            ensemble_mean: 0.0,
            mean_absolute_relative_deviation_percent: f64::INFINITY,
            maximum_absolute_relative_deviation_percent: f64::INFINITY,
            all_values_finite,
            enough_hypotheses,
            accepted: false,
        };
    }

    let ensemble_mean = values.iter().sum::<f64>() / values.len() as f64;
    let denominator = ensemble_mean.abs().max(f64::EPSILON);
    let relative_deviations = values
        .iter()
        .map(|value| ((value - ensemble_mean).abs() / denominator) * 100.0);
    let mean_absolute_relative_deviation_percent =
        relative_deviations.clone().sum::<f64>() / values.len() as f64;
    let maximum_absolute_relative_deviation_percent = relative_deviations.fold(0.0, f64::max);
    let accepted = enough_hypotheses
        && mean_absolute_relative_deviation_percent <= policy.mean_relative_deviation_limit_percent
        && maximum_absolute_relative_deviation_percent
            <= policy.maximum_relative_deviation_limit_percent;
    UncertaintyComparison {
        hypothesis_count: values.len(),
        ensemble_mean,
        mean_absolute_relative_deviation_percent,
        maximum_absolute_relative_deviation_percent,
        all_values_finite,
        enough_hypotheses,
        accepted,
    }
}

fn uncertainty_gate_fixtures() -> [UncertaintyGateFixture; 2] {
    let mut compact = [0.0; 32];
    for (index, value) in compact.iter_mut().enumerate() {
        *value = 100.0 + (index as f64 - 15.5) * 0.02;
    }
    let mut hidden_outlier = [100.0; 32];
    hidden_outlier[31] = 109.0;
    [
        UncertaintyGateFixture {
            name: "compact complete ensemble",
            expected_acceptance: true,
            comparison: compare_uncertainty_ensemble(&compact, uncertainty_acceptance_policy()),
        },
        UncertaintyGateFixture {
            name: "mean below limit with rejected outlier",
            expected_acceptance: false,
            comparison: compare_uncertainty_ensemble(
                &hidden_outlier,
                uncertainty_acceptance_policy(),
            ),
        },
    ]
}

fn active_device_uncertainty_status(
    assignment_sweep: &ActiveDeviceDcSweepValidation,
) -> ActiveDeviceUncertaintyStatus {
    ActiveDeviceUncertaintyStatus {
        expected_hypotheses: 32,
        attempted_hypotheses: assignment_sweep.attempted_assignments,
        numerically_converged_hypotheses: assignment_sweep.converged_assignments,
        evaluated_hypotheses: assignment_sweep.assignments_inside_all_dc_gates,
        required_metric_count: 5,
        evaluated_metric_count: 0,
        accepted_as_behaviorally_equivalent: false,
        exact_device_identity_solved: false,
    }
}

const fn active_clock_network_boundaries() -> [ActiveClockNetworkBoundary; 2] {
    [
        ActiveClockNetworkBoundary {
            channel: "A",
            triangle_input: "TP3 through R128 1 kOhm",
            positive_supply: "+15 V",
            negative_supply: "-15 V through the shared lower-rail network",
            timing_and_driver_nodes: "C53 150 pF and IC9 MN3101 OX1/OX2/OX3",
            clock_output: "IC9 CP1/CP2",
            bbd_destination: "IC7 MN3009",
        },
        ActiveClockNetworkBoundary {
            channel: "B",
            triangle_input: "TP4 through R148 1 kOhm",
            positive_supply: "+15 V",
            negative_supply: "-15 V through the shared lower-rail network",
            timing_and_driver_nodes: "C57 150 pF and IC11 MN3101 OX1/OX2/OX3",
            clock_output: "IC11 CP1/CP2",
            bbd_destination: "IC10 MN3009",
        },
    ]
}

const fn factory_adjustment() -> FactoryAdjustmentEvidence {
    FactoryAdjustmentEvidence {
        name: "chorus bias",
        test_instrument: "oscilloscope and audio generator",
        test_points: "TP1 channel 1 and TP2 channel 2 on jack board",
        setup: "VCA LEVEL 0, CHORUS I",
        injected_signal: "10 Vpp, 1 kHz sine wave into module-board TP2",
        adjusted_components: "VR1 channel 1 and VR2 channel 2 on jack board",
        acceptance: "positive and negative waveform halves symmetrical about the center horizontal line",
        constrains_clock_rate_or_depth: false,
    }
}

const fn transmitted_switch1_value(assigner_switch1_bits: u8) -> u8 {
    !assigner_switch1_bits & 0x7f
}

const fn module_chorus_latch_low_bits(transmitted_value: u8) -> u8 {
    // Voice B_2 at 0x015B sets latch bit 0 when incoming bit 5 is
    // clear (chorus disabled). At 0x015F it copies incoming bit 6 to
    // latch bit 1 (the I/II rate line).
    let disable = if transmitted_value & 0x20 == 0 { 1 } else { 0 };
    let mode = if transmitted_value & 0x40 != 0 { 2 } else { 0 };
    disable | mode
}

const fn firmware_chorus_state(
    panel_selection: &'static str,
    assigner_switch1_bits: u8,
    canonical_audio_mode: &'static str,
    distinct_audio_state: bool,
    normal_one_hot_state: bool,
) -> FirmwareChorusState {
    let transmitted_switch1_value = transmitted_switch1_value(assigner_switch1_bits);
    FirmwareChorusState {
        panel_selection,
        assigner_switch1_bits,
        transmitted_switch1_value,
        module_latch_low_bits: module_chorus_latch_low_bits(transmitted_switch1_value),
        canonical_audio_mode,
        distinct_audio_state,
        normal_one_hot_state,
    }
}

const fn firmware_chorus_states() -> [FirmwareChorusState; 4] {
    [
        firmware_chorus_state("off", 0x20, "off", true, true),
        firmware_chorus_state("i", 0x40, "i", true, true),
        firmware_chorus_state("ii", 0x80, "ii", true, true),
        firmware_chorus_state("coincident_i_and_ii", 0xc0, "i", false, false),
    ]
}

fn effective_drive_resistance(branch: BranchState, r5: f64, r8: f64, r4: f64) -> f64 {
    match branch {
        BranchState::Tr1Open => r5 + r8,
        // With Tr1 idealized as a short, R4 shunts the R5/R8 junction.
        // Nodal reduction gives the resistance that maps comparator output
        // voltage to the current entering the integrator's virtual ground.
        BranchState::Tr1IdealShunt => r5 + r8 + r5 * r8 / r4,
    }
}

fn schmitt_feedback_ratio(r15: f64, r6: f64) -> f64 {
    r15 / (r15 + r6)
}

fn ideal_oscillator_rate_hz(
    branch: BranchState,
    r5: f64,
    r8: f64,
    r4: f64,
    r6: f64,
    r15: f64,
    c3: f64,
) -> f64 {
    let threshold_ratio = schmitt_feedback_ratio(r15, r6);
    let tau = effective_drive_resistance(branch, r5, r8, r4) * c3;
    // Each half-cycle traverses twice the Schmitt threshold. Comparator
    // swing cancels because it drives both the threshold and the integrator.
    1.0 / (4.0 * threshold_ratio * tau)
}

fn rc_scale(branch: BranchState) -> RcScale {
    let nominal_resistance_ohms = effective_drive_resistance(branch, R5_OHMS, R8_OHMS, R4_OHMS);
    let nominal_seconds = nominal_resistance_ohms * C3_FARADS;
    let mut minimum_seconds = f64::INFINITY;
    let mut maximum_seconds = f64::NEG_INFINITY;

    for r5_scale in [1.0 - RESISTOR_TOLERANCE, 1.0 + RESISTOR_TOLERANCE] {
        for r8_scale in [1.0 - RESISTOR_TOLERANCE, 1.0 + RESISTOR_TOLERANCE] {
            for r4_scale in [1.0 - RESISTOR_TOLERANCE, 1.0 + RESISTOR_TOLERANCE] {
                for c3_scale in [1.0 - CAPACITOR_TOLERANCE, 1.0 + CAPACITOR_TOLERANCE] {
                    let seconds = effective_drive_resistance(
                        branch,
                        R5_OHMS * r5_scale,
                        R8_OHMS * r8_scale,
                        R4_OHMS * r4_scale,
                    ) * C3_FARADS
                        * c3_scale;
                    minimum_seconds = minimum_seconds.min(seconds);
                    maximum_seconds = maximum_seconds.max(seconds);
                }
            }
        }
    }

    RcScale {
        branch,
        nominal_resistance_ohms,
        nominal_seconds,
        minimum_seconds,
        maximum_seconds,
    }
}

fn ideal_oscillator_estimate(branch: BranchState) -> IdealOscillatorEstimate {
    let nominal_hz = ideal_oscillator_rate_hz(
        branch, R5_OHMS, R8_OHMS, R4_OHMS, R6_OHMS, R15_OHMS, C3_FARADS,
    );
    let mut minimum_hz = f64::INFINITY;
    let mut maximum_hz = f64::NEG_INFINITY;

    for r5_scale in [1.0 - RESISTOR_TOLERANCE, 1.0 + RESISTOR_TOLERANCE] {
        for r8_scale in [1.0 - RESISTOR_TOLERANCE, 1.0 + RESISTOR_TOLERANCE] {
            for r4_scale in [1.0 - RESISTOR_TOLERANCE, 1.0 + RESISTOR_TOLERANCE] {
                for r6_scale in [1.0 - RESISTOR_TOLERANCE, 1.0 + RESISTOR_TOLERANCE] {
                    for r15_scale in [1.0 - RESISTOR_TOLERANCE, 1.0 + RESISTOR_TOLERANCE] {
                        for c3_scale in [1.0 - CAPACITOR_TOLERANCE, 1.0 + CAPACITOR_TOLERANCE] {
                            let hz = ideal_oscillator_rate_hz(
                                branch,
                                R5_OHMS * r5_scale,
                                R8_OHMS * r8_scale,
                                R4_OHMS * r4_scale,
                                R6_OHMS * r6_scale,
                                R15_OHMS * r15_scale,
                                C3_FARADS * c3_scale,
                            );
                            minimum_hz = minimum_hz.min(hz);
                            maximum_hz = maximum_hz.max(hz);
                        }
                    }
                }
            }
        }
    }

    IdealOscillatorEstimate {
        branch,
        schmitt_feedback_ratio: schmitt_feedback_ratio(R15_OHMS, R6_OHMS),
        nominal_hz,
        minimum_hz,
        maximum_hz,
    }
}

fn hardware_control_states() -> [HardwareControlState; 3] {
    [
        HardwareControlState {
            panel_mode: "off",
            chorus_enable_line_volts: CONTROL_HIGH_VOLTS,
            chorus_rate_line_volts: CONTROL_HIGH_VOLTS,
            rate_line_is_dont_care: true,
            tr5_enable_clamp: "engaged",
            tr2_state: "off",
            d1_state: "forward_biases_tr1_gate_negative",
            tr1_gate_state: "cut_off_but_inaudible_while_tp4_is_clamped",
            oscillator_branch: "dont_care_while_disabled",
        },
        HardwareControlState {
            panel_mode: "i",
            chorus_enable_line_volts: CONTROL_LOW_VOLTS,
            chorus_rate_line_volts: CONTROL_LOW_VOLTS,
            rate_line_is_dont_care: false,
            tr5_enable_clamp: "released",
            tr2_state: "on",
            d1_state: "reverse_biased",
            tr1_gate_state: "vgs_approximately_zero_conducting",
            oscillator_branch: "r4_shunt_active_slow",
        },
        HardwareControlState {
            panel_mode: "ii",
            chorus_enable_line_volts: CONTROL_LOW_VOLTS,
            chorus_rate_line_volts: CONTROL_HIGH_VOLTS,
            rate_line_is_dont_care: false,
            tr5_enable_clamp: "released",
            tr2_state: "off",
            d1_state: "forward_biases_tr1_gate_negative",
            tr1_gate_state: "cut_off",
            oscillator_branch: "r4_shunt_open_fast",
        },
    ]
}

fn coordinate_comparison(
    mode: &'static str,
    current_hz: f64,
    branch: BranchState,
) -> CoordinateComparison {
    let estimate = ideal_oscillator_estimate(branch);
    let inside = current_hz >= estimate.minimum_hz && current_hz <= estimate.maximum_hz;
    let distance_above_maximum_percent = if current_hz > estimate.maximum_hz {
        (current_hz / estimate.maximum_hz - 1.0) * 100.0
    } else {
        0.0
    };

    CoordinateComparison {
        mode,
        current_hz,
        branch,
        reduced_nominal_hz: estimate.nominal_hz,
        reduced_minimum_hz: estimate.minimum_hz,
        reduced_maximum_hz: estimate.maximum_hz,
        inside_assumed_component_corners: inside,
        distance_above_maximum_percent,
    }
}

fn components() -> [ComponentValue; 10] {
    [
        ComponentValue {
            designator: "R5",
            nominal: R5_OHMS,
            unit: "ohm",
            role: "integrator drive",
        },
        ComponentValue {
            designator: "R8",
            nominal: R8_OHMS,
            unit: "ohm",
            role: "integrator drive",
        },
        ComponentValue {
            designator: "R4",
            nominal: R4_OHMS,
            unit: "ohm",
            role: "Tr1-controlled shunt",
        },
        ComponentValue {
            designator: "R6",
            nominal: R6_OHMS,
            unit: "ohm",
            role: "Schmitt positive feedback",
        },
        ComponentValue {
            designator: "R7",
            nominal: R7_OHMS,
            unit: "ohm",
            role: "TP4 comparator input",
        },
        ComponentValue {
            designator: "R15",
            nominal: R15_OHMS,
            unit: "ohm",
            role: "Schmitt threshold divider",
        },
        ComponentValue {
            designator: "C3",
            nominal: C3_FARADS,
            unit: "farad",
            role: "integrator feedback",
        },
        ComponentValue {
            designator: "R10",
            nominal: R10_OHMS,
            unit: "ohm",
            role: "TP3 inverter input",
        },
        ComponentValue {
            designator: "R9",
            nominal: R9_OHMS,
            unit: "ohm",
            role: "TP3 inverter feedback",
        },
        ComponentValue {
            designator: "C4",
            nominal: C4_FARADS,
            unit: "farad",
            role: "TP3 inverter compensation",
        },
    ]
}

fn build_report() -> Report {
    let active_network_dc_operating_points = active_network_dc_operating_points();
    let active_network_dc_solver_validation =
        active_network_dc_solver_validation_for(&active_network_dc_operating_points);
    let bjt_output_characteristic_clamped_point_resolutions =
        bjt_output_characteristic_clamped_point_resolutions(&active_network_dc_operating_points);
    let bjt_output_characteristic_clamped_resolution_validation =
        bjt_output_characteristic_clamped_resolution_validation(
            &bjt_output_characteristic_clamped_point_resolutions,
        );
    let active_device_dc_operating_points =
        active_device_dc_sweep_points(&active_network_dc_operating_points);
    let active_device_dc_sweep_points =
        active_device_dc_sweep_records(&active_device_dc_operating_points);
    let active_device_dc_sweep_validation =
        active_device_dc_sweep_validation(&active_device_dc_sweep_points);
    let bjt_low_vce_base_drive_reference_screen_points =
        bjt_low_vce_base_drive_reference_screen_points(&active_device_dc_operating_points);
    let bjt_low_vce_base_drive_sweep_points = bjt_low_vce_base_drive_sweep_points(
        &active_device_dc_operating_points,
        &bjt_low_vce_base_drive_reference_screen_points,
    );
    let bjt_low_vce_base_drive_sweep_validation = bjt_low_vce_base_drive_sweep_validation(
        &bjt_low_vce_base_drive_reference_screen_points,
        &bjt_low_vce_base_drive_sweep_points,
    );
    let bjt_reciprocal_transport_sweep_points =
        bjt_reciprocal_transport_sweep_points(&active_device_dc_operating_points);
    let bjt_reciprocal_transport_sweep_validation =
        bjt_reciprocal_transport_sweep_validation(&bjt_reciprocal_transport_sweep_points);
    let (bjt_reciprocal_transport_homotopy_stages, bjt_reciprocal_transport_homotopy_summaries) =
        bjt_reciprocal_transport_homotopy(&active_device_dc_operating_points);
    let bjt_reciprocal_transport_homotopy_validation =
        bjt_reciprocal_transport_homotopy_validation(&bjt_reciprocal_transport_homotopy_summaries);
    let bjt_reciprocal_transport_assignment_points =
        bjt_reciprocal_transport_assignment_points(&active_device_dc_operating_points);
    let bjt_reciprocal_transport_assignment_validation =
        bjt_reciprocal_transport_assignment_validation(&bjt_reciprocal_transport_assignment_points);
    let bjt_reciprocal_transport_dispersion_attribution =
        bjt_reciprocal_transport_dispersion_attribution(
            &bjt_reciprocal_transport_assignment_points,
        );
    let bjt_reciprocal_transport_device_evidence_envelopes =
        bjt_reciprocal_transport_device_evidence_envelopes(
            &bjt_reciprocal_transport_assignment_points,
        );
    let bjt_reciprocal_transport_tr19_evidence_audit = bjt_reciprocal_transport_tr19_evidence_audit(
        &bjt_reciprocal_transport_device_evidence_envelopes,
    );
    let bjt_reciprocal_transport_tr22_tr23_evidence_audit =
        bjt_reciprocal_transport_tr22_tr23_evidence_audit(
            &bjt_reciprocal_transport_device_evidence_envelopes,
        );
    let bjt_tr19_high_vce_conductance_requirement =
        bjt_tr19_high_vce_conductance_requirement(&bjt_reciprocal_transport_assignment_points);
    let bjt_tr22_tr23_reverse_beta_requirement =
        bjt_tr22_tr23_reverse_beta_requirement(&bjt_reciprocal_transport_assignment_points);
    let bjt_correlated_reverse_transport_frozen_residual_points =
        bjt_correlated_reverse_transport_frozen_residual_points(
            &bjt_reciprocal_transport_assignment_points,
            bjt_tr22_tr23_reverse_beta_requirement,
        );
    let bjt_correlated_reverse_transport_frozen_residual_audit =
        bjt_correlated_reverse_transport_frozen_residual_audit(
            &bjt_correlated_reverse_transport_frozen_residual_points,
        );
    let bjt_correlated_reverse_transport_resolve_points =
        bjt_correlated_reverse_transport_resolve_points(
            &bjt_reciprocal_transport_assignment_points,
            bjt_tr22_tr23_reverse_beta_requirement,
        );
    let bjt_correlated_reverse_transport_resolve_validation =
        bjt_correlated_reverse_transport_resolve_validation(
            &bjt_correlated_reverse_transport_resolve_points,
        );
    let bjt_correlated_reverse_transport_dispersion_attribution =
        bjt_correlated_reverse_transport_dispersion_attribution(
            &bjt_correlated_reverse_transport_resolve_points,
        );
    let bjt_correlated_reverse_transport_device_evidence_envelopes =
        bjt_correlated_reverse_transport_device_evidence_envelopes(
            &bjt_correlated_reverse_transport_resolve_points,
        );
    let bjt_correlated_reverse_transport_tr19_evidence_audit =
        bjt_correlated_reverse_transport_tr19_evidence_audit(
            &bjt_correlated_reverse_transport_device_evidence_envelopes,
        );
    let bjt_correlated_reverse_transport_tr22_tr23_evidence_audit =
        bjt_correlated_reverse_transport_tr22_tr23_evidence_audit(
            &bjt_correlated_reverse_transport_device_evidence_envelopes,
        );
    let mn3101_frozen_node_compatibility_points =
        mn3101_frozen_node_compatibility_points(&active_device_dc_operating_points);
    let mn3101_frozen_node_compatibility_summaries =
        mn3101_frozen_node_compatibility_summaries(&mn3101_frozen_node_compatibility_points);
    let mn3101_frozen_node_matrix_validation = mn3101_frozen_node_matrix_validation(
        &mn3101_frozen_node_compatibility_points,
        &mn3101_frozen_node_compatibility_summaries,
    );
    let (mn3101_homotopy_stages, mn3101_homotopy_summaries) =
        mn3101_homotopy_sweep(&active_network_dc_operating_points);
    let mn3101_homotopy_validation = mn3101_homotopy_validation(&mn3101_homotopy_summaries);
    let (bjt_saturation_base_drive_homotopy_stages, bjt_saturation_base_drive_homotopy_validation) =
        bjt_saturation_base_drive_homotopy(&active_network_dc_operating_points);
    Report {
        schema: "rf-106-chorus-schematic-audit-v41",
        evidence_boundary: "E1/E2 reduced Schmitt-integrator model, exact mirrored Tr19-Tr28/D9-D10/MN3101 connection graph, exact clocked-device chain, endpoint-bounded 150 pF resistance/conductance requirements plus E3 firmware decode, an E6 cross-revision device shortlist, 30 calibrated 25 C BJT transfer anchors, four direct 25 C IC-VCE-by-IB output-characteristic families totaling 33 labeled curves, native 2176-pixel Mitsubishi page-raster calibration, 30 calibrated Toshiba -25/25/100 C transfer anchors, 60 calibrated Toshiba thermal hFE/VCE(sat) anchors, two explicitly comparative Mitsubishi temperature intervals over their direct 25 C centers, 36 evidence-only forward-active beta/reading corners, four cutoff and forced-beta saturation endpoint envelopes, 96 explicit continuous 25 C cutoff/active/saturation/output hypotheses, two local h_oe output-resistance proxies, direct 0-125 C collector-power derating for all four BJT candidates, a shared dual-candidate diode endpoint interval plus eight candidate-specific 1S2473 curve anchors, nine continuous MN3101 OX macro hypotheses, a coupled 13-node KCL residual system, a complete 32-assignment continuation sweep, a 288-point frozen-node assignment-by-MN3101 matrix and sixteen-step current-port homotopy to every macro; the arbitrary first-labeled-IB low-VCE family is rejected; v31 adds reciprocal two-junction transport with reverse-beta 1/10/100 and reference homotopy; v32 continues the exact beta-R=1 endpoint across all 32 assignments and rejects its aggregate current/power spread; v33 attributes that spread principally to Tr19; v34-v36 isolate high-VCE Tr19, saturated Tr22 and reverse-VCE Tr23 evidence gaps; v37 screens correlated Tr22/Tr23 differential reverse-beta requirements at frozen nodes; v38 proves their best frozen restamp violates KCL; v39 fully re-solves and rejects that target; v40 recomputes its factorial attribution; v41 recomputes every device evidence envelope at the nonlinear endpoints and turns the unchanged gaps into exact source-domain requirements; transient TP3/TP4 audio-trajectory gates, MN3101 hidden supply current, measured TP3 range, bounded global output conductance, reverse-beta bounds, unit VBE spread, exact active-device assignments and voltage-to-frequency transfer remain open",
        component_sources: [
            "SRC-SVC-1984 PDF page 9 / printed page 15",
            "SRC-SVC-106S-1985 PDF pages 5, 6 and 17",
            "SRC-FET-2SK30A Toshiba discrete-device catalog rank table",
            "SRC-CLK-3101 PDF pages 1-4 / printed pages 58-61",
            "SRC-BBD-CAT PDF pages 38-40 / printed pages 36-38",
            "SRC-SVC-60-1983 PDF page 11, comparative device rule only",
        ],
        firmware_sources: [
            "Assigner A_5 SHA-256 D43CCE5578EE2F16B27C8B06BFF30743E3E2DFFC796D033811E565D5D578C52E, offsets 0x0BEB-0x0C0A",
            "Voice B_2 SHA-256 F3C48E14434E29E264407E9163F30C0473F94C20957799ECF75746099D1BD2A2, offsets 0x014F-0x0164",
        ],
        source_render_dpi: 1200,
        source_conflicts: [SourceConflict {
            designator: "R15",
            target_source_value: "1 MOhm",
            related_revision_value: "1 kOhm",
            resolution: "retain target-source 1 MOhm; 1 kOhm predicts about 37.5 Hz and is quarantined as an internally inconsistent related-revision marking",
        }],
        components: components(),
        semiconductors: [
            SemiconductorAudit {
                designators: "IC1",
                device: "TL-062CP",
                evidence: "schematic label 062 plus related-revision parts list",
                confidence: "high",
            },
            SemiconductorAudit {
                designators: "Tr1",
                device: "2SK30A-GR or 2SK30A-Y (K381)",
                evidence: "schematic device marking plus related-revision parts list",
                confidence: "high",
            },
            SemiconductorAudit {
                designators: "D1,D2",
                device: "1SS-133 candidate",
                evidence: "sole ordinary small-signal diode in the related-revision parts inventory; individual designators are not mapped",
                confidence: "medium",
            },
        ],
        fet_grades: [
            FetGrade {
                grade: "2SK30A-Y",
                minimum_idss_ma: 1.2,
                maximum_idss_ma: 3.0,
                evidence: "Toshiba discrete-device catalog rank table",
            },
            FetGrade {
                grade: "2SK30A-GR",
                minimum_idss_ma: 2.6,
                maximum_idss_ma: 6.5,
                evidence: "Toshiba discrete-device catalog rank table",
            },
        ],
        device_bounds: [DeviceBounds {
            device: "TL062",
            condition: "+/-15 V, 10 kOhm load, 25 C",
            guaranteed_output_swing_volts: TL062_OUTPUT_SWING_GUARANTEED_VOLTS,
            typical_output_swing_volts: TL062_OUTPUT_SWING_TYPICAL_VOLTS,
            minimum_slew_rate_volts_per_microsecond: TL062_SLEW_RATE_MIN_VOLTS_PER_MICROSECOND,
            typical_slew_rate_volts_per_microsecond: TL062_SLEW_RATE_TYPICAL_VOLTS_PER_MICROSECOND,
        }],
        control_paths: [
            ControlPath {
                input: "CHORUS ON/OFF",
                devices: "Tr6 -> R45/R44 -> Tr5",
                function: "clamps TP4 when chorus is disabled",
                oscillator_rate_dependency: "enable/disable only",
            },
            ControlPath {
                input: "CHORUS I/II rate line",
                devices: "R2/R1 -> Tr2/R11 -> D1 -> Tr1/R3",
                function: "0 V turns Tr2 on and leaves Tr1 at VGS approximately zero; +15 V turns Tr2 off and pulls the Tr1 gate negative through D1",
                oscillator_rate_dependency: "0 V selects the active R4 shunt (slow I); +15 V opens it (fast II)",
            },
        ],
        hardware_control_states: hardware_control_states(),
        resistance_tolerance: RESISTOR_TOLERANCE,
        capacitance_tolerance: CAPACITOR_TOLERANCE,
        rc_scales: [
            rc_scale(BranchState::Tr1Open),
            rc_scale(BranchState::Tr1IdealShunt),
        ],
        ideal_oscillator_estimates: [
            ideal_oscillator_estimate(BranchState::Tr1Open),
            ideal_oscillator_estimate(BranchState::Tr1IdealShunt),
        ],
        firmware_chorus_states: firmware_chorus_states(),
        mn3101_pins: mn3101_pins(),
        mn3101_oscillator_examples: mn3101_oscillator_examples(),
        mn3101_ox_electrical_bounds: mn3101_ox_electrical_bounds(),
        mn3101_ox_macro_hypotheses: mn3101_ox_macro_hypotheses(),
        mn3101_ox_macro_reference_points: mn3101_ox_macro_reference_points(),
        mn3101_ox_macro_readiness: mn3101_ox_macro_readiness(),
        clock_channels: clock_channels(),
        current_clock_envelopes: [
            clock_delay_envelope("i", CURRENT_CENTER_DELAY_MS, CURRENT_MODE_I_DEPTH_MS),
            clock_delay_envelope("ii", CURRENT_CENTER_DELAY_MS, CURRENT_MODE_II_DEPTH_MS),
        ],
        mn3101_endpoint_fits: mn3101_endpoint_fits(),
        c150_equivalent_r2_envelopes: [
            equivalent_r2_envelope(clock_delay_envelope(
                "i",
                CURRENT_CENTER_DELAY_MS,
                CURRENT_MODE_I_DEPTH_MS,
            )),
            equivalent_r2_envelope(clock_delay_envelope(
                "ii",
                CURRENT_CENTER_DELAY_MS,
                CURRENT_MODE_II_DEPTH_MS,
            )),
        ],
        c150_equivalent_conductance_envelopes: [
            equivalent_conductance_envelope(equivalent_r2_envelope(clock_delay_envelope(
                "i",
                CURRENT_CENTER_DELAY_MS,
                CURRENT_MODE_I_DEPTH_MS,
            ))),
            equivalent_conductance_envelope(equivalent_r2_envelope(clock_delay_envelope(
                "ii",
                CURRENT_CENTER_DELAY_MS,
                CURRENT_MODE_II_DEPTH_MS,
            ))),
        ],
        c150_interpolation_method: "log-frequency interpolation between endpoint power-law fits for the manufacturer's R1=22 kOhm, C1=100 pF and 200 pF examples",
        c150_interpolation_has_assigned_device_tolerance: false,
        mirrored_clock_components: mirrored_clock_components(),
        mirrored_active_network_elements: mirrored_active_network_elements(),
        mirrored_bipolar_connections: mirrored_bipolar_connections(),
        mirrored_mn3101_connections: mirrored_mn3101_connections(),
        active_network_netlist_audit: active_network_netlist_audit(),
        mirrored_active_devices: mirrored_active_devices(),
        mirrored_diode_candidate: mirrored_diode_candidate(),
        diode_candidate_electrical_evidence: diode_candidate_electrical_evidence(),
        diode_forward_plot_calibration: diode_forward_plot_calibration(),
        diode_forward_anchors: diode_forward_anchors(),
        diode_piecewise_forward_validation: diode_piecewise_forward_validation(),
        diode_shared_endpoint_envelope: diode_shared_endpoint_envelope(),
        diode_forward_reference_intervals: diode_forward_reference_intervals(),
        diode_model_readiness: diode_model_readiness(),
        board_wide_bipolar_inventory: board_wide_bipolar_inventory(),
        related_family_active_device_rule: related_family_active_device_rule(),
        comparative_candidate_shortlist: comparative_candidate_shortlist(),
        active_device_assignment_space: active_device_assignment_space(),
        active_device_hypotheses: active_device_hypotheses(),
        active_device_model_sources: [
            "SRC-BJT-2SA1015 Toshiba manufacturer data",
            "SRC-BJT-2SA1115 Mitsubishi manufacturer scan",
            "SRC-BJT-2SC1815 Toshiba manufacturer data",
            "SRC-BJT-2SC2603 Mitsubishi manufacturer scan",
        ],
        candidate_bipolar_model_evidence: candidate_bipolar_model_evidence(),
        bjt_output_characteristic_plot_evidence: bjt_output_characteristic_plot_evidence(),
        bjt_output_characteristic_evidence_validation:
            bjt_output_characteristic_evidence_validation(),
        bjt_output_characteristic_clamped_point_resolutions,
        bjt_output_characteristic_clamped_resolution_validation,
        bjt_low_vce_base_drive_hypotheses: bjt_low_vce_base_drive_hypotheses(),
        bjt_low_vce_base_drive_reference_screen_points,
        bjt_low_vce_base_drive_sweep_points,
        bjt_low_vce_base_drive_sweep_validation: bjt_low_vce_base_drive_sweep_validation.clone(),
        bjt_reciprocal_transport_hypotheses: bjt_reciprocal_transport_hypotheses(),
        bjt_reciprocal_transport_sweep_points,
        bjt_reciprocal_transport_sweep_validation: bjt_reciprocal_transport_sweep_validation
            .clone(),
        bjt_reciprocal_transport_homotopy_stages,
        bjt_reciprocal_transport_homotopy_summaries,
        bjt_reciprocal_transport_homotopy_validation,
        bjt_reciprocal_transport_assignment_points,
        bjt_reciprocal_transport_assignment_validation:
            bjt_reciprocal_transport_assignment_validation.clone(),
        bjt_reciprocal_transport_dispersion_attribution:
            bjt_reciprocal_transport_dispersion_attribution.clone(),
        bjt_reciprocal_transport_device_evidence_envelopes,
        bjt_reciprocal_transport_tr19_evidence_audit,
        bjt_tr19_high_vce_conductance_requirement,
        bjt_reciprocal_transport_tr22_tr23_evidence_audit,
        bjt_tr22_tr23_reverse_beta_requirement,
        bjt_correlated_reverse_transport_frozen_residual_points,
        bjt_correlated_reverse_transport_frozen_residual_audit,
        bjt_correlated_reverse_transport_resolve_points,
        bjt_correlated_reverse_transport_resolve_validation:
            bjt_correlated_reverse_transport_resolve_validation.clone(),
        bjt_correlated_reverse_transport_dispersion_attribution:
            bjt_correlated_reverse_transport_dispersion_attribution.clone(),
        bjt_correlated_reverse_transport_device_evidence_envelopes,
        bjt_correlated_reverse_transport_tr19_evidence_audit,
        bjt_correlated_reverse_transport_tr22_tr23_evidence_audit,
        bjt_transfer_plot_calibrations: bjt_transfer_plot_calibrations(),
        bjt_transfer_pixel_anchors: bjt_transfer_pixel_anchors(),
        bjt_transfer_electrical_anchors: bjt_transfer_electrical_anchors(),
        bjt_typical_transfer_fits: bjt_typical_transfer_fits(),
        bjt_piecewise_transfer_validations: bjt_piecewise_transfer_validations(),
        bjt_forward_active_envelopes: bjt_forward_active_envelopes(),
        bjt_evidence_corners: bjt_evidence_corners(),
        bjt_forward_active_reference_points: bjt_forward_active_reference_points(),
        bjt_dc_region_evidence_envelopes: bjt_dc_region_evidence_envelopes(),
        bjt_continuous_dc_hypotheses: bjt_continuous_dc_hypotheses(),
        bjt_continuous_dc_reference_points: bjt_continuous_dc_reference_points(),
        bjt_continuous_dc_validations: bjt_continuous_dc_validations(),
        active_network_dc_coordinates: active_network_dc_coordinates(),
        active_network_dc_operating_points,
        active_network_dc_solver_validation,
        active_device_dc_sweep_points,
        active_device_dc_sweep_validation: active_device_dc_sweep_validation.clone(),
        mn3101_frozen_node_compatibility_points,
        mn3101_frozen_node_compatibility_summaries,
        mn3101_frozen_node_matrix_validation,
        mn3101_homotopy_stages,
        mn3101_homotopy_summaries,
        mn3101_homotopy_validation,
        bjt_saturation_base_drive_homotopy_stages,
        bjt_saturation_base_drive_homotopy_validation,
        bjt_published_thermal_evidence: bjt_published_thermal_evidence(),
        bjt_thermal_transfer_plot_calibrations: bjt_thermal_transfer_plot_calibrations(),
        bjt_thermal_transfer_pixel_anchors: bjt_thermal_transfer_pixel_anchors(),
        bjt_thermal_transfer_electrical_anchors: bjt_thermal_transfer_electrical_anchors(),
        bjt_thermal_transfer_validations: bjt_thermal_transfer_validations(),
        bjt_thermal_transfer_reference_points: bjt_thermal_transfer_reference_points(),
        bjt_thermal_characteristic_plot_calibrations: bjt_thermal_characteristic_plot_calibrations(
        ),
        bjt_thermal_characteristic_pixel_anchors: bjt_thermal_characteristic_pixel_anchors(),
        bjt_thermal_characteristic_electrical_anchors:
            bjt_thermal_characteristic_electrical_anchors(),
        bjt_thermal_characteristic_validations: bjt_thermal_characteristic_validations(),
        bjt_thermal_characteristic_reference_points: bjt_thermal_characteristic_reference_points(),
        bjt_comparative_thermal_reference_intervals: bjt_comparative_thermal_reference_intervals(),
        bjt_comparative_thermal_envelope_validations: bjt_comparative_thermal_envelope_validations(
        ),
        bjt_collector_power_reference_points: bjt_collector_power_reference_points(),
        bjt_thermal_evidence_readiness: bjt_thermal_evidence_readiness(),
        bipolar_model_parameter_boundary: bipolar_model_parameter_boundary(),
        uncertainty_acceptance_policy: uncertainty_acceptance_policy(),
        uncertainty_gate_fixtures: uncertainty_gate_fixtures(),
        active_device_uncertainty_status: active_device_uncertainty_status(
            &active_device_dc_sweep_validation,
        ),
        active_network_solver_readiness: active_network_solver_readiness(
            ActiveNetworkSolverReadinessInputs {
                dc_validation: active_network_dc_solver_validation,
                assignment_sweep: &active_device_dc_sweep_validation,
                mn3101_matrix: mn3101_frozen_node_matrix_validation,
                mn3101_homotopy: mn3101_homotopy_validation,
                saturation_base_drive: bjt_saturation_base_drive_homotopy_validation,
                output_curve_resolution: bjt_output_characteristic_clamped_resolution_validation,
                low_vce_base_drive: &bjt_low_vce_base_drive_sweep_validation,
                reciprocal_transport_sweep: &bjt_reciprocal_transport_sweep_validation,
                reciprocal_transport_homotopy: bjt_reciprocal_transport_homotopy_validation,
                reciprocal_transport_assignment: &bjt_reciprocal_transport_assignment_validation,
                reciprocal_transport_attribution: &bjt_reciprocal_transport_dispersion_attribution,
                reciprocal_transport_tr19_evidence: bjt_reciprocal_transport_tr19_evidence_audit,
                tr19_high_vce_conductance: bjt_tr19_high_vce_conductance_requirement,
                reciprocal_transport_tr22_tr23_evidence:
                    bjt_reciprocal_transport_tr22_tr23_evidence_audit,
                tr22_tr23_reverse_beta: bjt_tr22_tr23_reverse_beta_requirement,
                correlated_reverse_transport_frozen_residual:
                    bjt_correlated_reverse_transport_frozen_residual_audit,
                correlated_reverse_transport_resolve:
                    &bjt_correlated_reverse_transport_resolve_validation,
                correlated_reverse_transport_attribution:
                    &bjt_correlated_reverse_transport_dispersion_attribution,
                correlated_reverse_transport_tr19_evidence:
                    bjt_correlated_reverse_transport_tr19_evidence_audit,
                correlated_reverse_transport_tr22_tr23_evidence:
                    bjt_correlated_reverse_transport_tr22_tr23_evidence_audit,
            },
        ),
        module_board_device_legend_applies_to_clock_network: false,
        active_clock_network_boundaries: active_clock_network_boundaries(),
        factory_adjustment: factory_adjustment(),
        factory_clock_target_present: false,
        mn3101_oscillator_to_cp_divisor: MN3101_OSCILLATOR_TO_CP_DIVISOR,
        mn3009_stages: MN3009_STAGES,
        clock_networks_are_nominally_mirrored: true,
        clock_frequency_chain_solved: true,
        mn3101_control_transfer_solved: false,
        active_network_schematic_topology_captured: true,
        active_network_dc_solver_topology_ready: true,
        exact_active_device_assignments_solved: false,
        active_network_operating_point_solved: false,
        active_network_voltage_to_r2_solved: false,
        provisional_coordinates: [
            ProvisionalCoordinate {
                name: "mode_i",
                current_hz: CURRENT_MODE_I_RATE_HZ,
                evaluated_by_complete_model: false,
            },
            ProvisionalCoordinate {
                name: "mode_ii",
                current_hz: CURRENT_MODE_II_RATE_HZ,
                evaluated_by_complete_model: false,
            },
        ],
        coordinate_comparisons: [
            coordinate_comparison("i", CURRENT_MODE_I_RATE_HZ, BranchState::Tr1IdealShunt),
            coordinate_comparison("ii", CURRENT_MODE_II_RATE_HZ, BranchState::Tr1Open),
        ],
        unsupported_combined_coordinate_removed: true,
        reduced_rate_model_solved: true,
        steady_state_control_solved: true,
        absolute_rate_solved: false,
        production_promotion_ready: false,
        blockers: [
            "actual oscillator component values and TL062 threshold/output behavior for absolute rate",
            "Tr2 exact device assignment and analog transient during mode changes",
            "bound Mitsubishi thermal gain/saturation, output conductance and unit VBE spread before promoting the 96 continuous BJT region hypotheses beyond sensitivity analysis",
            "replace the rejected forced-beta and first-labeled-IB shortcuts with a physics-constrained low-VCE carrier-transport family before resolving the transfer clamps; then continue MN3101 macros 0/1 across convergence and 6/7 across VCE polarity before mapping measured TP3/TP4 voltage to clock",
        ],
    }
}

#[cfg(not(test))]
fn report() -> Report {
    build_report()
}

#[cfg(test)]
fn report() -> Report {
    static REPORT: std::sync::OnceLock<Report> = std::sync::OnceLock::new();
    REPORT.get_or_init(build_report).clone()
}

fn main() {
    println!(
        "{}",
        serde_json::to_string_pretty(&report()).expect("static report must serialize")
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn target_schematic_r15_value_is_one_megohm() {
        assert_eq!(R15_OHMS, 1_000_000.0);
        assert_eq!(components()[5].designator, "R15");
        assert_eq!(components()[5].nominal, 1_000_000.0);
    }

    #[test]
    fn diode_type_remains_an_explicit_candidate_not_a_direct_assignment() {
        let report = report();
        assert_eq!(report.semiconductors[2].confidence, "medium");
        assert!(report.semiconductors[2].device.contains("candidate"));
        assert_eq!(report.diode_candidate_electrical_evidence.len(), 2);
        assert!(
            report
                .diode_candidate_electrical_evidence
                .iter()
                .all(|candidate| !candidate.direct_target_designator_evidence)
        );
        assert!(!report.diode_model_readiness.exact_target_type_solved);
    }

    #[test]
    fn diode_candidate_endpoints_preserve_the_documentary_disagreement() {
        let candidates = diode_candidate_electrical_evidence();
        let later_inventory = candidates
            .iter()
            .find(|candidate| candidate.candidate == "1SS133")
            .unwrap();
        let comparative = candidates
            .iter()
            .find(|candidate| candidate.candidate == "1S2473")
            .unwrap();
        assert_eq!(later_inventory.reverse_repetitive_peak_v, 90.0);
        assert_eq!(later_inventory.reverse_dc_v, 80.0);
        assert_eq!(comparative.reverse_repetitive_peak_v, 40.0);
        assert_eq!(comparative.reverse_dc_v, 35.0);
        assert_eq!(later_inventory.forward_voltage_max_v, 1.2);
        assert_eq!(comparative.forward_voltage_max_v, 1.2);
        assert_eq!(later_inventory.forward_voltage_test_current_ma, 100.0);
        assert_eq!(comparative.forward_voltage_test_current_ma, 100.0);
        assert!(!later_inventory.manufacturer_typical_forward_curve_available);
        assert!(comparative.manufacturer_typical_forward_curve_available);
        assert_eq!(comparative.terminal_capacitance_max_pf, Some(3.0));
        assert_eq!(comparative.reverse_recovery_max_ns, Some(4.0));
    }

    #[test]
    fn comparative_diode_curve_is_monotonic_and_remains_candidate_specific() {
        let report = report();
        let readiness = report.diode_model_readiness;
        assert_eq!(readiness.documentary_candidates, 2);
        assert_eq!(readiness.candidates_with_endpoint_limits, 2);
        assert_eq!(readiness.candidates_with_digitized_typical_curve, 1);
        assert_eq!(readiness.digitized_curve_anchor_count, 8);
        assert!(readiness.digitized_curve_strictly_monotonic);
        assert!(readiness.shared_endpoint_interval_model_ready);
        assert!(readiness.network_forward_domain_covered);
        assert!(readiness.network_reverse_domain_covered);
        assert!(!readiness.continuous_nominal_curve_for_every_candidate_ready);
        assert!(readiness.bounded_forward_iv_for_every_candidate);
        assert!(readiness.bounded_reverse_iv_for_every_candidate);
        assert_eq!(report.diode_forward_anchors.len(), 8);
        assert!(report.diode_forward_anchors.iter().all(|anchor| {
            anchor.candidate == "1S2473"
                && anchor.forward_voltage_v > 0.0
                && anchor.forward_current_a > 0.0
                && anchor.voltage_reading_uncertainty_mv > 0.0
                && anchor.current_reading_uncertainty_percent > 0.0
        }));
    }

    #[test]
    fn diode_piecewise_center_round_trips_and_refuses_extrapolation() {
        let validation = diode_piecewise_forward_validation();
        assert_eq!(validation.candidate, "1S2473");
        assert_eq!(validation.anchor_count, 8);
        assert!(validation.anchors_strictly_monotonic);
        assert!(validation.maximum_anchor_round_trip_error_mv < 1.0e-9);
        assert!(!validation.extrapolation_permitted);
        assert!(
            piecewise_diode_forward_voltage("1S2473", validation.minimum_forward_current_a * 0.99)
                .is_none()
        );
        assert!(
            piecewise_diode_forward_voltage("1S2473", validation.maximum_forward_current_a * 1.01)
                .is_none()
        );
        assert!(piecewise_diode_forward_voltage("1SS133", 1.0e-3).is_none());
    }

    #[test]
    fn shared_diode_interval_covers_network_without_solving_identity() {
        let envelope = diode_shared_endpoint_envelope();
        assert_eq!(envelope.documentary_candidates, 2);
        assert_eq!(envelope.shared_forward_endpoint_test_current_a, 0.1);
        assert_eq!(envelope.shared_forward_voltage_maximum_v, 1.2);
        assert!(envelope.network_forward_domain_covered);
        assert!(envelope.extend_forward_endpoint_to_lower_current_is_monotonicity_hypothesis);
        assert_eq!(envelope.conservative_network_reverse_peak_v, 30.0);
        assert_eq!(envelope.shared_reverse_test_voltage_minimum_v, 35.0);
        assert!((envelope.shared_reverse_current_maximum_a - 0.5e-6).abs() < 1.0e-15);
        assert!(envelope.network_reverse_domain_covered);
        assert!(envelope.extend_reverse_endpoint_to_lower_voltage_is_monotonicity_hypothesis);
        assert_eq!(envelope.candidate_specific_typical_centers, 1);
        assert!(envelope.interval_model_ready);
        assert!(!envelope.continuous_nominal_model_for_every_candidate_ready);
        assert!(!envelope.exact_target_identity_required_for_interval_model);

        let intervals = diode_forward_reference_intervals();
        assert_eq!(intervals.len(), 4);
        assert!(
            intervals[..2]
                .iter()
                .all(|interval| interval.candidate_specific_typical_center_v.is_none())
        );
        assert!(
            intervals[2..]
                .iter()
                .all(|interval| interval.candidate_specific_typical_center_v.is_some())
        );
        assert!(intervals.iter().all(|interval| {
            interval.shared_voltage_minimum_v == 0.0
                && interval.shared_voltage_maximum_v == 1.2
                && interval.interval_is_shared_endpoint_hypothesis
        }));
        assert!(
            evaluate_diode_forward_voltage_interval(
                "1SS133",
                envelope.conservative_network_forward_peak_current_a * 1.01
            )
            .is_none()
        );
    }

    #[test]
    fn related_revision_one_kilohm_marking_fails_the_chorus_sanity_check() {
        let hz = ideal_oscillator_rate_hz(
            BranchState::Tr1Open,
            R5_OHMS,
            R8_OHMS,
            R4_OHMS,
            R6_OHMS,
            1_000.0,
            C3_FARADS,
        );
        assert!((hz - 37.5).abs() < 1.0e-12);
    }

    #[test]
    fn open_branch_preserves_the_demonstrable_point_three_two_second_scale() {
        let open = rc_scale(BranchState::Tr1Open);
        assert_eq!(open.nominal_resistance_ohms, 3_200_000.0);
        assert!((open.nominal_seconds - 0.32).abs() < 1.0e-12);
    }

    #[test]
    fn reduced_model_predicts_the_open_branch_near_mode_two() {
        let open = ideal_oscillator_estimate(BranchState::Tr1Open);
        assert!((open.nominal_hz - 0.817_968_75).abs() < 1.0e-12);
        assert!(open.minimum_hz < open.nominal_hz);
        assert!(open.maximum_hz > open.nominal_hz);
    }

    #[test]
    fn ideal_tr1_shunt_is_slower_than_the_open_branch() {
        let open = ideal_oscillator_estimate(BranchState::Tr1Open);
        let shunted = ideal_oscillator_estimate(BranchState::Tr1IdealShunt);
        assert!(shunted.nominal_hz < open.nominal_hz);
        assert!(report().reduced_rate_model_solved);
        assert!(!report().absolute_rate_solved);
        assert!(!report().production_promotion_ready);
    }

    #[test]
    fn schematic_truth_table_closes_the_steady_state_control_path() {
        let states = hardware_control_states();
        assert_eq!(states[0].chorus_enable_line_volts, CONTROL_HIGH_VOLTS);
        assert!(states[0].rate_line_is_dont_care);
        assert_eq!(states[1].chorus_rate_line_volts, CONTROL_LOW_VOLTS);
        assert_eq!(states[1].oscillator_branch, "r4_shunt_active_slow");
        assert_eq!(states[2].chorus_rate_line_volts, CONTROL_HIGH_VOLTS);
        assert_eq!(states[2].oscillator_branch, "r4_shunt_open_fast");
        assert!(report().steady_state_control_solved);
    }

    #[test]
    fn current_coordinates_are_compared_without_inventing_fet_resistance() {
        let mode_i = coordinate_comparison("i", CURRENT_MODE_I_RATE_HZ, BranchState::Tr1IdealShunt);
        let mode_ii = coordinate_comparison("ii", CURRENT_MODE_II_RATE_HZ, BranchState::Tr1Open);
        assert!(!mode_i.inside_assumed_component_corners);
        assert!(mode_i.distance_above_maximum_percent < 3.0);
        assert!(mode_ii.inside_assumed_component_corners);
        let serialized = serde_json::to_string(&report()).unwrap();
        assert!(!serialized.contains("implied_tr1_channel"));
    }

    #[test]
    fn firmware_decode_has_three_audio_states_not_four() {
        let states = firmware_chorus_states();
        assert_eq!(states[0].assigner_switch1_bits, 0x20);
        assert_eq!(states[1].assigner_switch1_bits, 0x40);
        assert_eq!(states[2].assigner_switch1_bits, 0x80);
        assert_eq!(states[0].transmitted_switch1_value, 0x5f);
        assert_eq!(states[1].transmitted_switch1_value, 0x3f);
        assert_eq!(states[2].transmitted_switch1_value, 0x7f);
        assert_eq!(states[0].module_latch_low_bits, 0x03);
        assert_eq!(states[1].module_latch_low_bits, 0x00);
        assert_eq!(states[2].module_latch_low_bits, 0x02);
        assert_eq!(
            states[3].module_latch_low_bits,
            states[1].module_latch_low_bits
        );
        assert!(!states[3].distinct_audio_state);
        assert!(!states[3].normal_one_hot_state);
    }

    #[test]
    fn provisional_coordinates_are_not_misrepresented_as_complete_results() {
        let report = report();
        assert!(
            report
                .provisional_coordinates
                .iter()
                .all(|coordinate| !coordinate.evaluated_by_complete_model)
        );
    }

    #[test]
    fn mn3101_pinout_and_divide_by_two_are_explicit() {
        let pins = mn3101_pins();
        assert_eq!((pins[0].pin, pins[0].name), (1, "GND"));
        assert_eq!((pins[1].pin, pins[1].name), (2, "CP1"));
        assert_eq!((pins[3].pin, pins[3].name), (4, "CP2"));
        assert_eq!((pins[7].pin, pins[7].name), (8, "VGG"));
        for example in mn3101_oscillator_examples() {
            assert_eq!(example.oscillator_min_hz / example.cp_min_hz, 2.0);
            assert_eq!(example.oscillator_max_hz / example.cp_max_hz, 2.0);
        }
    }

    #[test]
    fn target_clock_channels_are_mirrored_and_antiphase() {
        let channels = clock_channels();
        assert_eq!(channels[0].triangle_source, "TP3");
        assert_eq!(channels[1].triangle_source, "TP4");
        assert_eq!(channels[0].oscillator_capacitor, "C53 150 pF");
        assert_eq!(channels[1].oscillator_capacitor, "C57 150 pF");
        assert_eq!(channels[0].oscillator_resistors[0], "R134 22 kOhm");
        assert_eq!(channels[1].oscillator_resistors[0], "R140 22 kOhm");
        assert!(report().clock_networks_are_nominally_mirrored);
    }

    #[test]
    fn current_delay_envelopes_round_trip_through_the_physical_clock_chain() {
        for envelope in report().current_clock_envelopes {
            assert!(
                (bbd_delay_ms_for_clock_hz(envelope.cp_max_hz) - envelope.delay_min_ms).abs()
                    < 1.0e-12
            );
            assert!(
                (bbd_delay_ms_for_clock_hz(envelope.cp_min_hz) - envelope.delay_max_ms).abs()
                    < 1.0e-12
            );
            assert_eq!(envelope.mn3101_oscillator_min_hz, envelope.cp_min_hz * 2.0);
            assert_eq!(envelope.mn3101_oscillator_max_hz, envelope.cp_max_hz * 2.0);
            assert!(envelope.inside_mn3009_clock_range);
        }
    }

    #[test]
    fn exact_frequency_chain_does_not_promote_the_unknown_analog_transfer() {
        let report = report();
        assert!(report.clock_frequency_chain_solved);
        assert!(!report.mn3101_control_transfer_solved);
        assert!(!report.production_promotion_ready);
    }

    #[test]
    fn endpoint_fits_reproduce_the_exact_manufacturer_table() {
        for fit in mn3101_endpoint_fits() {
            assert!(
                (cp_hz_from_endpoint_fit(fit, fit.r2_min_ohms) - fit.cp_at_r2_min_hz).abs()
                    < 1.0e-9
            );
            assert!(
                (cp_hz_from_endpoint_fit(fit, fit.r2_max_ohms) - fit.cp_at_r2_max_hz).abs()
                    < 1.0e-9
            );
            assert!(fit.log_log_exponent < 0.0);
        }
    }

    #[test]
    fn interpolated_150pf_clock_stays_between_the_two_published_capacitors() {
        let fits = mn3101_endpoint_fits();
        for r2_ohms in [5_000.0, 10_000.0, 30_000.0, 100_000.0, 1_000_000.0] {
            let cp_100 = cp_hz_from_endpoint_fit(fits[0], r2_ohms);
            let cp_150 = cp_hz_for_150pf_endpoint_interpolation(r2_ohms);
            let cp_200 = cp_hz_from_endpoint_fit(fits[1], r2_ohms);
            assert!(cp_200 < cp_150);
            assert!(cp_150 < cp_100);
        }
    }

    #[test]
    fn current_clocks_map_to_bounded_equivalent_r2_values() {
        for envelope in report().c150_equivalent_r2_envelopes {
            assert!(envelope.r2_at_cp_max_ohms < envelope.r2_at_cp_min_ohms);
            assert!(envelope.r2_at_cp_max_ohms > 5_000.0);
            assert!(envelope.r2_at_cp_min_ohms < 50_000.0);
            assert!(envelope.inside_manufacturer_sweep);
            assert!(
                (cp_hz_for_150pf_endpoint_interpolation(envelope.r2_at_cp_max_ohms)
                    - envelope.cp_max_hz)
                    .abs()
                    < 1.0e-8
            );
            assert!(
                (cp_hz_for_150pf_endpoint_interpolation(envelope.r2_at_cp_min_ohms)
                    - envelope.cp_min_hz)
                    .abs()
                    < 1.0e-8
            );
        }
    }

    #[test]
    fn equivalent_conductance_envelopes_round_trip_to_the_resistance_bounds() {
        let report = report();
        for (resistance, conductance) in report
            .c150_equivalent_r2_envelopes
            .into_iter()
            .zip(report.c150_equivalent_conductance_envelopes)
        {
            assert_eq!(resistance.mode, conductance.mode);
            assert!(conductance.minimum_microsiemens < conductance.maximum_microsiemens);
            assert!(conductance.span_ratio > 1.0);
            assert!(
                (1_000_000.0 / conductance.minimum_microsiemens - resistance.r2_at_cp_min_ohms)
                    .abs()
                    < 1.0e-9
            );
            assert!(
                (1_000_000.0 / conductance.maximum_microsiemens - resistance.r2_at_cp_max_ohms)
                    .abs()
                    < 1.0e-9
            );
        }
    }

    #[test]
    fn endpoint_interpolation_is_not_misrepresented_as_device_tolerance_or_active_transfer() {
        let report = report();
        assert!(!report.c150_interpolation_has_assigned_device_tolerance);
        assert!(!report.active_network_voltage_to_r2_solved);
        assert!(!report.mn3101_control_transfer_solved);
        assert!(!report.production_promotion_ready);
    }

    #[test]
    fn all_readable_clock_control_values_have_a_nominal_mirror() {
        let pairs = mirrored_clock_components();
        assert_eq!(pairs.len(), 17);
        assert_eq!(
            (pairs[0].channel_a_designator, pairs[0].channel_b_designator),
            ("R128", "R148")
        );
        assert_eq!(pairs[0].nominal, 1_000.0);
        assert_eq!(
            (
                pairs[13].channel_a_designator,
                pairs[13].channel_b_designator
            ),
            ("C53", "C57")
        );
        assert_eq!(pairs[13].nominal, 150.0);
        assert_eq!(
            (
                pairs[15].channel_a_designator,
                pairs[15].channel_b_designator
            ),
            ("R99", "R108")
        );
        assert_eq!(
            (
                pairs[16].channel_a_designator,
                pairs[16].channel_b_designator
            ),
            ("C46", "C49")
        );
        assert!(pairs.iter().all(|pair| pair.nominal > 0.0));
    }

    #[test]
    fn active_network_netlist_has_no_unresolved_schematic_connections() {
        let report = report();
        let audit = report.active_network_netlist_audit;
        assert_eq!(report.mirrored_active_network_elements.len(), 18);
        assert_eq!(report.mirrored_bipolar_connections.len(), 5);
        assert_eq!(report.mirrored_mn3101_connections.len(), 8);
        assert!(audit.cross_revision_connectivity_agreement);
        assert!(audit.mirrored_by_construction);
        assert_eq!(audit.unresolved_schematic_connections, 0);
        assert!(audit.dc_solver_topology_ready);
        assert!(!audit.nonlinear_device_models_ready);
        assert!(report.active_network_dc_solver_topology_ready);
        assert!(!report.active_network_operating_point_solved);
    }

    #[test]
    fn active_network_critical_nodes_have_the_expected_degree() {
        let two_terminal = mirrored_active_network_elements();
        let bipolars = mirrored_bipolar_connections();
        let controller = mirrored_mn3101_connections();
        let degree = |node| {
            two_terminal
                .iter()
                .map(|element| {
                    usize::from(element.terminal_1 == node)
                        + usize::from(element.terminal_2 == node)
                })
                .sum::<usize>()
                + bipolars
                    .iter()
                    .map(|device| {
                        usize::from(device.collector == node)
                            + usize::from(device.base == node)
                            + usize::from(device.emitter == node)
                    })
                    .sum::<usize>()
                + controller
                    .iter()
                    .map(|pin| usize::from(pin.node == node))
                    .sum::<usize>()
        };

        assert_eq!(degree(ActiveNetworkNode::Control), 6);
        assert_eq!(degree(ActiveNetworkNode::Timing), 3);
        assert_eq!(degree(ActiveNetworkNode::InputEmitterNode), 4);
        assert_eq!(degree(ActiveNetworkNode::FeedbackBase), 4);
        assert_eq!(degree(ActiveNetworkNode::OscillatorFeedback), 2);
        assert_eq!(degree(ActiveNetworkNode::PositiveSupply), 4);
        assert_eq!(degree(ActiveNetworkNode::LocalNegativeRail), 8);
    }

    #[test]
    fn diode_orientation_and_mn3101_oscillator_pins_are_explicit() {
        let elements = mirrored_active_network_elements();
        let diode = elements
            .iter()
            .find(|element| element.kind == NetlistElementKind::Diode)
            .expect("one mirrored control diode is required");
        assert_eq!(
            (diode.channel_a_designator, diode.channel_b_designator),
            ("D9", "D10")
        );
        assert_eq!(diode.terminal_1, ActiveNetworkNode::DiodeBase);
        assert_eq!(diode.terminal_2, ActiveNetworkNode::Control);
        assert!(diode.terminal_1_meaning.contains("cathode"));
        assert!(diode.terminal_2_meaning.contains("anode"));

        let pins = mirrored_mn3101_connections();
        assert_eq!(pins[4].node, ActiveNetworkNode::Control);
        assert_eq!(pins[5].node, ActiveNetworkNode::OscillatorFeedback);
        assert_eq!(pins[6].node, ActiveNetworkNode::FeedbackBase);
        assert_eq!(pins[7].node, ActiveNetworkNode::Vgg);
    }

    #[test]
    fn every_netlist_element_has_an_exact_channel_mirror() {
        let elements = mirrored_active_network_elements();
        assert!(elements.iter().all(|element| {
            !element.channel_a_designator.is_empty()
                && !element.channel_b_designator.is_empty()
                && element.channel_a_designator != element.channel_b_designator
        }));
        assert_eq!(
            elements
                .iter()
                .filter(|element| element.kind == NetlistElementKind::Resistor)
                .count(),
            14
        );
        assert_eq!(
            elements
                .iter()
                .filter(|element| element.kind == NetlistElementKind::Capacitor)
                .count(),
            3
        );
        assert_eq!(
            elements
                .iter()
                .filter(|element| element.kind == NetlistElementKind::Diode)
                .count(),
            1
        );
    }

    #[test]
    fn active_device_polarities_and_channel_mirrors_are_explicit() {
        let devices = mirrored_active_devices();
        assert_eq!(devices.len(), 5);
        assert_eq!(devices[0].polarity, BipolarPolarity::Pnp);
        assert_eq!(devices[1].polarity, BipolarPolarity::Pnp);
        assert!(
            devices[2..]
                .iter()
                .all(|device| device.polarity == BipolarPolarity::Npn)
        );
        assert_eq!(
            devices
                .iter()
                .map(|device| (device.channel_a_designator, device.channel_b_designator))
                .collect::<Vec<_>>(),
            vec![
                ("Tr19", "Tr24"),
                ("Tr20", "Tr25"),
                ("Tr21", "Tr26"),
                ("Tr22", "Tr27"),
                ("Tr23", "Tr28"),
            ]
        );
        assert!(
            devices
                .iter()
                .all(|device| device.exact_part_assignment.contains("unresolved"))
        );
    }

    #[test]
    fn board_inventory_does_not_claim_an_exact_active_device_assignment() {
        let report = report();
        assert!(report.active_network_schematic_topology_captured);
        assert!(!report.exact_active_device_assignments_solved);
        assert!(!report.active_network_operating_point_solved);
        assert!(!report.active_network_voltage_to_r2_solved);
        assert_eq!(report.board_wide_bipolar_inventory.pnp_candidates.len(), 4);
        assert_eq!(report.board_wide_bipolar_inventory.npn_candidates.len(), 6);
        assert!(
            report
                .board_wide_bipolar_inventory
                .scope
                .contains("does not map")
        );
        assert_eq!(report.mirrored_diode_candidate.candidate, "1SS-133");
        assert_eq!(report.mirrored_diode_candidate.confidence, "medium");
    }

    #[test]
    fn comparative_family_rule_only_narrows_the_bipolar_candidate_sets() {
        let report = report();
        assert_eq!(
            report.comparative_candidate_shortlist.pnp_candidates,
            ["2SA1015-GR/Y", "2SA1115-F"]
        );
        assert_eq!(
            report.comparative_candidate_shortlist.npn_candidates,
            ["2SC1815-GR/Y", "2SC2603-F"]
        );
        assert!(
            report
                .comparative_candidate_shortlist
                .diode_intersection
                .is_empty()
        );
        assert!(
            !report
                .comparative_candidate_shortlist
                .direct_target_evidence
        );
        assert!(!report.module_board_device_legend_applies_to_clock_network);
        assert!(!report.exact_active_device_assignments_solved);
    }

    #[test]
    fn assignment_space_counts_only_one_channel_because_the_other_is_mirrored() {
        let space = active_device_assignment_space();
        assert_eq!(space.pnp_positions_per_channel, 2);
        assert_eq!(space.npn_positions_per_channel, 3);
        assert_eq!(space.board_inventory_assignments, 3_456);
        assert_eq!(space.comparative_shortlist_assignments, 32);
        assert_eq!(space.uniform_family_per_polarity_hypotheses, 4);
        assert!(space.channel_b_is_constrained_to_mirror_channel_a);
    }

    #[test]
    fn all_thirty_two_device_hypotheses_are_enumerated_once() {
        let hypotheses = active_device_hypotheses();
        let unique = hypotheses
            .iter()
            .map(|hypothesis| (hypothesis.channel_a_pnp, hypothesis.channel_a_npn))
            .collect::<std::collections::HashSet<_>>();
        assert_eq!(hypotheses.len(), 32);
        assert_eq!(unique.len(), 32);
        assert!(
            hypotheses
                .iter()
                .enumerate()
                .all(|(index, hypothesis)| hypothesis.index == index)
        );
        assert!(
            hypotheses
                .iter()
                .all(|hypothesis| hypothesis.channel_b_uses_mirrored_assignments)
        );
        assert!(
            hypotheses
                .iter()
                .all(|hypothesis| !hypothesis.direct_target_evidence)
        );
    }

    #[test]
    fn every_shortlisted_family_has_exactly_one_primary_datasheet_record() {
        let report = report();
        assert_eq!(report.candidate_bipolar_model_evidence.len(), 4);
        assert!(
            report
                .bipolar_model_parameter_boundary
                .all_shortlist_candidates_have_a_record
        );
        for candidate in report
            .comparative_candidate_shortlist
            .pnp_candidates
            .into_iter()
            .chain(report.comparative_candidate_shortlist.npn_candidates)
        {
            assert_eq!(
                report
                    .candidate_bipolar_model_evidence
                    .iter()
                    .filter(|record| record.candidate_key == candidate)
                    .count(),
                1
            );
        }
    }

    #[test]
    fn published_rank_envelopes_preserve_toshiba_and_mitsubishi_differences() {
        let records = candidate_bipolar_model_evidence();
        let toshiba_pnp = records
            .iter()
            .find(|record| record.candidate_key == "2SA1015-GR/Y")
            .unwrap();
        let mitsubishi_pnp = records
            .iter()
            .find(|record| record.candidate_key == "2SA1115-F")
            .unwrap();
        let toshiba_npn = records
            .iter()
            .find(|record| record.candidate_key == "2SC1815-GR/Y")
            .unwrap();
        let mitsubishi_npn = records
            .iter()
            .find(|record| record.candidate_key == "2SC2603-F")
            .unwrap();

        assert_eq!(
            (toshiba_pnp.hfe_minimum, toshiba_pnp.hfe_maximum),
            (120.0, 400.0)
        );
        assert_eq!(
            (mitsubishi_pnp.hfe_minimum, mitsubishi_pnp.hfe_maximum),
            (250.0, 500.0)
        );
        assert_eq!(toshiba_pnp.collector_output_capacitance_typical_pf, 4.0);
        assert_eq!(mitsubishi_pnp.collector_output_capacitance_typical_pf, 4.0);
        assert_eq!(toshiba_npn.collector_output_capacitance_typical_pf, 2.0);
        assert_eq!(mitsubishi_npn.collector_output_capacitance_typical_pf, 2.5);
        assert_eq!(
            mitsubishi_pnp.common_emitter_hie_typical_ohms,
            Some(7_000.0)
        );
        assert_eq!(mitsubishi_pnp.common_emitter_hfe_typical, Some(250.0));
        assert_eq!(
            mitsubishi_pnp.common_emitter_hoe_typical_microsiemens,
            Some(18.0)
        );
        assert_eq!(
            mitsubishi_npn.common_emitter_hie_typical_ohms,
            Some(8_500.0)
        );
        assert_eq!(mitsubishi_npn.common_emitter_hfe_typical, Some(300.0));
        assert_eq!(
            mitsubishi_npn.common_emitter_hoe_typical_microsiemens,
            Some(5.5)
        );
        assert!(toshiba_pnp.common_emitter_h_parameter_condition.is_none());
        assert!(toshiba_npn.common_emitter_h_parameter_condition.is_none());
        assert_eq!(toshiba_npn.transition_frequency_limit_kind, "minimum");
        assert_eq!(mitsubishi_npn.transition_frequency_limit_kind, "typical");
    }

    #[test]
    fn datasheet_limits_are_not_promoted_to_complete_spice_models() {
        let report = report();
        assert!(
            report
                .bipolar_model_parameter_boundary
                .usable_for_absolute_limit_checks
        );
        assert!(
            report
                .bipolar_model_parameter_boundary
                .usable_for_rank_and_capacitance_corners
        );
        assert!(
            !report
                .bipolar_model_parameter_boundary
                .complete_large_signal_model_available
        );
        assert_eq!(
            report
                .bipolar_model_parameter_boundary
                .missing_model_parameter_groups
                .len(),
            10
        );
        assert!(
            report
                .candidate_bipolar_model_evidence
                .iter()
                .all(|record| !record.supports_complete_large_signal_model)
        );
        assert!(!report.active_network_operating_point_solved);
        assert!(!report.production_promotion_ready);
    }

    #[test]
    fn output_characteristic_families_cover_the_saturation_knee_without_claiming_a_surface() {
        let report = report();
        let validation = report.bjt_output_characteristic_evidence_validation;
        assert_eq!(report.bjt_output_characteristic_plot_evidence.len(), 4);
        assert_eq!(validation.candidate_count, 4);
        assert_eq!(validation.pnp_candidate_count, 2);
        assert_eq!(validation.npn_candidate_count, 2);
        assert_eq!(validation.total_labeled_base_current_curves, 33);
        assert!(validation.every_candidate_has_zero_base_current_curve);
        assert!(validation.every_positive_base_current_family_strictly_increasing);
        assert_eq!(
            validation.minimum_shared_collector_emitter_voltage_domain_v,
            5.0
        );
        assert_eq!(validation.minimum_shared_collector_current_domain_a, 0.05);
        assert!(validation.every_family_covers_conservative_network_current);
        assert!(validation.every_family_covers_saturation_knee_voltage);
        assert!(validation.direct_base_drive_evidence_available_for_every_candidate);
        assert!(validation.minimum_voltage_reading_uncertainty_mv > 38.0);
        assert!(validation.minimum_voltage_reading_uncertainty_mv < 39.0);
        assert!(validation.maximum_voltage_reading_uncertainty_mv > 74.0);
        assert!(validation.maximum_voltage_reading_uncertainty_mv < 75.0);
        assert!(validation.minimum_collector_current_reading_uncertainty_ma > 0.38);
        assert!(validation.minimum_collector_current_reading_uncertainty_ma < 0.40);
        assert!(validation.maximum_collector_current_reading_uncertainty_ma > 3.0);
        assert!(validation.interpolation_between_labeled_base_current_curves_is_hypothesis);
        assert!(!validation.digitized_output_surface_ready);
        assert!(!validation.suitable_as_confirmed_production_device_model);
        assert!(
            !report
                .active_network_solver_readiness
                .bjt_output_characteristic_surfaces_digitized
        );
        let pnp_mitsubishi = report
            .bjt_output_characteristic_plot_evidence
            .iter()
            .find(|record| record.candidate_key == "2SA1115-F")
            .unwrap();
        let npn_mitsubishi = report
            .bjt_output_characteristic_plot_evidence
            .iter()
            .find(|record| record.candidate_key == "2SC2603-F")
            .unwrap();
        assert_eq!(
            (pnp_mitsubishi.x_left_px, pnp_mitsubishi.x_right_px),
            (1322.0, 1839.0)
        );
        assert_eq!(
            (npn_mitsubishi.y_top_px, npn_mitsubishi.y_bottom_px),
            (522.0, 1038.0)
        );
        let resolutions = &report.bjt_output_characteristic_clamped_point_resolutions;
        assert_eq!(resolutions.len(), 2);
        assert!(resolutions.iter().all(|point| {
            point.preliminary_base_current_inside_zero_to_first_curve_bracket
                && !point.vce_meets_solver_resolution_margin
                && !point.directly_resolvable_from_archived_plot
        }));
        let tr22 = resolutions
            .iter()
            .find(|point| point.designator == "Tr22")
            .unwrap();
        let tr23 = resolutions
            .iter()
            .find(|point| point.designator == "Tr23")
            .unwrap();
        assert!(!tr22.vce_exceeds_plot_reading_uncertainty);
        assert!(tr22.vce_to_plot_reading_uncertainty_ratio < 0.05);
        assert!(tr23.vce_exceeds_plot_reading_uncertainty);
        assert!(tr23.vce_to_plot_reading_uncertainty_ratio > 1.0);
        assert!(tr23.vce_to_plot_reading_uncertainty_ratio < 1.1);
        let resolution_validation = report.bjt_output_characteristic_clamped_resolution_validation;
        assert_eq!(resolution_validation.evaluated_clamped_points, 2);
        assert_eq!(
            resolution_validation.points_inside_zero_to_first_base_current_curve_bracket,
            2
        );
        assert_eq!(
            resolution_validation.points_with_vce_above_plot_reading_uncertainty,
            1
        );
        assert_eq!(
            resolution_validation.points_meeting_solver_resolution_margin,
            0
        );
        assert_eq!(
            resolution_validation.minimum_solver_resolution_margin_radii,
            2.0
        );
        assert!(!resolution_validation.every_clamped_point_directly_resolvable);
        assert!(resolution_validation.higher_resolution_source_or_bounded_low_vce_model_required);
        assert!(!resolution_validation.suitable_for_immediate_solver_substitution);

        assert_eq!(report.bjt_low_vce_base_drive_hypotheses.len(), 6);
        let reference = &report.bjt_low_vce_base_drive_reference_screen_points;
        assert_eq!(reference.len(), 6);
        assert_eq!(
            reference
                .iter()
                .filter(|point| point.inside_supply_polarity_and_power_gates)
                .count(),
            4
        );
        assert!(
            reference
                .iter()
                .filter(|point| [2, 5].contains(&point.hypothesis_index))
                .all(|point| !point.inside_supply_polarity_and_power_gates)
        );
        let low_vce = &report.bjt_low_vce_base_drive_sweep_validation;
        assert_eq!(low_vce.defined_hypothesis_count, 6);
        assert_eq!(low_vce.reference_screen_survivor_count, 4);
        assert_eq!(low_vce.reference_screen_rejected_count, 2);
        assert_eq!(low_vce.expected_matrix_points, 128);
        assert_eq!(low_vce.evaluated_matrix_points, 128);
        assert_eq!(low_vce.converged_matrix_points, 48);
        assert_eq!(low_vce.points_inside_supply_polarity_and_power_gates, 32);
        assert_eq!(low_vce.points_without_transfer_clamping, 0);
        assert_eq!(low_vce.hypotheses_converged_for_all_assignments, 0);
        assert_eq!(low_vce.hypotheses_eliminating_clamps_for_all_assignments, 0);
        assert!(!low_vce.complete_dc_ensemble_accepted);
        assert!(!low_vce.required_audio_trajectory_metrics_evaluated);
        assert!(!low_vce.suitable_for_production_promotion);

        let reciprocal = &report.bjt_reciprocal_transport_sweep_validation;
        assert_eq!(report.bjt_reciprocal_transport_hypotheses.len(), 3);
        assert!(
            report
                .bjt_reciprocal_transport_hypotheses
                .iter()
                .all(
                    |hypothesis| hypothesis.forward_and_reverse_transport_share_reciprocal_scale
                        && hypothesis.reverse_beta_is_unpublished_sensitivity_axis
                        && !hypothesis.suitable_as_identified_device_law
                )
        );
        assert_eq!(reciprocal.expected_matrix_points, 96);
        assert_eq!(reciprocal.evaluated_matrix_points, 96);
        assert_eq!(reciprocal.converged_matrix_points, 6);
        assert_eq!(reciprocal.points_without_forward_transfer_clamping, 96);
        assert_eq!(reciprocal.points_without_any_transfer_clamping, 96);
        assert!(!reciprocal.complete_dc_ensemble_accepted);
        let reciprocal_homotopy = report.bjt_reciprocal_transport_homotopy_validation;
        assert_eq!(reciprocal_homotopy.hypothesis_count, 3);
        assert_eq!(reciprocal_homotopy.stages_per_hypothesis, 16);
        assert_eq!(reciprocal_homotopy.exact_targets_reached, 1);
        assert_eq!(reciprocal_homotopy.exact_targets_inside_all_dc_gates, 1);
        assert_eq!(
            reciprocal_homotopy.exact_targets_without_any_transfer_clamping,
            1
        );
        assert!(reciprocal_homotopy.homotopy_is_numerical_path_not_device_evidence);
        assert!(reciprocal_homotopy.reciprocal_transport_family_viable_at_reference);
        assert!(!reciprocal_homotopy.suitable_for_production_promotion);
        let beta_one = &report.bjt_reciprocal_transport_homotopy_summaries[0];
        assert_eq!(beta_one.reverse_beta, 1.0);
        assert!(beta_one.exact_target_inside_all_dc_gates);
        assert_eq!(beta_one.final_forward_transfer_clamp_count, 0);
        assert_eq!(beta_one.final_reverse_transfer_clamp_count, 0);
        assert!(
            report.bjt_reciprocal_transport_homotopy_summaries[1..]
                .iter()
                .all(|summary| !summary.exact_target_reached)
        );
        let assignment = &report.bjt_reciprocal_transport_assignment_validation;
        assert_eq!(report.bjt_reciprocal_transport_assignment_points.len(), 32);
        assert_eq!(assignment.attempted_assignments, 32);
        assert_eq!(assignment.converged_assignments, 32);
        assert_eq!(assignment.assignments_using_adaptive_homotopy, 1);
        assert_eq!(assignment.assignments_inside_all_dc_gates, 32);
        assert_eq!(assignment.assignments_without_any_transfer_clamping, 32);
        assert!(assignment.complete_32_assignment_ensemble);
        assert_eq!(assignment.metric_comparisons.len(), 5);
        assert!(assignment.metric_comparisons[..3].iter().all(|metric| {
            metric
                .comparison
                .is_some_and(|comparison| comparison.accepted)
        }));
        assert!(assignment.metric_comparisons[3..].iter().all(|metric| {
            metric
                .comparison
                .is_some_and(|comparison| !comparison.accepted)
        }));
        assert!(!assignment.dc_metrics_accepted_as_equivalent);
        assert!(!assignment.required_audio_trajectory_metrics_evaluated);
        assert!(!assignment.accepted_as_behaviorally_equivalent);
        assert!(!assignment.suitable_for_production_promotion);
        let attribution = &report.bjt_reciprocal_transport_dispersion_attribution;
        assert_eq!(attribution.assignment_count, 32);
        assert!(attribution.balanced_complete_two_level_factorial);
        assert_eq!(attribution.factor_effects.len(), 5);
        assert!(
            attribution
                .factor_effects
                .iter()
                .all(|effect| effect.samples_per_candidate == 16)
        );
        assert!(attribution.collector_current_main_effect_spread_share_percent > 99.99);
        assert!(attribution.collector_current_interaction_spread_share_percent < 0.01);
        assert!(attribution.maximum_power_main_effect_spread_share_percent > 99.99);
        assert!(attribution.maximum_power_interaction_spread_share_percent < 0.01);
        assert_eq!(attribution.dominant_collector_current_designator, "Tr19");
        assert_eq!(attribution.dominant_maximum_power_designator, "Tr19");
        assert_eq!(attribution.maximum_power_device_designator, "Tr19");
        assert!(attribution.maximum_power_device_is_constant_across_assignments);
        let tr19_effect = attribution
            .factor_effects
            .iter()
            .find(|effect| effect.designator == "Tr19")
            .unwrap();
        assert!(tr19_effect.collector_current_centered_spread_share_percent > 79.0);
        assert!(tr19_effect.maximum_power_centered_spread_share_percent > 99.99);
        assert!(attribution.attribution_is_numerical_sensitivity_not_device_identity);
        assert!(!attribution.suitable_for_production_parameter_narrowing);
        let evidence_envelopes = &report.bjt_reciprocal_transport_device_evidence_envelopes;
        assert_eq!(evidence_envelopes.len(), 10);
        assert!(
            evidence_envelopes
                .iter()
                .all(|envelope| envelope.sample_count == 16)
        );
        let tr19_envelopes = evidence_envelopes
            .iter()
            .filter(|envelope| envelope.designator == "Tr19")
            .collect::<Vec<_>>();
        assert_eq!(tr19_envelopes.len(), 2);
        assert!(tr19_envelopes.iter().all(|envelope| {
            envelope.minimum_vce_v > 27.0
                && envelope.maximum_vce_v < 29.0
                && !envelope.every_vce_inside_output_plot_domain
                && envelope.every_collector_current_inside_output_plot_domain
                && envelope.every_base_current_inside_labeled_curve_domain
                && !envelope.every_coordinate_resolvable_at_two_reading_radii
                && envelope.maximum_power_fraction_of_25c_limit_percent < 3.0
                && envelope.power_derating_critical_ambient_c > 122.0
                && !envelope.evidence_directly_bounds_operating_region
        }));
        let tr19_audit = report.bjt_reciprocal_transport_tr19_evidence_audit;
        assert_eq!(tr19_audit.evaluated_envelopes, 10);
        assert_eq!(tr19_audit.tr19_candidate_envelopes, 2);
        assert_eq!(tr19_audit.tr19_samples, 32);
        assert_eq!(
            tr19_audit.tr19_candidates_inside_output_plot_voltage_domain,
            0
        );
        assert_eq!(
            tr19_audit.tr19_candidates_inside_output_plot_current_domain,
            2
        );
        assert_eq!(
            tr19_audit.tr19_candidates_inside_labeled_base_current_domain,
            2
        );
        assert_eq!(
            tr19_audit.tr19_candidates_resolvable_at_two_reading_radii,
            0
        );
        assert_eq!(tr19_audit.tr19_candidates_with_direct_thermal_transfer, 1);
        assert_eq!(
            tr19_audit.tr19_candidates_with_comparative_thermal_interval,
            1
        );
        assert_eq!(tr19_audit.tr19_candidates_inside_25c_power_limit, 2);
        assert!(!tr19_audit.tr19_operating_region_directly_bounded_by_current_evidence);
        assert!(
            !tr19_audit.existing_documents_sufficient_to_narrow_reciprocal_current_power_spread
        );
        assert!(tr19_audit.global_output_conductance_evidence_required);
        assert!(tr19_audit.higher_voltage_output_characteristic_evidence_required);
        assert!(!tr19_audit.suitable_for_production_parameter_narrowing);
        let conductance = report.bjt_tr19_high_vce_conductance_requirement;
        assert_eq!(conductance.reference_voltage_v, 6.0);
        assert_eq!(conductance.evaluated_hypotheses, 4_001);
        assert_eq!(conductance.conductance_step_us, 0.001);
        assert!(conductance.current_acceptance_interval_minimum_us.is_none());
        assert!(conductance.current_acceptance_interval_maximum_us.is_none());
        assert_eq!(
            conductance.power_acceptance_interval_minimum_us,
            Some(1.441)
        );
        assert_eq!(
            conductance.power_acceptance_interval_maximum_us,
            Some(1.893)
        );
        assert!(conductance.joint_acceptance_interval_minimum_us.is_none());
        assert!(conductance.joint_acceptance_interval_maximum_us.is_none());
        assert_eq!(conductance.best_current_conductance_us, 2.128);
        assert!(
            conductance
                .best_current_comparison
                .mean_absolute_relative_deviation_percent
                > 2.2
        );
        assert!(
            conductance
                .best_current_comparison
                .maximum_absolute_relative_deviation_percent
                > 4.4
        );
        assert!(!conductance.best_current_comparison.accepted);
        assert_eq!(conductance.published_mitsubishi_local_hoe_us, 18.0);
        assert!(!conductance.tr19_differential_conductance_alone_can_pass_current_gate);
        assert!(conductance.tr19_differential_conductance_alone_can_pass_power_gate);
        assert!(!conductance.tr19_differential_conductance_alone_can_pass_both_gates);
        assert_eq!(
            conductance.remaining_transport_evidence_priorities,
            ["Tr23", "Tr22"]
        );
        assert!(conductance.frozen_node_linearized_screen_not_resolved_operating_points);
        assert!(!conductance.suitable_for_production_parameter_narrowing);
        let residual_evidence = report.bjt_reciprocal_transport_tr22_tr23_evidence_audit;
        assert_eq!(residual_evidence.evaluated_candidate_envelopes, 4);
        assert_eq!(residual_evidence.evaluated_device_samples, 64);
        assert_eq!(
            residual_evidence.tr22_candidates_inside_output_plot_voltage_domain,
            2
        );
        assert_eq!(
            residual_evidence.tr22_candidates_inside_output_plot_current_domain,
            2
        );
        assert_eq!(
            residual_evidence.tr22_candidates_inside_labeled_base_current_domain,
            1
        );
        assert_eq!(
            residual_evidence.tr22_candidates_resolvable_at_two_reading_radii,
            2
        );
        assert_eq!(residual_evidence.tr23_candidates_with_negative_vce, 2);
        assert_eq!(
            residual_evidence.tr23_candidates_inside_forward_output_plot_domain,
            0
        );
        assert_eq!(
            residual_evidence.tr23_candidates_with_reverse_transport_at_every_sample,
            2
        );
        assert_eq!(residual_evidence.candidates_with_direct_thermal_transfer, 2);
        assert_eq!(
            residual_evidence.candidates_with_only_comparative_thermal_interval,
            2
        );
        assert_eq!(residual_evidence.digitized_output_surfaces_available, 0);
        assert!(residual_evidence.tr22_output_surface_digitization_required);
        assert!(residual_evidence.tr22_mitsubishi_base_drive_extension_evidence_required);
        assert!(residual_evidence.tr23_reverse_transport_evidence_required);
        assert!(!residual_evidence.existing_documents_sufficient_to_bound_remaining_current_spread);
        assert!(!residual_evidence.suitable_for_production_parameter_narrowing);
        let tr22_tr23_envelopes = evidence_envelopes
            .iter()
            .filter(|envelope| ["Tr22", "Tr23"].contains(&envelope.designator))
            .collect::<Vec<_>>();
        assert!(tr22_tr23_envelopes.iter().all(|envelope| {
            envelope.minimum_reverse_transport_current_a > 0.0
                && envelope.maximum_reverse_transport_current_a
                    >= envelope.minimum_reverse_transport_current_a
                && envelope.maximum_vbc_v > 0.0
        }));
        assert!(
            tr22_tr23_envelopes
                .iter()
                .filter(|envelope| envelope.designator == "Tr23")
                .all(|envelope| envelope.maximum_vce_v < 0.0)
        );
        let reverse_beta = report.bjt_tr22_tr23_reverse_beta_requirement;
        assert_eq!(reverse_beta.tr19_differential_conductance_us, 1.667);
        assert!(reverse_beta.tr19_conductance_inside_power_acceptance_interval);
        assert_eq!(reverse_beta.evaluated_beta_pairs, 90_601);
        assert_eq!(reverse_beta.current_accepted_beta_pairs, 2_694);
        assert_eq!(reverse_beta.joint_current_power_accepted_beta_pairs, 2_694);
        assert_eq!(reverse_beta.tr22_accepted_reverse_beta_minimum, Some(0.91));
        assert_eq!(reverse_beta.tr22_accepted_reverse_beta_maximum, Some(0.995));
        assert_eq!(reverse_beta.tr23_accepted_reverse_beta_minimum, Some(0.615));
        assert_eq!(
            reverse_beta.tr23_accepted_reverse_beta_maximum,
            Some(0.8125)
        );
        assert!((reverse_beta.best_tr22_reverse_beta - 0.95).abs() < 1.0e-12);
        assert!((reverse_beta.best_tr23_reverse_beta - 0.6875).abs() < 1.0e-12);
        assert!(reverse_beta.best_current_comparison.accepted);
        assert!(
            reverse_beta
                .best_current_comparison
                .mean_absolute_relative_deviation_percent
                < 0.22
        );
        assert!(
            reverse_beta
                .best_current_comparison
                .maximum_absolute_relative_deviation_percent
                < 0.40
        );
        assert!(reverse_beta.power_comparison.accepted);
        assert!(reverse_beta.accepted_region_is_not_rectangular_bound);
        assert!(!reverse_beta.direct_reverse_beta_evidence_available);
        assert!(reverse_beta.frozen_node_screen_omits_base_kcl_resolve);
        assert!(!reverse_beta.suitable_for_solver_substitution);
        assert!(!reverse_beta.suitable_for_production_parameter_narrowing);
        let frozen_residual = report.bjt_correlated_reverse_transport_frozen_residual_audit;
        assert_eq!(frozen_residual.evaluated_assignments, 32);
        assert_eq!(frozen_residual.assignments_without_parameter_delta, 4);
        assert_eq!(
            frozen_residual.assignments_exceeding_solver_kcl_acceptance,
            28
        );
        assert_eq!(frozen_residual.solver_kcl_acceptance_a, 1.0e-7);
        assert!(
            (frozen_residual.maximum_parameter_delta_kcl_residual_a - 55.550_667_405e-6).abs()
                < 1.0e-12
        );
        assert!(frozen_residual.maximum_parameter_delta_kcl_residual_multiple > 555.5);
        assert_eq!(frozen_residual.worst_assignment_index, 8);
        assert_eq!(
            frozen_residual.worst_residual_node,
            ActiveNetworkNode::Control
        );
        assert!(
            (frozen_residual.maximum_tr19_collector_current_delta_a - 37.038_415_878e-6).abs()
                < 1.0e-12
        );
        assert!(
            (frozen_residual.maximum_tr22_base_current_delta_a - 8.891_534_371e-6).abs() < 1.0e-12
        );
        assert!(
            (frozen_residual.maximum_tr23_base_current_delta_a - 18.512_251_527e-6).abs() < 1.0e-12
        );
        assert!(!frozen_residual.frozen_node_requirement_is_kcl_compatible);
        assert!(frozen_residual.full_nonlinear_base_and_collector_kcl_resolve_required);
        assert!(!frozen_residual.direct_parameter_evidence_available);
        assert!(!frozen_residual.suitable_for_solver_substitution);
        assert!(!frozen_residual.suitable_for_production_parameter_narrowing);
        assert_eq!(
            report
                .bjt_correlated_reverse_transport_frozen_residual_points
                .len(),
            32
        );
        let resolved = report.bjt_correlated_reverse_transport_resolve_validation;
        assert_eq!(resolved.attempted_assignments, 32);
        assert_eq!(resolved.direct_target_converged_assignments, 32);
        assert_eq!(resolved.assignments_using_adaptive_homotopy, 0);
        assert_eq!(resolved.converged_assignments, 32);
        assert_eq!(resolved.assignments_inside_all_dc_gates, 32);
        assert_eq!(resolved.assignments_without_any_transfer_clamping, 32);
        assert!((resolved.maximum_node_voltage_displacement_v - 0.085_680_569_69).abs() < 1.0e-9);
        assert_eq!(resolved.maximum_displacement_assignment_index, 0);
        assert_eq!(
            resolved.maximum_displacement_node,
            ActiveNetworkNode::PnpBiasEmitter
        );
        assert!(resolved.metric_comparisons[..3].iter().all(|metric| {
            metric
                .comparison
                .is_some_and(|comparison| comparison.accepted)
        }));
        let resolved_current = resolved.metric_comparisons[3].comparison.unwrap();
        assert!(resolved_current.mean_absolute_relative_deviation_percent > 3.79);
        assert!(resolved_current.maximum_absolute_relative_deviation_percent > 7.19);
        assert!(!resolved_current.accepted);
        let resolved_power = resolved.metric_comparisons[4].comparison.unwrap();
        assert!(resolved_power.mean_absolute_relative_deviation_percent > 5.70);
        assert!(resolved_power.maximum_absolute_relative_deviation_percent > 5.70);
        assert!(!resolved_power.accepted);
        assert!(!resolved.resolved_dc_metrics_accepted_as_equivalent);
        assert!(!resolved.target_parameters_have_direct_evidence);
        assert!(resolved.numerical_path_suitable_for_evidence_testing);
        assert!(!resolved.required_audio_trajectory_metrics_evaluated);
        assert!(!resolved.suitable_for_production_parameter_narrowing);
        assert!(!resolved.suitable_for_production_promotion);
        assert_eq!(
            report.bjt_correlated_reverse_transport_resolve_points.len(),
            32
        );
        let resolved_attribution = report.bjt_correlated_reverse_transport_dispersion_attribution;
        assert_eq!(resolved_attribution.assignment_count, 32);
        assert!(resolved_attribution.balanced_complete_two_level_factorial);
        assert!(resolved_attribution.collector_current_main_effect_spread_share_percent > 99.99);
        assert!(resolved_attribution.collector_current_interaction_spread_share_percent < 0.01);
        assert!(resolved_attribution.maximum_power_main_effect_spread_share_percent > 99.999);
        assert!(resolved_attribution.maximum_power_interaction_spread_share_percent < 0.001);
        assert_eq!(
            resolved_attribution.dominant_collector_current_designator,
            "Tr19"
        );
        assert_eq!(
            resolved_attribution.dominant_maximum_power_designator,
            "Tr19"
        );
        let tr19_effect = resolved_attribution
            .factor_effects
            .iter()
            .find(|effect| effect.designator == "Tr19")
            .unwrap();
        let tr22_effect = resolved_attribution
            .factor_effects
            .iter()
            .find(|effect| effect.designator == "Tr22")
            .unwrap();
        let tr23_effect = resolved_attribution
            .factor_effects
            .iter()
            .find(|effect| effect.designator == "Tr23")
            .unwrap();
        assert!(
            (67.8..68.0).contains(&tr19_effect.collector_current_centered_spread_share_percent)
        );
        assert!((4.0..4.1).contains(&tr22_effect.collector_current_centered_spread_share_percent));
        assert!(
            (28.0..28.1).contains(&tr23_effect.collector_current_centered_spread_share_percent)
        );
        assert_eq!(resolved_attribution.maximum_power_device_designator, "Tr19");
        assert!(resolved_attribution.maximum_power_device_is_constant_across_assignments);
        assert!(resolved_attribution.attribution_is_numerical_sensitivity_not_device_identity);
        assert!(!resolved_attribution.suitable_for_production_parameter_narrowing);
        let resolved_envelopes = report.bjt_correlated_reverse_transport_device_evidence_envelopes;
        assert_eq!(resolved_envelopes.len(), 10);
        let resolved_tr19_toshiba = resolved_envelopes
            .iter()
            .find(|envelope| {
                envelope.designator == "Tr19" && envelope.candidate_key == "2SA1015-GR/Y"
            })
            .unwrap();
        assert!((28.13..28.14).contains(&resolved_tr19_toshiba.minimum_vce_v));
        assert!((224.7e-6..224.9e-6).contains(&resolved_tr19_toshiba.maximum_collector_current_a));
        assert!(!resolved_tr19_toshiba.every_vce_inside_output_plot_domain);
        let resolved_tr22_toshiba = resolved_envelopes
            .iter()
            .find(|envelope| {
                envelope.designator == "Tr22" && envelope.candidate_key == "2SC1815-GR/Y"
            })
            .unwrap();
        assert!((0.210..0.212).contains(&resolved_tr22_toshiba.minimum_vce_v));
        assert!(resolved_tr22_toshiba.every_coordinate_resolvable_at_two_reading_radii);
        assert!(resolved_tr22_toshiba.every_base_current_inside_labeled_curve_domain);
        let resolved_tr22_mitsubishi = resolved_envelopes
            .iter()
            .find(|envelope| envelope.designator == "Tr22" && envelope.candidate_key == "2SC2603-F")
            .unwrap();
        assert!(!resolved_tr22_mitsubishi.every_base_current_inside_labeled_curve_domain);
        let resolved_tr23 = resolved_envelopes
            .iter()
            .filter(|envelope| envelope.designator == "Tr23")
            .collect::<Vec<_>>();
        assert_eq!(resolved_tr23.len(), 2);
        assert!(resolved_tr23.iter().all(|envelope| {
            envelope.maximum_vce_v < 0.0
                && envelope.minimum_reverse_transport_current_a > 0.0
                && !envelope.every_vce_inside_output_plot_domain
        }));
        let resolved_tr19_audit = report.bjt_correlated_reverse_transport_tr19_evidence_audit;
        assert!(resolved_tr19_audit.higher_voltage_output_characteristic_evidence_required);
        assert!(
            !resolved_tr19_audit
                .existing_documents_sufficient_to_narrow_reciprocal_current_power_spread
        );
        let resolved_tr22_tr23_audit =
            report.bjt_correlated_reverse_transport_tr22_tr23_evidence_audit;
        assert!(resolved_tr22_tr23_audit.tr22_output_surface_digitization_required);
        assert!(resolved_tr22_tr23_audit.tr22_mitsubishi_base_drive_extension_evidence_required);
        assert!(resolved_tr22_tr23_audit.tr23_reverse_transport_evidence_required);
        assert!(
            !resolved_tr22_tr23_audit
                .existing_documents_sufficient_to_bound_remaining_current_spread
        );
    }

    #[test]
    fn low_vce_base_drive_laws_are_continuous_and_explicitly_non_identifying() {
        let hypotheses = bjt_low_vce_base_drive_hypotheses();
        assert_eq!(hypotheses.len(), 6);
        assert!(hypotheses.iter().all(|hypothesis| {
            hypothesis.first_labeled_curve_is_sensitivity_target_not_bound
                && !hypothesis.suitable_as_identified_device_law
        }));
        let linear_quarter = hypotheses[0];
        let at_zero = evaluate_bjt_low_vce_base_drive("2SC2603-F", 0.0, 5.0e-6, linear_quarter);
        assert!((at_zero.first_labeled_base_current_a - 20.0e-6).abs() < 1.0e-12);
        assert!((at_zero.base_current_a - 8.75e-6).abs() < 1.0e-12);
        let at_guard = evaluate_bjt_low_vce_base_drive(
            "2SC2603-F",
            at_zero.guard_voltage_v,
            5.0e-6,
            linear_quarter,
        );
        assert_eq!(at_guard.proximity_weight, 0.0);
        assert_eq!(at_guard.base_current_adjustment_a, 0.0);
        assert_eq!(at_guard.base_current_a, 5.0e-6);
        let smooth_full = hypotheses[5];
        let full_at_zero = evaluate_bjt_low_vce_base_drive("2SC2603-F", 0.0, 5.0e-6, smooth_full);
        assert_eq!(full_at_zero.base_current_a, 20.0e-6);
    }

    #[test]
    fn all_four_typical_transfer_curves_have_auditable_pixel_calibrations() {
        let report = report();
        assert_eq!(report.bjt_transfer_plot_calibrations.len(), 4);
        assert_eq!(report.bjt_transfer_pixel_anchors.len(), 30);
        assert_eq!(report.bjt_transfer_electrical_anchors.len(), 30);
        for calibration in report.bjt_transfer_plot_calibrations {
            let anchor_count = report
                .bjt_transfer_pixel_anchors
                .iter()
                .filter(|anchor| anchor.candidate_key == calibration.candidate_key)
                .count();
            if calibration.candidate_key == "2SA1115-F" {
                assert_eq!(anchor_count, 9);
            } else {
                assert_eq!(anchor_count, 7);
            }
            assert!(calibration.x_right_px > calibration.x_left_px);
            assert!(calibration.y_bottom_px > calibration.y_top_px);
            assert!(calibration.reading_radius_px > 0.0);
            assert!(calibration.curve_status.contains("typical curve"));
        }
        assert!(
            report
                .bjt_transfer_electrical_anchors
                .iter()
                .all(|anchor| anchor.vbe_v > 0.0
                    && anchor.current_a > 0.0
                    && anchor.vbe_reading_uncertainty_mv > 0.0
                    && anchor.current_reading_uncertainty_percent > 0.0)
        );
    }

    #[test]
    fn pixel_axes_reconstruct_known_transfer_coordinates() {
        let anchors = bjt_transfer_electrical_anchors();
        let toshiba_pnp = anchors
            .iter()
            .find(|anchor| anchor.candidate_key == "2SA1015-GR/Y")
            .unwrap();
        let mitsubishi_npn = anchors
            .iter()
            .find(|anchor| anchor.candidate_key == "2SC2603-F")
            .unwrap();
        assert_eq!(toshiba_pnp.current_kind, TransferCurrentKind::Base);
        assert!((toshiba_pnp.current_a - 1.0e-6).abs() < 0.02e-6);
        assert!((toshiba_pnp.vbe_v - 0.572_35).abs() < 0.000_1);
        assert_eq!(mitsubishi_npn.current_kind, TransferCurrentKind::Collector);
        assert!((mitsubishi_npn.current_a - 1.0e-3).abs() < 0.01e-3);
        assert!((mitsubishi_npn.vbe_v - 0.627_04).abs() < 0.000_1);
    }

    #[test]
    fn typical_transfer_diagnostics_preserve_curvature_and_missing_production_spread() {
        let fits = bjt_typical_transfer_fits();
        assert_eq!(fits.len(), 4);
        for fit in fits {
            if fit.candidate_key == "2SA1115-F" {
                assert_eq!(fit.anchor_count, 9);
            } else {
                assert_eq!(fit.anchor_count, 7);
            }
            assert!(fit.fitted_emission_coefficient > 0.8);
            assert!(fit.fitted_emission_coefficient < 1.5);
            assert!(fit.fitted_scale_current_a.is_finite());
            assert!(fit.fitted_scale_current_a > 0.0);
            assert!(fit.rms_vbe_residual_mv.is_finite());
            assert!(!fit.fit_covers_unit_to_unit_variation);
            assert!(fit.anchors_suitable_for_piecewise_hypothesis_center);
            assert!(!fit.suitable_as_guaranteed_device_bound);
        }
        assert!(!fits[0].single_exponential_covers_plot_reading_error);
        assert!(!fits[0].single_exponential_suitable_for_hypothesis_center);
        assert!(
            fits[1..]
                .iter()
                .all(|fit| fit.single_exponential_covers_plot_reading_error
                    && fit.single_exponential_suitable_for_hypothesis_center)
        );
        assert_eq!(fits[0].current_kind, TransferCurrentKind::Base);
        assert_eq!(fits[1].current_kind, TransferCurrentKind::Collector);
        assert_eq!(fits[2].current_kind, TransferCurrentKind::Base);
        assert_eq!(fits[3].current_kind, TransferCurrentKind::Collector);
    }

    #[test]
    fn piecewise_transfer_preserves_all_anchors_and_refuses_extrapolation() {
        for validation in bjt_piecewise_transfer_validations() {
            if validation.candidate_key == "2SA1115-F" {
                assert_eq!(validation.anchor_count, 9);
            } else {
                assert_eq!(validation.anchor_count, 7);
            }
            assert!(validation.anchors_strictly_monotonic);
            assert!(validation.maximum_anchor_round_trip_error_mv < 1.0e-9);
            assert!(!validation.extrapolation_permitted);
            assert!(
                piecewise_vbe_for_current(
                    validation.candidate_key,
                    validation.minimum_current_a * 0.99
                )
                .is_none()
            );
            assert!(
                piecewise_vbe_for_current(
                    validation.candidate_key,
                    validation.maximum_current_a * 1.01
                )
                .is_none()
            );
            let geometric_midpoint =
                (validation.minimum_current_a * validation.maximum_current_a).sqrt();
            let midpoint_vbe =
                piecewise_vbe_for_current(validation.candidate_key, geometric_midpoint).unwrap();
            assert!(midpoint_vbe > validation.minimum_vbe_v);
            assert!(midpoint_vbe < validation.maximum_vbe_v);
        }
    }

    #[test]
    fn forward_active_envelopes_cover_peak_current_but_not_all_dc_regions() {
        for envelope in bjt_forward_active_envelopes() {
            assert!(envelope.beta_minimum > 0.0);
            assert!(envelope.beta_geometric_center > envelope.beta_minimum);
            assert!(envelope.beta_geometric_center < envelope.beta_maximum);
            assert!(envelope.common_collector_current_minimum_a > 0.0);
            assert!(
                envelope.common_collector_current_maximum_a
                    > envelope.common_collector_current_minimum_a
            );
            assert!(envelope.digitized_domain_covers_network_peak);
            assert!(envelope.beta_rank_bounds_available);
            assert!(envelope.beta_rank_applied_outside_test_point_is_hypothesis);
            assert!(!envelope.unit_vbe_spread_available);
            assert!(!envelope.cutoff_model_ready);
            assert!(!envelope.saturation_model_ready);
            assert!(!envelope.early_effect_model_ready);
            assert!(envelope.preliminary_forward_active_domain_ready);
            assert!(!envelope.full_dc_model_ready);
        }
    }

    #[test]
    fn evidence_corner_grid_is_complete_but_does_not_claim_unit_spread() {
        let corners = bjt_evidence_corners();
        assert_eq!(corners.len(), 36);
        for envelope in bjt_forward_active_envelopes() {
            let candidate_corners = corners
                .iter()
                .filter(|corner| corner.candidate_key == envelope.candidate_key)
                .collect::<Vec<_>>();
            assert_eq!(candidate_corners.len(), 9);
            assert!(
                candidate_corners
                    .iter()
                    .all(|corner| corner.evidence_domain_only
                        && !corner.covers_unit_to_unit_variation)
            );
            assert!(
                candidate_corners
                    .iter()
                    .any(|corner| corner.beta_corner == "minimum"
                        && corner.vbe_reading_corner == "reading_low")
            );
            assert!(
                candidate_corners
                    .iter()
                    .any(|corner| corner.beta_corner == "maximum"
                        && corner.vbe_reading_corner == "reading_high")
            );
        }
    }

    #[test]
    fn forward_active_adapter_preserves_base_vs_collector_curve_semantics() {
        let base_curve_low_beta =
            evaluate_bjt_forward_active_point("2SA1015-GR/Y", 120.0, 1.0e-3, 0.0).unwrap();
        let base_curve_high_beta =
            evaluate_bjt_forward_active_point("2SA1015-GR/Y", 400.0, 1.0e-3, 0.0).unwrap();
        assert_eq!(base_curve_low_beta.collector_current_a, 1.0e-3);
        assert!((base_curve_low_beta.base_current_a - 1.0e-3 / 120.0).abs() < 1.0e-15);
        assert!(base_curve_high_beta.vbe_v < base_curve_low_beta.vbe_v);

        let collector_curve_low_beta =
            evaluate_bjt_forward_active_point("2SA1115-F", 250.0, 5.0e-3, 0.0).unwrap();
        let collector_curve_high_beta =
            evaluate_bjt_forward_active_point("2SA1115-F", 500.0, 5.0e-3, 10.0).unwrap();
        assert!((collector_curve_low_beta.base_current_a - 20.0e-6).abs() < 1.0e-15);
        assert!(
            (collector_curve_high_beta.vbe_v - collector_curve_low_beta.vbe_v - 0.010).abs()
                < 1.0e-12
        );
        assert!(evaluate_bjt_forward_active_point("2SC2603-F", 300.0, 0.1e-3, 0.0).is_none());
        for point in bjt_forward_active_reference_points() {
            assert!(point.beta > 0.0);
            assert!(point.collector_current_a > point.base_current_a);
            assert_eq!(point.vbe_offset_mv, 0.0);
            assert!(point.vbe_v > 0.0);
        }
    }

    #[test]
    fn cutoff_and_saturation_endpoints_remain_test_point_limits() {
        let envelopes = bjt_dc_region_evidence_envelopes();
        assert_eq!(envelopes.len(), 4);
        for envelope in envelopes {
            assert!((envelope.collector_cutoff_current_maximum_a - 0.1e-6).abs() < 1.0e-15);
            assert!((envelope.emitter_cutoff_current_maximum_a - 0.1e-6).abs() < 1.0e-15);
            assert!(envelope.cutoff_endpoints_are_maximum_limits);
            assert!(!envelope.cutoff_continuous_iv_ready);
            assert_eq!(envelope.saturation_test_collector_current_a, 0.1);
            assert_eq!(envelope.saturation_forced_beta, 10.0);
            assert!(envelope.saturation_vce_maximum_absolute_v <= 0.3);
            assert!(envelope.network_peak_below_saturation_test_current);
            assert!(envelope.extend_saturation_limit_to_lower_current_is_hypothesis);
            assert!(!envelope.saturation_continuous_iv_ready);
            assert!(!envelope.temperature_spread_ready);
            assert!(!envelope.full_dc_region_model_ready);
        }
        assert_eq!(envelopes[0].collector_cutoff_test_voltage_absolute_v, 50.0);
        assert_eq!(envelopes[1].emitter_cutoff_test_voltage_absolute_v, 6.0);
        assert_eq!(envelopes[2].collector_cutoff_test_voltage_absolute_v, 60.0);
        assert_eq!(envelopes[3].emitter_cutoff_test_voltage_absolute_v, 6.0);
        assert_eq!(envelopes[0].saturation_vbe_maximum_absolute_v, Some(1.1));
        assert_eq!(envelopes[2].saturation_vbe_maximum_absolute_v, Some(1.0));
        assert!(envelopes[1].saturation_vbe_maximum_absolute_v.is_none());
        assert!(envelopes[3].saturation_vbe_maximum_absolute_v.is_none());
    }

    #[test]
    fn local_output_admittance_is_not_promoted_to_global_early_voltage() {
        let envelopes = bjt_dc_region_evidence_envelopes();
        let pnp = envelopes
            .iter()
            .find(|envelope| envelope.candidate_key == "2SA1115-F")
            .unwrap();
        let npn = envelopes
            .iter()
            .find(|envelope| envelope.candidate_key == "2SC2603-F")
            .unwrap();
        assert_eq!(
            envelopes
                .iter()
                .filter(|envelope| envelope.local_output_admittance_typical_siemens.is_some())
                .count(),
            2
        );
        assert!((pnp.local_output_resistance_typical_ohms.unwrap() - 55_555.555_556).abs() < 0.001);
        assert!(
            (npn.local_output_resistance_typical_ohms.unwrap() - 181_818.181_818).abs() < 0.001
        );
        assert!((pnp.local_effective_early_voltage_absolute_v.unwrap() - 55.555_556).abs() < 0.001);
        assert!(
            (npn.local_effective_early_voltage_absolute_v.unwrap() - 181.818_182).abs() < 0.001
        );
        assert!(envelopes.iter().all(|envelope| {
            envelope.effective_early_voltage_is_local_proxy_not_spice_vaf
                && !envelope.early_effect_continuous_model_ready
        }));
    }

    #[test]
    fn continuous_bjt_region_hypotheses_cross_only_explicit_uncertainty_axes() {
        let hypotheses = bjt_continuous_dc_hypotheses();
        assert_eq!(hypotheses.len(), 96);
        for candidate_key in ["2SA1015-GR/Y", "2SA1115-F", "2SC1815-GR/Y", "2SC2603-F"] {
            let candidate = hypotheses
                .iter()
                .filter(|hypothesis| hypothesis.candidate_key == candidate_key)
                .collect::<Vec<_>>();
            let is_mitsubishi = candidate_key.contains("1115") || candidate_key.contains("2603");
            assert_eq!(candidate.len(), if is_mitsubishi { 32 } else { 16 });
            assert!(candidate.iter().all(|hypothesis| {
                !hypothesis.covers_unit_to_unit_variation
                    && !hypothesis.suitable_as_confirmed_device_model
            }));
            if is_mitsubishi {
                assert!(candidate.iter().any(|hypothesis| {
                    hypothesis.saturation_voltage_corner
                        == BjtSaturationVoltageCorner::ZeroLowerBound
                }));
                assert!(candidate.iter().any(|hypothesis| {
                    hypothesis.output_conductance_corner
                        == BjtOutputConductanceCorner::LocalTypicalHeldConstant
                        && hypothesis
                            .local_output_conductance_held_away_from_test_point_is_hypothesis
                }));
                assert!(!candidate.iter().any(|hypothesis| {
                    hypothesis.saturation_voltage_corner
                        == BjtSaturationVoltageCorner::DirectTypicalCurve
                }));
            } else {
                assert!(candidate.iter().any(|hypothesis| {
                    hypothesis.saturation_voltage_corner
                        == BjtSaturationVoltageCorner::DirectTypicalCurve
                        && hypothesis.saturation_typical_is_direct_toshiba_curve
                }));
                assert!(candidate.iter().all(|hypothesis| {
                    hypothesis.output_conductance_corner == BjtOutputConductanceCorner::Zero
                }));
            }
        }
        assert!(
            hypotheses
                .iter()
                .enumerate()
                .all(|(index, hypothesis)| hypothesis.index == index)
        );
    }

    #[test]
    fn continuous_bjt_region_family_is_numerically_ready_but_not_promotable() {
        let validations = bjt_continuous_dc_validations();
        assert_eq!(validations.len(), 4);
        assert_eq!(
            validations
                .iter()
                .map(|validation| validation.hypothesis_count)
                .sum::<usize>(),
            96
        );
        for validation in validations {
            assert!(validation.evaluated_grid_points >= 256);
            assert!(validation.every_point_finite_and_nonnegative);
            assert!(validation.cutoff_endpoints_respect_zero_to_published_maximum);
            assert!(validation.cutoff_to_forward_active_join_is_continuous);
            assert!(validation.saturation_current_is_monotonic_with_vce);
            assert!(validation.direct_typical_saturation_never_extrapolates);
            assert!(validation.published_maximum_extension_is_labeled_hypothesis);
            assert!(validation.local_hoe_never_promoted_to_global_early_voltage);
            assert!(validation.preliminary_region_sweep_ready);
            assert!(!validation.bounded_production_device_model_ready);
        }
        let references = bjt_continuous_dc_reference_points();
        assert_eq!(references.len(), 96);
        assert!(references.iter().all(|point| {
            point.region == BjtContinuousDcRegion::ForwardActive
                && point.saturation_current_scale == 1.0
                && point.output_conductance_correction_a.abs() < 1.0e-15
                && point.collector_power_inside_25c_published_limit
        }));
        let first = bjt_continuous_dc_hypotheses()
            .into_iter()
            .find(|hypothesis| hypothesis.candidate_key == "2SA1015-GR/Y")
            .unwrap();
        let transfer = bjt_piecewise_transfer_validation(first.candidate_key);
        assert!(
            evaluate_bjt_continuous_dc_point(
                first.index,
                200.0,
                transfer.maximum_vbe_v + 1.0e-6,
                6.0
            )
            .is_none()
        );
        assert!(evaluate_bjt_continuous_dc_point(first.index, 0.0, 0.5, 6.0).is_none());
    }

    #[test]
    fn coupled_active_network_residual_stamps_every_audited_dc_device_class() {
        let coordinate = active_network_dc_coordinates()[1];
        let seed = active_network_dc_seed(coordinate.triangle_input_v, 1);
        let evaluation = evaluate_active_network_dc_residuals(coordinate, &seed).unwrap();
        assert_eq!(
            evaluation.residuals_a.len(),
            ACTIVE_NETWORK_DC_UNKNOWNS.len()
        );
        assert_eq!(
            evaluation.device_points.len(),
            mirrored_bipolar_connections().len()
        );
        assert!(
            evaluation
                .residuals_a
                .iter()
                .all(|residual| residual.is_finite())
        );
        assert!(evaluation.diode_current_a.is_finite());
        assert!(evaluation.mn3101_point.combined_power_inside_absolute_limit);
        let points = active_network_dc_operating_points();
        let validation = active_network_dc_solver_validation_for(&points);
        assert!(validation.residual_assembly_covers_resistors);
        assert!(validation.residual_assembly_covers_bipolars);
        assert!(validation.residual_assembly_covers_diode_center);
        assert!(validation.residual_assembly_covers_mn3101_ox_ports);
        assert!(validation.capacitors_are_open_at_dc);
        assert!(!validation.mn3101_hidden_supply_current_is_modeled);
    }

    #[test]
    fn preliminary_fixed_point_passes_kcl_polarity_and_power_without_claiming_physics() {
        let points = active_network_dc_operating_points();
        let validation = active_network_dc_solver_validation_for(&points);
        assert_eq!(points.len(), 3);
        assert_eq!(validation.coordinate_count, 3);
        assert_eq!(validation.converged_coordinate_count, 1);
        assert!(validation.preliminary_fixed_points_ready);
        assert!(validation.all_converged_points_inside_supply_window);
        assert!(validation.all_converged_bjt_points_respect_vce_polarity);
        assert!(validation.all_converged_bjt_points_inside_power_limits);
        assert!(!validation.all_converged_bjt_points_inside_digitized_transfer_domain);
        assert!(!validation.confirmed_physical_operating_points_ready);
        let accepted = points.iter().find(|point| point.converged).unwrap();
        assert_eq!(accepted.coordinate.name, "tp3_zero_geometric_probe");
        assert!(accepted.maximum_kcl_residual_a <= 1.0e-7);
        assert!(accepted.suitable_only_as_preliminary_fixed_point);
        assert!(!accepted.suitable_as_confirmed_physical_operating_point);
        assert!(
            accepted
                .device_points
                .iter()
                .any(|device| { device.transfer_voltage_was_clamped_to_digitized_domain })
        );
        assert_eq!(points.iter().filter(|point| !point.converged).count(), 2);
    }

    #[test]
    fn thermal_evidence_separates_direct_toshiba_curves_from_mitsubishi_gap() {
        let records = bjt_published_thermal_evidence();
        assert_eq!(records.len(), 4);
        for record in records {
            assert!(record.temperature_curves_are_manufacturer_typical_not_limits);
            assert!(record.power_derating_is_direct_published_graph);
            assert_eq!(record.collector_power_graph_minimum_ambient_c, 0.0);
            assert_eq!(record.collector_power_flat_through_ambient_c, 25.0);
            assert_eq!(record.collector_power_zero_at_ambient_c, 125.0);
            if record.candidate_key.contains("1015") || record.candidate_key.contains("1815") {
                assert_eq!(record.transfer_curve_temperatures_c, [-25.0, 25.0, 100.0]);
                assert_eq!(record.saturation_curve_temperatures_c, [-25.0, 25.0, 100.0]);
                assert_eq!(record.hfe_curve_temperatures_c, [-25.0, 25.0, 100.0]);
                assert!(record.direct_candidate_temperature_transfer_available);
            } else {
                assert_eq!(record.transfer_curve_temperatures_c, [25.0]);
                assert_eq!(record.saturation_curve_temperatures_c, [25.0]);
                assert_eq!(record.hfe_curve_temperatures_c, [25.0]);
                assert!(!record.direct_candidate_temperature_transfer_available);
            }
        }

        let readiness = bjt_thermal_evidence_readiness();
        assert_eq!(readiness.candidate_count, 4);
        assert_eq!(readiness.candidates_with_direct_power_derating_graph, 4);
        assert_eq!(
            readiness.candidates_with_three_temperature_transfer_curves,
            2
        );
        assert_eq!(
            readiness.candidates_with_three_temperature_saturation_curves,
            2
        );
        assert_eq!(readiness.candidates_with_three_temperature_hfe_curves, 2);
        assert_eq!(
            readiness.candidates_with_digitized_three_temperature_transfer_curves,
            2
        );
        assert_eq!(
            readiness.candidates_with_digitized_three_temperature_hfe_curves,
            2
        );
        assert_eq!(
            readiness.candidates_with_digitized_three_temperature_saturation_curves,
            2
        );
        assert_eq!(
            readiness.candidates_with_comparative_temperature_transfer_intervals,
            2
        );
        assert!(readiness.power_derating_ready_for_all_candidates);
        assert!(readiness.direct_toshiba_temperature_interpolation_ready);
        assert!(readiness.direct_toshiba_thermal_gain_interpolation_ready);
        assert!(readiness.direct_toshiba_thermal_saturation_interpolation_ready);
        assert!(readiness.comparative_mitsubishi_temperature_sweep_ready);
        assert!(!readiness.direct_temperature_transfer_ready_for_all_candidates);
        assert!(readiness.comparative_temperature_borrowing_required_for_mitsubishi);
        assert!(!readiness.continuous_temperature_dc_models_ready);
    }

    #[test]
    fn toshiba_thermal_transfer_anchors_are_calibrated_and_ordered() {
        let calibrations = bjt_thermal_transfer_plot_calibrations();
        assert_eq!(calibrations.len(), 2);
        assert!(calibrations.iter().all(|calibration| {
            calibration.render_dpi == 600
                && calibration.reading_radius_px == 8.0
                && calibration.current_kind == TransferCurrentKind::Base
        }));
        let pixel_anchors = bjt_thermal_transfer_pixel_anchors();
        let electrical_anchors = bjt_thermal_transfer_electrical_anchors();
        assert_eq!(pixel_anchors.len(), 30);
        assert_eq!(electrical_anchors.len(), 30);
        assert!(electrical_anchors.iter().all(|anchor| {
            anchor.current_a > 0.0
                && anchor.vbe_absolute_v > 0.0
                && anchor.vbe_reading_uncertainty_mv > 0.0
                && anchor.current_reading_uncertainty_percent > 0.0
        }));
        for validation in bjt_thermal_transfer_validations() {
            assert_eq!(validation.temperatures_c, [-25.0, 25.0, 100.0]);
            assert_eq!(validation.anchors_per_temperature, 5);
            assert_eq!(validation.total_anchor_count, 15);
            assert!(validation.every_trace_strictly_monotonic);
            assert!(validation.thermal_order_preserved_at_every_current);
            assert!(validation.maximum_anchor_round_trip_error_mv < 1.0e-9);
            assert!(validation.interpolation_inside_temperature_and_current_domain_ready);
            assert!(!validation.extrapolation_permitted);
            assert!(!validation.typical_curve_covers_unit_to_unit_variation);
        }
    }

    #[test]
    fn toshiba_thermal_interpolation_preserves_direct_traces_and_refuses_extrapolation() {
        let references = bjt_thermal_transfer_reference_points();
        assert_eq!(references.len(), 6);
        for candidate_key in ["2SA1015-GR/Y", "2SC1815-GR/Y"] {
            let candidate = references
                .iter()
                .filter(|point| point.candidate_key == candidate_key)
                .collect::<Vec<_>>();
            let cold = candidate
                .iter()
                .find(|point| point.temperature_c == -25.0)
                .unwrap();
            let room = candidate
                .iter()
                .find(|point| point.temperature_c == 25.0)
                .unwrap();
            let hot = candidate
                .iter()
                .find(|point| point.temperature_c == 100.0)
                .unwrap();
            assert!(hot.vbe_absolute_v < room.vbe_absolute_v);
            assert!(room.vbe_absolute_v < cold.vbe_absolute_v);
            assert!(hot.vbe_shift_from_25c_mv < 0.0);
            assert_eq!(room.vbe_shift_from_25c_mv, 0.0);
            assert!(cold.vbe_shift_from_25c_mv > 0.0);
            assert!(candidate.iter().all(|point| {
                point.temperature_is_direct_published_trace
                    && point.manufacturer_typical_not_production_bound
            }));

            let source_current_a = room.source_current_a;
            let interpolated =
                evaluate_bjt_thermal_transfer_point(candidate_key, 50.0, source_current_a).unwrap();
            assert!(!interpolated.temperature_is_direct_published_trace);
            assert!(interpolated.vbe_absolute_v < room.vbe_absolute_v);
            assert!(interpolated.vbe_absolute_v > hot.vbe_absolute_v);
            assert!(
                evaluate_bjt_thermal_transfer_point(candidate_key, -25.01, source_current_a)
                    .is_none()
            );
            assert!(
                evaluate_bjt_thermal_transfer_point(candidate_key, 100.01, source_current_a)
                    .is_none()
            );
        }
        assert!(evaluate_bjt_thermal_transfer_point("2SA1115-F", 25.0, 1.0e-5).is_none());
    }

    #[test]
    fn toshiba_thermal_gain_and_saturation_anchors_preserve_conditions() {
        let calibrations = bjt_thermal_characteristic_plot_calibrations();
        assert_eq!(calibrations.len(), 4);
        assert!(calibrations.iter().all(|calibration| {
            calibration.render_dpi == 600
                && calibration.reading_radius_px == 10.0
                && calibration.axes_are_logarithmic
                && calibration.manufacturer_typical_not_limit
        }));
        assert_eq!(
            calibrations
                .iter()
                .filter(|calibration| {
                    calibration.curve_kind == BjtThermalCurveKind::ForwardCurrentGain
                        && calibration.condition.contains("6 V")
                })
                .count(),
            2
        );
        assert_eq!(
            calibrations
                .iter()
                .filter(|calibration| {
                    calibration.curve_kind == BjtThermalCurveKind::CollectorEmitterSaturationVoltage
                        && calibration.condition.contains("10")
                })
                .count(),
            2
        );
        let pixels = bjt_thermal_characteristic_pixel_anchors();
        let electrical = bjt_thermal_characteristic_electrical_anchors();
        assert_eq!(pixels.len(), 60);
        assert_eq!(electrical.len(), 60);
        assert!(electrical.iter().all(|anchor| {
            anchor.collector_current_a.is_finite()
                && anchor.collector_current_a > 0.0
                && anchor.value.is_finite()
                && anchor.value > 0.0
                && anchor.collector_current_reading_uncertainty_percent > 0.0
                && anchor.value_reading_uncertainty_percent > 0.0
        }));
    }

    #[test]
    fn toshiba_thermal_gain_and_saturation_interpolation_is_direct_and_bounded() {
        let validations = bjt_thermal_characteristic_validations();
        assert_eq!(validations.len(), 4);
        for validation in validations {
            assert_eq!(validation.temperatures_c, [-25.0, 25.0, 100.0]);
            assert_eq!(validation.anchors_per_temperature, 5);
            assert_eq!(validation.total_anchor_count, 15);
            assert!(validation.every_anchor_positive_and_finite);
            assert!(validation.hot_above_room_above_cold_at_every_reference_current);
            assert!(validation.maximum_anchor_round_trip_relative_error_percent < 1.0e-9);
            assert!(validation.interpolation_inside_temperature_and_current_domain_ready);
            assert!(!validation.extrapolation_permitted);
            assert!(!validation.typical_curve_covers_unit_to_unit_variation);
        }
        let references = bjt_thermal_characteristic_reference_points();
        assert_eq!(references.len(), 12);
        assert!(references.iter().all(|point| {
            point.temperature_is_direct_published_trace
                && point.manufacturer_typical_not_production_bound
        }));
        for (candidate_key, curve_kind) in [
            ("2SA1015-GR/Y", BjtThermalCurveKind::ForwardCurrentGain),
            (
                "2SA1015-GR/Y",
                BjtThermalCurveKind::CollectorEmitterSaturationVoltage,
            ),
            ("2SC1815-GR/Y", BjtThermalCurveKind::ForwardCurrentGain),
            (
                "2SC1815-GR/Y",
                BjtThermalCurveKind::CollectorEmitterSaturationVoltage,
            ),
        ] {
            let room = references
                .iter()
                .find(|point| {
                    point.candidate_key == candidate_key
                        && point.curve_kind == curve_kind
                        && (point.temperature_c - 25.0).abs() < 1.0e-9
                })
                .unwrap();
            let interpolated = evaluate_bjt_thermal_characteristic_point(
                candidate_key,
                curve_kind,
                50.0,
                room.collector_current_a,
            )
            .unwrap();
            assert!(!interpolated.temperature_is_direct_published_trace);
            assert!(interpolated.current_is_interpolated_between_digitized_anchors);
            assert!(interpolated.value > room.value);
            assert!(
                evaluate_bjt_thermal_characteristic_point(
                    candidate_key,
                    curve_kind,
                    -25.01,
                    room.collector_current_a
                )
                .is_none()
            );
            assert!(
                evaluate_bjt_thermal_characteristic_point(
                    candidate_key,
                    curve_kind,
                    100.01,
                    room.collector_current_a
                )
                .is_none()
            );
        }
    }

    #[test]
    fn mitsubishi_comparative_thermal_intervals_keep_direct_and_borrowed_evidence_distinct() {
        let intervals = bjt_comparative_thermal_reference_intervals();
        assert_eq!(intervals.len(), 30);
        for target_candidate_key in ["2SA1115-F", "2SC2603-F"] {
            let target = intervals
                .iter()
                .filter(|interval| interval.target_candidate_key == target_candidate_key)
                .collect::<Vec<_>>();
            assert_eq!(target.len(), 15);
            assert!(target.iter().all(|interval| {
                interval.donor_candidate_keys == ["2SA1015-GR/Y", "2SC1815-GR/Y"]
                    && interval.donor_current_kind == TransferCurrentKind::Base
                    && interval.target_current_kind == TransferCurrentKind::Collector
                    && interval.includes_digitization_reading_uncertainty
                    && interval.comparable_family_hypothesis_not_device_measurement
                    && !interval.covers_target_unit_to_unit_variation
                    && interval.lower_vbe_v <= interval.upper_vbe_v
            }));
            let room = target
                .iter()
                .filter(|interval| (interval.temperature_c - 25.0).abs() < 1.0e-9)
                .collect::<Vec<_>>();
            assert_eq!(room.len(), 3);
            assert!(room.iter().all(|interval| {
                interval.direct_target_temperature_evidence
                    && interval.lower_shift_from_25c_mv.abs() < 1.0e-9
                    && interval.upper_shift_from_25c_mv.abs() < 1.0e-9
                    && (interval.lower_vbe_v - interval.target_vbe_at_25c_v).abs() < 1.0e-12
                    && (interval.upper_vbe_v - interval.target_vbe_at_25c_v).abs() < 1.0e-12
            }));
            assert!(
                target
                    .iter()
                    .filter(|interval| (interval.temperature_c + 25.0).abs() < 1.0e-9)
                    .all(|interval| interval.lower_shift_from_25c_mv > 0.0)
            );
            assert!(
                target
                    .iter()
                    .filter(|interval| (interval.temperature_c - 100.0).abs() < 1.0e-9)
                    .all(|interval| interval.upper_shift_from_25c_mv < 0.0)
            );
        }
    }

    #[test]
    fn mitsubishi_comparative_thermal_sweep_is_bounded_and_non_promotable() {
        for validation in bjt_comparative_thermal_envelope_validations() {
            assert_eq!(validation.temperature_domain_c, [-25.0, 100.0]);
            assert_eq!(validation.normalized_current_domain, [0.0, 1.0]);
            assert_eq!(validation.temperature_samples, 7);
            assert_eq!(validation.normalized_current_samples, 11);
            assert_eq!(validation.evaluated_points, 77);
            assert!(validation.every_interval_is_ordered);
            assert!(validation.every_cold_interval_is_above_25c);
            assert!(validation.every_hot_interval_is_below_25c);
            assert!(validation.room_temperature_collapses_to_direct_target_curve);
            assert!(validation.rejects_temperature_extrapolation);
            assert!(validation.rejects_current_extrapolation);
            assert!(!validation.direct_mitsubishi_temperature_curve_available);
            assert!(validation.usable_as_comparative_sweep_interval);
            assert!(!validation.usable_as_confirmed_device_model);
        }
        assert!(
            evaluate_bjt_comparative_thermal_shift_interval("2SA1015-GR/Y", 25.0, 0.5).is_none()
        );
        assert!(
            evaluate_bjt_comparative_thermal_shift_interval("2SA1115-F", f64::NAN, 0.5).is_none()
        );
    }

    #[test]
    fn published_collector_power_derating_is_continuous_and_refuses_extrapolation() {
        for record in bjt_published_thermal_evidence() {
            let full_power = record.collector_power_at_25c_mw;
            assert_eq!(
                bjt_collector_power_limit_mw(record.candidate_key, 0.0),
                Some(full_power)
            );
            assert_eq!(
                bjt_collector_power_limit_mw(record.candidate_key, 25.0),
                Some(full_power)
            );
            assert_eq!(
                bjt_collector_power_limit_mw(record.candidate_key, 75.0),
                Some(full_power * 0.5)
            );
            assert_eq!(
                bjt_collector_power_limit_mw(record.candidate_key, 125.0),
                Some(0.0)
            );
            assert!(bjt_collector_power_limit_mw(record.candidate_key, -0.01).is_none());
            assert!(bjt_collector_power_limit_mw(record.candidate_key, 125.01).is_none());
            assert!(bjt_collector_power_limit_mw(record.candidate_key, f64::NAN).is_none());
        }
        let points = bjt_collector_power_reference_points();
        assert_eq!(points.len(), 16);
        assert!(points.iter().all(|point| {
            point.inside_published_graph_domain && point.collector_power_limit_mw >= 0.0
        }));
    }

    #[test]
    fn coupled_dc_solver_exposes_preliminary_fixed_point_without_promoting_it() {
        let report = report();
        let readiness = report.active_network_solver_readiness;
        assert!(readiness.explicit_connection_graph_ready);
        assert_eq!(readiness.published_bjt_dc_curve_sets, 4);
        assert!(readiness.digitized_bjt_typical_transfer_fits_ready);
        assert!(readiness.bjt_forward_active_evidence_corners_ready);
        assert!(readiness.bjt_cutoff_endpoint_limits_ready);
        assert!(readiness.bjt_saturation_endpoint_limits_ready);
        assert_eq!(readiness.bjt_local_output_admittance_records, 2);
        assert!(readiness.bjt_published_power_derating_ready);
        assert_eq!(
            readiness.bjt_candidates_with_direct_temperature_transfer_curves,
            2
        );
        assert_eq!(
            readiness.bjt_candidates_with_comparative_temperature_transfer_intervals,
            2
        );
        assert!(readiness.bjt_direct_toshiba_thermal_gain_curves_ready);
        assert!(readiness.bjt_direct_toshiba_thermal_saturation_curves_ready);
        assert_eq!(readiness.bjt_continuous_dc_hypothesis_count, 96);
        assert!(readiness.bjt_preliminary_continuous_region_sweep_ready);
        assert_eq!(readiness.bjt_low_vce_base_drive_hypotheses_defined, 6);
        assert_eq!(readiness.bjt_low_vce_base_drive_reference_survivors, 4);
        assert_eq!(
            readiness.bjt_low_vce_base_drive_matrix_points_evaluated,
            128
        );
        assert_eq!(readiness.bjt_low_vce_base_drive_matrix_points_converged, 48);
        assert_eq!(
            readiness.bjt_low_vce_base_drive_hypotheses_eliminating_all_clamps,
            0
        );
        assert!(!readiness.bjt_low_vce_base_drive_family_viable);
        assert_eq!(readiness.bjt_reciprocal_transport_hypotheses_defined, 3);
        assert_eq!(
            readiness.bjt_reciprocal_transport_matrix_points_evaluated,
            96
        );
        assert_eq!(
            readiness.bjt_reciprocal_transport_matrix_points_converged,
            6
        );
        assert_eq!(
            readiness.bjt_reciprocal_transport_exact_reference_targets_reached,
            1
        );
        assert_eq!(
            readiness.bjt_reciprocal_transport_exact_reference_targets_inside_all_gates,
            1
        );
        assert!(readiness.bjt_reciprocal_transport_reference_candidate_viable);
        assert_eq!(
            readiness.bjt_reciprocal_transport_assignment_endpoints_attempted,
            32
        );
        assert_eq!(
            readiness.bjt_reciprocal_transport_assignment_endpoints_converged,
            32
        );
        assert_eq!(
            readiness.bjt_reciprocal_transport_assignment_endpoints_inside_all_gates,
            32
        );
        assert_eq!(
            readiness.bjt_reciprocal_transport_assignment_endpoints_without_clamping,
            32
        );
        assert!(!readiness.bjt_reciprocal_transport_assignment_dc_metrics_accepted);
        assert!(readiness.bjt_reciprocal_transport_dispersion_attributed);
        assert_eq!(
            readiness.bjt_reciprocal_transport_dominant_current_designator,
            "Tr19"
        );
        assert_eq!(
            readiness.bjt_reciprocal_transport_dominant_power_designator,
            "Tr19"
        );
        assert_eq!(
            readiness.bjt_reciprocal_transport_tr19_candidate_envelopes,
            2
        );
        assert_eq!(
            readiness.bjt_reciprocal_transport_tr19_candidates_inside_output_plot_voltage_domain,
            0
        );
        assert_eq!(
            readiness.bjt_reciprocal_transport_tr19_candidates_inside_25c_power_limit,
            2
        );
        assert!(!readiness.bjt_reciprocal_transport_tr19_existing_documents_sufficient);
        assert!(readiness.bjt_reciprocal_transport_tr19_higher_voltage_output_evidence_required);
        assert_eq!(
            readiness.bjt_tr19_high_vce_conductance_hypotheses_screened,
            4_001
        );
        assert!(readiness.bjt_tr19_high_vce_conductance_can_pass_power_gate);
        assert!(!readiness.bjt_tr19_high_vce_conductance_can_pass_current_gate);
        assert!(!readiness.bjt_tr19_high_vce_conductance_can_pass_both_gates);
        assert_eq!(
            readiness.bjt_reciprocal_transport_tr22_candidates_resolvable,
            2
        );
        assert_eq!(
            readiness.bjt_reciprocal_transport_tr22_candidates_inside_base_drive_domain,
            1
        );
        assert_eq!(
            readiness.bjt_reciprocal_transport_tr23_reverse_transport_candidates,
            2
        );
        assert!(readiness.bjt_reciprocal_transport_tr23_reverse_transport_evidence_required);
        assert!(!readiness.bjt_reciprocal_transport_tr22_tr23_existing_documents_sufficient);
        assert_eq!(readiness.bjt_tr22_tr23_reverse_beta_pairs_screened, 90_601);
        assert_eq!(
            readiness.bjt_tr22_tr23_reverse_beta_pairs_passing_frozen_gates,
            2_694
        );
        assert!(!readiness.bjt_tr22_tr23_reverse_beta_direct_evidence_available);
        assert!(!readiness.bjt_tr22_tr23_reverse_beta_ready_for_solver_substitution);
        assert_eq!(
            readiness.bjt_correlated_reverse_transport_frozen_assignments_exceeding_kcl,
            28
        );
        assert!(readiness.bjt_correlated_reverse_transport_maximum_kcl_multiple > 555.5);
        assert!(readiness.bjt_correlated_reverse_transport_full_resolve_required);
        assert_eq!(
            readiness.bjt_correlated_reverse_transport_resolved_assignments,
            32
        );
        assert!(!readiness.bjt_correlated_reverse_transport_resolved_dc_metrics_accepted);
        assert!(readiness.bjt_correlated_reverse_transport_ready_for_evidence_testing);
        assert!(readiness.bjt_correlated_reverse_transport_dispersion_attributed);
        assert_eq!(
            readiness.bjt_correlated_reverse_transport_dominant_current_designator,
            "Tr19"
        );
        assert_eq!(
            readiness.bjt_correlated_reverse_transport_dominant_power_designator,
            "Tr19"
        );
        assert!(readiness.bjt_correlated_reverse_transport_tr19_high_voltage_evidence_required);
        assert!(readiness.bjt_correlated_reverse_transport_tr22_surface_evidence_required);
        assert!(readiness.bjt_correlated_reverse_transport_tr23_reverse_evidence_required);
        assert!(!readiness.bjt_forced_beta_ten_base_drive_homotopy_reached);
        assert!(!readiness.bjt_saturation_transfer_clamps_eliminated);
        assert!(readiness.coupled_dc_residual_assembly_ready);
        assert_eq!(readiness.preliminary_dc_fixed_point_count, 1);
        assert!(readiness.preliminary_dc_fixed_point_solver_ready);
        assert!(!readiness.bjt_cutoff_saturation_and_early_regions_ready);
        assert!(!readiness.bounded_bjt_dc_fits_ready);
        assert!(readiness.digitized_diode_typical_forward_curve_ready);
        assert!(readiness.bounded_diode_iv_model_ready);
        assert!(readiness.mn3101_ox_logic_and_endpoint_bounds_available);
        assert!(readiness.mn3101_ox_continuous_port_model_available);
        assert_eq!(readiness.mn3101_frozen_node_matrix_points_evaluated, 288);
        assert_eq!(
            readiness.mn3101_macros_compatible_with_all_frozen_assignments,
            1
        );
        assert_eq!(readiness.mn3101_homotopy_exact_targets_reached, 5);
        assert!(!readiness.mn3101_homotopy_all_targets_reached);
        assert!(!readiness.mn3101_full_nonlinear_macro_sweep_ready);
        assert!(!readiness.full_dc_problem_well_posed);
        assert_eq!(readiness.evaluated_assignment_hypotheses, 32);
        assert_eq!(readiness.required_missing_models.len(), 1);
        assert!(readiness.interpretation.contains("v41"));
        assert!(readiness.interpretation.contains("28.131-28.132 V"));
        assert!(readiness.interpretation.contains("negative-VCE"));
        let matrix = report.mn3101_frozen_node_matrix_validation;
        assert_eq!(matrix.expected_matrix_points, 288);
        assert_eq!(matrix.evaluated_matrix_points, 288);
        assert_eq!(
            matrix.macros_compatible_with_all_frozen_assignment_points,
            1
        );
        assert!(matrix.every_macro_point_inside_absolute_power_limit);
        assert!(matrix.matrix_is_screening_not_operating_point_solution);
        assert!(!matrix.actual_mn3101_transition_identified);
        let compatible = report
            .mn3101_frozen_node_compatibility_summaries
            .iter()
            .filter(|summary| summary.suitable_as_continuation_seed_for_all_assignments)
            .collect::<Vec<_>>();
        assert_eq!(compatible.len(), 1);
        assert_eq!(compatible[0].mn3101_macro_hypothesis_index, 4);
        assert_eq!(compatible[0].residual_passing_assignments, 32);
        assert!(
            report
                .mn3101_frozen_node_compatibility_summaries
                .iter()
                .all(|summary| summary.every_point_inside_mn3101_power_limit)
        );
        let homotopy = report.mn3101_homotopy_validation;
        assert_eq!(homotopy.source_macro_hypothesis_index, 4);
        assert_eq!(homotopy.target_macro_hypotheses, 9);
        assert_eq!(homotopy.exact_targets_reached, 5);
        assert_eq!(homotopy.exact_targets_inside_all_dc_gates, 5);
        assert_eq!(homotopy.exact_targets_without_bjt_transfer_clamping, 0);
        assert!(homotopy.homotopy_is_numerical_path_not_device_evidence);
        assert!(!homotopy.actual_mn3101_transition_identified);
        let reached = report
            .mn3101_homotopy_summaries
            .iter()
            .filter(|summary| summary.exact_target_macro_reached)
            .map(|summary| summary.target_macro_hypothesis_index)
            .collect::<Vec<_>>();
        assert_eq!(reached, vec![2, 3, 4, 5, 8]);
        for index in [0, 1] {
            let summary = &report.mn3101_homotopy_summaries[index];
            assert!(!summary.final_converged);
            assert!(summary.final_every_unknown_inside_supply_window);
            assert!(summary.final_every_bjt_vce_polarity_valid);
        }
        for index in [6, 7] {
            let summary = &report.mn3101_homotopy_summaries[index];
            assert!(summary.final_converged);
            assert!(!summary.final_every_bjt_vce_polarity_valid);
        }
        let base_drive = report.bjt_saturation_base_drive_homotopy_validation;
        assert_eq!(base_drive.attempted_stages, 3);
        assert_eq!(base_drive.converged_stages, 1);
        assert_eq!(base_drive.largest_converged_blend_fraction, 0.0625);
        assert_eq!(base_drive.initial_clamped_bjt_count, 2);
        assert_eq!(base_drive.final_clamped_bjt_count, 2);
        assert!(!base_drive.exact_forced_beta_ten_hypothesis_reached);
        assert!(!base_drive.exact_hypothesis_inside_all_dc_gates);
        assert!(!base_drive.transfer_domain_clamps_eliminated);
        assert!(base_drive.forced_beta_ten_is_published_test_condition);
        assert!(!base_drive.forced_beta_ten_is_continuous_device_law);
        assert!(!base_drive.suitable_as_confirmed_saturation_model);
        let base_drive_stages = &report.bjt_saturation_base_drive_homotopy_stages;
        assert_eq!(base_drive_stages.len(), 3);
        assert_eq!(
            base_drive_stages[0].maximum_base_current_multiplier_over_forward_beta,
            1.0
        );
        assert!(base_drive_stages[1].inside_supply_polarity_and_power_gates);
        assert!(!base_drive_stages[2].converged);
        assert_eq!(base_drive_stages[2].clamped_bjt_count, 1);
    }

    #[test]
    fn mn3101_ox_bounds_preserve_published_endpoints_without_inventing_a_transfer() {
        let bounds = report().mn3101_ox_electrical_bounds;
        assert_eq!(bounds.test_supply_v, -15.0);
        assert_eq!(bounds.power_dissipation_absolute_max_mw, 200.0);
        assert_eq!(bounds.ox1_guaranteed_high_range_v, [0.0, -1.0]);
        assert_eq!(bounds.ox1_guaranteed_low_range_v, [-14.0, -15.0]);
        assert_eq!(bounds.ox1_input_leakage_max_ua, 30.0);
        assert_eq!(bounds.ox2_output_high_current_min_ma, 0.6);
        assert_eq!(bounds.ox2_output_low_current_min_ma, 0.5);
        assert_eq!(bounds.ox3_output_high_current_min_ma, 1.5);
        assert_eq!(bounds.ox3_output_low_current_min_ma, 2.0);
        assert_eq!(bounds.ox2_output_leakage_max_ua, 30.0);
        assert_eq!(bounds.ox3_output_leakage_max_ua, 30.0);
        assert!((bounds.ox2_high_endpoint_resistance_max_ohms - 1_666.666_666).abs() < 0.001);
        assert_eq!(bounds.ox2_low_endpoint_resistance_max_ohms, 2_000.0);
        assert!((bounds.ox3_high_endpoint_resistance_max_ohms - 666.666_666).abs() < 0.001);
        assert_eq!(bounds.ox3_low_endpoint_resistance_max_ohms, 500.0);
        assert!(!bounds.transition_region_is_published);
        assert!(!bounds.continuous_output_iv_is_published);
    }

    #[test]
    fn mn3101_macro_grid_spans_transition_shape_and_output_strength() {
        let hypotheses = mn3101_ox_macro_hypotheses();
        let unique = hypotheses
            .iter()
            .map(|hypothesis| {
                (
                    hypothesis.transition_shape,
                    hypothesis.output_strength_corner,
                )
            })
            .collect::<std::collections::HashSet<_>>();
        assert_eq!(hypotheses.len(), 9);
        assert_eq!(unique.len(), 9);
        assert!(hypotheses.iter().enumerate().all(|(index, hypothesis)| {
            hypothesis.index == index
                && hypothesis.transition_shape_is_hypothesis
                && hypothesis.minimum_resistance_from_total_power_is_conservative_hypothesis
                && hypothesis.published_endpoint_currents_remain_hard_minima
        }));
    }

    #[test]
    fn mn3101_macro_preserves_endpoint_currents_polarity_and_power() {
        let bounds = mn3101_ox_electrical_bounds();
        for hypothesis in mn3101_ox_macro_hypotheses() {
            let low = evaluate_mn3101_ox_macro(hypothesis, -14.0, -1.0, -14.0).unwrap();
            assert_eq!(low.ox1_region, Mn3101Ox1Region::GuaranteedLow);
            assert_eq!(low.ox2_target_v, 0.0);
            assert_eq!(low.ox3_target_v, -15.0);
            assert!(low.ox2_output_current_a >= bounds.ox2_output_high_current_min_ma / 1_000.0);
            assert!(low.ox3_output_current_a <= -bounds.ox3_output_low_current_min_ma / 1_000.0);
            assert!(low.combined_power_inside_absolute_limit);

            let high = evaluate_mn3101_ox_macro(hypothesis, -1.0, -14.0, -1.0).unwrap();
            assert_eq!(high.ox1_region, Mn3101Ox1Region::GuaranteedHigh);
            assert_eq!(high.ox2_target_v, -15.0);
            assert_eq!(high.ox3_target_v, 0.0);
            assert!(high.ox2_output_current_a <= -bounds.ox2_output_low_current_min_ma / 1_000.0);
            assert!(high.ox3_output_current_a >= bounds.ox3_output_high_current_min_ma / 1_000.0);
            assert!(high.combined_power_inside_absolute_limit);
        }
    }

    #[test]
    fn mn3101_transition_hypotheses_bracket_linear_without_claiming_truth() {
        let hypotheses = mn3101_ox_macro_hypotheses();
        let point_for = |shape| {
            let hypothesis = hypotheses
                .iter()
                .find(|candidate| {
                    candidate.transition_shape == shape
                        && candidate.output_strength_corner == Mn3101OutputStrengthCorner::Geometric
                })
                .copied()
                .unwrap();
            evaluate_mn3101_ox_macro(hypothesis, -10.75, -7.5, -7.5).unwrap()
        };
        let early = point_for(Mn3101TransitionShape::Early);
        let linear = point_for(Mn3101TransitionShape::Linear);
        let late = point_for(Mn3101TransitionShape::Late);
        assert!(early.normalized_high_fraction > linear.normalized_high_fraction);
        assert!(linear.normalized_high_fraction > late.normalized_high_fraction);
        assert_eq!(linear.normalized_high_fraction, 0.25);
        assert!([early, linear, late].iter().all(|point| point.ox1_region
            == Mn3101Ox1Region::UnpublishedTransition
            && !point.transition_is_published
            && point.combined_power_inside_absolute_limit));

        let readiness = mn3101_ox_macro_readiness();
        assert_eq!(readiness.hypothesis_count, 9);
        assert!(readiness.guaranteed_logic_regions_preserved);
        assert!(readiness.endpoint_current_minima_preserved);
        assert!(readiness.combined_power_limit_enforced);
        assert!(readiness.complete_supply_domain_covered);
        assert!(!readiness.actual_transition_transfer_solved);
        assert!(readiness.suitable_for_interval_sweep);
        assert!(!readiness.suitable_as_identified_nominal_device_model);
        assert!(evaluate_mn3101_ox_macro(hypotheses[0], -15.1, -7.5, -7.5).is_none());
    }

    #[test]
    fn tightly_clustered_complete_ensemble_passes_the_uncertainty_gate() {
        let values = (0..32)
            .map(|index| 100.0 + (index as f64 - 15.5) * 0.02)
            .collect::<Vec<_>>();
        let comparison = compare_uncertainty_ensemble(&values, uncertainty_acceptance_policy());
        assert!(comparison.all_values_finite);
        assert!(comparison.enough_hypotheses);
        assert!(comparison.mean_absolute_relative_deviation_percent < 1.0);
        assert!(comparison.maximum_absolute_relative_deviation_percent < 3.0);
        assert!(comparison.accepted);
    }

    #[test]
    fn mean_below_limit_cannot_hide_an_outlier() {
        let mut values = vec![100.0; 32];
        values[31] = 109.0;
        let comparison = compare_uncertainty_ensemble(&values, uncertainty_acceptance_policy());
        assert!(comparison.mean_absolute_relative_deviation_percent < 1.0);
        assert!(comparison.maximum_absolute_relative_deviation_percent > 3.0);
        assert!(!comparison.accepted);
    }

    #[test]
    fn incomplete_or_nonfinite_ensemble_cannot_be_accepted() {
        let incomplete =
            compare_uncertainty_ensemble(&[100.0; 31], uncertainty_acceptance_policy());
        assert!(!incomplete.enough_hypotheses);
        assert!(!incomplete.accepted);

        let mut nonfinite_values = [100.0; 32];
        nonfinite_values[4] = f64::NAN;
        let nonfinite =
            compare_uncertainty_ensemble(&nonfinite_values, uncertainty_acceptance_policy());
        assert!(!nonfinite.all_values_finite);
        assert!(!nonfinite.accepted);
    }

    #[test]
    fn uncertainty_policy_is_defined_but_not_prematurely_satisfied() {
        let report = report();
        assert_eq!(
            report
                .uncertainty_acceptance_policy
                .mean_relative_deviation_limit_percent,
            1.0
        );
        assert_eq!(
            report
                .uncertainty_acceptance_policy
                .maximum_relative_deviation_limit_percent,
            3.0
        );
        assert_eq!(
            report.active_device_uncertainty_status.evaluated_hypotheses,
            32
        );
        assert_eq!(
            report.active_device_uncertainty_status.attempted_hypotheses,
            32
        );
        assert_eq!(
            report
                .active_device_uncertainty_status
                .numerically_converged_hypotheses,
            32
        );
        let sweep = &report.active_device_dc_sweep_validation;
        assert!(sweep.complete_32_assignment_ensemble);
        assert_eq!(sweep.assignments_inside_all_dc_gates, 32);
        assert_eq!(sweep.assignments_without_transfer_clamping, 0);
        assert_eq!(sweep.metric_comparisons.len(), 5);
        assert!(
            sweep.metric_comparisons[..3]
                .iter()
                .all(|metric| metric.comparison.unwrap().accepted)
        );
        assert!(
            sweep.metric_comparisons[3..]
                .iter()
                .all(|metric| !metric.comparison.unwrap().accepted)
        );
        assert!(!sweep.dc_metrics_accepted_as_equivalent);
        assert!(!sweep.required_audio_trajectory_metrics_evaluated);
        assert!(!sweep.accepted_as_behaviorally_equivalent);
        assert!(
            !report
                .active_device_uncertainty_status
                .accepted_as_behaviorally_equivalent
        );
        assert!(
            report
                .uncertainty_gate_fixtures
                .iter()
                .all(|fixture| fixture.comparison.accepted == fixture.expected_acceptance)
        );
        assert!(!report.production_promotion_ready);
    }

    #[test]
    fn active_clock_boundaries_end_at_the_two_bbd_channels() {
        let boundaries = active_clock_network_boundaries();
        assert_eq!(boundaries[0].triangle_input, "TP3 through R128 1 kOhm");
        assert_eq!(boundaries[0].bbd_destination, "IC7 MN3009");
        assert_eq!(boundaries[1].triangle_input, "TP4 through R148 1 kOhm");
        assert_eq!(boundaries[1].bbd_destination, "IC10 MN3009");
        assert!(
            boundaries
                .iter()
                .all(|boundary| boundary.positive_supply == "+15 V")
        );
    }

    #[test]
    fn factory_chorus_bias_adjustment_is_not_a_clock_calibration() {
        let adjustment = factory_adjustment();
        assert_eq!(
            adjustment.adjusted_components,
            "VR1 channel 1 and VR2 channel 2 on jack board"
        );
        assert!(adjustment.injected_signal.contains("1 kHz"));
        assert!(adjustment.acceptance.contains("symmetrical"));
        assert!(!adjustment.constrains_clock_rate_or_depth);
        assert!(!report().factory_clock_target_present);
    }
}
