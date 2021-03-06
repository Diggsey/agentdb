use agentdb_core::Error;
use serde::{de::DeserializeOwned, Serialize};

pub use JsonSerializer as DefaultSerializer;

pub struct JsonSerializer;

impl Serializer for JsonSerializer {
    fn serialize<T: Serialize + ?Sized>(&self, value: &T) -> Result<Vec<u8>, Error> {
        Ok(serde_json::to_vec(value)?)
    }

    fn deserialize<T: DeserializeOwned>(&self, slice: &[u8]) -> Result<T, Error> {
        Ok(serde_json::from_slice(slice)?)
    }
}

pub struct PostcardSerializer;

impl Serializer for PostcardSerializer {
    fn serialize<T: Serialize + ?Sized>(&self, value: &T) -> Result<Vec<u8>, Error> {
        Ok(postcard::to_stdvec(value)?)
    }

    fn deserialize<T: DeserializeOwned>(&self, slice: &[u8]) -> Result<T, Error> {
        Ok(postcard::from_bytes(slice)?)
    }
}

pub trait Serializer {
    fn serialize<T: Serialize + ?Sized>(&self, value: &T) -> Result<Vec<u8>, Error>;
    fn deserialize<T: DeserializeOwned>(&self, slice: &[u8]) -> Result<T, Error>;
}
