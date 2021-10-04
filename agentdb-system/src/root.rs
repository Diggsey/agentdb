use serde::{de::Visitor, Deserialize, Serialize};

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
    pub fn name(self) -> &'static str {
        self.name
    }
    pub fn to_bytes(self) -> Vec<u8> {
        self.name.as_bytes().to_vec()
    }
    pub fn from_str(name: &str) -> Option<Self> {
        for &root in inventory::iter::<Root> {
            if root.name == name {
                return Some(root);
            }
        }
        None
    }
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        Self::from_str(std::str::from_utf8(bytes).ok()?)
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
        Root::from_str(v)
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
