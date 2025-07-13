# codecfx
Convert .WAV files to a telecommunication codec and back again. Useful to apply the character of a codec to a sample.

## Usage
This project uses Rust's [cargo](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html) package manager. After [installing Rust](https://doc.rust-lang.org/book/ch01-01-installation.html#installation), you can run the command `cargo run` from the code folder. 

The code will default to expecting your input file(s) and/or folder(s) to be in the `input` subfolder, and will write .WAV files to the `output` sub-folder. Here are the commands to change the default options:
  - `-h, --help`        show this help message and exit
  - `-i, --input`       subfolder in which to look for files to import (string)
  - `-o, --output`      subfolder in which to write .WAV files (string)
  - `-s, --samplerate`  sample rate for codec
  - `-f, --format`      sample format in which to read the files (string: only 'vox' available for now; more to come)
<!--
  reading/writing .WAV files (int)
- `-S, --codec-sr`    sample rate for codec (int) -->

### Usage Examples
- Note the extra two dashes (`--`) between `cargo run` and the command-line options. This sends your options to the running program, rather than to cargo.

- Only accept files 1MB or larger; read as unsigned 8-bit integer values
```sh
cargo run -- -S 8000 -f 'vox'
```

- Read files from the `data` subfolder and output them to the code folder, rather than a subfolder
```sh
cargo run -- -i "samples" -o "."
```

