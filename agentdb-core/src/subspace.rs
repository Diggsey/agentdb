use std::{convert::TryInto, marker::PhantomData};

use byteorder::{ByteOrder, LittleEndian};
use foundationdb::tuple::Versionstamp;
use tupleops::{ConcatTuples, TupleConcat};
use uuid::Uuid;

use crate::Timestamp;

struct UntypedSubspace {
    parent: Option<&'static UntypedSubspace>,
    prefix: &'static [u8],
}

pub struct Subspace<T> {
    inner: UntypedSubspace,
    phantom: PhantomData<T>,
}

fn advance(mut key: Vec<u8>) -> Vec<u8> {
    let mut l = key.len();
    while l > 0 && key[l - 1] == 0xFF {
        l -= 1;
    }
    assert!(l > 0);
    key[l - 1] += 1;
    key.truncate(l);
    key
}

impl<T> Subspace<T> {
    pub const fn new(prefix: &'static [u8]) -> Self {
        Self {
            inner: UntypedSubspace {
                parent: None,
                prefix,
            },
            phantom: PhantomData,
        }
    }
    pub const fn subspace<U>(&'static self, prefix: &'static [u8]) -> Subspace<ConcatTuples<T, U>>
    where
        (T, U): TupleConcat<T, U>,
    {
        Subspace {
            inner: UntypedSubspace {
                parent: Some(&self.inner),
                prefix,
            },
            phantom: PhantomData,
        }
    }
    fn build<U, V>(&self, root: &[u8], args: U, is_end: bool) -> Vec<u8>
    where
        (U, V): TupleConcat<U, V, Type = T>,
        U: Pack,
    {
        let mut buffer = root.to_vec();
        let mut packer = Packer::new(&mut buffer);

        self.inner.pack(&mut packer);
        packer.buffer.push(b'/');
        args.pack(&mut packer);
        packer.complete();

        if is_end {
            advance(buffer)
        } else {
            buffer
        }
    }
    pub fn key(&self, root: &[u8], args: T) -> Vec<u8>
    where
        T: Pack,
        (T, ()): TupleConcat<T, (), Type = T>,
    {
        self.build::<T, ()>(root, args, false)
    }
    pub fn range<U, V>(&self, root: &[u8], args: U) -> (Vec<u8>, Vec<u8>)
    where
        (U, V): TupleConcat<U, V, Type = T>,
        U: Pack,
    {
        let start = self.build(root, args, false);
        let end = advance(start.clone());

        (start, end)
    }
    pub fn subrange<U, V, W, X>(&self, root: &[u8], from: U, to: W) -> (Vec<u8>, Vec<u8>)
    where
        (U, V): TupleConcat<U, V, Type = T>,
        (W, X): TupleConcat<W, X, Type = T>,
        U: Pack,
        W: Pack,
    {
        (self.build(root, from, false), self.build(root, to, true))
    }
    pub fn decode(&self, root: &[u8], mut key: &[u8]) -> Option<T>
    where
        T: Pack,
    {
        key = &key[root.len()..];
        if !self.inner.unpack(&mut key) {
            return None;
        }
        if key.is_empty() || key[0] != b'/' {
            return None;
        }
        key = &key[1..];
        let res = T::unpack(&mut key)?;
        if key.is_empty() {
            Some(res)
        } else {
            None
        }
    }
}

pub struct Packer<'a> {
    pub buffer: &'a mut Vec<u8>,
    ts_offset: Option<u32>,
}

impl<'a> Packer<'a> {
    pub fn new(buffer: &'a mut Vec<u8>) -> Self {
        Self {
            buffer,
            ts_offset: None,
        }
    }
    pub fn complete(&mut self) {
        if let Some(ts_offset) = self.ts_offset {
            let offset = self.buffer.len();
            self.buffer.resize(offset + 4, 0);
            LittleEndian::write_u32(&mut self.buffer[offset..], ts_offset);
        }
    }
}

pub trait Pack: Sized {
    fn pack(&self, packer: &mut Packer);
    fn unpack(buffer: &mut &[u8]) -> Option<Self>;
}

