use std::ffi::OsStr;
use std::path::PathBuf;

use clap::Parser;
use walkdir::WalkDir;

use crate::biquad::{AudioFilter, FilterAlgorithm};
use crate::codec::process_codec;
use crate::wav::{read_file_as_wav, write_file_as_wav};

pub mod biquad;
pub mod cli;
pub mod codec;
pub mod vox;
pub mod wav;

fn main() {
    // -------- CLI ARGS --------
    let args = cli::Args::parse();

    // -------- GET & PROCESS FILES --------
    WalkDir::new(&args.input)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            match entry.metadata() {
                Ok(metadata) => {
                    metadata.is_file() && match_ext(&entry, "wav")
                },
                Err(_) => { false },
            }
        })
        .for_each(|entry| {
            // make filter, calculate coeffs
            let mut filter = AudioFilter::new(FilterAlgorithm::Hpf2, 20.0, 0.707, 0.0, args.samplerate);
            filter.calculate_filter_coeffs();
            
            // read file
            let input = read_file_as_wav(entry.path());
            // apply codec
            let mut output = process_codec(input, &args.format);
            // apply filter
            for sample in &mut output {
                *sample = filter.process_sample(*sample as f64) as i16;
            }
            
            // make write path and write .WAV file
            let mut write_path = PathBuf::from(&args.output);
            
            if let Some(file_name) = entry.path().file_name() { 
                write_path.push(file_name); 
                write_file_as_wav(&output, &write_path, &args.samplerate);
            }
        });
}

// -------- HELPER FNS --------
fn match_ext(file: &walkdir::DirEntry, ext: &str) -> bool {
    file.path().extension().and_then(OsStr::to_str).map(|e| e == ext).unwrap_or(false)
}
