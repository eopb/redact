use crate::Secret;

use core::{
    ops,
    ops::{Deref, DerefMut},
};

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

use bytemuck::TransparentWrapper;

/// We introduce this private wrapper around `Secret` to safely implement [TransparentWrapper] without
/// inadvertently providing a way for `Secret`s to be exposed without using `Secret::expose_secret`
#[repr(transparent)]
struct Wrapper<T: ?Sized>(Secret<T>);

// SAFETY: `Secret` and `Wrapper` both contains only a single field and are `#[repr(transparent)]`.
// This meets the documented requirements [bytemuck::TransparentWrapper] as long as we
// do not override any of its methods.
unsafe impl<T: ?Sized> bytemuck::TransparentWrapper<T> for Wrapper<T> {}

impl<T> Deref for Secret<T>
where
    T: Deref,
{
    type Target = Secret<T::Target>;

    fn deref(&self) -> &Self::Target {
        &Wrapper::wrap_ref(self.0.deref()).0
    }
}

impl<T> DerefMut for Secret<T>
where
    T: DerefMut,
{
    fn deref_mut(&mut self) -> &mut Secret<T::Target> {
        &mut Wrapper::wrap_mut(self.0.deref_mut()).0
    }
}
