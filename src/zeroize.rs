use crate::Secret;

use zeroize::{TryZeroize, Zeroize, ZeroizeOnDrop};

impl<T: Zeroize + ?Sized> Zeroize for Secret<T> {
    fn zeroize(&mut self) {
        self.0.zeroize()
    }
}

impl<T: TryZeroize> TryZeroize for Secret<T> {
    fn try_zeroize(&mut self) -> bool {
        self.0.try_zeroize()
    }
}

impl<T: ZeroizeOnDrop> ZeroizeOnDrop for Secret<T> {}

impl<T: Zeroize> Secret<T> {
    pub fn zeroizing(self) -> zeroize::Zeroizing<Self> {
        zeroize::Zeroizing::new(self)
    }
}
