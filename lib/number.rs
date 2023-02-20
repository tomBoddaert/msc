use core::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Not, Sub};

/// Generic number trait to group other traits and provide
/// zero and one constants
pub trait Number
where
    Self: Add<Output = Self>
        + Sub<Output = Self>
        + Mul<Output = Self>
        + Div<Output = Self>
        + Not<Output = Self>
        + BitOr<Output = Self>
        + BitAnd<Output = Self>
        + BitXor<Output = Self>
        + Sized
        + Ord
        + Default
        + Copy,
{
    const ZERO: Self;
    const ONE: Self;
}

macro_rules! number_impl {
    ( $t:ty ) => {
        impl Number for $t {
            const ZERO: Self = 0;
            const ONE: Self = 1;
        }
    };
    ( $t:ty, wrap ) => {
        number_impl!($t);
        impl Number for core::num::Wrapping<$t> {
            const ZERO: Self = Self(0);
            const ONE: Self = Self(1);
        }
    };
}

number_impl!(u8, wrap);
number_impl!(u16, wrap);
number_impl!(u32, wrap);
number_impl!(u64, wrap);
number_impl!(u128, wrap);

number_impl!(i8, wrap);
number_impl!(i16, wrap);
number_impl!(i32, wrap);
number_impl!(i64, wrap);
number_impl!(i128, wrap);
