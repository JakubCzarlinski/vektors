//! JSON values and borrowed accessors used by manifest and settings readers.

use alloc::vec::Vec;
use core::fmt;

#[derive(Debug, PartialEq)]
pub(crate) enum Value {
    Null,
    Bool(bool),
    Number(Number),
    String(Vec<u8>),
    Array(Vec<Self>),
    Object(Object),
}

#[derive(Debug, PartialEq)]
pub(crate) enum Number {
    Unsigned(u64),
    Signed(i64),
    Float(f64),
}

#[derive(Debug, PartialEq)]
pub(crate) struct Object(pub(super) Vec<(Vec<u8>, Value)>);

impl Object {
    #[inline(never)]
    pub(crate) fn get(&self, key: &str) -> Option<&Value> {
        let key = key.as_bytes();
        if key.contains(&0) {
            return None;
        }
        self.0
            .iter()
            .find(|(name, _)| {
                name.starts_with(key) && name.get(key.len()).is_none_or(|byte| *byte == 0)
            })
            .map(|(_, value)| value)
    }

    #[inline(never)]
    pub(crate) fn get_ignore_ascii_case(&self, key: &str) -> Option<&Value> {
        let key = key.as_bytes();
        if key.contains(&0) {
            return None;
        }
        self.0
            .iter()
            .find(|(name, _)| {
                name.get(..key.len())
                    .is_some_and(|prefix| prefix.eq_ignore_ascii_case(key))
                    && name.get(key.len()).is_none_or(|byte| *byte == 0)
            })
            .map(|(_, value)| value)
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = (&[u8], &Value)> {
        self.0.iter().map(|(name, value)| (name.as_slice(), value))
    }

    pub(crate) fn values(&self) -> impl Iterator<Item = &Value> {
        self.0.iter().map(|(_, value)| value)
    }
}

impl Value {
    pub(crate) fn get(&self, key: &str) -> Option<&Self> {
        self.as_object()?.get(key)
    }
    pub(crate) fn field(&self, key: &str) -> Option<&Self> {
        self.as_object()?.get_ignore_ascii_case(key)
    }
    pub(crate) const fn as_str(&self) -> Option<&str> {
        match self.as_bytes() {
            Some(bytes) => match core::str::from_utf8(bytes) {
                Ok(text) => Some(text),
                Err(_) => None,
            },
            None => None,
        }
    }
    pub(crate) const fn as_bytes(&self) -> Option<&[u8]> {
        match self {
            Self::String(value) => Some(value.as_slice()),
            _ => None,
        }
    }
    pub(crate) const fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(value) => Some(*value),
            _ => None,
        }
    }
    pub(crate) const fn as_u64(&self) -> Option<u64> {
        match self {
            Self::Number(Number::Unsigned(value)) => Some(*value),
            _ => None,
        }
    }
    pub(crate) const fn as_array(&self) -> Option<&Vec<Self>> {
        match self {
            Self::Array(value) => Some(value),
            _ => None,
        }
    }
    pub(crate) const fn as_object(&self) -> Option<&Object> {
        match self {
            Self::Object(value) => Some(value),
            _ => None,
        }
    }
    pub(crate) const fn is_object(&self) -> bool {
        matches!(self, Self::Object(_))
    }
}

impl fmt::Display for Number {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unsigned(value) => value.fmt(output),
            Self::Signed(value) => value.fmt(output),
            Self::Float(value) => value.fmt(output),
        }
    }
}
