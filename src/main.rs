use std::fs;
use hound;
use vox::Vox;

pub mod biquad;
pub mod vox;

fn main() {
    let data: Vec<u8> = fs::read("input/beat_1_bip_2_F_mono.wav").expect("Error reading file");
    
    let mut encoded: Vec<u8> = Vec::new();
    let mut output: Vec<i16> = Vec::new();

    let mut filter = biquad::AudioFilter::new();
    filter.calculate_filter_coeffs();

    let input: Vec<i16> = data.chunks_exact(2)
        .map(|chunks| {
            i16::from_le_bytes(chunks.try_into().expect("Could not convert file into 16-bit Vec"))
        })
        .collect();
    
    // Vox encode/decode
    let mut vox = Vox::new();

    for sample in input {
        encoded.push(vox.vox_encode(&sample));
    }

    for sample in encoded {
        let decoded = vox.vox_decode(&sample);
        output.push(filter.process_sample(decoded as f64) as i16);
    }
    
    // write WAV file
    // spec
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    
    // writer
    let mut writer = hound::WavWriter::create("output/output.wav", spec).expect("Could not create writer");
    for t in 0..output.len() {
        writer.write_sample(output[t]).expect("Could not write sample");
    }
    writer.finalize().expect("Could not finalize WAV file");
}
