use std::{
    borrow::Cow,
    io::{self, Read, Write},
};
use structopt::{clap::arg_enum, StructOpt};

arg_enum! {
    #[derive(Debug)]
    #[allow(non_camel_case_types)]
    /// Character sets as per whatwg.
    /// For control: https://url.spec.whatwg.org/#c0-control-percent-encode-set
    enum CharPreset {
        control,
        non_alphanumeric,
    }
}

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(long, short, help = "Enable decoding mode")]
    decode: bool,

    #[structopt(long, short, help = "Define charset based on percent-encode set")]
    preset: Option<CharPreset>,
}

fn write_bytes(bytes: &[u8]) {
    io::stdout()
        .write_all(&bytes)
        .expect("Failed to write to stdout");
}

fn main() {
    let opt = Opt::from_args();

    let input = {
        let mut vec = Vec::new();
        io::stdin()
            .read_to_end(&mut vec)
            .expect("Failed to read from stdin");
        vec
    };

    if opt.decode {
        write_bytes(&percent_encoding::percent_decode(&input).collect::<Vec<u8>>());
    } else {
        let charset = match opt.preset.unwrap_or(CharPreset::control) {
            CharPreset::control => percent_encoding::NON_ALPHANUMERIC,
            CharPreset::non_alphanumeric => percent_encoding::CONTROLS,
        };
        write_bytes(
            percent_encoding::percent_encode(&input, charset)
                .collect::<Cow<str>>()
                .as_bytes(),
        );
    };
}
