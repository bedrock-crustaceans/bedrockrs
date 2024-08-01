use std::{fmt, ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
}};

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use crate::int::{BE, LE, VAR};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Add<Output = T>> Add for Vec3<T> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: AddAssign> AddAssign for Vec3<T> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T: Sub<Output = T>> Sub for Vec3<T> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T: SubAssign> SubAssign for Vec3<T> {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl<T: Mul<Output = T>> Mul for Vec3<T> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl<T: MulAssign> MulAssign for Vec3<T> {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl<T: Div<Output = T>> Div for Vec3<T> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl<T: DivAssign> DivAssign for Vec3<T> {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

impl<T: Rem<Output = T>> Rem for Vec3<T> {
    type Output = Self;

    #[inline]
    fn rem(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x % rhs.x,
            y: self.y % rhs.y,
            z: self.z % rhs.z,
        }
    }
}

impl<T: RemAssign> RemAssign for Vec3<T> {
    #[inline]
    fn rem_assign(&mut self, rhs: Self) {
        self.x %= rhs.x;
        self.y %= rhs.y;
        self.z %= rhs.z;
    }
}

impl<T: Neg<Output = T>> Neg for Vec3<T> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

// Serde

impl<T: Serialize> Serialize for Vec3<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        <[&T; 3]>::serialize(&[&self.x, &self.y, &self.z], serializer)
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Vec3<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let [x, y, z] = <[T; 3]>::deserialize(deserializer)?;

        Ok(Self { x, y, z })
    }
}


impl<T> Vec3<T> {
    // LE
    #[inline]
    pub fn to_le(self) -> Vec3<LE<T>> {
        Vec3 {
            x: LE::new(self.x),
            y: LE::new(self.y),
            z: LE::new(self.z),
        }
    }

    #[inline]
    pub fn from_le(le: Vec3<LE<T>>) -> Vec3<T> {
        Vec3 {
            x: le.x.into_inner(),
            y: le.y.into_inner(),
            z: le.z.into_inner(),
        }
    }

    // BE
    #[inline]
    pub fn to_be(self) -> Vec3<BE<T>> {
        Vec3 {
            x: BE::new(self.x),
            y: BE::new(self.y),
            z: BE::new(self.z),
        }
    }

    #[inline]
    pub fn from_be(be: Vec3<BE<T>>) -> Vec3<T> {
        Vec3 {
            x: be.x.into_inner(),
            y: be.y.into_inner(),
            z: be.z.into_inner(),
        }
    }

    // VAR
    #[inline]
    pub fn to_var(self) -> Vec3<VAR<T>> {
        Vec3 {
            x: VAR::new(self.x),
            y: VAR::new(self.y),
            z: VAR::new(self.z),
        }
    }

    #[inline]
    pub fn from_var(var: Vec3<VAR<T>>) -> Vec3<T> {
        Vec3 {
            x: var.x.into_inner(),
            y: var.y.into_inner(),
            z: var.z.into_inner(),
        }
    }
}