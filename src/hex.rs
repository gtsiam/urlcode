/// Converts a byte to (hi, lo) hex digits.
pub fn to_hex(byte: u8) -> (u8, u8) {
    const HEX_DIGITS: &[u8] = b"0123456789ABCDEF";
    (
        HEX_DIGITS[(byte >> 4) as usize],
        HEX_DIGITS[(byte & 0x0F) as usize],
    )
}

/// Converts a hi and lo hex digit to a byte.
pub fn from_hex(hi: u8, lo: u8) -> Option<u8> {
    Some((from_hex_digit(hi)? << 4) | from_hex_digit(lo)?)
}

/// Converts a hex digit into the corresponding value.
fn from_hex_digit(mut digit: u8) -> Option<u8> {
    // This relies heavily on the representation of ascii hexadecimal digits:
    // for 0..=9: 0011xxxx (with 0 starting at xxxx=0)
    // for a..=f: 0110xxxx (with a starting at xxxx=1)
    // for A..=F: 0100xxxx (with a starting at xxxx=1)

    // Check for decimal digit.
    // clippy: No. Asymmetry bad.
    #[allow(clippy::manual_is_ascii_check)]
    if matches!(digit, b'0'..=b'9') {
        return Some(digit & 0x0F);
    }

    // Make lowercase. This is okay because only the range A..=F will can end up in the a..=f range.
    digit |= 0x20;

    // Check for hex letter digit.
    if matches!(digit, b'a'..=b'f') {
        return Some(9 + (digit & 0x0F));
    }

    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn roundtrip() {
        // Test we can from all bytes to hex and back.
        for b in u8::MIN..=u8::MAX {
            let (hi, lo) = to_hex(b);
            assert_eq!(from_hex(hi, lo), Some(b));
        }
    }

    #[test]
    fn invalid() {
        // Test no non-hex digits parse as valid bytes.
        for hi in u8::MIN..=u8::MAX {
            for lo in u8::MIN..=u8::MAX {
                let res = from_hex(hi, lo);

                if hi.is_ascii_hexdigit() && lo.is_ascii_hexdigit() {
                    assert!(res.is_some());
                } else {
                    assert_eq!(res, None);
                }
            }
        }
    }
}
