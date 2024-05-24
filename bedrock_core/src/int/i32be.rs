#![allow(non_camel_case_types)]

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct i32be(pub i32);

impl Add for i32be {
    type Output = Self;

    #[inline]
    #[track_caller]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for i32be {
    #[inline]
    #[track_caller]
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl Sub for i32be {
    type Output = Self;

    #[inline]
    #[track_caller]
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl SubAssign for i32be {
    #[inline]
    #[track_caller]
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0
    }
}

impl Mul for i32be {
    type Output = Self;

    #[inline]
    #[track_caller]
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl MulAssign for i32be {
    #[inline]
    #[track_caller]
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0
    }
}

impl Div for i32be {
    type Output = Self;

    #[inline]
    #[track_caller]
    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl DivAssign for i32be {
    #[inline]
    #[track_caller]
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0
    }
}

impl Rem for i32be {
    type Output = Self;

    #[inline]
    #[track_caller]
    fn rem(self, rhs: Self) -> Self::Output {
        Self(self.0 % rhs.0)
    }
}

impl RemAssign for i32be {
    #[inline]
    #[track_caller]
    fn rem_assign(&mut self, rhs: Self) {
        self.0 %= rhs.0
    }
}

impl Neg for i32be {
    type Output = Self;

    #[inline]
    #[track_caller]
    fn neg(self) -> Self::Output {
        -Self(self.0)
    }
}
