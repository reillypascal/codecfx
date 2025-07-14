use clap::{Parser, ValueEnum};

// -------- CLI PARSER --------
#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short = 'i', long, default_value_t = String::from("input"))]
    pub input: String,

    #[arg(short = 'o', long, default_value_t = String::from("output"))]
    pub output: String,

    #[arg(short = 's', long, default_value_t = 8000)]
    pub samplerate: u32,

    //#[arg(short = 'S', long, default_value_t = 44100)]
    //pub codec_sr: u32,
    #[clap(short = 'c', long, value_enum, default_value_t=CodecChoice::Vox)]
    pub codec: CodecChoice,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum CodecChoice {
    Vox,
    // Gsm,
}
