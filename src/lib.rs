mod charset;
mod hex;

pub use charset::AsciiCharset;

/// A percent-encoding preset.
///
/// This is an [`AsciiCharset`] representing the characters to encode and a couple flags.
pub struct Preset {
    encode_charset: AsciiCharset,
    flags: u8,
}

impl Preset {
    /// Space should be encoded/decoded as a `+` character.
    const FLAG_SPACE_AS_PLUS: u8 = 1 << 0;

    /// Non-ascii characters should be encoded.
    const FLAG_ASCII_ONLY: u8 = 1 << 1;
}

impl Preset {
    pub const CONTROL: Self = Self::new(AsciiCharset::CONTROL);
    pub const FRAGMENT: Self = Self::new(AsciiCharset::FRAGMENT);
    pub const QUERY: Self = Self::new(AsciiCharset::QUERY);
    pub const SPECIAL_QUERY: Self = Self::new(AsciiCharset::SPECIAL_QUERY);
    pub const PATH: Self = Self::new(AsciiCharset::PATH);
    pub const USERINFO: Self = Self::new(AsciiCharset::USERINFO);
    pub const COMPONENT: Self = Self::new(AsciiCharset::COMPONENT);
    pub const X_WWW_FORM_URLENCODED: Self = {
        let mut ret = Self::new(AsciiCharset::X_WWW_FORM_URLENCODED);
        ret.set_space_as_plus(true);
        ret
    };

    // Percent-encodes all bytes.
    pub const ALL: Self = {
        let mut ret = Self::new(AsciiCharset::ALL);
        ret.set_ascii_only(true);
        ret
    };

    // Percent-encodes no bytes.
    pub const NONE: Self = Self::new(AsciiCharset::EMPTY);
}

impl Preset {
    /// Create a new preset.
    pub const fn new(encode_charset: AsciiCharset) -> Self {
        let flags = 0;
        Self {
            encode_charset,
            flags,
        }
    }

    /// Set the set of ascii characters that are expected to be percent-encoded.
    pub const fn set_encode_charset(&mut self, encode_charset: AsciiCharset) {
        self.encode_charset = encode_charset;
    }

    /// Get the set of ascii characters that are expected to be percent-encoded.
    pub const fn encode_charset(&self) -> AsciiCharset {
        self.encode_charset
    }

    /// Encodes spaces as '+' when set.
    ///
    /// Th url spec does not mention what decoding with this setting means, but this implementation
    /// will decode '+' into ' ' when this option is set.
    ///
    /// Defaults to false, though needs to be set for `application/x-www-form-urlencoded`.
    pub const fn set_space_as_plus(&mut self, space_as_plus: bool) {
        match space_as_plus {
            true => self.flags |= Self::FLAG_SPACE_AS_PLUS,
            false => self.flags &= !Self::FLAG_SPACE_AS_PLUS,
        }
    }

    /// Get the value of the 'space as plus' setting.
    pub const fn space_as_plus(&self) -> bool {
        (self.flags & Self::FLAG_SPACE_AS_PLUS) != 0
    }

    /// Encodes all non-ascii bytes when set.
    ///
    /// Defaults to false, as the [spec](https://url.spec.whatwg.org/#percent-encoded-bytes) would
    /// suggest.
    pub const fn set_ascii_only(&mut self, ascii_only: bool) {
        match ascii_only {
            true => self.flags |= Self::FLAG_ASCII_ONLY,
            false => self.flags &= !Self::FLAG_ASCII_ONLY,
        }
    }

    /// Get the value of the 'ascii only' setting.
    pub const fn ascii_only(&self) -> bool {
        (self.flags & Self::FLAG_ASCII_ONLY) != 0
    }
}

impl Preset {
    /// Appends percent-encoded bytes from `src` into `dst`.
    pub fn encode_into(&self, src: &[u8], dst: &mut Vec<u8>) {
        let space_as_plus = self.space_as_plus();
        let ascii_only = self.ascii_only();

        for &c in src {
            if c == b' ' && space_as_plus {
                dst.push(b'+');
                continue;
            }

            if self.encode_charset.contains(c) || (ascii_only && !c.is_ascii()) {
                dst.reserve(3);
                dst.push(b'%');
                let (hi, lo) = hex::to_hex(c);
                dst.push(hi);
                dst.push(lo);
            } else {
                dst.push(c);
            }
        }
    }

    /// Appends percent-decoded bytes from `src` into `dst`.
    pub fn decode_into(&self, src: &[u8], dst: &mut Vec<u8>) {
        let mut src = src.iter().copied();

        let space_as_plus = self.space_as_plus();
        while let Some(c) = src.next() {
            // Process single bytes.
            if c != b'%' {
                if space_as_plus && (c == b'+') {
                    dst.push(b' ');
                } else {
                    dst.push(c);
                }

                continue;
            }

            // Decode a %-encoded byte.
            let mut peek = src.clone();
            if let Some(hi) = peek.next()
                && let Some(lo) = peek.next()
                && let Some(byte) = hex::from_hex(hi, lo)
            {
                dst.push(byte);
                src = peek;
            } else {
                dst.push(c);
            }
        }
    }
}
