use std::fmt::Display;

use serde::{de::Visitor, Deserialize, Serialize};
use uuid::Uuid;

use crate::AgentRef;

/// An AgentDB root
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Root {
    name: &'static str,
}

inventory::collect!(Root);

impl Root {
    #[doc(hidden)]
    pub const fn new(name: &'static str) -> Self {
        Self { name }
    }
    /// Obtain the name of this root.
    pub fn name(self) -> &'static str {
        self.name
    }

    /// Attempt to get the root with the given name. The root must
    /// have been defined somewhere in this crate or its dependencies.
    pub fn from_name(name: &str) -> Option<Self> {
        for &root in inventory::iter::<Root> {
            if root.name == name {
                return Some(root);
            }
        }
        None
    }

    /// Const-construct an AgentRef directly form this root and an agent ID, encoded as a `u128`.
    pub const fn const_ref<A>(self, id: u128) -> AgentRef<A> {
        AgentRef::from_parts_unchecked(self, Uuid::from_u128(id))
    }
}

impl Display for Root {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.name().fmt(f)
    }
}

impl Serialize for Root {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.name)
    }
}

impl<'de> Deserialize<'de> for Root {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(RootVisitor)
    }
}

struct RootVisitor;

impl<'de> Visitor<'de> for RootVisitor {
    type Value = Root;

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Root::from_name(v)
            .ok_or_else(|| serde::de::Error::invalid_value(serde::de::Unexpected::Str(v), &self))
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match std::str::from_utf8(v) {
            Ok(s) => self.visit_str(s),
            Err(_) => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Bytes(v),
                &self,
            )),
        }
    }

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string")
    }
}
