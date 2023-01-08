#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]

use core::{any::type_name, fmt, ops, str::FromStr};

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
    fn from(secret: Option<Secret<T>>) -> Self {
        Self(secret.map(|Secret(s)| s))
    }
}

impl<T, E> From<Result<Secret<T>, E>> for Secret<Result<T, E>> {
    fn from(secret: Result<Secret<T>, E>) -> Self {
        Self(secret.map(|Secret(s)| s))
    }
}

impl<T, E> From<Result<T, Secret<E>>> for Secret<Result<T, E>> {
    fn from(secret: Result<T, Secret<E>>) -> Self {
        Self(secret.map_err(|Secret(s)| s))
    }
}

impl<T, E> From<Result<Secret<T>, Secret<E>>> for Secret<Result<T, E>> {
    fn from(secret: Result<Secret<T>, Secret<E>>) -> Self {
        Self(secret.map(|Secret(s)| s).map_err(|Secret(s)| s))
    }
}

impl<S: FromIterator<T>, T> FromIterator<Secret<T>> for Secret<S> {
    fn from_iter<I: IntoIterator<Item = Secret<T>>>(iter: I) -> Self {
        Self(S::from_iter(iter.into_iter().map(|Secret(s)| s)))
    }
}

macro_rules! ops {
    { ($type:tt, $trait:ident, $method:ident), $($tt:tt)* } => {
        ops!(($type, $trait, $method));
        ops!($($tt)*);
    };
    { (binary, $trait:ident, $method:ident)} => {
        impl<T, U> ops::$trait<Secret<U>> for Secret<T>
        where
            T: ops::$trait<U>,
        {
            type Output = Secret<T::Output>;
            #[inline]
            fn $method(self, rhs: Secret<U>) -> Self::Output {
                Secret(self.0.$method(rhs.0))
            }
        }
    };
    { (assign, $trait:ident, $method:ident)} => {
        impl<T, U> ops::$trait<Secret<U>> for Secret<T>
        where
            T: ops::$trait<U>,
        {
            #[inline]
            fn $method(&mut self, rhs: Secret<U>) {
                self.0.$method(rhs.0)
            }
        }
    };
    { (unary, $trait:ident, $method:ident)} => {
        impl<T> ops::$trait for Secret<T>
        where
            T: ops::$trait,
        {
            type Output = Secret<T::Output>;
            #[inline]
            fn $method(self) -> Self::Output {
                Secret(self.0.$method())
            }
        }
    };
    () => ()
}

ops! {
    (binary, Add, add),
    (assign, AddAssign, add_assign),
    (binary, BitAnd, bitand),
    (assign, BitAndAssign, bitand_assign),
    (binary, BitOr, bitor),
    (assign, BitOrAssign, bitor_assign),
    (binary, BitXor, bitxor),
    (assign, BitXorAssign, bitxor_assign),
    (binary, Div, div),
    (assign, DivAssign, div_assign),
    (binary, Mul, mul),
    (assign, MulAssign, mul_assign),
    (binary, Rem, rem),
    (assign, RemAssign, rem_assign),
    (binary, Shl, shl),
    (assign, ShlAssign, shl_assign),
    (binary, Shr, shr),
    (assign, ShrAssign, shr_assign),
    (binary, Sub, sub),
    (assign, SubAssign, sub_assign),
    (unary, Neg, neg),
    (unary, Not, not),
}

#[cfg(feature = "std")]
mod error {
    use crate::Secret;

    use core::fmt;
    use std::error::Error;

    impl<E: Error> fmt::Display for Secret<E> {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            <Self as fmt::Debug>::fmt(self, f)
        }
    }

    impl<E: Error> Error for Secret<E> {}
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
