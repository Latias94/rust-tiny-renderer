use num::{Float, Num, NumCast};
use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, RemAssign, Sub, SubAssign};

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
        Self { x, y, z }
    }

    pub fn from_slice(slice: &[T]) -> Self {
        Self {
            x: slice[0],
            y: slice[1],
            z: slice[2],
        }
    }

    pub fn cross_product(self, rhs: Vec3<T>) -> Self {
        Self {
            x: (self.y * rhs.z - self.z * rhs.y),
            y: (self.z * rhs.z - self.x * rhs.z),
            z: (self.x * rhs.y - self.y * rhs.x),
        }
    }
}
impl<T: BaseNum> Default for Vec3<T> {
    fn default() -> Self {
        Self {
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
        Self { x, y }
    }

    pub fn from_slice(slice: &[T]) -> Self {
        Self {
            x: slice[0],
            y: slice[1],
        }
    }
}
impl<T: BaseNum> Default for Vec2<T> {
    fn default() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
        }
    }
}

impl<T: BaseNum> Add for Vec2<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: BaseNum> Add for Vec3<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: BaseNum> Sub for Vec2<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: BaseNum> Sub for Vec3<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T: BaseNum> Mul for Vec2<T> {
    type Output = T;

    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl<T: BaseNum> Mul for Vec3<T> {
    type Output = T;

    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl Mul<f64> for Vec2<isize> {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: (self.x as f64 * rhs) as isize,
            y: (self.y as f64 * rhs) as isize,
        }
    }
}

impl Mul<f64> for Vec3<isize> {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: (self.x as f64 * rhs) as isize,
            y: (self.y as f64 * rhs) as isize,
            z: (self.z as f64 * rhs) as isize,
        }
    }
}

impl Mul<f32> for Vec2<isize> {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: (self.x as f32 * rhs) as isize,
            y: (self.y as f32 * rhs) as isize,
        }
    }
}

impl Mul<f32> for Vec3<isize> {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: (self.x as f32 * rhs) as isize,
            y: (self.y as f32 * rhs) as isize,
            z: (self.z as f32 * rhs) as isize,
        }
    }
}

impl<T: BaseNum> Div<T> for Vec3<T> {
    type Output = Self;

    fn div(self, m: T) -> Self::Output {
        Self {
            x: self.x / m,
            y: self.y / m,
            z: self.z / m,
        }
    }
}

impl Vec3<f32> {
    pub fn normalize(self) -> Self {
        let m = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        self / m
    }
}
