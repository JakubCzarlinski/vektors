//! Flat preorder JSON storage and borrowed accessors used by manifest readers.

use alloc::{borrow::Cow, vec::Vec};
use core::fmt;

#[derive(Debug, PartialEq)]
pub(super) enum Kind<'a> {
    Null,
    Bool(bool),
    Number(Number<'a>),
    String(Cow<'a, [u8]>),
    Array(usize),
    Object,
}

#[derive(Debug, PartialEq)]
pub(super) struct Node<'a> {
    pub(super) kind: Kind<'a>,
    pub(super) name: Option<Cow<'a, [u8]>>,
    /// Byte span occupied by this value and all of its descendants.
    pub(super) subtree_bytes: usize,
}

#[derive(Debug, PartialEq)]
pub(crate) enum Number<'a> {
    Unsigned(u64),
    Signed(i64),
    Float(&'a [u8]),
}

#[derive(Debug, PartialEq)]
pub(crate) struct Document<'a> {
    pub(super) nodes: Vec<Node<'a>>,
}

impl Document<'_> {
    pub(crate) fn root(&self) -> Value<'_> {
        Value {
            node: &self.nodes[0],
        }
    }

    #[cfg(test)]
    pub(crate) fn get(&self, key: &str) -> Option<Value<'_>> {
        self.root().get(key)
    }

    #[cfg(test)]
    pub(crate) fn as_object(&self) -> Option<Object<'_>> {
        self.root().as_object()
    }

    #[cfg(test)]
    pub(crate) fn as_str(&self) -> Option<&str> {
        self.root().as_str()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Value<'a> {
    node: &'a Node<'a>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Object<'a>(Value<'a>);

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Array<'a>(Value<'a>);

#[derive(Clone)]
pub(crate) struct Values<'a> {
    next: *const Node<'a>,
    end: *const Node<'a>,
    marker: core::marker::PhantomData<&'a Node<'a>>,
}

#[derive(Debug, PartialEq)]
pub(crate) enum ValueKind<'a> {
    Null,
    Bool(bool),
    Number(&'a Number<'a>),
    String(&'a [u8]),
    Array(Array<'a>),
    Object(Object<'a>),
}

impl<'a> Iterator for Values<'a> {
    type Item = Value<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next == self.end {
            return None;
        }
        // SAFETY: both pointers delimit the live document's preorder node tape.
        let node = unsafe { &*self.next };
        // SAFETY: subtree lengths are created from ranges within the same tape.
        self.next = unsafe { self.next.byte_add(node.subtree_bytes) };
        Some(Value { node })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }
}

impl<'a> Array<'a> {
    pub(crate) fn iter(self) -> Values<'a> {
        self.0.children()
    }

    pub(crate) fn first(self) -> Option<Value<'a>> {
        self.iter().next()
    }

    pub(crate) fn len(self) -> usize {
        match self.0.node.kind {
            Kind::Array(length) => length,
            _ => 0,
        }
    }

    pub(crate) fn is_empty(self) -> bool {
        self.len() == 0
    }
}

impl<'a> IntoIterator for Array<'a> {
    type Item = Value<'a>;
    type IntoIter = Values<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> Object<'a> {
    pub(crate) fn get(self, key: &str) -> Option<Value<'a>> {
        let key = key.as_bytes();
        if key.contains(&0) {
            return None;
        }
        self.get_valid_key(key)
    }

    #[inline(never)]
    fn get_valid_key(self, key: &[u8]) -> Option<Value<'a>> {
        self.iter().find_map(|(name, value)| {
            (name.starts_with(key) && name.get(key.len()).is_none_or(|byte| *byte == 0))
                .then_some(value)
        })
    }

    pub(crate) fn iter(self) -> impl Iterator<Item = (&'a [u8], Value<'a>)> {
        self.0.children().map(|value| {
            let name = value.node.name.as_deref().unwrap_or_default();
            (name, value)
        })
    }

    pub(crate) fn values(self) -> Values<'a> {
        self.0.children()
    }
}

impl<'a> Value<'a> {
    fn children(self) -> Values<'a> {
        let next = core::ptr::from_ref(self.node).wrapping_add(1);
        Values {
            next,
            end: core::ptr::from_ref(self.node).wrapping_byte_add(self.node.subtree_bytes),
            marker: core::marker::PhantomData,
        }
    }

    pub(crate) fn get(self, key: &str) -> Option<Self> {
        self.as_object()?.get(key)
    }

    pub(crate) fn as_str(self) -> Option<&'a str> {
        core::str::from_utf8(self.as_bytes()?).ok()
    }

    pub(crate) fn as_bytes(self) -> Option<&'a [u8]> {
        match &self.node.kind {
            Kind::String(value) => Some(value),
            _ => None,
        }
    }

    pub(crate) fn as_bool(self) -> Option<bool> {
        match self.node.kind {
            Kind::Bool(value) => Some(value),
            _ => None,
        }
    }

    pub(crate) fn as_u64(self) -> Option<u64> {
        match self.node.kind {
            Kind::Number(Number::Unsigned(value)) => Some(value),
            _ => None,
        }
    }

    pub(crate) fn as_array(self) -> Option<Array<'a>> {
        matches!(self.node.kind, Kind::Array(_)).then_some(Array(self))
    }

    pub(crate) fn as_object(self) -> Option<Object<'a>> {
        matches!(self.node.kind, Kind::Object).then_some(Object(self))
    }

    pub(crate) fn is_object(self) -> bool {
        matches!(self.node.kind, Kind::Object)
    }

    pub(crate) fn kind(self) -> ValueKind<'a> {
        match &self.node.kind {
            Kind::Null => ValueKind::Null,
            Kind::Bool(value) => ValueKind::Bool(*value),
            Kind::Number(value) => ValueKind::Number(value),
            Kind::String(value) => ValueKind::String(value),
            Kind::Array(_) => ValueKind::Array(Array(self)),
            Kind::Object => ValueKind::Object(Object(self)),
        }
    }
}

impl fmt::Display for Number<'_> {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unsigned(value) => value.fmt(output),
            Self::Signed(value) => value.fmt(output),
            Self::Float(value) => {
                output.write_str(unsafe { core::str::from_utf8_unchecked(value) })
            }
        }
    }
}
