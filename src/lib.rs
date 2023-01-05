#![doc = include_str!("../README.md")]
#![no_std]
#![forbid(unsafe_code)]

use core::{any::type_name, fmt, str::FromStr};

/// See [module level documentation][crate]
#[derive(Default, Hash, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Secret<T>(T);

impl<T> Secret<T> {
    /// See [module level documentation][crate]
    #[inline]
    pub const fn new(secret: T) -> Self {
        Self(secret)
    }
    /// See [module level documentation][crate]
    #[inline]
    pub const fn expose_secret(&self) -> &T {
        &self.0
    }
}

impl<T> From<T> for Secret<T> {
    #[inline]
    fn from(secret: T) -> Self {
        Self::new(secret)
    }
}

impl<T: FromStr> FromStr for Secret<T> {
    type Err = T::Err;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse().map(Self)
    }
}

impl<T> fmt::Debug for Secret<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[REDACTED {}]", type_name::<T>())
    }
}

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[cfg(feature = "serde")]
/// *This API requires the following crate features to be activated: `serde`*
impl<'de, T: Deserialize<'de>> Deserialize<'de> for Secret<T> {
    #[inline]
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        T::deserialize(deserializer).map(Self)
    }
}

#[cfg(feature = "serde")]
/// Exposes a [Secret] for serialization.
///
/// For general-purpose secret exposing see [Secret::expose_secret].
///
/// See [module level documentation][crate] for usage example.
///
/// *This API requires the following crate features to be activated: `serde`*
#[inline]
pub fn expose_secret<S: Serializer, T: Serialize>(
    secret: &Secret<T>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    secret.expose_secret().serialize(serializer)
}
