use hound;
use std::f32::consts::PI;
use std::path::Path;

const SAMPLE_RATE: u32 = 44100;
const AMPLITUDE: f32 = 0.3;

fn generate_tone(frequency: f32, duration_ms: u32, sample_rate: u32) -> Vec<f32> {
    let num_samples = (sample_rate * duration_ms / 1000) as usize;
    let mut samples = Vec::with_capacity(num_samples);

    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        let sample = (2.0 * PI * frequency * t).sin() * AMPLITUDE;

        let envelope = if i < num_samples / 10 {
            i as f32 / (num_samples as f32 / 10.0)
        } else if i > num_samples * 9 / 10 {
            (num_samples - i) as f32 / (num_samples as f32 / 10.0)
        } else {
            1.0
        };

        samples.push(sample * envelope);
    }

    samples
}

fn generate_silence(duration_ms: u32, sample_rate: u32) -> Vec<f32> {
    let num_samples = (sample_rate * duration_ms / 1000) as usize;
    vec![0.0; num_samples]
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output_path = Path::new("sounds/notification.wav");

    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create(output_path, spec)?;

    let bb4 = 2093.0;
    let c5 = 2498.0;
    let eb5 = 2794.0;
    let g7 = 3136.0;

    let dotted_quarter = 1125;
    let eighth = 375;
    let quarter = 750;

    let melody_parts = vec![
        ("rest", 0.0, eighth),
        ("note", bb4, eighth),
        ("note", c5, eighth),
        ("rest", 0.0, eighth),
        ("note", eb5, eighth),
        ("note", c5, eighth),
        ("rest", 0.0, eighth),
    ];

    for (part_type, freq, duration) in melody_parts {
        let samples = if part_type == "note" {
            generate_tone(freq, duration, SAMPLE_RATE)
        } else {
            generate_silence(duration, SAMPLE_RATE)
        };

        for sample in samples {
            let amplitude = (sample * i16::MAX as f32) as i16;
            writer.write_sample(amplitude)?;
        }
    }

    writer.finalize()?;

    println!("Generated notification sound at: {}", output_path.display());
    Ok(())
}
