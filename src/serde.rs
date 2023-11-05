use crate::Secret;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// *This API requires the following crate features to be activated: `serde`*
impl<'de, T: Deserialize<'de>> Deserialize<'de> for Secret<T> {
    #[inline]
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        T::deserialize(deserializer).map(Self)
    }
}

/// A serializable type that contains a secret.
///
/// This abstraction enables [expose_secret] to be used to serialize both `Secret<T>` and
/// `Option<Secret<T>>`.
///
/// This type is currently private. If you feel that a public `SerializableSecret` could
/// be useful for your use case please open a PR :)
pub trait SerializableSecret<T> {
    type Exposed<'a>: Serialize
    where
        Self: 'a;
    /// To reduce the number of functions that are able to expose secrets we require
    /// that the [`Secret::expose_secret`] function is passed in here.
    fn expose_via<'a>(&'a self, expose: impl Fn(&Secret<T>) -> &T) -> Self::Exposed<'a>;
}

impl<T: Serialize> SerializableSecret<T> for Secret<T> {
    type Exposed<'a> = &'a T where T: 'a;

    fn expose_via<'a>(&'a self, expose: impl Fn(&Secret<T>) -> &T) -> Self::Exposed<'a> {
        expose(self)
    }
}

impl<T: Serialize> SerializableSecret<T> for Option<Secret<T>> {
    type Exposed<'a> = Option<&'a T> where T: 'a;

    fn expose_via<'a>(&'a self, expose: impl Fn(&Secret<T>) -> &T) -> Self::Exposed<'a> {
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
    secret: &impl SerializableSecret<T>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    secret
        .expose_via(Secret::expose_secret)
        .serialize(serializer)
}

#[cfg(feature = "serde")]
#[inline]
pub fn redacted<S: Serializer, T>(secret: &Secret<T>, serializer: S) -> Result<S::Ok, S::Error> {
    format!("{secret:?}").serialize(serializer)
}

#[cfg(test)]
mod tests {
    use super::*;
    use fake::{Fake, Faker};
    use serde::{Deserialize, Serialize};

    #[test]
    fn deserialize_the_serialized() {
        #[derive(Serialize, Deserialize, fake::Dummy, PartialEq, Debug)]
        struct Test {
            #[serde(serialize_with = "expose_secret")]
            one: Secret<String>,
            #[serde(serialize_with = "expose_secret")]
            two: Option<Secret<String>>,
        }

        let to_serialize: Test = Faker.fake();

        let serialized = serde_json::to_string(&to_serialize).unwrap();

        let deserialized = serde_json::from_str(&serialized).unwrap();

        assert_eq!(to_serialize, deserialized)
    }
}
