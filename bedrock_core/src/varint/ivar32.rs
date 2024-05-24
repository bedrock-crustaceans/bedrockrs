#![allow(non_camel_case_types)]

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign};

#[derive(Debug, Copy, Clone)]
pub struct ivar32(pub i32);

impl Add for ivar32 {
    type Output = Self;

    #[inline]
    #[track_caller]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for ivar32 {
    #[inline]
    #[track_caller]
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl Sub for ivar32 {
    type Output = Self;

    #[inline]
    #[track_caller]
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl SubAssign for ivar32 {
    #[inline]
    #[track_caller]
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0
    }
}

impl Mul for ivar32 {
    type Output = Self;

    #[inline]
    #[track_caller]
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl MulAssign for ivar32 {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0
    }
}

impl Div for ivar32 {
    type Output = Self;

    #[inline]
    #[track_caller]
    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl DivAssign for ivar32 {
    #[inline]
    #[track_caller]
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0
    }
}

impl Rem for ivar32 {
    type Output = Self;

    #[inline]
    #[track_caller]
    fn rem(self, rhs: Self) -> Self::Output {
        Self(self.0 % rhs.0)
    }
}

impl RemAssign for ivar32 {
    #[inline]
    #[track_caller]
    fn rem_assign(&mut self, rhs: Self) {
        self.0 %= rhs.0
    }
}

impl Neg for ivar32 {
    type Output = Self;

    #[inline]
    #[track_caller]
    fn neg(self) -> Self::Output {
        -Self(self.0)
    }
}
