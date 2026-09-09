//! Compatibility parser for the loader's historical cJSON input behavior.

mod value;

use alloc::{borrow::Cow, vec::Vec};
use core::mem::MaybeUninit;

use value::Object;
pub(crate) use value::{Number, Value};

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Error {
    Invalid,
    OutOfMemory,
}

const NESTING_LIMIT: usize = 1_000;

pub(crate) fn parse(bytes: &[u8]) -> Result<Value<'_>, Error> {
    let mut parser = Parser {
        bytes,
        index: if bytes.starts_with(&[0xef, 0xbb, 0xbf]) {
            3
        } else {
            0
        },
        depth: 0,
        allocation_failed: false,
    };
    parser.skip_whitespace();
    let value = parser.value();
    if parser.allocation_failed {
        Err(Error::OutOfMemory)
    } else {
        value.ok_or(Error::Invalid)
    }
}

/// Finds a quote or escape without branching on every byte of long strings.
#[inline(never)]
fn string_delimiter(bytes: &[u8]) -> Option<usize> {
    let mut offset = 0;
    while bytes.len() - offset >= 8 {
        // SAFETY: At least eight bytes remain in the input slice.
        let word =
            u64::from_le(unsafe { bytes.as_ptr().add(offset).cast::<u64>().read_unaligned() });
        let zero_bytes =
            |word: u64| word.wrapping_sub(0x0101_0101_0101_0101) & !word & 0x8080_8080_8080_8080;
        let delimiters =
            zero_bytes(word ^ 0x2222_2222_2222_2222) | zero_bytes(word ^ 0x5c5c_5c5c_5c5c_5c5c);
        if delimiters != 0 {
            // Borrow can mark bytes after a zero, but never before the first
            // zero. Little-endian order makes the lowest bit the first match.
            return Some(offset + delimiters.trailing_zeros() as usize / 8);
        }
        offset += 8;
    }
    bytes[offset..]
        .iter()
        .position(|byte| matches!(byte, b'"' | b'\\'))
        .map(|index| offset + index)
}

struct Parser<'a> {
    bytes: &'a [u8],
    index: usize,
    depth: usize,
    allocation_failed: bool,
}

impl<'a> Parser<'a> {
    fn reserve<T>(&mut self, values: &mut Vec<T>, additional: usize) -> Option<()> {
        if values.try_reserve(additional).is_err() {
            self.allocation_failed = true;
            return None;
        }
        Some(())
    }

    fn skip_whitespace(&mut self) {
        while self.bytes.get(self.index).is_some_and(|byte| *byte <= b' ') {
            self.index += 1;
        }
    }

