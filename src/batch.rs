use std::ffi::OsStr;
use std::path::PathBuf;

use hound::WavSpec;
use walkdir::WalkDir;

use crate::biquad::{AudioFilter, AudioFilterParameters};
use crate::cli::Args;
use crate::codec::process_codec;
use crate::wav::{read_file_as_wav, write_file_as_wav};

pub fn process_batch(args: &Args, filter_params: &AudioFilterParameters, wav_spec: &WavSpec) {
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
            let mut filter = AudioFilter::new(&filter_params, args.samplerate);
            filter.calculate_filter_coeffs();
            
            // read file
            let mut input: Vec<i16> = Vec::new();
            // will both (try to) read the file and match the result, printing if there is an error
            match read_file_as_wav(entry.path()) {
                Ok(file) => { input = file; },
                Err(e) => { eprintln!("Error reading {:?} as .WAV file: {}", entry.path(), e); },
            };
            
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
                // will both (try to) write the file and match the result, printing if there is an error
                match write_file_as_wav(&output, &write_path, &wav_spec) {
                    Ok(_) => {},
                    Err(e) => { eprintln!("Error writing {:?} as .WAV file: {}", entry.path(), e); },
                };
            }
        });
}

fn match_ext(file: &walkdir::DirEntry, ext: &str) -> bool {
    file.path().extension().and_then(OsStr::to_str).map(|e| e == ext).unwrap_or(false)
}
