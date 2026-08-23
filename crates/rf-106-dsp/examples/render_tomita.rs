use std::{env, error::Error, path::PathBuf};

use rf_106_dsp::Synth;

const SAMPLE_RATE: u32 = 48_000;
const TOMITA_B35: u32 = 84;
const DURATION_SECONDS: u32 = 6;
const NOTE_OFF_SECONDS: u32 = 4;

fn measure_program(program: u32, samples: u32, resonance: Option<f64>) -> (f32, f64) {
    let mut synth = Synth::default();
    assert!(synth.prepare(f64::from(SAMPLE_RATE)));
    assert!(synth.load_factory_preset(program));
    if let Some(resonance) = resonance {
        assert!(synth.set_parameter(11, resonance));
        for _ in 0..(SAMPLE_RATE as f32 * 3.0 * rf_106_voice::CONTROL_CYCLE_SECONDS) as u32 {
            synth.process_circuit_sample();
        }
    }
    synth.note_on(60, 127);
    let mut peak = 0.0_f32;
    let mut sum_squares = 0.0_f64;
    for _ in 0..samples {
        let (left, right) = synth.process_circuit_sample();
        peak = peak.max(left.abs()).max(right.abs());
        sum_squares += f64::from(left * left + right * right);
    }
    (peak, (sum_squares / f64::from(samples * 2)).sqrt())
}

fn measure_tomita_after_idle(idle_samples: u32) -> (f32, f64) {
    let mut synth = Synth::default();
    assert!(synth.prepare(f64::from(SAMPLE_RATE)));
    assert!(synth.load_factory_preset(TOMITA_B35));
    for _ in 0..idle_samples {
        synth.process_circuit_sample();
    }
    synth.note_on(60, 127);

    let mut peak = 0.0_f32;
    let mut sum_squares = 0.0_f64;
    for _ in 0..SAMPLE_RATE {
        let (left, right) = synth.process_circuit_sample();
        peak = peak.max(left.abs()).max(right.abs());
        sum_squares += f64::from(left * left + right * right);
    }
    (peak, (sum_squares / f64::from(SAMPLE_RATE * 2)).sqrt())
}

fn main() -> Result<(), Box<dyn Error>> {
    let output_dir = env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("artifacts/audio"));
    std::fs::create_dir_all(&output_dir)?;
    let output = output_dir.join("rf-106-b35-tomita.wav");

    let mut synth = Synth::default();
    assert!(synth.prepare(f64::from(SAMPLE_RATE)));
    assert!(synth.load_factory_preset(TOMITA_B35));
    synth.note_on(60, 127);

    let specification = hound::WavSpec {
        channels: 2,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 24,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create(&output, specification)?;
    let full_scale = ((1_i32 << 23) - 1) as f32;
    let mut peak = 0.0_f32;
    let mut sum_squares = 0.0_f64;
    let total_samples = SAMPLE_RATE * DURATION_SECONDS;

    for sample_index in 0..total_samples {
        if sample_index == SAMPLE_RATE * NOTE_OFF_SECONDS {
            synth.note_off(60);
        }
        let (left, right) = synth.process_circuit_sample();
        peak = peak.max(left.abs()).max(right.abs());
        sum_squares += f64::from(left * left + right * right);
        writer.write_sample((left.clamp(-1.0, 1.0) * full_scale).round() as i32)?;
        writer.write_sample((right.clamp(-1.0, 1.0) * full_scale).round() as i32)?;
    }
    writer.finalize()?;

    let rms = (sum_squares / f64::from(total_samples * 2)).sqrt();
    let (_, tomita_sustain_rms) = measure_program(TOMITA_B35, SAMPLE_RATE, None);
    let (_, a11_sustain_rms) = measure_program(0, SAMPLE_RATE, None);
    let relative_db = 20.0 * (tomita_sustain_rms / a11_sustain_rms).log10();
    println!("Rendered {}", output.display());
    println!("B35 peak: {peak:.6}; RMS: {rms:.6}");
    println!("B35 one-second RMS relative to A11: {relative_db:.2} dB");
    for raw_resonance in [0, 13, 64, 96, 112, 120, 123, 125, 127] {
        let (sweep_peak, sweep_rms) = measure_program(
            TOMITA_B35,
            SAMPLE_RATE * 3,
            Some(f64::from(raw_resonance) / 127.0),
        );
        println!("B35 RES {raw_resonance}/127: peak {sweep_peak:.6}; RMS {sweep_rms:.6}");
    }
    for idle_ms in [0, 10, 100, 1_000, 10_000] {
        let (idle_peak, idle_rms) = measure_tomita_after_idle(SAMPLE_RATE * idle_ms / 1_000);
        println!("B35 after {idle_ms} ms idle: peak {idle_peak:.6}; RMS {idle_rms:.6}");
    }
    Ok(())
}