    fn value(&mut self) -> Option<Value<'a>> {
        match self.bytes.get(self.index).copied()? {
            b'n' if self.take_literal(b"null") => Some(Value::Null),
            b'f' if self.take_literal(b"false") => Some(Value::Bool(false)),
            b't' if self.take_literal(b"true") => Some(Value::Bool(true)),
            b'"' => self.string().map(Value::String),
            b'-' | b'0'..=b'9' => self.number(),
            b'[' => self.array(),
            b'{' => self.object(),
            _ => None,
        }
    }

    fn take_literal(&mut self, literal: &[u8]) -> bool {
        if self
            .bytes
            .get(self.index..self.index.saturating_add(literal.len()))
            != Some(literal)
        {
            return false;
        }
        self.index += literal.len();
        true
    }

    fn string(&mut self) -> Option<Cow<'a, [u8]>> {
        self.index += 1;
        let start = self.index;
        let remaining = &self.bytes[start..];
        let delimiter = string_delimiter(remaining)?;
        // SAFETY: string_delimiter returns the position of an existing quote
        // or backslash in this exact slice, so the position is strictly in bounds.
        let (bytes, tail) = unsafe { remaining.split_at_unchecked(delimiter) };
        self.index = start + delimiter;
        // SAFETY: The delimiter position is strictly below remaining.len().
        if unsafe { *tail.get_unchecked(0) } == b'"' {
            self.index += 1;
            // Unescaped strings remain valid while the input document is live.
            return Some(Cow::Borrowed(bytes));
        }
        self.escaped_string(bytes).map(Cow::Owned)
    }

    #[cold]
    fn escaped_string(&mut self, prefix: &[u8]) -> Option<Vec<u8>> {
        let mut output = Vec::new();
        self.reserve(&mut output, prefix.len())?;
        output.extend_from_slice(prefix);
        while let Some(byte) = self.bytes.get(self.index).copied() {
            self.index += 1;
            match byte {
                // Upstream cJSON copies unescaped bytes without UTF-8 repair.
                b'"' => return Some(output),
                b'\\' => {
                    self.reserve(&mut output, 4)?;
                    self.escape(&mut output)?;
                }
                _ => {
                    let start = self.index - 1;
                    let remaining = &self.bytes[self.index..];
                    self.index += string_delimiter(remaining).unwrap_or(remaining.len());
                    let bytes = &self.bytes[start..self.index];
                    self.reserve(&mut output, bytes.len())?;
                    output.extend_from_slice(bytes);
                }
            }
        }
        None
    }

    fn escape(&mut self, output: &mut Vec<u8>) -> Option<()> {
        let escaped = self.bytes.get(self.index).copied()?;
        self.index += 1;
        match escaped {
            b'b' => output.push(8),
            b'f' => output.push(12),
            b'n' => output.push(b'\n'),
            b'r' => output.push(b'\r'),
            b't' => output.push(b'\t'),
            b'"' | b'\\' | b'/' => output.push(escaped),
            b'u' => self.unicode_escape(output)?,
            _ => return None,
        }
        Some(())
    }

    fn unicode_escape(&mut self, output: &mut Vec<u8>) -> Option<()> {
        let first = self.hex_quad()?;
        let scalar = if (0xd800..=0xdbff).contains(&first) {
            if self.bytes.get(self.index..self.index.saturating_add(2)) != Some(b"\\u") {
                return None;
            }
            self.index += 2;
            let second = self.hex_quad()?;
            if !(0xdc00..=0xdfff).contains(&second) {
                return None;
            }
            0x1_0000 + ((u32::from(first) - 0xd800) << 10) + (u32::from(second) - 0xdc00)
        } else if (0xdc00..=0xdfff).contains(&first) {
            return None;
        } else {
            u32::from(first)
        };
        let character = char::from_u32(scalar)?;
        let mut encoded = [0_u8; 4];
        output.extend_from_slice(character.encode_utf8(&mut encoded).as_bytes());
        Some(())
    }

    fn hex_quad(&mut self) -> Option<u16> {
        let bytes = self.bytes.get(self.index..self.index.saturating_add(4))?;
        let mut value = 0_u16;
        for byte in bytes {
            let digit = match byte {
                b'0'..=b'9' => byte - b'0',
                b'a'..=b'f' => byte - b'a' + 10,
                b'A'..=b'F' => byte - b'A' + 10,
                // The loader's cJSON fork maps an invalid hex quartet to
                // codepoint zero instead of rejecting the string.
                _ => {
                    self.index += 4;
                    return Some(0);
                }
            };
            value = value.checked_mul(16)?.checked_add(u16::from(digit))?;
        }
        self.index += 4;
        Some(value)
    }

    fn number(&mut self) -> Option<Value<'a>> {
        let start = self.index;
        let remaining = self.bytes.get(start..)?;
        // Initialize only the copied prefix and its terminator, as strtod
        // never needs the unused suffix of this bounded stack buffer.
        let mut input = [MaybeUninit::<u8>::uninit(); 64];
        let mut length = 0;
        for (&byte, slot) in remaining.iter().zip(&mut input[..63]) {
            if !matches!(byte, b'0'..=b'9' | b'+' | b'-' | b'e' | b'E' | b'.') {
                break;
            }
            slot.write(byte);
            length += 1;
        }
        input[length].write(0);
        let input = input.as_ptr().cast::<core::ffi::c_char>();
        let mut end = core::ptr::null_mut();
        // SAFETY: `input` is a live NUL-terminated byte string and `end` is a
        // writable out-pointer for `strtod`'s position within that string.
        let value = unsafe { libc::strtod(input, &raw mut end) };
        if end == input.cast_mut() {
            return None;
        }
        // SAFETY: `strtod` returns either the input pointer or a pointer within
        // the same NUL-terminated allocation; equality was rejected above.
        let consumed = unsafe { end.offset_from(input) } as usize;
        self.index = start + consumed;
        // SAFETY: The scanner copied only ASCII bytes. strtod's consumed prefix
        // ends within that initialized, NUL-terminated token.
        let token = unsafe { core::str::from_utf8_unchecked(&remaining[..consumed]) };
        let number = match token.parse::<u64>() {
            Ok(value) => Number::Unsigned(value),
            Err(_) => match token.parse::<i64>() {
                Ok(value) => Number::Signed(value),
                Err(_) if value.is_finite() => Number::Float(value),
                Err(_) => return Some(Value::Null),
            },
        };
        Some(Value::Number(number))
    }

    fn array(&mut self) -> Option<Value<'a>> {
        self.enter()?;
        self.index += 1;
        self.skip_whitespace();
        let mut values = Vec::new();
        if self.bytes.get(self.index) == Some(&b']') {
            self.index += 1;
            self.leave();
            return Some(Value::Array(values));
        }
        loop {
            self.reserve(&mut values, 1)?;
            values.push(self.value()?);
            self.skip_whitespace();
            match self.bytes.get(self.index) {
                Some(b',') => {
                    self.index += 1;
                    self.skip_whitespace();
                }
                Some(b']') => {
                    self.index += 1;
                    self.leave();
                    return Some(Value::Array(values));
                }
                _ => return None,
            }
        }
    }

    fn object(&mut self) -> Option<Value<'a>> {
        self.enter()?;
        self.index += 1;
        self.skip_whitespace();
        let mut values = Object(Vec::new());
        if self.bytes.get(self.index) == Some(&b'}') {
            self.index += 1;
            self.leave();
            return Some(Value::Object(values));
        }
        loop {
            if self.bytes.get(self.index) != Some(&b'"') {
                return None;
            }
            let key = self.string()?;
            self.skip_whitespace();
            if self.bytes.get(self.index) != Some(&b':') {
                return None;
            }
            self.index += 1;
            self.skip_whitespace();
            let value = self.value();
            // Check validity without moving the payload across the fallible
            // reserve; extracting it here introduces extra stack copies.
            value.as_ref()?;
            self.reserve(&mut values.0, 1)?;
            values.0.push((key, value?));
            self.skip_whitespace();
            match self.bytes.get(self.index) {
                Some(b',') => {
                    self.index += 1;
                    self.skip_whitespace();
                }
                Some(b'}') => {
                    self.index += 1;
                    self.leave();
                    return Some(Value::Object(values));
                }
                _ => return None,
            }
        }
    }

    fn enter(&mut self) -> Option<()> {
        if self.depth >= NESTING_LIMIT {
            return None;
        }
        self.depth += 1;
        Some(())
    }

    fn leave(&mut self) {
        self.depth -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::{Number, Parser, Value, parse};

    #[test]
    fn string_delimiters_preserve_first_match_at_word_boundaries() {
        for byte in 0..=u8::MAX {
            for alignment in 0..8 {
                for position in 0..32 {
                    for delimiter in *b"\"\\" {
                        let mut bytes = [byte; 64];
                        bytes[alignment + position] = delimiter;
                        let expected = if matches!(byte, b'"' | b'\\') {
                            0
                        } else {
                            position
                        };
                        for tail in [0, 8] {
                            let input = &bytes[alignment..alignment + position + 1 + tail];
                            assert_eq!(super::string_delimiter(input), Some(expected));
                        }
                    }
                }
            }
            for len in 0..32 {
                let bytes = [byte; 32];
                let expected = (len != 0 && matches!(byte, b'"' | b'\\')).then_some(0);
                assert_eq!(super::string_delimiter(&bytes[..len]), expected);
            }
        }
    }

    #[test]
    fn object_lookup_preserves_nul_termination_and_first_match() {
        let value =
            parse(br#"{"name\u0000suffix":1,"name":2,"names":3,"\u0000hidden":4,"other":5}"#)
                .unwrap();
        for key in [
            "name",
            "NAME",
            "names",
            "",
            "other",
            "missing",
            "na",
            "name\0suffix",
            "\0",
        ] {
            let object = value.as_object().unwrap();
            let expected = object
                .0
                .iter()
                .find(|(name, _)| name.split(|byte| *byte == 0).next().unwrap() == key.as_bytes())
                .map(|(_, value)| value);
            let expected_insensitive = object
                .0
                .iter()
                .find(|(name, _)| {
                    name.split(|byte| *byte == 0)
                        .next()
                        .unwrap()
                        .eq_ignore_ascii_case(key.as_bytes())
                })
                .map(|(_, value)| value);
            assert_eq!(value.get(key), expected, "{key:?}");
            assert_eq!(value.field(key), expected_insensitive, "{key:?}");
        }
        assert_eq!(value.get("name").and_then(Value::as_u64), Some(1));
        assert_eq!(value.field("NAME").and_then(Value::as_u64), Some(1));
        assert_eq!(value.get("").and_then(Value::as_u64), Some(4));
    }

    #[test]
    fn preserves_first_duplicate_keys_and_exact_unsigned_integers() {
        let value =
            parse(br#"{"name":"first","name":"last","count":18446744073709551615}"#).unwrap();
        assert_eq!(value.get("name").and_then(Value::as_str), Some("first"));
        assert_eq!(value.get("count").and_then(Value::as_u64), Some(u64::MAX));
    }

    #[test]
    fn allocation_overflow_is_distinct_from_invalid_input() {
        let mut parser = Parser {
            bytes: &[],
            index: 0,
            depth: 0,
            allocation_failed: false,
        };
        assert!(parser.reserve(&mut Vec::<u64>::new(), usize::MAX).is_none());
        assert!(parser.allocation_failed);
        assert_eq!(parse(b"nonsense"), Err(super::Error::Invalid));
    }

    #[test]
    fn preserves_literals_and_unicode_escape_compatibility() {
        assert_eq!(parse(b"null"), Ok(Value::Null));
        assert_eq!(parse(b"false"), Ok(Value::Bool(false)));
        assert_eq!(
            parse(br#""\b\f\r\t\ud83d\ude00""#).unwrap().as_str(),
            Some("\u{8}\u{c}\r\t\u{1f600}")
        );
        assert!(parse(b"-").is_err());
    }
    #[test]
    fn compatibility_strings_and_numbers() {
        let value = parse(b"{\"path\":\"a\xffb\n\"} trailing").unwrap();
        assert_eq!(
            value.get("path").and_then(Value::as_bytes),
            Some(b"a\xffb\n".as_slice())
        );
        assert_eq!(parse(b"01.5suffix"), Ok(Value::Number(Number::Float(1.5))));
        assert_eq!(
            parse(br#""ab\ncd\\ef""#).unwrap().as_str(),
            Some("ab\ncd\\ef")
        );
    }
    #[test]
    fn numeric_prefixes_preserve_limits_and_integer_precision() {
        for (bytes, expected) in [
            (
                b"18446744073709551615".as_slice(),
                Number::Unsigned(u64::MAX),
            ),
            (b"-9223372036854775808".as_slice(), Number::Signed(i64::MIN)),
            (b"1e+".as_slice(), Number::Unsigned(1)),
            (b"12\xff".as_slice(), Number::Unsigned(12)),
        ] {
            assert_eq!(parse(bytes), Ok(Value::Number(expected)));
        }
        let mut bytes = [b'0'; 64];
        bytes[63] = b'9';
        let mut parser = Parser {
            bytes: &bytes,
            index: 0,
            depth: 0,
            allocation_failed: false,
        };
        assert_eq!(parser.value(), Some(Value::Number(Number::Unsigned(0))));
        assert_eq!(parser.index, 63);
        assert_eq!(parse(b"1e999"), Ok(Value::Null));
    }

    #[test]
    fn object_keys_use_c_string_boundaries() {
        let value =
            parse(br#"{"library_path\u0000suffix":"first","LIBRARY_PATH":"second"}"#).unwrap();
        assert_eq!(
            value.get("library_path").and_then(Value::as_str),
            Some("first")
        );
        assert_eq!(
            value.field("LIBRARY_PATH").and_then(Value::as_str),
            Some("first")
        );
        assert_eq!(value.get("library_path_suffix"), None);
    }
    #[test]
    fn rejects_invalid_surrogates() {
        for bytes in [
            br#""\ud83d""#.as_slice(),
            br#""\ud83d\u0041""#,
            br#""\ude00""#,
        ] {
            assert!(parse(bytes).is_err());
        }
    }
}
