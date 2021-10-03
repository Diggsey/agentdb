use std::{
    convert::TryFrom,
    fmt::{Debug, Display},
};

use foundationdb::FdbError;

pub struct Error(pub anyhow::Error);

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl TryFrom<Error> for FdbError {
    type Error = Error;

    fn try_from(value: Error) -> Result<Self, Self::Error> {
        value.0.downcast().map_err(Error)
    }
}

impl<E> From<E> for Error
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn from(e: E) -> Self {
        Self(e.into())
    }
}

impl From<Error> for anyhow::Error {
    fn from(e: Error) -> Self {
        e.0
    }
}