pub trait HasPrefix<T> {}

macro_rules! tuple_impls {
    (@expand2 ($($prefix:ident),*)) => {
        impl<$($prefix,)*> HasPrefix<($($prefix,)*)> for ($($prefix,)*) {}
    };
    (@expand2 ($($prefix:ident),*) $head:ident $(, $tail:ident)*) => {
        tuple_impls!(@expand2 ($($prefix,)* $head) $($tail),*);

        impl<$($prefix,)* $head $(, $tail)*> HasPrefix<($($prefix,)*)> for ($($prefix,)* $head, $($tail,)*) {}
    };
    (@expand $($t:ident),*) => {
        tuple_impls!(@expand2 () $($t),*);

        impl<$($t),*> Pack for ($($t,)*)
        where
            $($t: Pack),*
        {
            fn pack(&self, #[allow(unused)] packer: &mut Packer) {
                #[allow(non_snake_case)]
                let ($($t,)*) = self;
                $($t.pack(packer);)*
            }
            fn unpack(#[allow(unused)] buffer: &mut &[u8]) -> Option<Self> {
                Some(($($t::unpack(buffer)?,)*))
            }
        }
    };
    ($h:ident $(, $t:ident)*) => {
        tuple_impls!(@expand $h $(,$t)*);
        tuple_impls!($($t),*);
    };
    () => {
        tuple_impls!(@expand);
    };
}

tuple_impls!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9);

impl UntypedSubspace {
    fn pack(&self, packer: &mut Packer) {
        if let Some(parent) = self.parent {
            parent.pack(packer);
            packer.buffer.push(b'.');
        }
        packer.buffer.extend_from_slice(self.prefix);
    }
    fn unpack(&self, buffer: &mut &[u8]) -> bool {
        if let Some(parent) = self.parent {
            parent.unpack(buffer);
            if buffer.is_empty() || buffer[0] != b'.' {
                return false;
            }
            *buffer = &buffer[1..];
        }
        if !buffer.starts_with(self.prefix) {
            return false;
        }
        *buffer = &buffer[self.prefix.len()..];
        true
    }
}

impl Pack for Uuid {
    fn pack(&self, packer: &mut Packer) {
        packer.buffer.extend_from_slice(self.as_bytes());
    }
    fn unpack(buffer: &mut &[u8]) -> Option<Self> {
        if buffer.len() >= 16 {
            let res = Uuid::from_slice(&buffer[0..16]).ok();
            *buffer = &buffer[16..];
            res
        } else {
            None
        }
    }
}

impl Pack for Versionstamp {
    fn pack(&self, packer: &mut Packer) {
        let offset = packer.buffer.len();
        packer.buffer.extend_from_slice(self.as_bytes());
        if !self.is_complete() {
            packer.ts_offset = Some(offset as u32);
        }
    }
    fn unpack(buffer: &mut &[u8]) -> Option<Self> {
        if buffer.len() >= 12 {
            let bytes: [u8; 12] = buffer[0..12].try_into().ok()?;
            *buffer = &buffer[12..];
            Some(bytes.into())
        } else {
            None
        }
    }
}

impl Pack for Timestamp {
    fn pack(&self, packer: &mut Packer) {
        self.millis().pack(packer)
    }
    fn unpack(buffer: &mut &[u8]) -> Option<Self> {
        Some(Self::from_millis(i64::unpack(buffer)?))
    }
}

macro_rules! int_impls {
    ($($t:ident),*) => {
        $(
            impl Pack for $t {
                fn pack(&self, packer: &mut Packer) {
                    packer.buffer.extend_from_slice(&self.to_be_bytes());
                }
                fn unpack(buffer: &mut &[u8]) -> Option<Self> {
                    const S: usize = ::std::mem::size_of::<$t>();
                    if buffer.len() >= S {
                        let bytes: [u8; S] = buffer[0..S].try_into().ok()?;
                        *buffer = &buffer[S..];
                        Some($t::from_be_bytes(bytes))
                    } else {
                        None
                    }
                }
            }
        )*
    };
}

int_impls!(u8, i8, u16, i16, u32, i32, u64, i64);
