#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]

#[cfg(feature = "std")]
mod error;
#[cfg(feature = "fake")]
mod fake;
mod ops;
#[cfg(feature = "serde")]
mod serde;

#[cfg(feature = "serde")]
pub use crate::serde::expose_secret;

use core::{any::type_name, fmt, str::FromStr};

/// See [module level documentation][crate]
#[derive(Default, Hash, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Secret<T>(T);

impl<T> Secret<T> {
    /// See [module level documentation][crate]
    #[inline]
    pub const fn new(secret: T) -> Self {
        Self(secret)
    }
    #[inline]
    pub fn from(secret: impl Into<T>) -> Self {
        Self(secret.into())
    }
    #[inline]
    pub fn try_from<U: TryInto<T>>(secret: U) -> Result<Self, Secret<U::Error>> {
        secret.try_into().map(Self).map_err(Secret)
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

impl<T: FromStr> FromStr for Secret<T> {
    type Err = Secret<T::Err>;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse().map(Self).map_err(Secret)
    }
}

impl<T> From<Option<Secret<T>>> for Secret<Option<T>> {
    #[inline]
    fn from(secret: Option<Secret<T>>) -> Self {
        Self(secret.map(|Secret(s)| s))
    }
}

impl<T, E> From<Result<Secret<T>, E>> for Secret<Result<T, E>> {
    #[inline]
    fn from(secret: Result<Secret<T>, E>) -> Self {
        Self(secret.map(|Secret(s)| s))
    }
}

impl<T, E> From<Result<T, Secret<E>>> for Secret<Result<T, E>> {
    #[inline]
    fn from(secret: Result<T, Secret<E>>) -> Self {
        Self(secret.map_err(|Secret(s)| s))
    }
}

impl<T, E> From<Result<Secret<T>, Secret<E>>> for Secret<Result<T, E>> {
    #[inline]
    fn from(secret: Result<Secret<T>, Secret<E>>) -> Self {
        Self(secret.map(|Secret(s)| s).map_err(|Secret(s)| s))
    }
}

impl<S: FromIterator<T>, T> FromIterator<Secret<T>> for Secret<S> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = Secret<T>>>(iter: I) -> Self {
        Self(S::from_iter(iter.into_iter().map(|Secret(s)| s)))
    }
}
