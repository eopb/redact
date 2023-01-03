#![forbid(unsafe_code)]
// #![warn(missing_docs)]

use std::{any::type_name, fmt};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq)]
#[serde(transparent)]
pub struct Secret<T, const DISPLAY_TYPE_NAME: bool = true>(T);

impl<T> fmt::Debug for Secret<T, false> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[REDACTED]")
    }
}

impl<T> fmt::Debug for Secret<T, true> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[REDACTED {}]", type_name::<T>())
    }
}

pub trait ExposeSecret: Sealed {
    type Secret;
    fn expose_secret(&self) -> &Self::Secret;
}

use private::Sealed;
mod private {
    pub trait Sealed {}
    impl<T, const DISPLAY_TYPE_NAME: bool> Sealed for super::Secret<T, DISPLAY_TYPE_NAME> {}
}

impl<T, const DISPLAY_TYPE_NAME: bool> ExposeSecret for Secret<T, DISPLAY_TYPE_NAME> {
    type Secret = T;
    fn expose_secret(&self) -> &Self::Secret {
        &self.0
    }
}
