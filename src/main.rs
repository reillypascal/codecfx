// use std::ffi::OsStr;
// use std::path::PathBuf;

use clap::Parser;
use hound::{SampleFormat, WavSpec};
// use walkdir::WalkDir;

use crate::biquad::{AudioFilterParameters, FilterAlgorithm};
use crate::cli::Args;
// use crate::codec::process_codec;
use crate::walk_dir::walk_dir;
// use crate::wav::{read_file_as_wav, write_file_as_wav};

pub mod biquad;
pub mod cli;
pub mod codec;
pub mod vox;
pub mod walk_dir;
pub mod wav;

fn main() {
    // set up args, filter, wav spec
    let args = Args::parse();
    let filter_params = AudioFilterParameters::new(FilterAlgorithm::Hpf2, 20.0, 0.707, 0.0);
    let wav_spec = WavSpec {
        channels: 1,
        sample_rate: args.samplerate,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };

    // get & process files 
    walk_dir(&args, &filter_params, &wav_spec);
}
