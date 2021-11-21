use std::{
    fmt::Debug,
    marker::PhantomData,
    ops::{Bound, RangeBounds},
};

use foundationdb::{
    directory::Directory,
    tuple::{Subspace, TuplePack, TupleUnpack},
    Transaction,
};
use serde::{Deserialize, Serialize};

use crate::{utils::next_key, Error};

pub trait HasPrefix<T> {}
pub trait IsPrefix {}

macro_rules! tuple_impls {
    (@expand2 ($($prefix:ident),*)) => {
        impl<$($prefix,)*> HasPrefix<($($prefix,)*)> for ($($prefix,)*) {}
        impl<$($prefix,)*> IsPrefix for (($($prefix,)*), ($($prefix,)*)) {}
    };
    (@expand2 ($($prefix:ident),*) $head:ident $(, $tail:ident)*) => {
        tuple_impls!(@expand2 ($($prefix,)* $head) $($tail),*);

        impl<$($prefix,)* $head $(, $tail)*> HasPrefix<($($prefix,)*)> for ($($prefix,)* $head, $($tail,)*) {}
        impl<$($prefix,)* $head $(, $tail)*> IsPrefix for (($($prefix,)*), ($($prefix,)* $head, $($tail,)*)) {}
    };
    (@expand $($t:ident),*) => {
        tuple_impls!(@expand2 () $($t),*);
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

pub struct TypedSubspace<T> {
    inner: Subspace,
    phantom: PhantomData<T>,
}

impl<T> Debug for TypedSubspace<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TypedSubspace")
            .field("inner", &self.inner)
            .finish()
    }
}

impl<T> Serialize for TypedSubspace<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.inner.bytes().serialize(serializer)
    }
}

impl<'de, T> Deserialize<'de> for TypedSubspace<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let bytes = Vec::<u8>::deserialize(deserializer)?;
        Ok(Self {
            inner: Subspace::from_bytes(&bytes),
            phantom: PhantomData,
        })
    }
}

impl<T> TypedSubspace<T> {
    pub async fn open_or_create(
        tx: &Transaction,
        dir: &(dyn Directory + Send + Sync),
        name: &str,
    ) -> Result<Self, Error> {
        let subdir = dir
            .create_or_open(tx, vec![name.into()], None, None)
            .await
            .map_err(Error::from_dir)?;
        Ok(Self {
            inner: Subspace::from_bytes(subdir.bytes()),
            phantom: PhantomData,
        })
    }
    pub fn range(&self) -> (Vec<u8>, Vec<u8>) {
        self.inner.range()
    }
}

fn advance_tuple_key(key: &mut [u8]) {
    // Increment the last byte.
    for b in key.iter_mut().rev() {
        *b = b.wrapping_add(1);
        if *b != 0 {
            break;
        }
    }
}

impl<T: TuplePack> TypedSubspace<T> {
    pub fn pack(&self, t: &T) -> Vec<u8> {
        self.inner.pack(t)
    }
    pub fn subrange(&self, r: impl RangeBounds<T>) -> (Vec<u8>, Vec<u8>) {
        let a = match r.start_bound() {
            Bound::Included(x) => self.inner.pack(x),
            Bound::Excluded(x) => next_key(&self.inner.pack(x)),
            Bound::Unbounded => self.inner.range().0,
        };
        let b = match r.end_bound() {
            Bound::Included(y) => next_key(&self.inner.pack(y)),
            Bound::Excluded(y) => self.inner.pack(y),
            Bound::Unbounded => self.inner.range().1,
        };
        (a, b)
    }
    pub fn nested_range<U: TuplePack>(&self, t: &U) -> (Vec<u8>, Vec<u8>)
    where
        T: HasPrefix<U>,
    {
        let a = self.inner.pack(t);
        let mut b = a.clone();
        advance_tuple_key(&mut b);
        (a, b)
    }
    pub fn nested_range2<U: TuplePack, V: TuplePack>(&self, x: &U, y: &V) -> (Vec<u8>, Vec<u8>)
    where
        T: HasPrefix<U> + HasPrefix<V>,
    {
        let a = self.inner.pack(x);
        let mut b = self.inner.pack(y);
        advance_tuple_key(&mut b);
        (a, b)
    }
}

impl<'de, T: TupleUnpack<'de>> TypedSubspace<T> {
    pub fn unpack(&self, key: &'de [u8]) -> Result<T, Error> {
        self.inner.unpack(key).map_err(Into::into)
    }
}
