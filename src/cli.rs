use std::{fmt, process};

pub struct Args {
    pub decode: bool,
    pub preset: Preset,
}

pub enum Preset {
    Control,
    NonAlphanumeric,
    All,
    None,
}

impl Args {
    pub fn parse() -> Self {
        match Self::try_parse() {
            Ok(args) => args,
            Err(err) => {
                eprintln!("{}\nerror: {err}", Help);
                process::exit(1);
            }
        }
    }

    fn try_parse() -> Result<Self, lexopt::Error> {
        let mut parser = lexopt::Parser::from_env();

        let mut decode = false;
        let mut preset = Preset::Control;

        while let Some(arg) = parser.next()? {
            use lexopt::Arg::*;

            match arg {
                Short('d') | Long("decode") => decode = true,
                Short('p') | Long("preset") => {
                    preset = match parser.value()?.as_encoded_bytes() {
                        b"control" => Preset::Control,
                        b"non_alphanumeric" => Preset::NonAlphanumeric,
                        b"all" => Preset::All,
                        b"none" => Preset::None,
                        _ => return Err("unknown parser preset".into()),
                    }
                }
                Short('h') | Long("help") => {
                    print!("{}", Help);
                    process::exit(0)
                }
                Short('V') | Long("version") => {
                    print!("{}", Version);
                    process::exit(0)
                }
                arg => return Err(arg.unexpected()),
            }
        }

        Ok(Self { decode, preset })
    }
}

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

struct Version;

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{NAME} {VERSION}")
    }
}

struct Help;

impl fmt::Display for Help {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\
USAGE:
    {NAME} [OPTIONS...]

OPTIONS:
  -h, --help          Print help information.
  -V, --version       Print version information.
  
  -d, --decode        Enable decoding mode.
  -p, --preset=PRESET Define charset based on percent-encode set.
                        One of: all, none, control, non_alphanumeric
"
        )
    }
}
