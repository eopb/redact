use crate::Secret;

use fake::Dummy;
use rand::Rng;

impl<T: Dummy<U>, U> Dummy<U> for Secret<T> {
    #[inline]
    fn dummy_with_rng<R: Rng + ?Sized>(config: &U, rng: &mut R) -> Self {
        Secret(T::dummy_with_rng(config, rng))
    }

    #[inline]
    fn dummy(config: &U) -> Self {
        Secret(T::dummy(config))
    }
}
