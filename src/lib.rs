#![doc = include_str!("../README.md")]
#![no_std]
#![forbid(unsafe_code)]

use core::{any::type_name, fmt};

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// See [module level documentation][crate]
#[derive(Default, Copy, Clone, Eq, PartialEq)]
pub struct Secret<T>(T);

impl<T: Serialize> Serialize for Secret<T> {
    #[inline]
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.expose_secret().serialize(serializer)
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Secret<T> {
    #[inline]
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        T::deserialize(deserializer).map(Self)
    }
}

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

impl<T> fmt::Debug for Secret<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[REDACTED {}]", type_name::<T>())
    }
}
