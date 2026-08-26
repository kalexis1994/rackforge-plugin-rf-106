#[cfg(target_arch = "wasm32")]
use rf_106_contract::PUBLIC_KEY_TRANSPOSE_TRIGGER_INDEX;
#[cfg(any(target_arch = "wasm32", test))]
use serde::Deserialize;
#[cfg(target_arch = "wasm32")]
use serde::Serialize;

#[cfg(any(target_arch = "wasm32", test))]
const PROTOCOL: &str = "rackforge.plugin.web@1";

#[cfg(any(target_arch = "wasm32", test))]
const MODEL_ID: &str = "rf106";
#[cfg(any(target_arch = "wasm32", test))]
const MODEL_NAME: &str = "RF-106";

#[cfg(any(target_arch = "wasm32", test))]
#[derive(Clone, Debug, Deserialize)]
struct Sound {
    id: String,
    name: String,
    bank: String,
}

#[cfg(any(target_arch = "wasm32", test))]
fn is_rf106_sound(sound: &Sound) -> bool {
    sound.bank == "factory.rf106" || sound.id.starts_with("factory.rf106.")
}

#[cfg(any(target_arch = "wasm32", test))]
fn patch_prefix_len(name: &str) -> Option<usize> {
    let bytes = name.as_bytes();
    for length in [3, 2] {
        if bytes.len() > length
            && bytes[0].is_ascii_alphabetic()
            && bytes[1..length]
                .iter()
                .all(|byte| (b'1'..=b'8').contains(byte))
            && bytes[length].is_ascii_whitespace()
        {
            return Some(length);
        }
    }
    None
}

#[cfg(any(target_arch = "wasm32", test))]
fn patch_code(sound: &Sound) -> String {
    if let Some(length) = patch_prefix_len(&sound.name) {
        return sound.name[..length].to_ascii_uppercase();
    }
    let number = sound
        .id
        .rsplit('.')
        .next()
        .and_then(|value| value.parse::<u32>().ok())
        .unwrap_or(0);
    let bank = if number < 64 { 'A' } else { 'B' };
    let cell = number % 64;
    format!("{bank}{}{}", cell / 8 + 1, cell % 8 + 1)
}

#[cfg(any(target_arch = "wasm32", test))]
fn clean_patch_name(sound: &Sound) -> &str {
    if let Some(length) = patch_prefix_len(&sound.name) {
        sound.name[length..].trim_start()
    } else {
        &sound.name
    }
}

#[cfg(any(target_arch = "wasm32", test))]
fn escape_html(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for character in value.chars() {
        match character {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#39;"),
            _ => escaped.push(character),
        }
    }
    escaped
}

#[cfg(any(target_arch = "wasm32", test))]
fn relative_vertical_fader_value(
    start_value: f64,
    delta_y: f64,
    track_height: f64,
    minimum: f64,
    maximum: f64,
    step: f64,
) -> f64 {
    if !start_value.is_finite()
        || !delta_y.is_finite()
        || !track_height.is_finite()
        || track_height <= 0.0
        || !minimum.is_finite()
        || !maximum.is_finite()
        || maximum <= minimum
    {
        return minimum;
    }

    // The native range reserves half of its 19 px thumb width at both ends.
    // After the -90 degree rotation that becomes the vertical travel inset.
    let inset = 9.5_f64.min(track_height / 2.0);
    let travel = (track_height - inset * 2.0).max(1.0);
    let raw = start_value + delta_y / travel * (maximum - minimum);
    if step.is_finite() && step > 0.0 {
        (minimum + ((raw - minimum) / step).round() * step).clamp(minimum, maximum)
    } else {
        raw.clamp(minimum, maximum)
    }
}

#[cfg(any(target_arch = "wasm32", test))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ParameterChangeUpdate {
    FullRender,
    TargetOnly,
    TargetAndDeferredRender,
}

#[cfg(any(target_arch = "wasm32", test))]
fn parameter_change_update(
    active_parameter_drag: Option<u32>,
    changed_parameter: u32,
) -> ParameterChangeUpdate {
    match active_parameter_drag {
        None => ParameterChangeUpdate::FullRender,
        Some(active) if active == changed_parameter => ParameterChangeUpdate::TargetOnly,
        Some(_) => ParameterChangeUpdate::TargetAndDeferredRender,
    }
}

#[cfg(any(target_arch = "wasm32", test))]
fn bender_pointer_state(
    client_x: f64,
    client_y: f64,
    field_left: f64,
    field_top: f64,
    field_width: f64,
    field_height: f64,
) -> (f64, bool) {
    if !client_x.is_finite()
        || !client_y.is_finite()
        || !field_left.is_finite()
        || !field_top.is_finite()
        || !field_width.is_finite()
        || !field_height.is_finite()
        || field_width <= 0.0
        || field_height <= 0.0
    {
        return (0.0, false);
    }
    let horizontal_inset = 24.0_f64.min(field_width * 0.25);
    let travel = (field_width - horizontal_inset * 2.0).max(1.0);
    let position =
        ((client_x - field_left - horizontal_inset) / travel * 2.0 - 1.0).clamp(-1.0, 1.0);
    // The physical lever closes LFO TRIG only when pushed away from the
    // player. Keep a dead region around its neutral resting position.
    let lfo_trigger = client_y < field_top + field_height * 0.42;
    (position, lfo_trigger)
}

#[cfg(any(target_arch = "wasm32", test))]
#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(untagged)]
enum ParameterDefault {
    Number(f64),
    Boolean(bool),
}

#[cfg(any(target_arch = "wasm32", test))]
impl ParameterDefault {
    const fn as_f64(self) -> f64 {
        match self {
            Self::Number(value) => value,
            Self::Boolean(value) => value as u8 as f64,
        }
    }
}

#[cfg(target_arch = "wasm32")]
mod browser {
    use super::*;
    use js_sys::{Object, Reflect};
    use std::{cell::RefCell, collections::BTreeMap, rc::Rc};
    use wasm_bindgen::{JsCast, JsValue, closure::Closure, prelude::wasm_bindgen};
    use web_sys::{Document, Element, Event, MessageEvent, MouseEvent, PointerEvent, Window};

    type AppHandle = Rc<RefCell<App>>;
    type ResponseHandler = Box<dyn FnOnce(&AppHandle, Result<JsValue, String>)>;

    #[derive(Debug, Deserialize)]
    struct HostContext {
        instance: Instance,
    }

    #[derive(Debug, Deserialize)]
    struct Instance {
        selected_sound_id: String,
        sounds: Vec<Sound>,
    }

    #[derive(Clone, Debug, Deserialize)]
    struct ParameterSnapshot {
        schema: ParameterSchema,
        values: Vec<ParameterValue>,
    }

    #[derive(Clone, Debug, Deserialize)]
    struct ParameterSchema {
        pages: Vec<Page>,
        parameters: Vec<Parameter>,
    }

    #[derive(Clone, Debug, Deserialize)]
    struct Page {
        id: String,
    }

    #[derive(Clone, Debug, Deserialize)]
    struct Parameter {
        index: u32,
        id: String,
        name: String,
        page: String,
        order: i32,
        kind: ParameterKind,
    }

    #[derive(Clone, Debug, Deserialize)]
    struct ParameterKind {
        #[serde(rename = "type")]
        kind: String,
        minimum: Option<f64>,
        maximum: Option<f64>,
        default: Option<ParameterDefault>,
        step: Option<f64>,
        unit: Option<String>,
        #[serde(default)]
        choices: Vec<Choice>,
    }

    impl ParameterKind {
        fn default_value(&self) -> f64 {
            self.default.map(ParameterDefault::as_f64).unwrap_or(0.0)
        }
    }

    #[derive(Clone, Debug, Deserialize)]
    struct Choice {
        name: String,
        value: f64,
    }

    #[derive(Clone, Copy, Debug, Deserialize)]
    struct ParameterValue {
        index: u32,
        value: f64,
    }

