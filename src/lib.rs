#![forbid(unsafe_code)]
// #![warn(missing_docs)]

use core::{any::type_name, fmt};

use serde::{Deserialize, Deserializer, Serialize, Serializer};
/// ```
/// use redact::Secret;
/// let encryption_key: Secret<&str> = Secret::new("hello world");
/// assert_eq!("[REDACTED &str]", format!("{encryption_key:?}"))
/// ```
/// ```
/// use redact::Secret;
/// use redact::ExposeSecret;
/// let encryption_key: Secret<&str> = Secret::new("hello world");
/// assert_eq!("hello world", *encryption_key.expose_secret())
/// ```
#[derive(Default, Copy, Clone, Eq, PartialEq)]
pub struct Secret<T>(T);

impl<T: Serialize> Serialize for Secret<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Secret<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        T::deserialize(deserializer).map(Self)
    }
}

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
