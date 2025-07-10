// use std::fs;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

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
            // can do let chains in Rust 1.88
            if let Ok(metadata) = entry.metadata() && metadata.is_file() && match_ext(&entry, "wav") {
                // -------- READ FILE --------
                let input = read_file_as_wav(entry.path());
                let mut output: Vec<i16> = Vec::new();
                
                // -------- CHOOSE/APPLY CODEC --------
                match args.format {
                    CodecChoice::Vox => {
                        // Vox encode/decode
                        let mut vox = Vox::new();
                        let mut encoded: Vec<u8> = Vec::new();
                        
                        // encode
                        for sample in input {
                            encoded.push(vox.vox_encode(&sample));
                        };
                        // decode
                        for sample in encoded {
                            output.push(vox.vox_decode(&sample));
                        };
                    },
                    CodecChoice::Gsm => {},
                }

                // -------- FILTER --------
                let mut filter = biquad::AudioFilter::new();
                filter.calculate_filter_coeffs();
                
                for i in 0..output.len() {
                    output[i] = filter.process_sample(output[i] as f64) as i16;
                }
            
                // -------- WRITE FILE --------
                let mut write_path = PathBuf::from(&args.output);
                
                if let Some(file_name) = entry.path().file_name() { write_path.push(file_name); }
                
                write_file_as_wav(&output, &write_path, &args.samplerate);
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
    
    #[arg(short = 's', long, default_value_t = 44100)]
    samplerate: u32,

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

fn read_file_as_wav(path: &Path) -> Vec<i16> {
    let mut reader = hound::WavReader::open(path).expect("Error reading file");
    let input: Vec<i16> = reader.samples::<i16>().map(|s| s.expect("Could not read sample")).collect();
    
    input
}

fn write_file_as_wav(data: &Vec<i16>, path: &PathBuf, sample_rate: &u32) {
    // write WAV file
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