    #[derive(Serialize)]
    struct Request<'a> {
        protocol: &'static str,
        kind: &'static str,
        request_id: &'a str,
        method: &'a str,
        params: serde_json::Value,
    }

    #[derive(Serialize)]
    struct Ready {
        protocol: &'static str,
        kind: &'static str,
    }

    struct App {
        window: Window,
        document: Document,
        root: Element,
        host_origin: String,
        context: Option<HostContext>,
        snapshot: Option<ParameterSnapshot>,
        parameter_values: BTreeMap<u32, f64>,
        pending: BTreeMap<String, ResponseHandler>,
        sequence: u64,
        pending_sound_id: Option<String>,
        parameter_refresh_generation: u64,
        active_parameter_drag: Option<u32>,
        render_after_parameter_drag: bool,
        refresh_parameters_after_drag: bool,
        active_section: String,
        search_query: String,
        bridge_error: String,
    }

    impl App {
        fn new() -> Result<AppHandle, JsValue> {
            let window = web_sys::window().ok_or_else(|| JsValue::from_str("missing window"))?;
            let document = window
                .document()
                .ok_or_else(|| JsValue::from_str("missing document"))?;
            let root = document
                .get_element_by_id("plugin-root")
                .ok_or_else(|| JsValue::from_str("missing #plugin-root"))?;
            let active_section = "lfo".to_owned();
            let host_origin = window.location().origin()?;
            Ok(Rc::new(RefCell::new(Self {
                window,
                document,
                root,
                host_origin,
                context: None,
                snapshot: None,
                parameter_values: BTreeMap::new(),
                pending: BTreeMap::new(),
                sequence: 0,
                pending_sound_id: None,
                parameter_refresh_generation: 0,
                active_parameter_drag: None,
                render_after_parameter_drag: false,
                refresh_parameters_after_drag: false,
                active_section,
                search_query: String::new(),
                bridge_error: String::new(),
            })))
        }

        fn selected_sound(&self) -> Option<&Sound> {
            let context = self.context.as_ref()?;
            context
                .instance
                .sounds
                .iter()
                .find(|sound| sound.id == context.instance.selected_sound_id)
        }

        fn parameter_value(&self, index: u32, fallback: f64) -> f64 {
            self.parameter_values
                .get(&index)
                .copied()
                .unwrap_or(fallback)
        }

        fn render(&self) {
            if self.context.is_none() {
                return;
            }
            let mut html = String::from("<div class=\"synth-chassis\">");
            html.push_str(&self.render_hero());
            html.push_str(&self.render_section_tabs());
            html.push_str(&self.render_config_body());
            html.push_str(&self.render_program_selector());
            html.push_str("</div>");
            html.push_str(&self.render_program_library());
            self.root.set_inner_html(&html);
            self.sync_program_group_routing();

            // Re-measure after the browser has committed the new grid layout.
            // The resize listener uses the same DOM anchors and also keeps the
            // route synchronized when RackForge changes the panel width.
            let resize_window = self.window.clone();
            let after_layout = Closure::once_into_js(move || {
                if let Ok(event) = Event::new("resize") {
                    let _ = resize_window.dispatch_event(&event);
                }
            });
            let _ = self
                .window
                .request_animation_frame(after_layout.unchecked_ref());
        }

        fn sync_program_group_routing(&self) {
            self.sync_performance_portamento_routing();
            let Ok(Some(hardware)) = self.root.query_selector(".program-hardware") else {
                return;
            };
            let Ok(Some(svg)) = hardware.query_selector(".program-group-routing") else {
                return;
            };
            let Ok(Some(path)) = svg.query_selector("path") else {
                return;
            };
            let Ok(Some(red_led)) = hardware
                .query_selector(".program-group-indicators > span:first-child .panel-led-lens")
            else {
                return;
            };
            let Ok(Some(green_led)) = hardware
                .query_selector(".program-group-indicators > span:last-child .panel-led-lens")
            else {
                return;
            };
            let Ok(Some(bank_one)) = hardware.query_selector(
                ".bank-selector .program-selector-position:first-child .program-selector-label",
            ) else {
                return;
            };

            let svg_rect = svg.get_bounding_client_rect();
            let red_rect = red_led.get_bounding_client_rect();
            let green_rect = green_led.get_bounding_client_rect();
            let bank_rect = bank_one.get_bounding_client_rect();
            if svg_rect.width() <= 0.0 || svg_rect.height() <= 0.0 {
                return;
            }

            let style = self.window.get_computed_style(&hardware).ok().flatten();
            let route_value = |name: &str, fallback: f64| {
                style
                    .as_ref()
                    .and_then(|style| style.get_property_value(name).ok())
                    .and_then(|value| value.trim().trim_end_matches("px").parse::<f64>().ok())
                    .unwrap_or(fallback)
            };
            let red_gap = route_value("--program-route-red-gap", 5.0);
            let green_gap = route_value("--program-route-green-gap", 5.0);
            let bank_gap = route_value("--program-route-bank-gap", 2.0);
            let red_lead = route_value("--program-route-red-lead", 12.0);
            let diagonal_run = route_value("--program-route-diagonal-run", 19.0);

            let red_x = red_rect.right() - svg_rect.left() + red_gap;
            let red_y = (red_rect.top() + red_rect.bottom()) * 0.5 - svg_rect.top();
            let green_x = green_rect.right() - svg_rect.left() + green_gap;
            let green_y = (green_rect.top() + green_rect.bottom()) * 0.5 - svg_rect.top();
            let bank_x = bank_rect.left() - svg_rect.left() - bank_gap;
            let red_corner_x = red_x + red_lead;
            let join_x = red_corner_x + diagonal_run;

            let _ = svg.set_attribute(
                "viewBox",
                &format!("0 0 {:.3} {:.3}", svg_rect.width(), svg_rect.height()),
            );
            let _ = path.set_attribute(
                "d",
                &format!(
                    "M {green_x:.3} {green_y:.3} H {bank_x:.3} M {join_x:.3} {green_y:.3} L {red_corner_x:.3} {red_y:.3} H {red_x:.3}"
                ),
            );
            let _ = svg.set_attribute("data-route-ready", "true");

            if let (
                Ok(Some(poly_svg)),
                Ok(Some(poly_path)),
                Ok(Some(poly_label)),
                Ok(Some(left_led)),
                Ok(Some(right_led)),
            ) = (
                hardware.query_selector(".poly-routing-line"),
                hardware.query_selector(".poly-routing-line path"),
                hardware.query_selector(".poly-control-group .parameter-head strong"),
                hardware.query_selector(
                    ".poly-button-row .panel-button-stack:first-child .panel-led-lens",
                ),
                hardware.query_selector(
                    ".poly-button-row .panel-button-stack:last-child .panel-led-lens",
                ),
            ) {
                let poly_rect = poly_svg.get_bounding_client_rect();
                let label_rect = poly_label.get_bounding_client_rect();
                let left_rect = left_led.get_bounding_client_rect();
                let right_rect = right_led.get_bounding_client_rect();
                if poly_rect.width() > 0.0 && poly_rect.height() > 0.0 {
                    let left_x = (left_rect.left() + left_rect.right()) * 0.5 - poly_rect.left();
                    let right_x = (right_rect.left() + right_rect.right()) * 0.5 - poly_rect.left();
                    let center_x = (left_x + right_x) * 0.5;
                    let start_y = label_rect.bottom() - poly_rect.top() + 4.0;
                    let led_y = ((left_rect.top() + left_rect.bottom())
                        + (right_rect.top() + right_rect.bottom()))
                        * 0.25
                        - poly_rect.top();
                    let _ = poly_svg.set_attribute(
                        "viewBox",
                        &format!("0 0 {:.3} {:.3}", poly_rect.width(), poly_rect.height()),
                    );
                    let _ = poly_path.set_attribute(
                        "d",
                        &format!(
                            "M {left_x:.3} {led_y:.3} H {right_x:.3} M {center_x:.3} {start_y:.3} V {led_y:.3}"
                        ),
                    );
                    let _ = poly_svg.set_attribute("data-route-ready", "true");
                }
            }

            if let (
                Ok(Some(midi_svg)),
                Ok(Some(above_path)),
                Ok(Some(below_path)),
                Ok(Some(continuation_path)),
                Ok(Some(midi_label)),
                Ok(Some(midi_button)),
                Ok(Some(bank_midi_one)),
                Ok(Some(bank_midi_eight)),
                Ok(Some(patch_midi_nine)),
            ) = (
                hardware.query_selector(".midi-channel-routing"),
                hardware.query_selector(".midi-channel-route-above"),
                hardware.query_selector(".midi-channel-route-below"),
                hardware.query_selector(".midi-channel-route-continuation"),
                hardware.query_selector(".control-midi-channel .parameter-head strong"),
                hardware.query_selector(".midi-channel-button"),
                hardware.query_selector(
                    ".bank-selector .program-selector-position:first-child .program-midi-number",
                ),
                hardware.query_selector(
                    ".bank-selector .program-selector-position:last-child .program-midi-number",
                ),
                hardware.query_selector(
                    ".patch-selector .program-selector-position:first-child .program-midi-number",
                ),
            ) {
                let midi_rect = midi_svg.get_bounding_client_rect();
                let label_rect = midi_label.get_bounding_client_rect();
                let button_rect = midi_button.get_bounding_client_rect();
                let bank_midi_rect = bank_midi_one.get_bounding_client_rect();
                let bank_eight_rect = bank_midi_eight.get_bounding_client_rect();
                let patch_nine_rect = patch_midi_nine.get_bounding_client_rect();
                if midi_rect.width() > 0.0 && midi_rect.height() > 0.0 {
                    let button_x =
                        (button_rect.left() + button_rect.right()) * 0.5 - midi_rect.left();
                    let label_bottom = label_rect.bottom() - midi_rect.top() + 7.0;
                    let button_top = button_rect.top() - midi_rect.top() - 3.0;
                    let button_bottom = button_rect.bottom() - midi_rect.top() + 3.0;
                    let bank_x = bank_midi_rect.left() - midi_rect.left() - 5.0;
                    let bank_y =
                        (bank_midi_rect.top() + bank_midi_rect.bottom()) * 0.5 - midi_rect.top();
                    let bank_eight_x = bank_eight_rect.right() - midi_rect.left() + 5.0;
                    let patch_nine_x = patch_nine_rect.left() - midi_rect.left() - 5.0;
                    let continuation_y = ((bank_eight_rect.top() + bank_eight_rect.bottom())
                        + (patch_nine_rect.top() + patch_nine_rect.bottom()))
                        * 0.25
                        - midi_rect.top();
                    let _ = midi_svg.set_attribute(
                        "viewBox",
                        &format!("0 0 {:.3} {:.3}", midi_rect.width(), midi_rect.height()),
                    );
                    let _ = above_path.set_attribute(
                        "d",
                        &format!("M {button_x:.3} {label_bottom:.3} V {button_top:.3}"),
                    );
                    let _ = below_path.set_attribute(
                        "d",
                        &format!("M {button_x:.3} {button_bottom:.3} V {bank_y:.3} H {bank_x:.3}"),
                    );
                    let _ = continuation_path.set_attribute(
                        "d",
                        &format!("M {bank_eight_x:.3} {continuation_y:.3} H {patch_nine_x:.3}"),
                    );
                    let _ = midi_svg.set_attribute("data-route-ready", "true");
                }
            }

            if let (
                Ok(Some(display_svg)),
                Ok(Some(bank_path)),
                Ok(Some(patch_path)),
                Ok(Some(bank_strip)),
                Ok(Some(patch_strip)),
                Ok(Some(bank_label)),
                Ok(Some(patch_label)),
            ) = (
                hardware.query_selector(".program-display-routing"),
                hardware.query_selector(".program-display-bank-route"),
                hardware.query_selector(".program-display-patch-route"),
                hardware.query_selector(".bank-selector > strong"),
                hardware.query_selector(".patch-selector > strong"),
                hardware.query_selector(".program-code-bank-label"),
                hardware.query_selector(".program-code-patch-label"),
            ) {
                let display_rect = display_svg.get_bounding_client_rect();
                let bank_strip_rect = bank_strip.get_bounding_client_rect();
                let patch_strip_rect = patch_strip.get_bounding_client_rect();
                let bank_label_rect = bank_label.get_bounding_client_rect();
                let patch_label_rect = patch_label.get_bounding_client_rect();
                if display_rect.width() > 0.0 && display_rect.height() > 0.0 {
                    let bank_start_x = bank_strip_rect.right() - display_rect.left();
                    let bank_start_y = (bank_strip_rect.top() + bank_strip_rect.bottom()) * 0.5
                        - display_rect.top();
                    let bank_end_x = (bank_label_rect.left() + bank_label_rect.right()) * 0.5
                        - display_rect.left();
                    let bank_end_y = bank_label_rect.top() - display_rect.top() - 3.0;
                    let patch_start_x = patch_strip_rect.left() - display_rect.left();
                    let patch_start_y = (patch_strip_rect.top() + patch_strip_rect.bottom()) * 0.5
                        - display_rect.top();
                    let patch_end_x = (patch_label_rect.left() + patch_label_rect.right()) * 0.5
                        - display_rect.left();
                    let patch_end_y = patch_label_rect.top() - display_rect.top() - 3.0;
                    let _ = display_svg.set_attribute(
                        "viewBox",
                        &format!(
                            "0 0 {:.3} {:.3}",
                            display_rect.width(),
                            display_rect.height()
                        ),
                    );
                    let _ = bank_path.set_attribute(
                        "d",
                        &format!(
                            "M {bank_start_x:.3} {bank_start_y:.3} H {bank_end_x:.3} V {bank_end_y:.3}"
                        ),
                    );
                    let _ = patch_path.set_attribute(
                        "d",
                        &format!(
                            "M {patch_start_x:.3} {patch_start_y:.3} H {patch_end_x:.3} V {patch_end_y:.3}"
                        ),
                    );
                    let _ = display_svg.set_attribute("data-route-ready", "true");
                }
            }
        }

        fn sync_performance_portamento_routing(&self) {
            let (
                Ok(Some(route_svg)),
                Ok(Some(route_path)),
                Ok(Some(portamento_knob)),
                Ok(Some(portamento_switch)),
            ) = (
                self.root.query_selector(".performance-portamento-routing"),
                self.root
                    .query_selector(".performance-portamento-routing path"),
                self.root.query_selector(".control-portamento .rf-knob-cap"),
                self.root
                    .query_selector(".control-portamento-switch .slide-switch-rail"),
            )
            else {
                return;
            };
            let route_rect = route_svg.get_bounding_client_rect();
            let knob_rect = portamento_knob.get_bounding_client_rect();
            let switch_rect = portamento_switch.get_bounding_client_rect();
            if route_rect.width() <= 0.0 || route_rect.height() <= 0.0 {
                return;
            }
            let start_x = switch_rect.right() - route_rect.left() + 4.0;
            let start_y = (switch_rect.top() + switch_rect.bottom()) * 0.5 - route_rect.top();
            let corner_x = start_x + 18.0;
            // Stop outside the graduated ring. The cap radius is 29 px and the
            // scale radius is 47 px; the extra gap avoids the white marks.
            let knob_x = knob_rect.right() - route_rect.left() + 24.0;
            let knob_center_y = (knob_rect.top() + knob_rect.bottom()) * 0.5;
            let knob_y = knob_center_y - route_rect.top();
            let clearance_y =
                if knob_center_y >= switch_rect.top() && knob_center_y <= switch_rect.bottom() {
                    switch_rect.top() - route_rect.top() - 8.0
                } else {
                    knob_y
                };
            let _ = route_svg.set_attribute(
                "viewBox",
                &format!("0 0 {:.3} {:.3}", route_rect.width(), route_rect.height()),
            );
            let _ = route_path.set_attribute(
                "d",
                &format!(
                    "M {start_x:.3} {start_y:.3} H {corner_x:.3} V {clearance_y:.3} H {knob_x:.3} V {knob_y:.3}"
                ),
            );
            let _ = route_svg.set_attribute("data-route-ready", "true");
        }

        fn render_hero(&self) -> String {
            format!(
                "<header class=\"synth-hero {MODEL_ID}\"><div class=\"hero-topline\"><div class=\"instrument-identity\"><strong class=\"instrument-name\">RF-106</strong><span class=\"instrument-identity-stripe\" aria-hidden=\"true\"></span><span class=\"voice-label\">PROGRAMMABLE POLYPHONIC SYNTHESIZER</span></div></div></header>"
            )
        }

        fn render_section_tabs(&self) -> String {
            let tabs = [
                ("lfo", "LFO", "MODULATION", "red"),
                ("dco", "DCO", "OSCILLATOR", "red"),
                ("hpf", "HPF", "HIGH PASS", "red"),
                ("filter", "VCF", "LOW PASS", "red"),
                ("vca", "VCA", "AMPLIFIER", "red"),
                ("envelope", "ENV", "ENVELOPE", "red"),
                ("chorus", "CHORUS", "STEREO", "red"),
                ("performance", "PLAY", "PERFORMANCE", "blue"),
                ("level", "OUTPUT", "LEVEL", "blue"),
            ];
            let mut html =
                String::from("<nav class=\"signal-path\" aria-label=\"RF-106 panel sections\">");
            for (id, label, detail, tone) in tabs {
                let active = self.active_section == id;
                html.push_str(&format!(
                    "<button class=\"signal-module {tone}{}\" type=\"button\" data-action=\"section\" data-section=\"{id}\" aria-pressed=\"{active}\"><strong>{label}</strong><small>{detail}</small></button>",
                    if active { " active" } else { "" }
                ));
            }
            html.push_str("</nav>");
            html
        }

        fn render_program_library(&self) -> String {
            let Some(context) = &self.context else {
                return String::new();
            };
            let query = self.search_query.trim().to_ascii_lowercase();
            let displayed_sound_id = self
                .pending_sound_id
                .as_deref()
                .unwrap_or(&context.instance.selected_sound_id);
            let mut patches = String::new();
            for sound in context
                .instance
                .sounds
                .iter()
                .filter(|sound| is_rf106_sound(sound))
                .filter(|sound| {
                    query.is_empty()
                        || sound.name.to_ascii_lowercase().contains(&query)
                        || patch_code(sound).to_ascii_lowercase().contains(&query)
                })
            {
                let selected = sound.id == displayed_sound_id;
                let pending = self.pending_sound_id.as_deref() == Some(sound.id.as_str());
                let class = match (selected, pending) {
                    (true, true) => " selected loading",
                    (true, false) => " selected",
                    _ => "",
                };
                let status = if pending {
                    "LOADING"
                } else if selected {
                    "PLAYING"
                } else {
                    MODEL_NAME
                };
                patches.push_str(&format!(
                    "<button class=\"patch-button{class}\" type=\"button\" data-action=\"sound\" data-sound-id=\"{}\" aria-pressed=\"{}\"><span class=\"patch-button-code\">{}</span><strong>{}</strong><small>{}</small></button>",
                    escape_html(&sound.id), selected, escape_html(&patch_code(sound)),
                    escape_html(clean_patch_name(sound)), escape_html(status)
                ));
            }
            if patches.is_empty() {
                patches.push_str("<p class=\"empty-state\">No patches match this search.</p>");
            }
            let mut html = format!(
                "<section class=\"patch-library\"><div class=\"library-toolbar\"><div><span class=\"section-kicker\">{} MEMORY</span><h2>Factory patches</h2><p>Eight banks, eight patches per row. Changes are heard immediately.</p></div><input class=\"patch-search\" type=\"search\" data-action=\"search\" placeholder=\"Search {} patches\" value=\"{}\" aria-label=\"Search patches\"></div><div class=\"patch-grid\">{patches}</div></section>",
                MODEL_NAME,
                MODEL_NAME,
                escape_html(&self.search_query)
            );
            if !self.bridge_error.is_empty() {
                html.push_str(&format!(
                    "<p class=\"bridge-error\">{}</p>",
                    escape_html(&self.bridge_error)
                ));
            }
            html
        }

        fn render_program_selector_button(
            &self,
            tone: &str,
            label: &str,
            midi_label: &str,
            target_code: &str,
            active: bool,
        ) -> String {
            let sound_id = self.context.as_ref().and_then(|context| {
                context
                    .instance
                    .sounds
                    .iter()
                    .find(|sound| patch_code(sound) == target_code)
                    .map(|sound| sound.id.as_str())
            });
            let Some(sound_id) = sound_id else {
                return String::new();
            };
            format!(
                "<div class=\"program-selector-position\"><span class=\"program-selector-label\">{}</span><button class=\"panel-button panel-button-{tone} program-selector-button\" type=\"button\" data-action=\"sound\" data-sound-id=\"{}\" aria-label=\"Program {target_code}\" aria-pressed=\"{active}\"></button><span class=\"program-midi-number\">{}</span></div>",
                escape_html(label),
                escape_html(sound_id),
                escape_html(midi_label),
            )
        }

        fn render_program_selector(&self) -> String {
            let code = self
                .selected_sound()
                .map(patch_code)
                .unwrap_or_else(|| "A11".to_owned());
            let mut characters = code.chars();
            let group = characters.next().unwrap_or('A');
            let bank = characters.next().unwrap_or('1');
            let patch = characters.next().unwrap_or('1');
            let render_digit = |digit: char, class_name: &str| {
                format!(
                    "<span class=\"seven-segment-digit {class_name}\" data-digit=\"{digit}\" aria-hidden=\"true\"><i class=\"segment a\"></i><i class=\"segment b\"></i><i class=\"segment c\"></i><i class=\"segment d\"></i><i class=\"segment e\"></i><i class=\"segment f\"></i><i class=\"segment g\"></i></span>"
                )
            };
            let display_digits = format!(
                "{}{}",
                render_digit(bank, "bank-digit"),
                render_digit(patch, "patch-digit")
            );

            let target_group = if group == 'A' { 'B' } else { 'A' };
            let target_group_code = format!("{target_group}{bank}{patch}");
            let target_group_sound_id = self.context.as_ref().and_then(|context| {
                context
                    .instance
                    .sounds
                    .iter()
                    .find(|sound| patch_code(sound) == target_group_code)
                    .map(|sound| sound.id.as_str())
            });
            let group_a_led = Self::render_panel_led_with_tone(
                group == 'A',
                if group == 'A' {
                    "Group A selected"
                } else {
                    "Group A not selected"
                },
                "red",
            );
            let group_b_led = Self::render_panel_led_with_tone(
                group == 'B',
                if group == 'B' {
                    "Group B selected"
                } else {
                    "Group B not selected"
                },
                "green",
            );
            let group_button = target_group_sound_id
                .map(|sound_id| {
                    format!(
                        "<button class=\"panel-button panel-button-white program-group-button\" type=\"button\" data-action=\"sound\" data-sound-id=\"{}\" aria-label=\"Switch to group {target_group}\"></button>",
                        escape_html(sound_id),
                    )
                })
                .unwrap_or_default();

            let mut banks = String::new();
            for candidate in '1'..='8' {
                banks.push_str(&self.render_program_selector_button(
                    "blue",
                    &candidate.to_string(),
                    &candidate.to_string(),
                    &format!("{group}{candidate}{patch}"),
                    candidate == bank,
                ));
            }

            let mut patches = String::new();
            for (offset, candidate) in ('1'..='8').enumerate() {
                patches.push_str(&self.render_program_selector_button(
                    "blue",
                    &candidate.to_string(),
                    &(offset + 9).to_string(),
                    &format!("{group}{bank}{candidate}"),
                    candidate == patch,
                ));
            }

            let panel_parameter = |id: &str| {
                self.snapshot.as_ref().and_then(|snapshot| {
                    snapshot
                        .schema
                        .parameters
                        .iter()
                        .find(|parameter| parameter.id == id)
                })
            };
            let key_transpose = panel_parameter("key-transpose");
            let allocation_mode = panel_parameter("allocation-mode");
            let midi_channel = panel_parameter("midi-channel");
            let program_controls = match (allocation_mode, midi_channel) {
                (Some(allocation_mode), Some(midi_channel)) => format!(
                    "<div class=\"program-front-controls\">{}{}</div>",
                    self.render_program_key_poly_controls(key_transpose, allocation_mode),
                    self.render_midi_channel_control(
                        midi_channel,
                        self.parameter_value(midi_channel.index, midi_channel.kind.default_value())
                    ),
                ),
                _ => String::new(),
            };

            format!(
                "<section class=\"program-hardware\">{program_controls}<div class=\"program-selector-block group-selector\"><strong>SELECT</strong><div class=\"program-group-indicators\"><span><strong>GROUP A</strong>{group_a_led}</span><span><strong>GROUP B</strong>{group_b_led}</span></div>{group_button}</div><svg class=\"program-group-routing\" preserveAspectRatio=\"none\" aria-hidden=\"true\"><path /></svg><svg class=\"midi-channel-routing\" preserveAspectRatio=\"none\" aria-hidden=\"true\"><path class=\"midi-channel-route-above\" /><path class=\"midi-channel-route-below\" /><path class=\"midi-channel-route-continuation\" /></svg><svg class=\"program-display-routing\" preserveAspectRatio=\"none\" aria-hidden=\"true\"><path class=\"program-display-bank-route\" /><path class=\"program-display-patch-route\" /></svg><div class=\"program-selector-block bank-selector\"><strong>BANK</strong><div>{banks}</div></div><div class=\"program-code-block\"><div class=\"program-code-labels\"><strong class=\"program-code-bank-label\">BANK</strong><strong class=\"program-code-patch-label\"><span>PATCH</span><span>NUMBER</span></strong></div><div class=\"program-code-display\" aria-label=\"Current program {code}\">{display_digits}</div></div><div class=\"program-selector-block patch-selector\"><strong>PATCH NUMBER</strong><div>{patches}</div></div></section>"
            )
        }

        fn render_config_body(&self) -> String {
            if self.snapshot.is_some() {
                self.render_control_workspace()
            } else if self.bridge_error.is_empty() {
                "<div class=\"panel-loading\"><p class=\"boot-message\">Reading the front-panel state…</p></div>".to_owned()
            } else {
                format!(
                    "<div class=\"panel-loading\"><p class=\"bridge-error\">{}</p><button class=\"panel-retry\" type=\"button\" data-action=\"retry-parameters\">RETRY PANEL</button></div>",
                    escape_html(&self.bridge_error)
                )
            }
        }

        fn render_control_workspace(&self) -> String {
            let Some(snapshot) = &self.snapshot else {
                return String::new();
            };
            let schema_page_id = self.active_section.as_str();
            let Some(page) = snapshot
                .schema
                .pages
                .iter()
                .find(|page| page.id == schema_page_id)
            else {
                return "<p class=\"boot-message\">This panel section is unavailable.</p>"
                    .to_owned();
            };
            let panel = format!(
                "<div class=\"parameter-modules section-single\">{}</div>",
                self.render_parameter_module(page)
            );
            let error = if self.bridge_error.is_empty() {
                String::new()
            } else {
                format!(
                    "<p class=\"bridge-error\">{}</p>",
                    escape_html(&self.bridge_error)
                )
            };
            format!(
                "<section class=\"config-workspace section-workspace\">{panel}{error}</section>",
            )
        }

        fn render_parameter_module(&self, page: &Page) -> String {
            let visual_section_id = page.id.as_str();
            let tone = if matches!(visual_section_id, "performance" | "level") {
                "blue"
            } else {
                "red"
            };
            let (short, full) = match visual_section_id {
                "level" => ("OUT".to_owned(), "RACKFORGE OUTPUT".to_owned()),
                "lfo" => ("LFO".to_owned(), "LOW FREQUENCY OSCILLATOR".to_owned()),
                "dco" => (
                    "DCO".to_owned(),
                    "DIGITALLY CONTROLLED OSCILLATOR".to_owned(),
                ),
                "hpf" => ("HPF".to_owned(), "HIGH PASS FILTER".to_owned()),
                "filter" => ("VCF".to_owned(), "VOLTAGE CONTROLLED FILTER".to_owned()),
                "vca" => ("VCA".to_owned(), "VOLTAGE CONTROLLED AMPLIFIER".to_owned()),
                "envelope" => ("ENV-1".to_owned(), "AMPLIFIER ENVELOPE".to_owned()),
                "chorus" => ("CHORUS".to_owned(), "MN3009 STEREO CHORUS".to_owned()),
                "performance" => ("PLAY".to_owned(), "PERFORMANCE CONTROLS".to_owned()),
                value => (value.to_ascii_uppercase(), "LIVE CONTROL".to_owned()),
            };
            let mut controls = String::new();
            let mut parameters = self
                .snapshot
                .as_ref()
                .map(|snapshot| {
                    snapshot
                        .schema
                        .parameters
                        .iter()
                        .filter(|parameter| {
                            parameter.page == page.id
                                || (visual_section_id == "performance"
                                    && parameter.id == "physical-volume")
                        })
                        .filter(|parameter| match visual_section_id {
                            "performance" => !matches!(
                                parameter.id.as_str(),
                                "key-transpose"
                                    | "allocation-mode"
                                    | "midi-channel"
                                    | "midi-function"
                                    | "tuning"
                            ),
                            "level" => parameter.id != "physical-volume",
                            _ => true,
                        })
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            parameters.sort_by_key(|parameter| match parameter.id.as_str() {
                "vcf-cutoff" => 0,
                "vcf-resonance" => 1,
                "vcf-envelope-polarity" => 2,
                "vcf-envelope" => 3,
                "vcf-lfo" => 4,
                "vcf-keyboard" => 5,
                _ => parameter.order,
            });
            let find_parameter = |id: &str| {
                parameters
                    .iter()
                    .copied()
                    .find(|parameter| parameter.id == id)
            };
            let pulse = find_parameter("dco-pulse");
            let saw = find_parameter("dco-saw");
            let dco_lfo = find_parameter("dco-lfo");
            let dco_pwm = find_parameter("dco-pwm");
            let pwm_mode = find_parameter("pwm-mode");
            let vcf_polarity = find_parameter("vcf-envelope-polarity");
            let vcf_envelope = find_parameter("vcf-envelope");
            let vcf_lfo = find_parameter("vcf-lfo");
            let vcf_keyboard = find_parameter("vcf-keyboard");
            let vca_mode = find_parameter("vca-mode");
            let vca_level = find_parameter("vca-level");
            let lfo_trigger = find_parameter("lfo-trigger");
            let bender_position = find_parameter("bender-position");
            let has_dco_core = dco_lfo.is_some()
                && dco_pwm.is_some()
                && pwm_mode.is_some()
                && pulse.is_some()
                && saw.is_some();
            let has_vcf_mod = vcf_polarity.is_some()
                && vcf_envelope.is_some()
                && vcf_lfo.is_some()
                && vcf_keyboard.is_some();
            let has_vca_core = vca_mode.is_some() && vca_level.is_some();
            for parameter in parameters.iter().copied() {
                if parameter.id == "dco-lfo"
                    && let (Some(dco_lfo), Some(dco_pwm), Some(pwm_mode), Some(pulse), Some(saw)) =
                        (dco_lfo, dco_pwm, pwm_mode, pulse, saw)
                {
                    let faders = [dco_lfo, dco_pwm];
                    controls.push_str(&format!(
                        "<div class=\"joined-control-section joined-dco-core\">{}{}{}</div>",
                        self.render_shared_fader_group("shared-dco-mod", &faders),
                        self.render_parameter_control(pwm_mode),
                        self.render_waveform_controls(pulse, saw),
                    ));
                    continue;
                }
                if has_dco_core
                    && matches!(
                        parameter.id.as_str(),
                        "dco-pwm" | "pwm-mode" | "dco-pulse" | "dco-saw"
                    )
                {
                    continue;
                }
                if parameter.id == "vcf-envelope-polarity"
                    && let (
                        Some(vcf_polarity),
                        Some(vcf_envelope),
                        Some(vcf_lfo),
                        Some(vcf_keyboard),
                    ) = (vcf_polarity, vcf_envelope, vcf_lfo, vcf_keyboard)
                {
                    let faders = [vcf_envelope, vcf_lfo, vcf_keyboard];
                    controls.push_str(&format!(
                        "<div class=\"joined-control-section joined-vcf-mod\">{}{}</div>",
                        self.render_parameter_control(vcf_polarity),
                        self.render_shared_fader_group("shared-vcf-mod", &faders),
                    ));
                    continue;
                }
                if has_vcf_mod
                    && matches!(
                        parameter.id.as_str(),
                        "vcf-envelope" | "vcf-lfo" | "vcf-keyboard"
                    )
                {
                    continue;
                }
                if parameter.id == "vca-mode"
                    && let (Some(vca_mode), Some(vca_level)) = (vca_mode, vca_level)
                {
                    controls.push_str(&format!(
                        "<div class=\"joined-control-section joined-vca-core\">{}{}</div>",
                        self.render_parameter_control(vca_mode),
                        self.render_parameter_control(vca_level),
                    ));
                    continue;
                }
                if has_vca_core && parameter.id == "vca-level" {
                    continue;
                }
                let shared_fader_group: Option<(&str, &[&str])> = match parameter.id.as_str() {
                    "lfo-rate" => Some(("shared-lfo", &["lfo-rate", "lfo-delay"])),
                    "dco-lfo" => Some(("shared-dco-mod", &["dco-lfo", "dco-pwm"])),
                    "dco-sub" => Some(("shared-dco-mix", &["dco-sub", "dco-noise"])),
                    "vcf-cutoff" => Some(("shared-vcf-main", &["vcf-cutoff", "vcf-resonance"])),
                    "vcf-envelope" => Some((
                        "shared-vcf-mod",
                        &["vcf-envelope", "vcf-lfo", "vcf-keyboard"],
                    )),
                    "env-attack" => Some((
                        "shared-envelope",
                        &["env-attack", "env-decay", "env-sustain", "env-release"],
                    )),
                    "bender-dco-sensitivity" => Some((
                        "shared-performance-bender",
                        &[
                            "bender-dco-sensitivity",
                            "bender-vcf-sensitivity",
                            "lfo-trigger-sensitivity",
                        ],
                    )),
                    _ => None,
                };
                if let Some((class_name, ids)) = shared_fader_group {
                    let group = ids
                        .iter()
                        .filter_map(|id| {
                            parameters
                                .iter()
                                .copied()
                                .find(|candidate| candidate.id == *id)
                        })
                        .collect::<Vec<_>>();
                    if group.len() == ids.len() {
                        controls.push_str(&self.render_shared_fader_group(class_name, &group));
                        continue;
                    }
                }
                if matches!(
                    parameter.id.as_str(),
                    "lfo-delay"
                        | "dco-pwm"
                        | "dco-noise"
                        | "vcf-resonance"
                        | "vcf-lfo"
                        | "vcf-keyboard"
                        | "env-decay"
                        | "env-sustain"
                        | "env-release"
                        | "bender-vcf-sensitivity"
                        | "lfo-trigger-sensitivity"
                ) {
                    continue;
                }
                if parameter.id == "dco-pulse"
                    && let (Some(pulse), Some(saw)) = (pulse, saw)
                {
                    controls.push_str(&self.render_waveform_controls(pulse, saw));
                    continue;
                }
                if parameter.id == "dco-saw" && pulse.is_some() && saw.is_some() {
                    continue;
                }
                if parameter.id == "lfo-trigger" && visual_section_id == "performance" {
                    continue;
                }
                if parameter.id == "bender-position"
                    && visual_section_id == "performance"
                    && let (Some(position), Some(trigger)) = (bender_position, lfo_trigger)
                {
                    controls.push_str(&self.render_bender_hardware(position, trigger));
                    continue;
                }
                controls.push_str(&self.render_parameter_control(parameter));
            }
            if visual_section_id == "performance" {
                controls.push_str(
                    "<svg class=\"performance-portamento-routing\" preserveAspectRatio=\"none\" aria-hidden=\"true\"><path /></svg>",
                );
            }
            let body = if page.id == "envelope" {
                format!(
                    "<div class=\"envelope-workbench\">{}<div class=\"module-controls\">{controls}</div></div>",
                    self.render_envelope(),
                )
            } else {
                format!("<div class=\"module-controls\">{controls}</div>")
            };
            format!(
                "<section class=\"parameter-module page-{}{}\"><div class=\"module-heading {tone}\"><strong>{short}</strong><small>{full}</small></div>{body}</section>",
                visual_section_id,
                if page.id == "envelope" {
                    " envelope-parameters"
                } else {
                    ""
                }
            )
        }

        fn render_panel_led(active: bool, label: &str) -> String {
            Self::render_panel_led_with_tone(active, label, "red")
        }

        fn render_panel_led_with_tone(active: bool, label: &str, tone: &str) -> String {
            format!(
                "<span class=\"panel-led panel-led-{tone}{}\" role=\"img\" aria-label=\"{}\"><span class=\"panel-led-lens\" aria-hidden=\"true\"></span></span>",
                if active { " active" } else { "" },
                escape_html(label),
            )
        }

        fn uses_shared_fader_scale(parameter_id: &str) -> bool {
            matches!(
                parameter_id,
                "lfo-rate"
                    | "lfo-delay"
                    | "dco-lfo"
                    | "dco-pwm"
                    | "dco-sub"
                    | "dco-noise"
                    | "vcf-cutoff"
                    | "vcf-resonance"
                    | "vcf-envelope"
                    | "vcf-lfo"
                    | "vcf-keyboard"
                    | "env-attack"
                    | "env-decay"
                    | "env-sustain"
                    | "env-release"
                    | "bender-dco-sensitivity"
                    | "bender-vcf-sensitivity"
                    | "lfo-trigger-sensitivity"
            )
        }

        fn render_fader_ticks(tick_count: usize, major_ticks: bool, line_end: u32) -> String {
            let mut ticks = String::new();
            for position in 0..tick_count {
                let y = 8.0 + (position as f64 * 140.0 / (tick_count - 1) as f64);
                let major = major_ticks
                    && (position == 0
                        || position == (tick_count - 1) / 2
                        || position == tick_count - 1);
                ticks.push_str(&format!(
                    "<line class=\"rf-fader-tick{}\" x1=\"0\" y1=\"{y:.1}\" x2=\"{line_end}\" y2=\"{y:.1}\"></line>",
                    if major { " major" } else { "" },
                ));
            }
            ticks
        }

        fn render_hpf_fader_labels() -> String {
            let mut labels = String::new();
            for position in 0..4 {
                let y = 8.0 + (position as f64 * 140.0 / 3.0);
                let value = 3 - position;
                labels.push_str(&format!(
                    "<text class=\"hpf-fader-number\" x=\"-4\" y=\"{y:.1}\" text-anchor=\"end\">{value}</text><text class=\"hpf-fader-number\" x=\"52\" y=\"{y:.1}\" text-anchor=\"start\">{value}</text>"
                ));
            }
            labels
        }

        fn render_shared_fader_group(&self, class_name: &str, parameters: &[&Parameter]) -> String {
            let controls = parameters
                .iter()
                .map(|parameter| self.render_parameter_control(parameter))
                .collect::<String>();
            let ticks = Self::render_fader_ticks(11, true, 100);
            format!(
                "<div class=\"shared-fader-group {class_name}\" style=\"--shared-fader-count: {}\"><div class=\"shared-fader-cluster\"><svg class=\"shared-fader-graduations\" viewBox=\"0 0 100 156\" preserveAspectRatio=\"none\" aria-hidden=\"true\">{ticks}</svg><span class=\"shared-fader-number left top\" aria-hidden=\"true\">10</span><span class=\"shared-fader-number left middle\" aria-hidden=\"true\">5</span><span class=\"shared-fader-number left bottom\" aria-hidden=\"true\">0</span><span class=\"shared-fader-number right top\" aria-hidden=\"true\">10</span><span class=\"shared-fader-number right middle\" aria-hidden=\"true\">5</span><span class=\"shared-fader-number right bottom\" aria-hidden=\"true\">0</span>{controls}</div></div>",
                parameters.len()
            )
        }

        fn render_panel_button(
            tone: &str,
            extra_class: &str,
            action: &str,
            index: u32,
            value: Option<f64>,
            label: &str,
            pressed: bool,
            content: &str,
        ) -> String {
            let value_attribute = value
                .map(|value| format!(" data-value=\"{value}\""))
                .unwrap_or_default();
            format!(
                "<button class=\"panel-button panel-button-{tone}{extra_class}\" type=\"button\" data-action=\"{action}\" data-index=\"{index}\" data-rackforge-parameter-index=\"{index}\"{value_attribute} aria-label=\"{}\" aria-pressed=\"{pressed}\">{content}</button>",
                escape_html(label),
            )
        }

        fn render_fader(&self, parameter: &Parameter, value: f64) -> String {
            let (minimum, maximum, step) = if parameter.id == "hpf-frequency" {
                (0.0, 3.0, 1.0)
            } else {
                (
                    parameter.kind.minimum.unwrap_or(0.0),
                    parameter.kind.maximum.unwrap_or(1.0),
                    parameter.kind.step.unwrap_or(0.01),
                )
            };
            let scale_class = if parameter.id == "hpf-frequency" {
                " hpf-scale"
            } else {
                ""
            };
            let numbers = if Self::uses_shared_fader_scale(&parameter.id) {
                ""
            } else {
                match parameter.id.as_str() {
                    "hpf-frequency" => "",
                    "vca-level" => {
                        "<span class=\"rf-fader-number top\" aria-hidden=\"true\">+5</span><span class=\"rf-fader-number middle\" aria-hidden=\"true\">0</span><span class=\"rf-fader-number bottom\" aria-hidden=\"true\">-5</span>"
                    }
                    _ => {
                        "<span class=\"rf-fader-number top\" aria-hidden=\"true\">10</span><span class=\"rf-fader-number middle\" aria-hidden=\"true\">5</span><span class=\"rf-fader-number bottom\" aria-hidden=\"true\">0</span>"
                    }
                }
            };
            let ticks = if Self::uses_shared_fader_scale(&parameter.id) {
                String::new()
            } else if parameter.id == "hpf-frequency" {
                format!(
                    "{}{}",
                    Self::render_fader_ticks(4, false, 48),
                    Self::render_hpf_fader_labels()
                )
            } else {
                Self::render_fader_ticks(11, true, 48)
            };
            let gradient_id = format!("fader-channel-{}", parameter.index);
            let scale = format!(
                "<svg class=\"rf-fader-scale\" viewBox=\"0 0 48 156\" preserveAspectRatio=\"none\" aria-hidden=\"true\"><defs><linearGradient id=\"{gradient_id}\" x1=\"0\" y1=\"0\" x2=\"1\" y2=\"0\"><stop offset=\"0\" stop-color=\"#040405\"></stop><stop offset=\"0.5\" stop-color=\"#202124\"></stop><stop offset=\"1\" stop-color=\"#050506\"></stop></linearGradient></defs><g>{ticks}</g><rect class=\"rf-fader-channel\" x=\"20\" y=\"0.5\" width=\"10\" height=\"155\" rx=\"5\" ry=\"3\" fill=\"url(#{gradient_id})\"></rect></svg>"
            );
            format!(
                "<div class=\"rf-fader{scale_class}\">{scale}{numbers}<input class=\"parameter-slider\" type=\"range\" data-action=\"parameter\" data-index=\"{}\" data-rackforge-parameter-index=\"{}\" min=\"{}\" max=\"{}\" step=\"{}\" value=\"{}\" aria-label=\"{}\"></div>",
                parameter.index,
                parameter.index,
                minimum,
                maximum,
                step,
                value,
                escape_html(&parameter.name)
            )
        }

        fn render_knob(&self, parameter: &Parameter, value: f64) -> String {
            let minimum = parameter.kind.minimum.unwrap_or(0.0);
            let maximum = parameter.kind.maximum.unwrap_or(1.0);
            let normalized = if maximum > minimum {
                ((value - minimum) / (maximum - minimum)).clamp(0.0, 1.0)
            } else {
                0.0
            };
            let angle = -135.0 + normalized * 270.0;
            let scale_ticks = (-135..=135)
                .step_by(27)
                .enumerate()
                .map(|(index, tick_angle)| {
                    let class_name = if matches!(index, 0 | 5 | 10) {
                        "rf-knob-tick major"
                    } else {
                        "rf-knob-tick"
                    };
                    format!(
                        "<line class=\"{class_name}\" x1=\"47\" y1=\"1\" x2=\"47\" y2=\"9\" transform=\"rotate({tick_angle} 47 47)\"></line>"
                    )
                })
                .collect::<String>();
            format!(
                "<div class=\"rf-knob\" data-knob-index=\"{}\" style=\"--knob-turn: {angle}deg\"><svg class=\"rf-knob-scale\" viewBox=\"0 0 94 94\" aria-hidden=\"true\">{scale_ticks}</svg><span class=\"rf-knob-number zero\" aria-hidden=\"true\">0</span><span class=\"rf-knob-number ten\" aria-hidden=\"true\">10</span><span class=\"rf-knob-cap\" aria-hidden=\"true\"><span class=\"rf-knob-marker\"></span></span><input class=\"parameter-knob-input\" type=\"range\" data-action=\"parameter\" data-index=\"{}\" data-rackforge-parameter-index=\"{}\" min=\"{minimum}\" max=\"{maximum}\" step=\"{}\" value=\"{value}\" aria-label=\"{}\"></div>",
                parameter.index,
                parameter.index,
                parameter.index,
                parameter.kind.step.unwrap_or(0.01),
                escape_html(&parameter.name),
            )
        }

        fn render_two_position_switch(
            &self,
            parameter: &Parameter,
            value: f64,
            top: &str,
            bottom: &str,
            top_value: f64,
            bottom_value: f64,
        ) -> String {
            let bottom_active = (value - bottom_value).abs() < f64::EPSILON;
            let hardware_class = if parameter.id == "portamento-switch" {
                "portamento-lever-switch"
            } else {
                ""
            };
            let handle = if parameter.id == "portamento-switch" {
                r##"<span class="slide-switch-handle"><svg class="portamento-lever-svg" viewBox="0 0 30 36" aria-hidden="true" focusable="false"><defs><linearGradient id="portamento-lever-body" x1="0%" y1="0%" x2="100%" y2="0%"><stop offset="0%" stop-color="#202128"></stop><stop offset="32%" stop-color="#18191f"></stop><stop offset="68%" stop-color="#0f1014"></stop><stop offset="100%" stop-color="#18191f"></stop></linearGradient><linearGradient id="portamento-lever-side" x1="0%" y1="0%" x2="100%" y2="0%"><stop offset="0%" stop-color="#17181d"></stop><stop offset="100%" stop-color="#090a0d"></stop></linearGradient><linearGradient id="portamento-lever-tip-top" x1="0%" y1="0%" x2="100%" y2="62%"><stop offset="0%" stop-color="#3a3b44"></stop><stop offset="48%" stop-color="#282930"></stop><stop offset="100%" stop-color="#15161b"></stop></linearGradient><linearGradient id="portamento-lever-tip-bottom" x1="0%" y1="0%" x2="100%" y2="62%"><stop offset="0%" stop-color="#303139"></stop><stop offset="52%" stop-color="#202128"></stop><stop offset="100%" stop-color="#0f1014"></stop></linearGradient></defs><g class="portamento-lever-geometry top"><path class="portamento-lever-body" d="M4 18H24L27 18V33L24 35.5H4Z" fill="url(#portamento-lever-body)"></path><path class="portamento-lever-side" d="M24 18H27V33L24 35.5Z" fill="url(#portamento-lever-side)"></path><path class="portamento-lever-tip" d="M1.5 .5H28.5V18.5H1.5Z" fill="url(#portamento-lever-tip-top)"></path><path class="portamento-lever-outline" d="M1.5 .5H28.5V18H27V33L24 35.5H4V18H1.5Z"></path></g><g class="portamento-lever-geometry bottom"><path class="portamento-lever-body" d="M4 .5H24L27 3V18H4Z" fill="url(#portamento-lever-body)"></path><path class="portamento-lever-side" d="M24 .5L27 3V18H24Z" fill="url(#portamento-lever-side)"></path><path class="portamento-lever-tip" d="M1.5 17.5H28.5V35.5H1.5Z" fill="url(#portamento-lever-tip-bottom)"></path><path class="portamento-lever-outline" d="M4 .5H24L27 3V18H28.5V35.5H1.5V18H4Z"></path></g></svg></span>"##
            } else {
                r#"<span class="slide-switch-handle"></span>"#
            };
            format!(
                "<button class=\"parameter-slide-switch {hardware_class} {}\" type=\"button\" data-action=\"choice\" data-index=\"{}\" data-rackforge-parameter-index=\"{}\" data-value=\"{}\" aria-label=\"{}\"><span class=\"slide-switch-label top\">{top}</span><span class=\"slide-switch-rail\" aria-hidden=\"true\">{handle}</span><span class=\"slide-switch-label bottom\">{bottom}</span></button>",
                if bottom_active {
                    "bottom-position"
                } else {
                    "top-position"
                },
                parameter.index,
                parameter.index,
                if bottom_active {
                    top_value
                } else {
                    bottom_value
                },
                escape_html(&parameter.name),
            )
        }

        fn render_chorus_control(&self, parameter: &Parameter, value: f64) -> String {
            let mut buttons = String::new();
            for (button_value, label, tone) in [
                (0.0, "OFF", "white"),
                (1.0, "I", "amber"),
                (2.0, "II", "amber"),
            ] {
                let active = (button_value - value.round()).abs() < f64::EPSILON;
                let indicator = if button_value == 0.0 {
                    "<span class=\"chorus-off-line\" aria-hidden=\"true\"></span>".to_owned()
                } else {
                    Self::render_panel_led(
                        active,
                        &format!("Chorus {label} LED {}", if active { "on" } else { "off" }),
                    )
                };
                let button = Self::render_panel_button(
                    tone,
                    " chorus-button",
                    "choice",
                    parameter.index,
                    Some(button_value),
                    &format!("Chorus {label}"),
                    active,
                    "",
                );
                buttons.push_str(&format!(
                    "<div class=\"panel-button-stack\"><span class=\"panel-button-label\">{label}</span>{indicator}{button}</div>"
                ));
            }
            format!(
                "<div class=\"parameter-control control-chorus-mode chorus-control-group\"><output class=\"visually-hidden\" data-output-index=\"{}\">{}</output>{buttons}</div>",
                parameter.index,
                escape_html(&value_label(parameter, value)),
            )
        }

        fn render_poly_control(&self, parameter: &Parameter, value: f64) -> String {
            let selected = value.round() as i32;
            let mut buttons = String::new();
            for position in [1, 2] {
                let active = selected == 0 || selected == position;
                let led = Self::render_panel_led(
                    active,
                    &format!("Poly {position} LED {}", if active { "on" } else { "off" }),
                );
                buttons.push_str(&format!(
                    "<div class=\"panel-button-stack compact\"><span class=\"panel-button-label\">{position}</span>{led}<button class=\"panel-button panel-button-white poly-button\" type=\"button\" data-action=\"allocation-toggle\" data-index=\"{}\" data-rackforge-parameter-index=\"{}\" data-position=\"{position}\" aria-label=\"Poly {position}\" aria-pressed=\"{active}\"></button></div>",
                    parameter.index,
                    parameter.index,
                ));
            }
            format!(
                "<div class=\"parameter-control control-allocation-mode poly-control-group\"><div class=\"parameter-head\"><strong>POLY</strong><output class=\"parameter-readout\" data-output-index=\"{}\">{}</output></div><svg class=\"poly-routing-line\" viewBox=\"0 0 128 160\" preserveAspectRatio=\"none\" aria-hidden=\"true\"><path d=\"M36 158H92M64 2V158\" /></svg><div class=\"poly-button-row\">{buttons}</div></div>",
                parameter.index,
                escape_html(&value_label(parameter, value)),
            )
        }

        fn render_program_key_poly_controls(
            &self,
            key_transpose: Option<&Parameter>,
            allocation_mode: &Parameter,
        ) -> String {
            let indicator = key_transpose
                .map(|parameter| {
                    self.parameter_value(parameter.index, parameter.kind.default_value()) >= 0.5
                })
                .unwrap_or(false);
            let led = Self::render_panel_led(
                indicator,
                if indicator {
                    "Key Transpose LED on"
                } else {
                    "Key Transpose LED off"
                },
            );
            let key_button = Self::render_panel_button(
                "amber",
                " key-transpose-button",
                "trigger",
                PUBLIC_KEY_TRANSPOSE_TRIGGER_INDEX,
                None,
                "Hold Key Transpose and play a key",
                false,
                "",
            );
            let allocation_value =
                self.parameter_value(allocation_mode.index, allocation_mode.kind.default_value());
            format!(
                "<div class=\"midi-key-poly-hardware\"><div class=\"parameter-control key-transpose-control\"><div class=\"parameter-head\"><strong><span>KEY</span><span>TRANSPOSE</span></strong></div>{led}{key_button}</div>{}</div>",
                self.render_poly_control(allocation_mode, allocation_value),
            )
        }

        fn render_midi_channel_control(&self, parameter: &Parameter, value: f64) -> String {
            let channel = value.round().clamp(1.0, 16.0);
            let next = if channel >= 16.0 { 1.0 } else { channel + 1.0 };
            let button = Self::render_panel_button(
                "blue",
                " midi-channel-button",
                "choice",
                parameter.index,
                Some(next),
                "Advance MIDI channel",
                false,
                "",
            );
            format!(
                "<div class=\"parameter-control control-midi-channel\"><div class=\"parameter-head\"><strong>MIDI CH</strong><output class=\"parameter-readout\" data-output-index=\"{}\">{}</output></div>{button}</div>",
                parameter.index,
                escape_html(&value_label(parameter, value)),
            )
        }

        fn render_waveform_controls(&self, pulse: &Parameter, saw: &Parameter) -> String {
            let pulse_active = self.parameter_value(pulse.index, pulse.kind.default_value()) >= 0.5;
            let saw_active = self.parameter_value(saw.index, saw.kind.default_value()) >= 0.5;
            let pulse_led = Self::render_panel_led(
                pulse_active,
                if pulse_active {
                    "Pulse LED on"
                } else {
                    "Pulse LED off"
                },
            );
            let saw_led = Self::render_panel_led(
                saw_active,
                if saw_active {
                    "Saw LED on"
                } else {
                    "Saw LED off"
                },
            );
            format!(
                "<div class=\"parameter-control waveform-control-pair\"><div class=\"waveform-control\"><span class=\"waveform-panel-symbol\" aria-hidden=\"true\"><svg class=\"waveform-symbol pulse-waveform-symbol\" viewBox=\"0 0 42 22\"><path d=\"M2 20V2H20V20H40V2\"></path><rect x=\"24\" y=\"1\" width=\"2.5\" height=\"2.5\"></rect><rect x=\"29\" y=\"1\" width=\"2.5\" height=\"2.5\"></rect><rect x=\"29\" y=\"7.5\" width=\"2.5\" height=\"2.5\"></rect><rect x=\"29\" y=\"14\" width=\"2.5\" height=\"2.5\"></rect></svg></span><output class=\"visually-hidden\" data-output-index=\"{}\">{}</output>{pulse_led}<button class=\"panel-button panel-button-white waveform-button\" type=\"button\" data-action=\"toggle\" data-index=\"{}\" data-rackforge-parameter-index=\"{}\" aria-label=\"{}\" aria-pressed=\"{}\"></button></div><div class=\"waveform-control\"><span class=\"waveform-panel-symbol\" aria-hidden=\"true\"><svg class=\"waveform-symbol\" viewBox=\"0 0 42 22\"><path d=\"M5 18L35 4V18\"></path></svg></span><output class=\"visually-hidden\" data-output-index=\"{}\">{}</output>{saw_led}<button class=\"panel-button panel-button-white waveform-button\" type=\"button\" data-action=\"toggle\" data-index=\"{}\" data-rackforge-parameter-index=\"{}\" aria-label=\"{}\" aria-pressed=\"{}\"></button></div></div>",
                pulse.index,
                pulse.index,
                if pulse_active { "ON" } else { "OFF" },
                pulse.index,
                escape_html(&pulse.name),
                pulse_active,
                saw.index,
                saw.index,
                if saw_active { "ON" } else { "OFF" },
                saw.index,
                escape_html(&saw.name),
                saw_active,
            )
        }

        fn render_bender_hardware(&self, position: &Parameter, trigger: &Parameter) -> String {
            let position_value = self
                .parameter_value(position.index, position.kind.default_value())
                .clamp(-1.0, 1.0);
            let trigger_active =
                self.parameter_value(trigger.index, trigger.kind.default_value()) >= 0.5;
            let left = (position_value + 1.0) * 50.0;
            format!(
                "<div class=\"bender-hardware\"><div class=\"bender-panel-labels\"><strong>BENDER</strong><span>LEFT</span><span>LFO TRIG</span><span>RIGHT</span></div><div class=\"bender-motion-field\" data-action=\"bender\" data-index=\"{}\" data-trigger-index=\"{}\" data-rackforge-parameter-index=\"{}\" role=\"slider\" tabindex=\"0\" aria-label=\"Bender: left and right bend pitch, push upward triggers LFO\" aria-valuemin=\"-1\" aria-valuemax=\"1\" aria-valuenow=\"{position_value}\" style=\"--bender-left: {left:.3}%; --bender-push: {}px\"><svg class=\"bender-gate\" viewBox=\"0 0 300 82\" preserveAspectRatio=\"none\" aria-hidden=\"true\"><defs><linearGradient id=\"bender-gate-depth\" x1=\"0\" y1=\"0\" x2=\"0\" y2=\"1\"><stop offset=\"0\" stop-color=\"#020203\"/><stop offset=\"0.48\" stop-color=\"#15161a\"/><stop offset=\"1\" stop-color=\"#030304\"/></linearGradient></defs><path d=\"M20 12H280V70H20Z\" fill=\"url(#bender-gate-depth)\"/><path d=\"M20 41H280 M150 12V70\"/></svg><span class=\"bender-lever{}\" aria-hidden=\"true\"><span class=\"bender-lever-stem\"></span><span class=\"bender-lever-cap\"></span></span></div><output class=\"visually-hidden\" data-output-index=\"{}\">{position_value:.3}</output></div>",
                position.index,
                trigger.index,
                position.index,
                if trigger_active { -18 } else { 0 },
                if trigger_active { " lfo-active" } else { "" },
                position.index,
            )
        }

        fn render_parameter_control(&self, parameter: &Parameter) -> String {
            let value = self.parameter_value(parameter.index, parameter.kind.default_value());
            let label = value_label(parameter, value);
            let control = match parameter.kind.kind.as_str() {
                "trigger" => Self::render_panel_button(
                    if parameter.id == "lfo-trigger" {
                        "white"
                    } else {
                        "amber"
                    },
                    if parameter.id == "lfo-trigger" {
                        " parameter-trigger lfo-trigger-point"
                    } else {
                        " parameter-trigger"
                    },
                    "trigger",
                    parameter.index,
                    None,
                    &parameter.name,
                    false,
                    "",
                ),
                "boolean" => {
                    if parameter.id == "portamento-switch" {
                        self.render_two_position_switch(parameter, value, "OFF", "ON", 0.0, 1.0)
                    } else if parameter.id == "power" {
                        let active = value >= 0.5;
                        format!(
                            "<button class=\"power-rocker-switch {}\" type=\"button\" data-action=\"toggle\" data-index=\"{}\" data-rackforge-parameter-index=\"{}\" aria-label=\"Power\" aria-pressed=\"{}\"><span class=\"power-rocker-label on\">ON</span><span class=\"power-rocker-bezel\" aria-hidden=\"true\"><span class=\"power-rocker-face\"><span class=\"power-rocker-glyph on\">I</span><span class=\"power-rocker-glyph off\">O</span></span></span><span class=\"power-rocker-label off\">OFF</span></button>",
                            if active { "on" } else { "off" },
                            parameter.index,
                            parameter.index,
                            active,
                        )
                    } else {
                        let (waveform_class, content) = match parameter.id.as_str() {
                        "dco-pulse" => (
                            " waveform-toggle",
                            "<svg class=\"waveform-symbol pulse-waveform-symbol\" viewBox=\"0 0 42 22\" aria-hidden=\"true\"><path d=\"M2 20V2H20V20H40V2\"></path><rect x=\"24\" y=\"1\" width=\"2.5\" height=\"2.5\"></rect><rect x=\"29\" y=\"1\" width=\"2.5\" height=\"2.5\"></rect><rect x=\"29\" y=\"7.5\" width=\"2.5\" height=\"2.5\"></rect><rect x=\"29\" y=\"14\" width=\"2.5\" height=\"2.5\"></rect></svg>".to_owned(),
                        ),
                        "dco-saw" => (
                            " waveform-toggle",
                            "<svg class=\"waveform-symbol\" viewBox=\"0 0 42 22\" aria-hidden=\"true\"><path d=\"M5 18L35 4V18\"></path></svg>".to_owned(),
                        ),
                        _ => (
                            "",
                            if value >= 0.5 {
                                "ON".to_owned()
                            } else {
                                "OFF".to_owned()
                            },
                        ),
                    };
                        format!(
                            "<button class=\"panel-button panel-button-white parameter-toggle{waveform_class}{}\" type=\"button\" data-action=\"toggle\" data-index=\"{}\" data-rackforge-parameter-index=\"{}\" aria-label=\"{}\" aria-pressed=\"{}\">{content}</button>",
                            if value >= 0.5 { " active" } else { "" },
                            parameter.index,
                            parameter.index,
                            escape_html(&parameter.name),
                            value >= 0.5,
                        )
                    }
                }
                "enum" => {
                    if parameter.id == "hpf-frequency" {
                        self.render_fader(parameter, value)
                    } else if parameter.id == "dco-range" {
                        let mut positions = String::new();
                        for choice in &parameter.kind.choices {
                            let active = (choice.value - value.round()).abs() < f64::EPSILON;
                            let led = Self::render_panel_led(
                                active,
                                &format!(
                                    "{} range LED {}",
                                    choice.name,
                                    if active { "on" } else { "off" }
                                ),
                            );
                            positions.push_str(&format!(
                                "<div class=\"range-position\"><span class=\"range-position-label\">{}</span>{led}<button class=\"panel-button panel-button-white range-position-button\" type=\"button\" data-action=\"choice\" data-index=\"{}\" data-rackforge-parameter-index=\"{}\" data-value=\"{}\" aria-label=\"Range {}\" aria-pressed=\"{}\"></button></div>",
                                escape_html(&choice.name),
                                parameter.index,
                                parameter.index,
                                choice.value,
                                escape_html(&choice.name),
                                active,
                            ));
                        }
                        return format!(
                            "<div class=\"parameter-control range-control-group\"><span class=\"range-group-label\">RANGE</span><output class=\"visually-hidden\" data-output-index=\"{}\">{}</output>{positions}</div>",
                            parameter.index,
                            escape_html(&label),
                        );
                    } else if parameter.id == "pwm-mode" {
                        return format!(
                            "<div class=\"parameter-control control-pwm-mode\"><div class=\"parameter-head\"><strong>{}</strong><output class=\"parameter-readout\" data-output-index=\"{}\">{}</output></div>{}</div>",
                            escape_html(panel_parameter_label(parameter)),
                            parameter.index,
                            escape_html(&label),
                            self.render_two_position_switch(
                                parameter, value, "LFO", "MAN", 0.0, 1.0
                            ),
                        );
                    } else if parameter.id == "vcf-envelope-polarity" {
                        let switch = self.render_two_position_switch(
                            parameter,
                            value,
                            "<img class=\"switch-symbol envelope-symbol\" src=\"assets/envelope-positive.svg\" alt=\"\" aria-hidden=\"true\">",
                            "<img class=\"switch-symbol envelope-symbol inverted\" src=\"assets/envelope-positive.svg\" alt=\"\" aria-hidden=\"true\">",
                            0.0,
                            1.0,
                        );
                        return format!(
                            "<div class=\"parameter-control control-vcf-envelope-polarity\"><output class=\"visually-hidden\" data-output-index=\"{}\">{}</output>{switch}</div>",
                            parameter.index,
                            escape_html(&label),
                        );
                    } else if parameter.id == "vca-mode" {
                        let switch = self.render_two_position_switch(
                            parameter,
                            value,
                            "<span class=\"vca-mode-option\"><img class=\"switch-symbol envelope-symbol vca-envelope-symbol\" src=\"assets/envelope-positive.svg\" alt=\"\" aria-hidden=\"true\"><strong>ENV</strong></span>",
                            "<span class=\"vca-mode-option\"><strong>GATE</strong><svg class=\"switch-symbol vca-gate-symbol\" viewBox=\"0 0 42 22\" aria-hidden=\"true\"><path d=\"M1 18H11V4H31V18H41\"/></svg></span>",
                            0.0,
                            1.0,
                        );
                        return format!(
                            "<div class=\"parameter-control control-vca-mode\"><output class=\"visually-hidden\" data-output-index=\"{}\">{}</output>{switch}</div>",
                            parameter.index,
                            escape_html(&label),
                        );
                    } else if parameter.id == "chorus-mode" {
                        return self.render_chorus_control(parameter, value);
                    } else if parameter.id == "allocation-mode" {
                        return self.render_poly_control(parameter, value);
                    } else if parameter.id == "midi-channel" {
                        return self.render_midi_channel_control(parameter, value);
                    } else {
                        let tone =
                            if matches!(parameter.id.as_str(), "midi-channel" | "midi-function") {
                                "blue"
                            } else {
                                "white"
                            };
                        let mut choices = String::new();
                        for choice in &parameter.kind.choices {
                            let active = (choice.value - value.round()).abs() < f64::EPSILON;
                            choices.push_str(&format!(
                                "<button class=\"panel-button panel-button-{tone} parameter-choice{}\" type=\"button\" data-action=\"choice\" data-index=\"{}\" data-rackforge-parameter-index=\"{}\" data-value=\"{}\" aria-pressed=\"{}\">{}</button>",
                                if active { " active" } else { "" },
                                parameter.index,
                                parameter.index,
                                choice.value,
                                active,
                                escape_html(&choice.name)
                            ));
                        }
                        format!(
                            "<div class=\"parameter-choice-group\" role=\"group\" aria-label=\"{}\">{choices}</div>",
                            escape_html(&parameter.name),
                        )
                    }
                }
                _ if matches!(
                    parameter.id.as_str(),
                    "tuning" | "portamento" | "physical-volume"
                ) =>
                {
                    self.render_knob(parameter, value)
                }
                _ => self.render_fader(parameter, value),
            };
            let heading = if parameter.id == "dco-sub" {
                format!(
                    "<svg class=\"sub-waveform-symbol\" viewBox=\"0 0 20 16\" aria-hidden=\"true\"><path d=\"M2 15V1H10V15H18V1\"></path></svg><strong>{}</strong>",
                    escape_html(panel_parameter_label(parameter))
                )
            } else if parameter.id == "lfo-delay" {
                "<strong>DELAY<br>TIME</strong>".to_owned()
            } else {
                format!(
                    "<strong>{}</strong>",
                    escape_html(panel_parameter_label(parameter))
                )
            };
            format!(
                "<div class=\"parameter-control control-{}\"><div class=\"parameter-head\">{heading}<output class=\"parameter-readout\" data-output-index=\"{}\">{}</output></div>{control}</div>",
                parameter.id,
                parameter.index,
                escape_html(&label)
            )
        }

        fn envelope_values(&self) -> [f64; 4] {
            let defaults = [0.25, 0.25, 0.9, 0.25];
            let ids = ["env-attack", "env-decay", "env-sustain", "env-release"];
            let Some(snapshot) = &self.snapshot else {
                return defaults;
            };
            let mut values = defaults;
            for (slot, id) in values.iter_mut().zip(ids) {
                if let Some(parameter) = snapshot
                    .schema
                    .parameters
                    .iter()
                    .find(|parameter| parameter.id == id)
                {
                    *slot = self.parameter_value(parameter.index, *slot);
                }
            }
            values
        }

        fn render_envelope(&self) -> String {
            let [attack, decay, sustain, release] = self.envelope_values();
            let attack_x = 102.0 + attack * 100.0;
            let decay_x = attack_x + 70.0 + decay * 100.0;
            let sustain_x = (decay_x + 60.0).max(598.0 - release * 100.0);
            let sustain_y = 24.0 + (1.0 - sustain) * 152.0;
            let path = format!(
                "M 32 176 L {attack_x} 24 L {decay_x} {sustain_y} L {sustain_x} {sustain_y} L 668 176"
            );
            let stats = [("A", attack), ("D", decay), ("S", sustain), ("R", release)]
                .into_iter()
                .map(|(label, value)| {
                    format!(
                        "<span><small>{label}</small><strong>{:.0}</strong></span>",
                        value * 100.0
                    )
                })
                .collect::<String>();
            format!(
                "<section class=\"envelope-display\"><svg viewBox=\"0 0 700 200\" role=\"img\" aria-label=\"Current amplitude envelope\"><defs><linearGradient id=\"envGlow\" x1=\"0\" x2=\"1\"><stop stop-color=\"#ff4055\"/><stop offset=\"1\" stop-color=\"#2d5bff\"/></linearGradient></defs><g class=\"env-grid\"><path d=\"M32 62 H668 M32 100 H668 M32 138 H668\"/><path d=\"M191 24 V176 M350 24 V176 M509 24 V176\"/></g><path class=\"env-shadow\" d=\"{path}\"/><path class=\"env-line\" d=\"{path}\"/><g class=\"env-points\"><circle cx=\"{attack_x}\" cy=\"24\" r=\"5\"/><circle cx=\"{decay_x}\" cy=\"{sustain_y}\" r=\"5\"/><circle cx=\"{sustain_x}\" cy=\"{sustain_y}\" r=\"5\"/></g></svg><div class=\"envelope-stats\">{stats}</div></section>"
            )
        }
    }

    fn panel_parameter_label(parameter: &Parameter) -> &str {
        match parameter.id.as_str() {
            "lfo-rate" => "RATE",
            "lfo-delay" => "DELAY TIME",
            "hpf-frequency" | "vcf-cutoff" => "FREQ",
            "vcf-resonance" => "RES",
            "vcf-envelope" => "ENV",
            "vcf-envelope-polarity" => "ENV POLARITY",
            "vcf-keyboard" => "KYBD",
            "vca-mode" => "ENV / GATE",
            "env-attack" => "A",
            "env-decay" => "D",
            "env-sustain" => "S",
            "env-release" => "R",
            "bender-dco-sensitivity" => "DCO",
            "bender-vcf-sensitivity" => "VCF",
            "lfo-trigger-sensitivity" => "LFO",
            "physical-volume" => "VOLUME",
            "portamento-switch" => "PORTAMENTO",
            "allocation-mode" => "POLY",
            "midi-channel" => "MIDI CH",
            id if id == "dco-lfo" || id == "vcf-lfo" => "LFO",
            _ => &parameter.name,
        }
    }

    fn value_label(parameter: &Parameter, value: f64) -> String {
        if parameter.id == "vca-level" {
            return format!("{:+.1}", value * 10.0 - 5.0);
        }
        if matches!(
            parameter.id.as_str(),
            "vcf-cutoff"
                | "vcf-resonance"
                | "env-attack"
                | "env-decay"
                | "env-sustain"
                | "env-release"
                | "bender-dco-sensitivity"
                | "bender-vcf-sensitivity"
                | "lfo-trigger-sensitivity"
                | "portamento"
                | "physical-volume"
                | "lfo-rate"
                | "lfo-delay"
                | "dco-lfo"
                | "dco-pwm"
                | "dco-sub"
                | "dco-noise"
                | "vcf-envelope"
                | "vcf-lfo"
                | "vcf-keyboard"
        ) {
            return format!("{:.1}", value * 10.0);
        }
        match parameter.kind.kind.as_str() {
            "trigger" => if value >= 0.5 { "PRESSED" } else { "READY" }.to_owned(),
            "boolean" => if value >= 0.5 { "ON" } else { "OFF" }.to_owned(),
            "enum" => parameter
                .kind
                .choices
                .iter()
                .find(|choice| (choice.value - value.round()).abs() < f64::EPSILON)
                .map(|choice| choice.name.clone())
                .unwrap_or_else(|| value.to_string()),
            _ if parameter.kind.minimum == Some(0.0) && parameter.kind.maximum == Some(1.0) => {
                format!("{:.0}%", value * 100.0)
            }
            _ => format!(
                "{value:.2}{}",
                parameter
                    .kind
                    .unit
                    .as_deref()
                    .map(|unit| format!(" {unit}"))
                    .unwrap_or_default()
            ),
        }
    }

    fn request(
        app: &AppHandle,
        method: &str,
        params: serde_json::Value,
        handler: impl FnOnce(&AppHandle, Result<JsValue, String>) + 'static,
    ) {
        let (id, window, origin) = {
            let mut app = app.borrow_mut();
            app.sequence += 1;
            let id = format!("rf-106-{}", app.sequence);
            app.pending.insert(id.clone(), Box::new(handler));
            (id, app.window.clone(), app.host_origin.clone())
        };
        let outgoing = Request {
            protocol: PROTOCOL,
            kind: "request",
            request_id: &id,
            method,
            params,
        };
        // RackForge's web bridge consumes `params` as a plain JavaScript
        // object (`params.sound_id`, `params.parameter_index`, ...).  The
        // serde-wasm-bindgen default represents maps, including the object
        // inside serde_json::Value, as JavaScript Map instances instead.
        // Keep the entire wire envelope JSON-compatible so the host can
        // validate and dispatch every plugin request.
        let serializer = serde_wasm_bindgen::Serializer::json_compatible();
        let message = match outgoing.serialize(&serializer) {
            Ok(message) => message,
            Err(error) => {
                resolve(app, &id, Err(error.to_string()));
                return;
            }
        };
        match window.parent().ok().flatten() {
            Some(parent) => {
                if let Err(error) = parent.post_message(&message, &origin) {
                    resolve(app, &id, Err(format!("postMessage failed: {error:?}")));
                    return;
                }
            }
            None => {
                resolve(
                    app,
                    &id,
                    Err("RackForge parent window is missing.".to_owned()),
                );
                return;
            }
        }
        let weak = Rc::downgrade(app);
        let timeout_id = id.clone();
        let timeout = Closure::once_into_js(move || {
            if let Some(app) = weak.upgrade() {
                resolve(
                    &app,
                    &timeout_id,
                    Err("RackForge did not answer in time.".to_owned()),
                );
            }
        });
        let _ = window
            .set_timeout_with_callback_and_timeout_and_arguments_0(timeout.unchecked_ref(), 4_000);
    }

    fn resolve(app: &AppHandle, id: &str, result: Result<JsValue, String>) {
        let handler = app.borrow_mut().pending.remove(id);
        if let Some(handler) = handler {
            handler(app, result);
        }
    }

    fn refresh_parameters(app: &AppHandle) {
        if app.borrow().context.is_none() {
            return;
        }
        let generation = {
            let mut app = app.borrow_mut();
            app.parameter_refresh_generation += 1;
            app.parameter_refresh_generation
        };
        request(
            app,
            "plugin.parameters",
            serde_json::json!({}),
            move |app, result| {
                if app.borrow().parameter_refresh_generation != generation {
                    return;
                }
                match result.and_then(|value| {
                    serde_wasm_bindgen::from_value::<ParameterSnapshot>(value)
                        .map_err(|error| error.to_string())
                }) {
                    Ok(snapshot) => {
                        let values = snapshot
                            .values
                            .iter()
                            .map(|value| (value.index, value.value))
                            .collect();
                        let mut app = app.borrow_mut();
                        app.snapshot = Some(snapshot);
                        app.parameter_values = values;
                        app.bridge_error.clear();
                    }
                    Err(error) => app.borrow_mut().bridge_error = error,
                }
                render_or_defer(app);
            },
        );
    }

    fn send_parameter(app: &AppHandle, index: u32, value: f64) {
        app.borrow_mut().parameter_values.insert(index, value);
        let refresh_after_success = index == PUBLIC_KEY_TRANSPOSE_TRIGGER_INDEX && value < 0.5;
        request(
            app,
            "plugin.set_parameter",
            serde_json::json!({ "parameter_index": index, "value": value }),
            move |app, result| {
                if let Err(error) = result {
                    app.borrow_mut().bridge_error = error;
                    refresh_parameters(app);
                } else if refresh_after_success {
                    refresh_parameters(app);
                }
            },
        );
    }

    fn update_parameter_dom(app: &AppHandle, index: u32) {
        let (document, value, label, redraw_envelope, knob_angle) = {
            let app = app.borrow();
            let parameter = app.snapshot.as_ref().and_then(|snapshot| {
                snapshot
                    .schema
                    .parameters
                    .iter()
                    .find(|parameter| parameter.index == index)
            });
            let Some(parameter) = parameter else { return };
            let value = app.parameter_value(index, parameter.kind.default_value());
            let knob_angle = if matches!(
                parameter.id.as_str(),
                "tuning" | "portamento" | "physical-volume"
            ) {
                let minimum = parameter.kind.minimum.unwrap_or(0.0);
                let maximum = parameter.kind.maximum.unwrap_or(1.0);
                let normalized = if maximum > minimum {
                    ((value - minimum) / (maximum - minimum)).clamp(0.0, 1.0)
                } else {
                    0.0
                };
                Some(-135.0 + normalized * 270.0)
            } else {
                None
            };
            (
                app.document.clone(),
                value,
                value_label(parameter, value),
                parameter.page == "envelope",
                knob_angle,
            )
        };
        if let Ok(Some(input)) = document.query_selector(&format!("input[data-index=\"{index}\"]"))
        {
            let _ = Reflect::set(
                input.as_ref(),
                &JsValue::from_str("value"),
                &JsValue::from_str(&value.to_string()),
            );
        }
        if let Ok(Some(output)) =
            document.query_selector(&format!("[data-output-index=\"{index}\"]"))
        {
            output.set_text_content(Some(&label));
        }
        if let Some(angle) = knob_angle
            && let Ok(Some(knob)) =
                document.query_selector(&format!("[data-knob-index=\"{index}\"]"))
        {
            let _ = knob.set_attribute("style", &format!("--knob-turn: {angle}deg"));
        }
        if redraw_envelope && let Ok(Some(envelope)) = document.query_selector(".envelope-display")
        {
            envelope.set_outer_html(&app.borrow().render_envelope());
        }
    }

    fn render_or_defer(app: &AppHandle) {
        let render_now = {
            let mut state = app.borrow_mut();
            if state.active_parameter_drag.is_some() {
                state.render_after_parameter_drag = true;
                false
            } else {
                true
            }
        };
        if render_now {
            app.borrow().render();
        }
    }

    fn finish_parameter_drag(app: &AppHandle) {
        let (refresh, render) = {
            let mut state = app.borrow_mut();
            state.active_parameter_drag = None;
            let refresh = state.refresh_parameters_after_drag;
            let render = state.render_after_parameter_drag;
            state.refresh_parameters_after_drag = false;
            state.render_after_parameter_drag = false;
            (refresh, render)
        };
        if refresh {
            refresh_parameters(app);
        } else if render {
            app.borrow().render();
        }
    }

    fn element_from_event(event: &Event) -> Option<Element> {
        event
            .target()?
            .dyn_into::<Element>()
            .ok()?
            .closest("[data-action]")
            .ok()
            .flatten()
    }

    fn set_css_class(element: &Element, class: &str, enabled: bool) {
        let mut classes = element
            .class_name()
            .split_ascii_whitespace()
            .filter(|candidate| *candidate != class)
            .map(str::to_owned)
            .collect::<Vec<_>>();
        if enabled {
            classes.push(class.to_owned());
        }
        element.set_class_name(&classes.join(" "));
    }

    fn numeric_value(element: &Element) -> Option<f64> {
        Reflect::get(element.as_ref(), &JsValue::from_str("value"))
            .ok()?
            .as_string()?
            .parse()
            .ok()
    }

    fn fader_from_pointer_event(event: &PointerEvent) -> Option<(Element, Element)> {
        let surface = event
            .target()?
            .dyn_into::<Element>()
            .ok()?
            .closest(".rf-fader")
            .ok()
            .flatten()?;
        let input = surface
            .query_selector("input.parameter-slider")
            .ok()
            .flatten()?;
        Some((input, surface))
    }

    fn bender_from_pointer_event(event: &PointerEvent) -> Option<Element> {
        event
            .target()?
            .dyn_into::<Element>()
            .ok()?
            .closest("[data-action=bender]")
            .ok()
            .flatten()
    }

    fn set_bender_visual(element: &Element, position: f64, trigger: bool) {
        let left = (position.clamp(-1.0, 1.0) + 1.0) * 50.0;
        let push = if trigger { -18 } else { 0 };
        let _ = element.set_attribute(
            "style",
            &format!("--bender-left: {left:.3}%; --bender-push: {push}px"),
        );
        let _ = element.set_attribute("aria-valuenow", &format!("{position:.3}"));
        if let Ok(Some(lever)) = element.query_selector(".bender-lever") {
            lever.set_class_name(if trigger {
                "bender-lever lfo-active"
            } else {
                "bender-lever"
            });
        }
    }

    fn update_bender_from_pointer(app: &AppHandle, element: &Element, event: &PointerEvent) {
        let Some(pitch_index) = element
            .get_attribute("data-index")
            .and_then(|value| value.parse().ok())
        else {
            return;
        };
        let Some(trigger_index) = element
            .get_attribute("data-trigger-index")
            .and_then(|value| value.parse().ok())
        else {
            return;
        };
        let rect = element.get_bounding_client_rect();
        let (position, trigger) = bender_pointer_state(
            f64::from(event.client_x()),
            f64::from(event.client_y()),
            rect.left(),
            rect.top(),
            rect.width(),
            rect.height(),
        );
        let trigger_value = f64::from(trigger as u8);
        let pitch_changed =
            (app.borrow().parameter_value(pitch_index, 0.0) - position).abs() > 1.0e-4;
        let trigger_changed = app.borrow().parameter_value(trigger_index, 0.0) != trigger_value;
        if pitch_changed {
            send_parameter(app, pitch_index, position);
        }
        if trigger_changed {
            send_parameter(app, trigger_index, trigger_value);
        }
        set_bender_visual(element, position, trigger);
    }

    fn update_fader_from_drag(
        app: &AppHandle,
        element: &Element,
        start_y: f64,
        current_y: f64,
        start_value: f64,
    ) {
        let Some(index) = element
            .get_attribute("data-index")
            .and_then(|value| value.parse().ok())
        else {
            return;
        };
        let minimum = element
            .get_attribute("min")
            .and_then(|value| value.parse().ok())
            .unwrap_or(0.0);
        let maximum = element
            .get_attribute("max")
            .and_then(|value| value.parse().ok())
            .unwrap_or(1.0);
        let step = element
            .get_attribute("step")
            .and_then(|value| value.parse().ok())
            .unwrap_or(0.0);
        let rect = element.get_bounding_client_rect();
        let value = relative_vertical_fader_value(
            start_value,
            start_y - current_y,
            rect.height(),
            minimum,
            maximum,
            step,
        );
        if numeric_value(element).is_some_and(|current| (current - value).abs() < f64::EPSILON) {
            return;
        }
        let _ = Reflect::set(
            element.as_ref(),
            &JsValue::from_str("value"),
            &JsValue::from_str(&value.to_string()),
        );
        send_parameter(app, index, value);
        update_parameter_dom(app, index);
    }

    fn install_events(app: &AppHandle) -> Result<(), JsValue> {
        let resize_app = app.clone();
        let resize = Closure::<dyn FnMut(Event)>::new(move |_| {
            resize_app.borrow().sync_program_group_routing();
        });
        app.borrow()
            .window
            .add_event_listener_with_callback("resize", resize.as_ref().unchecked_ref())?;
        resize.forget();

        // A secondary mouse press must never look or behave like a hardware
        // activation, including controls such as program keys that do not map
        // to a public parameter and therefore have no RackForge context menu.
        let non_primary_press = Rc::new(RefCell::new(None::<Element>));
        for event_name in [
            "pointerdown",
            "pointerup",
            "pointercancel",
            "lostpointercapture",
        ] {
            let pressed = non_primary_press.clone();
            let guard =
                Closure::<dyn FnMut(PointerEvent)>::new(
                    move |event: PointerEvent| match event_name {
                        "pointerdown" if event.pointer_type() == "mouse" && event.button() != 0 => {
                            let Some(element) = element_from_event(&event) else {
                                return;
                            };
                            if let Some(previous) = pressed.borrow_mut().replace(element.clone()) {
                                set_css_class(&previous, "rackforge-context-press", false);
                            }
                            set_css_class(&element, "rackforge-context-press", true);
                            event.prevent_default();
                            event.stop_immediate_propagation();
                        }
                        "pointerup" | "pointercancel" | "lostpointercapture" => {
                            if let Some(element) = pressed.borrow_mut().take() {
                                set_css_class(&element, "rackforge-context-press", false);
                                event.prevent_default();
                                event.stop_immediate_propagation();
                            }
                        }
                        _ => {}
                    },
                );
            app.borrow()
                .root
                .add_event_listener_with_callback_and_bool(
                    event_name,
                    guard.as_ref().unchecked_ref(),
                    true,
                )?;
            guard.forget();
        }

        let click_app = app.clone();
        let click = Closure::<dyn FnMut(MouseEvent)>::new(move |event: MouseEvent| {
            if event.button() != 0 {
                return;
            }
            let Some(element) = element_from_event(&event) else {
                return;
            };
            match element.get_attribute("data-action").as_deref() {
                Some("section") => {
                    if let Some(section) = element.get_attribute("data-section")
                        && matches!(
                            section.as_str(),
                            "lfo"
                                | "dco"
                                | "hpf"
                                | "filter"
                                | "vca"
                                | "envelope"
                                | "chorus"
                                | "performance"
                                | "level"
                        )
                    {
                        let needs_refresh = {
                            let mut app = click_app.borrow_mut();
                            app.active_section = section;
                            app.snapshot.is_none()
                        };
                        click_app.borrow().render();
                        if needs_refresh {
                            refresh_parameters(&click_app);
                        }
                    }
                }
                Some("retry-parameters") => {
                    click_app.borrow_mut().bridge_error.clear();
                    click_app.borrow().render();
                    refresh_parameters(&click_app);
                }
                Some("sound") => {
                    if let Some(sound_id) = element.get_attribute("data-sound-id") {
                        let selected_sound_id = sound_id.clone();
                        {
                            let mut app = click_app.borrow_mut();
                            app.pending_sound_id = Some(sound_id.clone());
                            app.bridge_error.clear();
                        }
                        click_app.borrow().render();
                        request(
                            &click_app,
                            "plugin.select_sound",
                            serde_json::json!({"sound_id": sound_id}),
                            move |app, result| match result {
                                Ok(_) => {
                                    {
                                        let mut app = app.borrow_mut();
                                        if let Some(context) = app.context.as_mut() {
                                            context.instance.selected_sound_id = selected_sound_id;
                                        }
                                        // Keep the current panel painted until the
                                        // replacement snapshot arrives. Clearing it
                                        // here flashes the initial loading screen.
                                        app.pending_sound_id = None;
                                        app.bridge_error.clear();
                                    }
                                    refresh_parameters(app);
                                }
                                Err(error) => {
                                    {
                                        let mut state = app.borrow_mut();
                                        state.pending_sound_id = None;
                                        state.bridge_error = error;
                                    }
                                    app.borrow().render();
                                }
                            },
                        );
                    }
                }
                Some("toggle") => {
                    if let Some(index) = element
                        .get_attribute("data-index")
                        .and_then(|value| value.parse().ok())
                    {
                        let value = if click_app.borrow().parameter_value(index, 0.0) >= 0.5 {
                            0.0
                        } else {
                            1.0
                        };
                        send_parameter(&click_app, index, value);
                        click_app.borrow().render();
                    }
                }
                Some("choice") => {
                    let index = element
                        .get_attribute("data-index")
                        .and_then(|value| value.parse().ok());
                    let value = element
                        .get_attribute("data-value")
                        .and_then(|value| value.parse().ok());
                    if let (Some(index), Some(value)) = (index, value) {
                        send_parameter(&click_app, index, value);
                        click_app.borrow().render();
                    }
                }
                Some("allocation-toggle") => {
                    let index = element
                        .get_attribute("data-index")
                        .and_then(|value| value.parse().ok());
                    let position = element
                        .get_attribute("data-position")
                        .and_then(|value| value.parse::<i32>().ok());
                    if let (Some(index), Some(position)) = (index, position) {
                        let current = click_app.borrow().parameter_value(index, 1.0).round() as i32;
                        let value = match (current, position) {
                            (0, 1) => 2.0,
                            (0, 2) => 1.0,
                            (1, 2) | (2, 1) => 0.0,
                            (_, selected) => selected as f64,
                        };
                        send_parameter(&click_app, index, value);
                        click_app.borrow().render();
                    }
                }
                _ => {}
            }
        });
        app.borrow()
            .root
            .add_event_listener_with_callback("click", click.as_ref().unchecked_ref())?;
        click.forget();

        let input_app = app.clone();
        let input = Closure::<dyn FnMut(Event)>::new(move |event| {
            let Some(element) = element_from_event(&event) else {
                return;
            };
            match element.get_attribute("data-action").as_deref() {
                Some("search") => {
                    input_app.borrow_mut().search_query =
                        Reflect::get(element.as_ref(), &JsValue::from_str("value"))
                            .ok()
                            .and_then(|value| value.as_string())
                            .unwrap_or_default();
                    input_app.borrow().render();
                    if let Some(search) = input_app
                        .borrow()
                        .document
                        .query_selector("[data-action=search]")
                        .ok()
                        .flatten()
                    {
                        let _ = search
                            .dyn_into::<web_sys::HtmlElement>()
                            .map(|element| element.focus());
                    }
                }
                Some("parameter") => {
                    let index = element
                        .get_attribute("data-index")
                        .and_then(|value| value.parse().ok());
                    if let (Some(index), Some(value)) = (index, numeric_value(&element)) {
                        send_parameter(&input_app, index, value);
                        update_parameter_dom(&input_app, index);
                    }
                }
                _ => {}
            }
        });
        app.borrow()
            .root
            .add_event_listener_with_callback("input", input.as_ref().unchecked_ref())?;
        app.borrow()
            .root
            .add_event_listener_with_callback("change", input.as_ref().unchecked_ref())?;
        input.forget();

        // Chromium maps a rotated native range on its unrotated horizontal
        // axis, which can overwrite a vertical gesture with the minimum value.
        // Pointer input belongs to the fader surface instead; the range remains
        // focusable for keyboard access. Drag relative to the value at press
        // time so grabbing the handle never causes an initial jump.
        let active_fader_drag = Rc::new(RefCell::new(None::<(i32, Element, Element, f64, f64)>));
        for event_name in [
            "pointerdown",
            "pointermove",
            "pointerup",
            "pointercancel",
            "lostpointercapture",
        ] {
            let drag_app = app.clone();
            let drag_state = active_fader_drag.clone();
            let drag = Closure::<dyn FnMut(PointerEvent)>::new(move |event: PointerEvent| {
                let pointer_id = event.pointer_id();
                match event_name {
                    "pointerdown" => {
                        if !event.is_primary() || event.button() != 0 {
                            return;
                        }
                        let Some((element, surface)) = fader_from_pointer_event(&event) else {
                            return;
                        };
                        event.prevent_default();
                        let start_value = numeric_value(&element).unwrap_or(0.0);
                        let start_y = f64::from(event.client_y());
                        let parameter_index = element
                            .get_attribute("data-index")
                            .and_then(|value| value.parse().ok());
                        let _ = surface.set_pointer_capture(pointer_id);
                        let _ = element
                            .clone()
                            .dyn_into::<web_sys::HtmlElement>()
                            .map(|element| element.focus());
                        *drag_state.borrow_mut() =
                            Some((pointer_id, element, surface, start_y, start_value));
                        if let Some(parameter_index) = parameter_index {
                            let mut app = drag_app.borrow_mut();
                            app.active_parameter_drag = Some(parameter_index);
                            app.render_after_parameter_drag = false;
                            app.refresh_parameters_after_drag = false;
                        }
                    }
                    "pointermove" => {
                        let drag = drag_state.borrow().as_ref().and_then(
                            |(active, element, _, start_y, start_value)| {
                                (*active == pointer_id)
                                    .then(|| (element.clone(), *start_y, *start_value))
                            },
                        );
                        if let Some((element, start_y, start_value)) = drag {
                            event.prevent_default();
                            update_fader_from_drag(
                                &drag_app,
                                &element,
                                start_y,
                                f64::from(event.client_y()),
                                start_value,
                            );
                        }
                    }
                    "pointerup" => {
                        let drag = drag_state.borrow().as_ref().and_then(
                            |(active, element, surface, start_y, start_value)| {
                                (*active == pointer_id).then(|| {
                                    (element.clone(), surface.clone(), *start_y, *start_value)
                                })
                            },
                        );
                        if let Some((element, surface, start_y, start_value)) = drag {
                            event.prevent_default();
                            update_fader_from_drag(
                                &drag_app,
                                &element,
                                start_y,
                                f64::from(event.client_y()),
                                start_value,
                            );
                            let _ = surface.release_pointer_capture(pointer_id);
                            *drag_state.borrow_mut() = None;
                            finish_parameter_drag(&drag_app);
                        }
                    }
                    "pointercancel" | "lostpointercapture" => {
                        let is_active = drag_state
                            .borrow()
                            .as_ref()
                            .is_some_and(|(active, _, _, _, _)| *active == pointer_id);
                        if is_active {
                            *drag_state.borrow_mut() = None;
                            finish_parameter_drag(&drag_app);
                        }
                    }
                    _ => {}
                }
            });
            app.borrow()
                .root
                .add_event_listener_with_callback(event_name, drag.as_ref().unchecked_ref())?;
            drag.forget();
        }

        let active_bender_drag = Rc::new(RefCell::new(None::<(i32, Element)>));
        for event_name in [
            "pointerdown",
            "pointermove",
            "pointerup",
            "pointercancel",
            "lostpointercapture",
        ] {
            let bender_app = app.clone();
            let drag_state = active_bender_drag.clone();
            let drag = Closure::<dyn FnMut(PointerEvent)>::new(move |event: PointerEvent| {
                let pointer_id = event.pointer_id();
                match event_name {
                    "pointerdown" => {
                        if !event.is_primary() || event.button() != 0 {
                            return;
                        }
                        let Some(element) = bender_from_pointer_event(&event) else {
                            return;
                        };
                        event.prevent_default();
                        let _ = element.set_pointer_capture(pointer_id);
                        *drag_state.borrow_mut() = Some((pointer_id, element.clone()));
                        update_bender_from_pointer(&bender_app, &element, &event);
                    }
                    "pointermove" => {
                        let element = drag_state.borrow().as_ref().and_then(|(active, element)| {
                            (*active == pointer_id).then(|| element.clone())
                        });
                        if let Some(element) = element {
                            event.prevent_default();
                            update_bender_from_pointer(&bender_app, &element, &event);
                        }
                    }
                    "pointerup" | "pointercancel" | "lostpointercapture" => {
                        let element = drag_state.borrow().as_ref().and_then(|(active, element)| {
                            (*active == pointer_id).then(|| element.clone())
                        });
                        if let Some(element) = element {
                            event.prevent_default();
                            let pitch_index = element
                                .get_attribute("data-index")
                                .and_then(|value| value.parse().ok());
                            let trigger_index = element
                                .get_attribute("data-trigger-index")
                                .and_then(|value| value.parse().ok());
                            if let Some(index) = pitch_index {
                                send_parameter(&bender_app, index, 0.0);
                            }
                            if let Some(index) = trigger_index {
                                send_parameter(&bender_app, index, 0.0);
                            }
                            set_bender_visual(&element, 0.0, false);
                            let _ = element.release_pointer_capture(pointer_id);
                            *drag_state.borrow_mut() = None;
                        }
                    }
                    _ => {}
                }
            });
            app.borrow()
                .root
                .add_event_listener_with_callback(event_name, drag.as_ref().unchecked_ref())?;
            drag.forget();
        }

        for (event_name, value) in [
            ("pointerdown", 1.0),
            ("pointerup", 0.0),
            ("pointercancel", 0.0),
            ("lostpointercapture", 0.0),
        ] {
            let trigger_app = app.clone();
            let trigger = Closure::<dyn FnMut(PointerEvent)>::new(move |event: PointerEvent| {
                if matches!(event_name, "pointerdown" | "pointerup") && event.button() != 0 {
                    return;
                }
                let Some(element) = element_from_event(&event) else {
                    return;
                };
                if element.get_attribute("data-action").as_deref() != Some("trigger") {
                    return;
                }
                event.prevent_default();
                if let Some(index) = element
                    .get_attribute("data-index")
                    .and_then(|index| index.parse().ok())
                {
                    send_parameter(&trigger_app, index, value);
                    update_parameter_dom(&trigger_app, index);
                    let _ = element
                        .set_attribute("aria-pressed", if value >= 0.5 { "true" } else { "false" });
                    let mut classes = element.class_name();
                    let has_active = classes
                        .split_ascii_whitespace()
                        .any(|class| class == "active");
                    if value >= 0.5 && !has_active {
                        classes.push_str(" active");
                    } else if value < 0.5 && has_active {
                        classes = classes
                            .split_ascii_whitespace()
                            .filter(|class| *class != "active")
                            .collect::<Vec<_>>()
                            .join(" ");
                    }
                    element.set_class_name(&classes);
                }
            });
            app.borrow()
                .root
                .add_event_listener_with_callback(event_name, trigger.as_ref().unchecked_ref())?;
            trigger.forget();
        }

        let message_app = app.clone();
        let message = Closure::<dyn FnMut(MessageEvent)>::new(move |event: MessageEvent| {
            let source_is_parent = message_app
                .borrow()
                .window
                .parent()
                .ok()
                .flatten()
                .zip(event.source())
                .is_some_and(|(parent, source)| Object::is(parent.as_ref(), source.as_ref()));
            if !source_is_parent || event.origin() != message_app.borrow().host_origin {
                return;
            }
            let data = event.data();
            if Reflect::get(&data, &JsValue::from_str("protocol"))
                .ok()
                .and_then(|value| value.as_string())
                .as_deref()
                != Some(PROTOCOL)
            {
                return;
            }
            match Reflect::get(&data, &JsValue::from_str("kind"))
                .ok()
                .and_then(|value| value.as_string())
                .as_deref()
            {
                Some("context") => {
                    if let Ok(context) = serde_wasm_bindgen::from_value::<HostContext>(data) {
                        let previous = message_app
                            .borrow()
                            .context
                            .as_ref()
                            .map(|context| context.instance.selected_sound_id.clone());
                        let changed = previous.as_deref()
                            != Some(context.instance.selected_sound_id.as_str());
                        let had_snapshot = message_app.borrow().snapshot.is_some();
                        let active_drag = {
                            let mut app = message_app.borrow_mut();
                            app.context = Some(context);
                            if app.active_parameter_drag.is_some() {
                                if changed && had_snapshot {
                                    app.refresh_parameters_after_drag = true;
                                } else {
                                    app.render_after_parameter_drag = true;
                                }
                                true
                            } else {
                                false
                            }
                        };
                        if active_drag {
                            return;
                        } else if changed && had_snapshot {
                            refresh_parameters(&message_app);
                        } else {
                            message_app.borrow().render();
                            if message_app.borrow().snapshot.is_none() {
                                refresh_parameters(&message_app);
                            }
                        }
                    }
                }
                Some("parameter_changed") => {
                    let index = Reflect::get(&data, &JsValue::from_str("parameter_index"))
                        .ok()
                        .and_then(|value| value.as_f64())
                        .filter(|value| value.is_finite() && value.fract() == 0.0)
                        .map(|value| value as u32);
                    let value = Reflect::get(&data, &JsValue::from_str("value"))
                        .ok()
                        .and_then(|value| value.as_f64())
                        .filter(|value| value.is_finite());
                    if let (Some(index), Some(value)) = (index, value) {
                        let update = {
                            let mut app = message_app.borrow_mut();
                            app.parameter_values.insert(index, value);
                            let update = parameter_change_update(app.active_parameter_drag, index);
                            if update == ParameterChangeUpdate::TargetAndDeferredRender {
                                app.render_after_parameter_drag = true;
                            }
                            update
                        };
                        match update {
                            ParameterChangeUpdate::FullRender => message_app.borrow().render(),
                            ParameterChangeUpdate::TargetOnly
                            | ParameterChangeUpdate::TargetAndDeferredRender => {
                                update_parameter_dom(&message_app, index);
                            }
                        }
                    }
                }
                Some("response") => {
                    let request_id = Reflect::get(&data, &JsValue::from_str("request_id"))
                        .ok()
                        .and_then(|value| value.as_string());
                    if let Some(request_id) = request_id {
                        let ok = Reflect::get(&data, &JsValue::from_str("ok"))
                            .ok()
                            .and_then(|value| value.as_bool())
                            .unwrap_or(false);
                        let result = if ok {
                            Ok(Reflect::get(&data, &JsValue::from_str("result"))
                                .unwrap_or(JsValue::UNDEFINED))
                        } else {
                            Err(Reflect::get(&data, &JsValue::from_str("error"))
                                .ok()
                                .and_then(|value| value.as_string())
                                .unwrap_or_else(|| "RackForge rejected this request.".to_owned()))
                        };
                        resolve(&message_app, &request_id, result);
                    }
                }
                _ => {}
            }
        });
        app.borrow()
            .window
            .add_event_listener_with_callback("message", message.as_ref().unchecked_ref())?;
        message.forget();
        Ok(())
    }

    #[wasm_bindgen(start)]
    pub fn start() -> Result<(), JsValue> {
        let app = App::new()?;
        install_events(&app)?;
        let ready = serde_wasm_bindgen::to_value(&Ready {
            protocol: PROTOCOL,
            kind: "ready",
        })?;
        let (parent, origin) = {
            let app = app.borrow();
            (
                app.window
                    .parent()?
                    .ok_or_else(|| JsValue::from_str("missing parent"))?,
                app.host_origin.clone(),
            )
        };
        parent.post_message(&ready, &origin)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sound(id: &str, name: &str, bank: &str) -> Sound {
        Sound {
            id: id.to_owned(),
            name: name.to_owned(),
            bank: bank.to_owned(),
        }
    }

    #[test]
    fn patch_identity_matches_the_existing_surface_contract() {
        let named = sound("factory.rf106.000", "A11 Brass", "factory.rf106");
        assert!(is_rf106_sound(&named));
        assert_eq!(MODEL_ID, "rf106");
        assert_eq!(MODEL_NAME, "RF-106");
        assert_eq!(PROTOCOL, "rackforge.plugin.web@1");
        assert_eq!(patch_code(&named), "A11");
        assert_eq!(clean_patch_name(&named), "Brass");

        let numeric = sound("factory.other.127", "No Prefix", "factory.other");
        assert!(!is_rf106_sound(&numeric));
        assert_eq!(patch_code(&numeric), "B88");
        assert_eq!(clean_patch_name(&numeric), "No Prefix");
    }

    #[test]
    fn html_text_is_escaped_before_rendering() {
        assert_eq!(
            escape_html("<RF & \"RF\">"),
            "&lt;RF &amp; &quot;RF&quot;&gt;"
        );
    }

    #[test]
    fn vertical_fader_drag_is_relative_quantized_and_clamped() {
        let height = 159.0;
        assert_eq!(
            relative_vertical_fader_value(5.0, 0.0, height, 0.0, 10.0, 1.0),
            5.0
        );
        assert_eq!(
            relative_vertical_fader_value(5.0, 28.0, height, 0.0, 10.0, 1.0),
            7.0
        );
        assert_eq!(
            relative_vertical_fader_value(5.0, -28.0, height, 0.0, 10.0, 1.0),
            3.0
        );
        assert_eq!(
            relative_vertical_fader_value(8.0, 500.0, height, 0.0, 10.0, 1.0),
            10.0
        );
        assert_eq!(
            relative_vertical_fader_value(2.0, -500.0, height, 0.0, 10.0, 1.0),
            0.0
        );
    }

    #[test]
    fn parameter_confirmation_does_not_replace_an_active_fader() {
        assert_eq!(
            parameter_change_update(None, 17),
            ParameterChangeUpdate::FullRender
        );
        assert_eq!(
            parameter_change_update(Some(17), 17),
            ParameterChangeUpdate::TargetOnly
        );
        assert_eq!(
            parameter_change_update(Some(17), 18),
            ParameterChangeUpdate::TargetAndDeferredRender
        );
    }

    #[test]
    fn bender_pointer_maps_horizontal_bend_and_forward_lfo_trigger() {
        assert_eq!(
            bender_pointer_state(24.0, 60.0, 0.0, 0.0, 300.0, 82.0),
            (-1.0, false)
        );
        assert_eq!(
            bender_pointer_state(150.0, 60.0, 0.0, 0.0, 300.0, 82.0),
            (0.0, false)
        );
        assert_eq!(
            bender_pointer_state(276.0, 20.0, 0.0, 0.0, 300.0, 82.0),
            (1.0, true)
        );
    }

    #[test]
    fn parameter_defaults_accept_host_numbers_and_booleans() {
        let number: ParameterDefault = serde_json::from_str("0.625").unwrap();
        let enabled: ParameterDefault = serde_json::from_str("true").unwrap();
        let disabled: ParameterDefault = serde_json::from_str("false").unwrap();
        assert_eq!(number.as_f64(), 0.625);
        assert_eq!(enabled.as_f64(), 1.0);
        assert_eq!(disabled.as_f64(), 0.0);
    }

    #[test]
    fn packaged_schema_exposes_the_complete_audio_panel_append_only() {
        let schema: serde_json::Value = serde_json::from_str(include_str!(
            "../../plugin/package/metadata/parameters.json"
        ))
        .unwrap();
        assert_eq!(schema["schema_version"].as_u64(), Some(3));
        assert_eq!(schema["display_decimals"].as_u64(), Some(2));
        let parameters = schema["parameters"].as_array().unwrap();
        assert_eq!(parameters.len(), rf_106_contract::PUBLIC_PARAMETER_COUNT);
        for (index, parameter) in parameters.iter().enumerate() {
            assert_eq!(parameter["index"].as_u64(), Some(index as u64));
            assert_eq!(parameter["flags"]["read_only"].as_bool(), Some(false));
            assert!(
                matches!(
                    parameter["kind"]["type"].as_str(),
                    Some("float" | "integer" | "boolean" | "enum" | "trigger")
                ),
                "parameter {} uses a control kind LITTLE cannot edit",
                parameter["id"]
            );
        }
        for id in [
            "lfo-rate",
            "lfo-delay",
            "dco-range",
            "dco-pwm",
            "pwm-mode",
            "dco-pulse",
            "dco-saw",
            "dco-sub",
            "dco-noise",
            "hpf-frequency",
            "vcf-envelope",
            "vcf-envelope-polarity",
            "vcf-lfo",
            "vcf-keyboard",
            "vca-mode",
            "vca-level",
            "chorus-mode",
            "portamento-switch",
            "physical-volume",
            "key-transpose",
            "midi-channel",
            "midi-function",
            "bender-position",
        ] {
            assert!(
                parameters
                    .iter()
                    .any(|parameter| parameter["id"].as_str() == Some(id)),
                "missing {id}"
            );
        }
        let chorus = parameters
            .iter()
            .find(|parameter| parameter["id"].as_str() == Some("chorus-mode"))
            .unwrap();
        let chorus_choices = chorus["kind"]["choices"].as_array().unwrap();
        assert_eq!(chorus_choices.len(), 3);
        assert_eq!(chorus_choices[0]["value"].as_u64(), Some(0));
        assert_eq!(chorus_choices[1]["value"].as_u64(), Some(1));
        assert_eq!(chorus_choices[2]["value"].as_u64(), Some(2));
        let pages = schema["pages"].as_array().unwrap();
        for page in [
            "level",
            "lfo",
            "dco",
            "hpf",
            "filter",
            "vca",
            "envelope",
            "chorus",
            "performance",
        ] {
            assert!(
                pages
                    .iter()
                    .any(|candidate| candidate["id"].as_str() == Some(page))
            );
            assert!(
                parameters
                    .iter()
                    .any(|parameter| parameter["page"].as_str() == Some(page)),
                "LITTLE page {page} must expose at least one control"
            );
        }
    }

    #[test]
    fn packaged_catalog_exposes_only_rf106_factory_memory() {
        let catalog: serde_json::Value =
            serde_json::from_str(include_str!("../../plugin/package/metadata/presets.json"))
                .unwrap();
        let banks = catalog["banks"].as_array().unwrap();
        let presets = catalog["presets"].as_array().unwrap();
        assert_eq!(banks.len(), 1);
        assert_eq!(banks[0]["id"], "factory.rf106");
        assert_eq!(presets.len(), 128);
        assert!(
            presets
                .iter()
                .all(|preset| preset["bank"] == "factory.rf106")
        );
    }
}
