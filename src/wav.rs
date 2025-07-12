use std::path::{Path, PathBuf};
use hound;

pub fn read_file_as_wav(path: &Path) -> Vec<i16> {
    // reader
    let mut reader = hound::WavReader::open(path).expect("Error reading file");
    // use reader
    let input: Vec<i16> = reader.samples::<i16>().map(|s| s.expect("Could not read sample")).collect();
    // return
    input
}

pub fn write_file_as_wav(data: &Vec<i16>, path: &PathBuf, sample_rate: &u32) {
    // spec
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: *sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    
    // writer
    let mut writer = hound::WavWriter::create(path, spec).expect("Could not create writer");
    for t in 0..data.len() {
        writer.write_sample(data[t]).expect("Could not write sample");
    }
    writer.finalize().expect("Could not finalize WAV file");
}
