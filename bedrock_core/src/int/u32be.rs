#![allow(non_camel_case_types)]

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct u32be(pub u32);

impl Add for u32be {
    type Output = Self;

    #[inline]
    #[track_caller]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for u32be {
    #[inline]
    #[track_caller]
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl Sub for u32be {
    type Output = Self;

    #[inline]
    #[track_caller]
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl SubAssign for u32be {
    #[inline]
    #[track_caller]
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0
    }
}

impl Mul for u32be {
    type Output = Self;

    #[inline]
    #[track_caller]
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl MulAssign for u32be {
    #[inline]
    #[track_caller]
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0
    }
}

impl Div for u32be {
    type Output = Self;

    #[inline]
    #[track_caller]
    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl DivAssign for u32be {
    #[inline]
    #[track_caller]
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0
    }
}

impl Rem for u32be {
    type Output = Self;

    #[inline]
    #[track_caller]
    fn rem(self, rhs: Self) -> Self::Output {
        Self(self.0 % rhs.0)
    }
}

impl RemAssign for u32be {
    #[inline]
    #[track_caller]
    fn rem_assign(&mut self, rhs: Self) {
        self.0 %= rhs.0
    }
}
