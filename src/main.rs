use std::{
    borrow::Cow,
    io::{self, Read, Write},
};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(long, short)]
    decode: bool,
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
        write_bytes(
            percent_encoding::percent_encode(&input, percent_encoding::NON_ALPHANUMERIC)
                .collect::<Cow<str>>()
                .as_bytes(),
        );
    };
}
