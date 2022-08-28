use num::{Float, Num, NumCast};
use std::fmt;
use std::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};

// ref: cgmath crate

/// Base numeric types with partial ordering
pub trait BaseNum:
    Copy
    + Clone
    + fmt::Debug
    + Num
    + NumCast
    + PartialOrd
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + RemAssign
{
}

macro_rules! impl_basenum_int (
    ($T: ident) => (
        impl BaseNum for $T {}
    )
);

impl_basenum_int!(i8);
impl_basenum_int!(i16);
impl_basenum_int!(i32);
impl_basenum_int!(i64);
impl_basenum_int!(u8);
impl_basenum_int!(u16);
impl_basenum_int!(u32);
impl_basenum_int!(u64);
impl_basenum_int!(isize);
impl_basenum_int!(usize);

macro_rules! impl_basenum_float (
    ($T: ident) => (
        impl BaseNum for $T {}
    )
);

impl_basenum_float!(f32);
impl_basenum_float!(f64);

/// Base integer types
pub trait BaseInt: BaseNum {}

impl BaseInt for i8 {}
impl BaseInt for i16 {}
impl BaseInt for i32 {}
impl BaseInt for i64 {}
impl BaseInt for isize {}
impl BaseInt for u8 {}
impl BaseInt for u16 {}
impl BaseInt for u32 {}
impl BaseInt for u64 {}
impl BaseInt for usize {}

/// Base floating point types
pub trait BaseFloat: BaseNum + Float {}

impl BaseFloat for f32 {}
impl BaseFloat for f64 {}

#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: BaseNum> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Vec3 { x, y, z }
    }

    pub fn from_slice(slice: &[T]) -> Self {
        Vec3 {
            x: slice[0],
            y: slice[1],
            z: slice[2],
        }
    }
}
impl<T: BaseNum> Default for Vec3<T> {
    fn default() -> Self {
        Vec3 {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T: BaseNum> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Vec2 { x, y }
    }

    pub fn from_slice(slice: &[T]) -> Self {
        Vec2 {
            x: slice[0],
            y: slice[1],
        }
    }
}
impl<T: BaseNum> Default for Vec2<T> {
    fn default() -> Self {
        Vec2 {
            x: T::zero(),
            y: T::zero(),
        }
    }
}
