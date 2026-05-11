use std::{fmt, process};

use urlcode::Preset;

pub struct Args {
    pub decode: bool,
    pub preset: Preset,
    pub separator: &'static str,
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
        let mut preset = Preset::X_WWW_FORM_URLENCODED;
        let mut separator = "\n";

        while let Some(arg) = parser.next()? {
            use lexopt::Arg::*;

            match arg {
                Short('d') | Long("decode") => decode = true,
                Short('p') | Long("preset") => {
                    preset = match parser.value()?.to_ascii_lowercase().as_encoded_bytes() {
                        b"control" => Preset::CONTROL,
                        b"all" => Preset::ALL,
                        b"none" => Preset::NONE,
                        b"fragment" => Preset::FRAGMENT,
                        b"query" => Preset::QUERY,
                        b"squery" => Preset::SPECIAL_QUERY,
                        b"path" => Preset::PATH,
                        b"userinfo" => Preset::USERINFO,
                        b"component" => Preset::COMPONENT,
                        b"urlencoded" => Preset::X_WWW_FORM_URLENCODED,
                        _ => return Err("unknown preset".into()),
                    };
                }
                Short('l') | Long("list-presets") => {
                    print!("{}", ListPresets);
                    process::exit(0);
                }
                Short('e') | Long("encoded") => {
                    let chars = parser.value()?;
                    let chars = chars.as_encoded_bytes();
                    ensure_ascii(chars)?;
                    preset.set_encode_charset(preset.encode_charset().add_all(chars));
                }
                Short('u') | Long("unencoded") => {
                    let chars = parser.value()?;
                    let chars = chars.as_encoded_bytes();
                    ensure_ascii(chars)?;
                    preset.set_encode_charset(preset.encode_charset().remove_all(chars));
                }
                Short('s') | Long("space-plus") => preset.set_space_as_plus(true),
                Short('S') | Long("no-space-plus") => preset.set_space_as_plus(false),
                Long("ascii-only") => preset.set_ascii_only(true),
                Short('0') | Long("null-delimiter") => separator = "\0",
                Short('n') | Long("no-delimiter") => separator = "",
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

        Ok(Self {
            decode,
            preset,
            separator,
        })
    }
}

fn ensure_ascii(chars: &[u8]) -> Result<(), lexopt::Error> {
    for &c in chars {
        if !c.is_ascii() {
            return Err("cannot process non-ascii charset".into());
        }
    }
    Ok(())
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
    {NAME} [-d] [-p PRESET] [OPTIONS...]

OPTIONS:
  -d, --decode              Enable decoding mode.
  -p, --preset=PRESET       Define charset based on percent-encode set (default: urlencoded).
  -l, --list-presets        List available presets.
  -e, --encoded=CHARS       Mark ascii characters for percent-encoding.
  -u, --unencoded=CHARS     Unmark ascii characters for percent-encoding.
  -s, --space-plus          Encode spaces with a single '+'.
  -S, --no-space-plus       Do not encode spaces with a single '+'.
      --ascii-only          Encode all non-ascii characters.

  -0, --null-delimiter      Delimit results with null bytes.
  -n, --no-delimiter        Do not delimit results.

  -h, --help                Print help information.
  -V, --version             Print version information.
"
        )
    }
}

struct ListPresets;

impl fmt::Display for ListPresets {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_preset(f, "none", &Preset::NONE)?;
        self.fmt_preset(f, "control", &Preset::CONTROL)?;
        self.fmt_preset(f, "fragment", &Preset::FRAGMENT)?;
        self.fmt_preset(f, "query", &Preset::QUERY)?;
        self.fmt_preset(f, "squery", &Preset::SPECIAL_QUERY)?;
        self.fmt_preset(f, "path", &Preset::PATH)?;
        self.fmt_preset(f, "userinfo", &Preset::USERINFO)?;
        self.fmt_preset(f, "component", &Preset::COMPONENT)?;
        self.fmt_preset(f, "urlencoded", &Preset::X_WWW_FORM_URLENCODED)?;
        self.fmt_preset(f, "all", &Preset::ALL)?;

        Ok(())
    }
}

impl ListPresets {
    fn fmt_preset(&self, f: &mut fmt::Formatter<'_>, name: &str, preset: &Preset) -> fmt::Result {
        write!(f, "{name}:")?;

        if preset.space_as_plus() {
            write!(f, " space-plus")?;
        }

        if preset.ascii_only() {
            write!(f, " ascii-only")?;
        }

        let charset = preset.encode_charset();

        write!(f, "\n  unencoded: \"")?;
        for c in charset.complement() {
            let c = char::from_u32(c as _).unwrap().escape_debug();
            write!(f, "{c}")?;
        }
        write!(f, "\"\n\n")
    }
}
