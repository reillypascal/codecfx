use std::ffi::OsStr;
use std::path::PathBuf;

use clap::Parser;
use walkdir::WalkDir;

use crate::biquad::AudioFilter;
use crate::codec_processor::process_codec;
use crate::wav::{read_file_as_wav, write_file_as_wav};

pub mod biquad;
pub mod cli;
pub mod codec_processor;
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
            // -------- MAKE FILTER --------
            let mut filter = AudioFilter::new(biquad::FilterAlgorithm::Hpf2, 20.0, 0.707, 0.0, args.samplerate);
            
            // -------- READ FILE & PROCESS --------
            let input = read_file_as_wav(entry.path());
            
            let mut output = process_codec(input, &args.format);
            
            for sample in &mut output {
                *sample = filter.process_sample(*sample as f64) as i16;
            }
            
            // -------- WRITE FILE --------
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
