# Url coding utility

[![crates.io](https://img.shields.io/crates/v/urlcode.svg)](https://crates.io/crates/urlcode)
[![License](https://img.shields.io/github/license/gtsiam/urlcode)](https://github.com/gtsiam/urlcode/blob/master/LICENSE)

This is a convenience tool for managing percent-encoding of arbitrary data from the command line,
inspired by the `basenc` utility.

Examples:

```console
$ echo -n "hello there/ asd23@#%23" | urlcode
hello+there%2F+asd23%40%23%2523
$ echo -n "hello+there%2F+asd23%40%23%2523" | urlcode -d
hello there/ asd23@#%23
```

It is fairly customizable:
```
USAGE:
    urlcode [-d] [-p PRESET] [OPTIONS...]

OPTIONS:
  -d, --decode              Enable decoding mode.
  -p, --preset=PRESET       Define charset based on percent-encode set (default: urlencoded).
  -l, --list-presets        List available presets.
  -e, --encoded=CHARS       Mark ascii characters for percent-encoding.
  -u, --unencoded=CHARS     Unmark ascii characters for percent-encoding.
  -s, --space-plus          Encode spaces with a single '+'.
  -S, --no-space-plus       Do not encode spaces with a single '+'.
      --ascii-only          Encode all non-ascii characters.

  -h, --help                Print help information.
  -V, --version             Print version information.
```

For instance, you can encode every character:
```console
$ echo -n 'a sdf' | urlcode -pall
%61%20%73%64%66
```

With spaces as pluses (as is done by default on the `urlencoded` preset):
```console
$ echo -n 'a sdf' | urlcode -pall -s
%61+%73%64%66
```

Or only specific characters:
```console
$ echo -n 'a sdf' | urlcode -pnone -ead
%61 s%64f
```

By default, non-ascii bytes are left as-is:
```console
$ echo -n 'abc αβγ' | urlcode
abc+αβγ
```

You can disable this behaviour (though for UTF-8 it is not recommended):
```console
$ echo -n 'abc αβγ' | urlcode --ascii-only
abc+%CE%B1%CE%B2%CE%B3
```

Note: There are no checks to ensure the input is valid UTF-8, as the encoder works on a byte-by-byte
basis. Non-ascii bytes are simply those with codepoints >=128.
