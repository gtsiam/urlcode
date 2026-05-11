use std::{fmt, iter::FusedIterator, ops};

/// A set of ascii characters.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct AsciiCharset([u64; 2]);

impl AsciiCharset {
    /// An empty character set.
    pub const EMPTY: Self = Self([0, 0]);

    /// The set of all ascii characters (U+0000 to U+007F inclusive).
    pub const ALL: Self = Self::EMPTY.complement();

    /// [C0 control percent-encode set](https://url.spec.whatwg.org/#c0-control-percent-encode-set).
    ///
    /// This includes all characters from U+0000 NULL to U+001F INFORMATION SEPARATOR ONE, inclusive
    /// and U+007F DEL.
    pub const CONTROL: Self = Self([0xFFFFFFFF, 0]).add(0x7F);

    /// [Fragment percent-encode set](https://url.spec.whatwg.org/#fragment-percent-encode-set).
    ///
    /// This includes [`CONTROL`](Self::CONTROL) and U+0020 SPACE, U+0022 ("), U+003C (<), U+003E
    /// (>), U+0060 (\`).
    pub const FRAGMENT: Self = Self::CONTROL.add_all(br#" "<>`"#);

    /// [Query percent-encode set](https://url.spec.whatwg.org/#query-percent-encode-set).
    ///
    /// This includes [`CONTROL`](Self::CONTROL) and U+0020 SPACE, U+0022 ("), U+0023 (#), U+003C
    /// (<), U+003E (>).
    pub const QUERY: Self = Self::CONTROL.add_all(br#" "<>#"#);

    /// [Special query percent-encode
    /// set](https://url.spec.whatwg.org/#special-query-percent-encode-set).
    ///
    /// This includes [`QUERY`](Self::QUERY) and U+0027 (').
    pub const SPECIAL_QUERY: Self = Self::QUERY.add(b'\'');

    /// [Path percent-encode set](https://url.spec.whatwg.org/#path-percent-encode-set).
    ///
    /// This includes [`QUERY`](Self::QUERY) and U+003F (?), U+005E (^), U+0060 (`), U+007B ({),
    /// U+007D (}).
    pub const PATH: Self = Self::QUERY.add_all(br#"?^`{}"#);

    /// [Userinfo percent-encode set](https://url.spec.whatwg.org/#userinfo-percent-encode-set).
    ///
    /// This includes [`PATH`](Self::PATH) and U+002F (/), U+003A (:), U+003B (;), U+003D (=),
    /// U+0040 (@), U+005B ([), U+005C(\\), U+005D (]), U+007C (|).
    pub const USERINFO: Self = Self::PATH.add_all(br#"/:;=@[\]|"#);

    /// [Component percent-encode set](https://url.spec.whatwg.org/#component-percent-encode-set).
    ///
    /// This includes [`USERINFO`](Self::USERINFO) and U+0024 ($), U+0025 (%), U+0026 (&), U+002B
    /// (+), U+002C (,).
    pub const COMPONENT: Self = Self::USERINFO.add_all(br#"$%&+,"#);

    /// [application/x-www-form-urlencoded percent-encode
    /// set](https://url.spec.whatwg.org/#application-x-www-form-urlencoded-percent-encode-set).
    ///
    /// This includes [`COMPONENT`](Self::COMPONENT) and U+0021 (!), U+0027 ('), U+0028 RIGHT
    /// PARENTHESIS, U+0029 RIGHT PARENTHESIS, U+007E (~).
    pub const X_WWW_FORM_URLENCODED: Self = Self::COMPONENT.add_all(br#"!'()~"#);
}

impl AsciiCharset {
    /// Create a new charset containing the provided ascii characters.
    ///
    /// # Panics
    ///
    /// This function will panic if it is given a non-ascii character.
    pub const fn new(mask: &[u8]) -> Self {
        Self::EMPTY.add_all(mask)
    }

    /// Add an ascii character to the mask.
    ///
    /// # Panics
    ///
    /// This function will panic if it is given a non-ascii character.
    pub const fn add(mut self, c: u8) -> Self {
        ensure_ascii(c);
        self.0[(c >> 6) as usize] |= 1 << (c & 0x3F);
        self
    }

    /// Add a set of ascii characters to the mask.
    ///
    /// # Panics
    ///
    /// This function will panic if it is given a non-ascii character.
    pub const fn add_all(mut self, mut mask: &[u8]) -> Self {
        while let [c, rest @ ..] = mask {
            self = self.add(*c);
            mask = rest;
        }
        self
    }

    /// Remove an ascii character from the mask.
    ///
    /// # Panics
    ///
    /// This function will panic if it is given a non-ascii character.
    pub const fn remove(mut self, c: u8) -> Self {
        ensure_ascii(c);
        self.0[(c >> 6) as usize] &= !(1 << (c & 0x3F));
        self
    }

    /// Remove a set of ascii characters from the mask.
    ///
    /// # Panics
    ///
    /// This function will panic if it is given a non-ascii character.
    pub const fn remove_all(mut self, mut mask: &[u8]) -> Self {
        while let [c, rest @ ..] = mask {
            self = self.remove(*c);
            mask = rest;
        }
        self
    }

    /// Check if an ascii character is part of the character set.
    ///
    /// Will return `false` for non-ascii characters.
    pub const fn contains(&self, c: u8) -> bool {
        c.is_ascii() && (self.0[(c >> 6) as usize] & (1 << (c & 0x3F))) != 0
    }

    /// Returns the complement of this character set.
    ///
    /// **NOTE**: non-ascii characters will still be outside the set.
    pub const fn complement(mut self) -> Self {
        self.0[0] = !self.0[0];
        self.0[1] = !self.0[1];
        self
    }

    /// Returns the union of two character sets.
    pub const fn union(mut self, rhs: Self) -> Self {
        self.0[0] |= rhs.0[0];
        self.0[1] |= rhs.0[1];
        self
    }

    /// Returns the intersection of two character sets.
    pub const fn intersection(mut self, rhs: Self) -> Self {
        self.0[0] &= rhs.0[0];
        self.0[1] &= rhs.0[1];
        self
    }

    /// Returns the difference of two character sets.
    pub const fn difference(self, rhs: Self) -> Self {
        self.intersection(rhs.complement())
    }
}

#[track_caller]
const fn ensure_ascii(c: u8) {
    if !c.is_ascii() {
        panic!("only ascii characters are allowed")
    }
}

impl ops::Not for AsciiCharset {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.complement()
    }
}

impl ops::Add for AsciiCharset {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl ops::Add<u8> for AsciiCharset {
    type Output = Self;

    fn add(self, rhs: u8) -> Self::Output {
        self.add(rhs)
    }
}

impl<T> ops::AddAssign<T> for AsciiCharset
where
    Self: ops::Add<T, Output = Self>,
{
    fn add_assign(&mut self, rhs: T) {
        *self = *self + rhs
    }
}

impl ops::Sub for AsciiCharset {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.difference(rhs)
    }
}

impl ops::Sub<u8> for AsciiCharset {
    type Output = Self;

    fn sub(self, rhs: u8) -> Self::Output {
        self.remove(rhs)
    }
}

impl<T> ops::SubAssign<T> for AsciiCharset
where
    Self: ops::Sub<T, Output = Self>,
{
    fn sub_assign(&mut self, rhs: T) {
        *self = *self - rhs;
    }
}

impl IntoIterator for AsciiCharset {
    type Item = u8;
    type IntoIter = AsciiCharsetIterator;

    fn into_iter(self) -> Self::IntoIter {
        AsciiCharsetIterator {
            charset: self,
            current: 0,
        }
    }
}

pub struct AsciiCharsetIterator {
    charset: AsciiCharset,
    current: u8,
}

impl Iterator for AsciiCharsetIterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        while self.current < 0x80 {
            let c = self.current;
            if self.charset.contains(c) {
                self.current += 1;
                return Some(c);
            }

            self.current += 1;
        }

        None
    }
}

impl FusedIterator for AsciiCharsetIterator {}

impl fmt::Debug for AsciiCharset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AsciiCharset(\"")?;

        for c in *self {
            let c = char::from_u32(c as u32).unwrap().escape_debug();
            write!(f, "{c}")?;
        }

        write!(f, "\")")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create() {
        let set = AsciiCharset::new(b"abcd1234");
        assert!(set.contains(b'b'));
        assert!(set.contains(b'3'));
        assert!(!set.contains(b'e'));
        assert!(!set.contains(b'5'));

        assert_eq!(set.remove_all(b"c3"), AsciiCharset::new(b"abd124"));
    }

    #[test]
    fn op_add() {
        let mut set = AsciiCharset::new(b"a1");
        set += b'b';
        assert_eq!(set, AsciiCharset::new(b"ab1"));
    }

    #[test]
    fn op_remove() {
        let mut set = AsciiCharset::new(b"a1");
        set -= b'a';
        assert_eq!(set, AsciiCharset::new(b"1"));
    }

    #[test]
    fn complement() {
        let a = AsciiCharset::new(b"ab12");
        let res = !a;
        assert_eq!(res, a.complement());
        assert!(!res.contains(b'a'));
        assert!(!res.contains(b'b'));
        assert!(!res.contains(b'1'));
        assert!(!res.contains(b'2'));
    }

    #[test]
    fn union() {
        let a = AsciiCharset::new(b"ab12");
        let b = AsciiCharset::new(b"cd34");
        let res = a + b;
        assert_eq!(res, a.union(b));
        assert_eq!(res, b.union(a));
        assert_eq!(res, AsciiCharset::new(b"abcd1234"));
    }

    #[test]
    fn intersection() {
        let a = AsciiCharset::new(b"abc123");
        let b = AsciiCharset::new(b"cd34");
        let res = a.intersection(b);
        assert_eq!(res, b.intersection(a));
        assert_eq!(res, AsciiCharset::new(b"c3"));
    }

    #[test]
    fn difference() {
        let set_a = AsciiCharset::new(b"abc123");
        let set_b = AsciiCharset::new(b"cd34");
        assert_eq!(set_a - set_b, AsciiCharset::new(b"ab12"));
    }

    #[test]
    fn debug() {
        assert_eq!(
            format!("{:?}", AsciiCharset::new(b"\0aabc123")),
            r#"AsciiCharset("\0123abc")"#
        );
    }

    #[test]
    fn control() {
        let set = AsciiCharset::CONTROL;
        assert!(set.contains(0x1F));
        assert!(!set.contains(0x20));
        assert!(!set.contains(0x7E));
        assert!(set.contains(0x7F));
        assert!(!set.contains(0x80));
    }

    #[test]
    fn x_www_form_urlencoded() {
        assert_eq!(
            AsciiCharset::X_WWW_FORM_URLENCODED.complement(),
            AsciiCharset::new(
                b"*-.0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ_abcdefghijklmnopqrstuvwxyz"
            )
        )
    }

    #[test]
    #[should_panic]
    fn no_ascii() {
        AsciiCharset::new(b"\x80");
    }
}
