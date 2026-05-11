mod cli;

use percent_encoding::AsciiSet;
use std::{
    borrow::Cow,
    io::{self, Read, Write},
};

use crate::cli::{Args, Preset};

fn write_bytes(bytes: &[u8]) {
    io::stdout()
        .write_all(&bytes)
        .expect("Failed to write to stdout");
}

fn main() {
    let args = Args::parse();

    let input = {
        let mut vec = Vec::new();
        io::stdin()
            .read_to_end(&mut vec)
            .expect("Failed to read from stdin");
        vec
    };

    const ALL: &AsciiSet = &AsciiSet::EMPTY.complement();

    if args.decode {
        write_bytes(&percent_encoding::percent_decode(&input).collect::<Vec<u8>>());
    } else {
        let charset = match args.preset {
            Preset::Control => percent_encoding::NON_ALPHANUMERIC,
            Preset::NonAlphanumeric => percent_encoding::CONTROLS,
            Preset::All => ALL,
            Preset::None => &AsciiSet::EMPTY,
        };
        write_bytes(
            percent_encoding::percent_encode(&input, charset)
                .collect::<Cow<str>>()
                .as_bytes(),
        );
    };
}
