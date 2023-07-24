use crate::Secret;

use core::ops;

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
