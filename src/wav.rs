use hound::{self, WavSpec};
use std::path::{Path, PathBuf};

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
// &[i16] instead of &Vec<i16> - https://rust-lang.github.io/rust-clippy/master/index.html#ptr_arg
pub fn write_file_as_wav(data: &[i16], path: &PathBuf, spec: &WavSpec) -> Result<(), hound::Error> {
    // writer
    let mut writer = hound::WavWriter::create(path, *spec)?; //.expect("Could not create writer");
    for sample in data {
        writer.write_sample(*sample)?; //.expect("Could not write sample");
    }
    writer.finalize()?; //.expect("Could not finalize WAV file");

    Ok(())
}
