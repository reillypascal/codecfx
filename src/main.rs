use std::fs;
use hound;
use vox::VoxState;

pub mod vox;

fn main() {
    let data: Vec<u8> = fs::read("input/im-afraid-not.wav").expect("Error reading file");
    
    let mut output: Vec<i16> = Vec::new();
    let mut vox_state = VoxState::new();
    
    for i in 0..data.len() {
        for nibble in [(data[i] >> 4) & 0xf, data[i] & 0xf].iter() {
            // vox output is 12-bit, from i16::MIN <-> i16::MAX/2
            // *don't* shift — changes spectrum, envelope!
            output.push(vox_state.vox_decode(nibble));
        }
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
    let mut writer = hound::WavWriter::create("output/im-afraid-not.wav", spec).expect("Could not create writer");
    for t in 0..output.len() {
        writer.write_sample(output[t]).expect("Could not write sample");
    }
    writer.finalize().expect("Could not finalize WAV file");
}
