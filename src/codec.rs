use crate::cli::CodecChoice;
use crate::vox::Vox;

pub fn process_codec(input: Vec<i16>, codec: &CodecChoice) -> Vec<i16> {
    match *codec {
        CodecChoice::Vox => {
            // create Vox
            let mut vox = Vox::new();
            // encode samples and collect
            let output: Vec<u8> = input.iter().map(|s| { vox.vox_encode(s) }).collect();
            // decode samples, collect, and return
            output.iter().map(|s| { vox.vox_decode(s) }).collect()
        }
        // ,CodecChoice::Gsm => {},
    }
}
