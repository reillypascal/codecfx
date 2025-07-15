use hound::WavSpec;
use rubato::{
    Resampler, SincFixedIn, SincInterpolationParameters, SincInterpolationType, WindowFunction,
};

use crate::cli::{Args, CodecChoice};
use crate::vox::Vox;

pub fn process_codec(input: Vec<i16>, args: &Args, wav_spec: &WavSpec) -> Vec<i16> {
    let in_resamp_params = SincInterpolationParameters {
        sinc_len: 256,
        f_cutoff: 0.95,
        interpolation: SincInterpolationType::Cubic,
        oversampling_factor: 256,
        window: WindowFunction::Blackman,
    };

    let mut resampler_in = SincFixedIn::<f64>::new(
        wav_spec.sample_rate as f64 / args.samplerate as f64,
        2.0,
        in_resamp_params,
        1024,
        wav_spec.channels as usize,
    )
    .expect("Could not create input resampler");

    let out_resamp_params = SincInterpolationParameters {
        sinc_len: 256,
        f_cutoff: 0.95,
        interpolation: SincInterpolationType::Cubic,
        oversampling_factor: 256,
        window: WindowFunction::Blackman,
    };

    let mut resampler_out = SincFixedIn::<f64>::new(
        args.samplerate as f64 / wav_spec.sample_rate as f64,
        2.0,
        out_resamp_params,
        1024,
        wav_spec.channels as usize,
    )
    .expect("Could not create output resampler");

    match args.codec {
        CodecChoice::Vox => {
            // create Vox
            let mut vox = Vox::new();
            // encode samples and collect
            let output: Vec<u8> = input.iter().map(|s| vox.vox_encode(s)).collect();
            // decode samples, collect, and return
            output.iter().map(|s| vox.vox_decode(s)).collect()
        } // ,CodecChoice::Gsm => {},
    }
}
