use std::{env, error::Error, f32::consts::TAU, path::PathBuf};

use rf_106_chorus::mn3009_nominal_transfer;

const SAMPLE_RATE: u32 = 48_000;
const DURATION_SECONDS: f32 = 6.0;
const OLD_DRIVE: f32 = 0.12;

fn legacy_transfer(input: f32) -> f32 {
    (input * OLD_DRIVE).tanh() / OLD_DRIVE
}

fn source_sample(index: usize) -> f32 {
    let time = index as f32 / SAMPLE_RATE as f32;
    let frequencies = [130.8128_f32, 164.8138, 195.9977, 261.6256];
    let mut sample = 0.0;
    for (voice, frequency) in frequencies.into_iter().enumerate() {
        let detune = 1.0 + (voice as f32 - 1.5) * 0.0009;
        for harmonic in 1..=24 {
            let harmonic_frequency = frequency * detune * harmonic as f32;
            if harmonic_frequency >= 16_000.0 {
                break;
            }
            sample += (TAU * harmonic_frequency * time).sin() / harmonic as f32;
        }
    }
    let fade = (time / 0.08).min(1.0) * ((DURATION_SECONDS - time) / 0.25).clamp(0.0, 1.0);
    sample * (0.8 / 6.2) * fade
}

fn render(path: PathBuf, transfer: fn(f32) -> f32) -> Result<(), Box<dyn Error>> {
    let specification = hound::WavSpec {
        channels: 2,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 24,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create(path, specification)?;
    let full_scale = ((1_i32 << 23) - 1) as f32;
    for index in 0..(SAMPLE_RATE as f32 * DURATION_SECONDS) as usize {
        let output = transfer(source_sample(index)).clamp(-1.0, 1.0);
        let encoded = (output * full_scale).round() as i32;
        writer.write_sample(encoded)?;
        writer.write_sample(encoded)?;
    }
    writer.finalize()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let output_dir = env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("artifacts/audio"));
    std::fs::create_dir_all(&output_dir)?;
    render(output_dir.join("mn3009-before.wav"), legacy_transfer)?;
    render(output_dir.join("mn3009-after.wav"), mn3009_nominal_transfer)?;
    let mut source_power = 0.0_f64;
    let mut difference_power = 0.0_f64;
    let sample_count = (SAMPLE_RATE as f32 * DURATION_SECONDS) as usize;
    for index in 0..sample_count {
        let source = source_sample(index);
        let before = legacy_transfer(source);
        let after = mn3009_nominal_transfer(source);
        source_power += f64::from(source * source);
        difference_power += f64::from((after - before) * (after - before));
    }
    let source_rms = (source_power / sample_count as f64).sqrt();
    let difference_rms = (difference_power / sample_count as f64).sqrt();
    let difference_db = 20.0 * (difference_rms / source_rms).log10();
    println!(
        "Rendered MN3009 transfer comparison to {}",
        output_dir.display()
    );
    println!("A/B nonlinear difference: {difference_db:.2} dB relative to the source RMS");
    Ok(())
}
