#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

use core::{any::type_name, fmt};

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Default, Copy, Clone, Eq, PartialEq)]
#[serde(transparent)]
pub struct Secret<T>(T);

/// See [module level documentation][crate]
impl<T> Secret<T> {
    #[inline]
    pub fn new(secret: T) -> Self {
        Self(secret)
    }
}

impl<T> From<T> for Secret<T> {
    #[inline]
    fn from(secret: T) -> Self {
        Self::new(secret)
    }
}

impl<T> fmt::Debug for Secret<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[REDACTED {}]", type_name::<T>())
    }
}

/// See [module level documentation][crate]
pub trait ExposeSecret: Sealed {
    type Secret;
    fn expose_secret(&self) -> &Self::Secret;
}

use private::Sealed;
mod private {
    pub trait Sealed {}
    impl<T> Sealed for super::Secret<T> {}
}

impl<T> ExposeSecret for Secret<T> {
    type Secret = T;
    #[inline]
    fn expose_secret(&self) -> &Self::Secret {
        &self.0
    }
}
