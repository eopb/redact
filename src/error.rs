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
