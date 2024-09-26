use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

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

impl<T> From<[T; 3]> for Vec3<T> {
    fn from(value: [T; 3]) -> Self {
        let [x, y, z] = value;

        Self { x, y, z }
    }
}

impl<T> From<Vec3<T>> for [T; 3] {
    fn from(value: Vec3<T>) -> Self {
        [value.x, value.y, value.z]
    }
}

impl<T> From<(T, T, T)> for Vec3<T> {
    fn from(value: (T, T, T)) -> Self {
        let (x, y, z) = value;
        Self { x, y, z }
    }
}

impl<T> From<Vec3<T>> for (T, T, T) {
    fn from(value: Vec3<T>) -> Self {
        (value.x, value.y, value.z)
    }
}
