use crate::cli::CodecChoice;
use crate::vox::Vox;

pub fn process_codec(input: Vec<i16>, codec: &CodecChoice) -> Vec<i16> {
    // -------- CHOOSE/APPLY CODEC --------
    match *codec {
        CodecChoice::Vox => {
            // Vox encode/decode
            let mut vox = Vox::new();
            
            let output: Vec<u8> = input.iter().map(|s| { vox.vox_encode(s) }).collect();
            
            output.iter().map(|s| { vox.vox_decode(s) }).collect()
        }
        // ,CodecChoice::Gsm => {},
    }
}
