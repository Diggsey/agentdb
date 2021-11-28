use std::io;

use foundationdb::tuple::{
    pack, unpack, Element, PackError, PackResult, TupleDepth, TuplePack, TupleUnpack,
    VersionstampOffset,
};
use serde::{Deserialize, Serialize};

/// Stores arbitrary data which has already been packed into a FoundationDB tuple.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Prepacked(pub Vec<u8>);

const NIL: u8 = 0x00;
const NESTED: u8 = 0x05;

impl Prepacked {
    /// Pack the given value.
    pub fn new<T: TuplePack>(value: &T) -> Self {
        Self(pack(value))
    }
    /// Unpack into the desired type.
    pub fn unpack<'de, T: TupleUnpack<'de>>(&'de self) -> Result<T, PackError> {
        unpack(&self.0)
    }
}

impl TuplePack for Prepacked {
    fn pack<W: io::Write>(
        &self,
        w: &mut W,
        tuple_depth: TupleDepth,
    ) -> io::Result<VersionstampOffset> {
        let mut offset = VersionstampOffset::None {
            size: self.0.len() as u32,
        };
        if tuple_depth.depth() > 0 {
            w.write_all(&[NESTED])?;
            offset += 1;
        }
        w.write_all(&self.0)?;
        if tuple_depth.depth() > 0 {
            w.write_all(&[NIL])?;
            offset += 1;
        }
        Ok(offset)
    }
}

impl<'de> TupleUnpack<'de> for Prepacked {
    fn unpack(input: &'de [u8], tuple_depth: TupleDepth) -> PackResult<(&'de [u8], Self)> {
        Ok(if tuple_depth.depth() > 0 {
            let (remainder, _) = Vec::<Element>::unpack(input, tuple_depth)?;
            let inner_extent = input.len() - remainder.len() - 1;
            (remainder, Self(input[1..inner_extent].to_vec()))
        } else {
            (&[], Self(input.to_vec()))
        })
    }
}
