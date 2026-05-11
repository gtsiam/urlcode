mod cli;

use std::io::{self, Read, Write};

fn main() {
    let args = cli::Args::parse();

    let input = {
        let mut vec = Vec::new();
        io::stdin()
            .read_to_end(&mut vec)
            .expect("Failed to read from stdin");
        vec
    };

    let mut output = Vec::new();
    if args.decode {
        args.preset.decode_into(&input, &mut output);
    } else {
        args.preset.encode_into(&input, &mut output);
    };
    output.extend_from_slice(args.separator.as_bytes());

    io::stdout()
        .write_all(&output)
        .expect("Failed to write to stdout");
}
