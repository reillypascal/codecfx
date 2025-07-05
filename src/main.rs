use std::fs;
use std::ffi::OsStr;
use std::path::{self, PathBuf};

use clap::{Parser, ValueEnum};
use hound;
use walkdir::WalkDir;

use vox::Vox;

pub mod biquad;
pub mod vox;

fn main() {
    // -------- CLI ARGS --------
    let args = Args::parse();

    // -------- GET & PROCESS FILES --------
    WalkDir::new(&args.input)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .for_each(|entry| {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_file() && match_ext(&entry, "wav") {
                    let mut reader = hound::WavReader::open(entry.path()).expect("Error reading file");
                    let input = reader.samples::<i16>();
                    
                    let mut encoded: Vec<u8> = Vec::new();
                    let mut output: Vec<i16> = Vec::new();
                
                    let mut filter = biquad::AudioFilter::new();
                    filter.calculate_filter_coeffs();
                    
                    // -------- CHOOSE CODEC --------
                    match args.format {
                        CodecChoice::Vox => {
                            // Vox encode/decode
                            let mut vox = Vox::new();
                        
                            // for sample in input
                            input.for_each(|sample| {
                                encoded.push(vox.vox_encode(&sample.expect("Zrror reading sample")));
                            });
                            // could do as for in; reader works well w/ for_each(), so doing similar here
                            encoded.iter().for_each(|sample| {
                                output.push(vox.vox_decode(&sample));
                            });
                        },
                        CodecChoice::Gsm => {},
                    }

                    // -------- FILTER --------
                    for i in 0..output.len() {
                        output[i] = filter.process_sample(output[i] as f64) as i16;
                    }
                
                    let mut write_path = PathBuf::from(&args.output);
                    
                    if let Some(file_name) = entry.path().file_name() {
                        write_path.push(file_name);
                        // write WAV file
                        // spec
                        let spec = hound::WavSpec {
                            channels: 1,
                            sample_rate: 44100,
                            bits_per_sample: 16,
                            sample_format: hound::SampleFormat::Int,
                        };
                        
                        // writer
                        let mut writer = hound::WavWriter::create(write_path, spec).expect("Could not create writer");
                        for t in 0..output.len() {
                            writer.write_sample(output[t]).expect("Could not write sample");
                        }
                        writer.finalize().expect("Could not finalize WAV file");
                    }
                }
            }
    });
}

// -------- CLI PARSER --------
#[derive(Parser, Debug)]
struct Args {
    #[arg(short = 'i', long, default_value_t = String::from("input"))]
    input: String,

    #[arg(short = 'o', long, default_value_t = String::from("output"))]
    output: String,

    #[clap(short = 'f', long, value_enum, default_value_t=CodecChoice::Vox)]
    format: CodecChoice,
}

#[derive(ValueEnum, Clone, Debug)]
enum CodecChoice {
    Vox,
    Gsm,
}

// -------- HELPER FNS --------
fn match_ext(file: &walkdir::DirEntry, ext: &str) -> bool {
    file.path().extension().and_then(OsStr::to_str).map(|e| e == ext).unwrap_or(false)
}
