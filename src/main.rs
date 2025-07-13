// use std::ffi::OsStr;
// use std::path::PathBuf;

use clap::Parser;
use hound::{SampleFormat, WavSpec};
// use walkdir::WalkDir;

use crate::batch::process_batch;
use crate::biquad::{AudioFilterParameters, FilterAlgorithm};
use crate::cli::Args;
// use crate::codec::process_codec;
// use crate::wav::{read_file_as_wav, write_file_as_wav};

pub mod batch;
pub mod biquad;
pub mod cli;
pub mod codec;
pub mod vox;
pub mod wav;

fn main() {
    // set up args, filter, wav spec
    let args = Args::parse();
    let filter_params = AudioFilterParameters::new(FilterAlgorithm::Hpf2, 20.0, 0.707, 0.0);
    let mut wav_spec = WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };

    // get & process files
    process_batch(&args, &filter_params, &mut wav_spec);
}
