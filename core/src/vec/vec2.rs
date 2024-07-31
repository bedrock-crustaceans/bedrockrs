use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use crate::int::{BE, LE, VAR};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T: Add<Output = T>> Add for Vec2<T> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: AddAssign> AddAssign for Vec2<T> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Sub<Output = T>> Sub for Vec2<T> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: SubAssign> SubAssign for Vec2<T> {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: Mul<Output = T>> Mul for Vec2<T> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl<T: MulAssign> MulAssign for Vec2<T> {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl<T: Div<Output = T>> Div for Vec2<T> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl<T: DivAssign> DivAssign for Vec2<T> {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl<T: Rem<Output = T>> Rem for Vec2<T> {
    type Output = Self;

    #[inline]
    fn rem(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x % rhs.x,
            y: self.y % rhs.y,
        }
    }
}

impl<T: RemAssign> RemAssign for Vec2<T> {
    #[inline]
    fn rem_assign(&mut self, rhs: Self) {
        self.x %= rhs.x;
        self.y %= rhs.y;
    }
}

impl<T: Neg<Output = T>> Neg for Vec2<T> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

// Serde

impl<T: Serialize> Serialize for Vec2<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        <[&T; 2]>::serialize(&[&self.x, &self.y], serializer)
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Vec2<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let [x, y] = <[T; 2]>::deserialize(deserializer)?;

        Ok(Self { x, y })
    }
}

impl<T> Vec2<T> {
    // LE
    #[inline]
    pub fn to_le(self) -> Vec2<LE<T>> {
        Vec2 {
            x: LE::new(self.x),
            y: LE::new(self.y),
        }
    }

    #[inline]
    pub fn from_le(le: Vec2<LE<T>>) -> Vec2<T> {
        Vec2 {
            x: le.x.into_inner(),
            y: le.y.into_inner(),
        }
    }

    // BE
    #[inline]
    pub fn to_be(self) -> Vec2<BE<T>> {
        Vec2 {
            x: BE::new(self.x),
            y: BE::new(self.y),
        }
    }

    #[inline]
    pub fn from_be(be: Vec2<BE<T>>) -> Vec2<T> {
        Vec2 {
            x: be.x.into_inner(),
            y: be.y.into_inner(),
        }
    }

    // VAR
    #[inline]
    pub fn to_var(self) -> Vec2<VAR<T>> {
        Vec2 {
            x: VAR::new(self.x),
            y: VAR::new(self.y),
        }
    }

    #[inline]
    pub fn from_var(var: Vec2<VAR<T>>) -> Vec2<T> {
        Vec2 {
            x: var.x.into_inner(),
            y: var.y.into_inner(),
        }
    }
}
