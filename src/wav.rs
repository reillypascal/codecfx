use std::path::{Path, PathBuf};
use hound::{self, WavSpec};

pub fn read_file_as_wav(path: &Path) -> Result<Vec<i16>, hound::Error> {
    // // reader
    // let mut reader = hound::WavReader::open(path).expect("Error reading file");
    // // use reader
    // let input: Vec<i16> = reader.samples::<i16>().map(|s| s.expect("Could not read sample")).collect();
    // ? operator is like match expression for Result
    let input: Vec<i16> = hound::WavReader::open(path)?
        .samples::<i16>()
        .collect::<Result<Vec<i16>, hound::Error>>()?;
    // return
    Ok(input)
}

pub fn write_file_as_wav(data: &Vec<i16>, path: &PathBuf, spec: &WavSpec) -> Result<(), hound::Error> {
    // writer
    let mut writer = hound::WavWriter::create(path, *spec)?;//.expect("Could not create writer");
    for t in 0..data.len() {
        writer.write_sample(data[t])?;//.expect("Could not write sample");
    }
    writer.finalize()?;//.expect("Could not finalize WAV file");
    
    Ok(())
}
