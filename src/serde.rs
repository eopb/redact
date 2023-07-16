use crate::Secret;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// *This API requires the following crate features to be activated: `serde`*
impl<'de, T: Deserialize<'de>> Deserialize<'de> for Secret<T> {
    #[inline]
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        T::deserialize(deserializer).map(Self)
    }
}

#[cfg(feature = "serde")]
#[derive(Serialize)]
struct Test {
    #[serde(serialize_with = "expose_secret")]
    one: Secret<String>,
    #[serde(serialize_with = "expose_secret")]
    two: Option<Secret<String>>,
}

pub trait ContainsSecret<T> {
    type Exposed<'a>: Serialize
    where
        Self: 'a;
    fn expose_secret<'a>(&'a self, expose: impl Fn(&Secret<T>) -> &T) -> Self::Exposed<'a>;
}

impl<T: Serialize> ContainsSecret<T> for Secret<T> {
    type Exposed<'a> = &'a T where T: 'a;

    fn expose_secret<'a>(&'a self, expose: impl Fn(&Secret<T>) -> &T) -> Self::Exposed<'a> {
        expose(self)
    }
}

impl<T: Serialize> ContainsSecret<T> for Option<Secret<T>> {
    type Exposed<'a> = Option<&'a T> where T: 'a;

    fn expose_secret<'a>(&'a self, expose: impl Fn(&Secret<T>) -> &T) -> Self::Exposed<'a> {
        self.as_ref().map(expose)
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
    secret: &impl ContainsSecret<T>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    secret
        .expose_secret(Secret::expose_secret)
        .serialize(serializer)
}
