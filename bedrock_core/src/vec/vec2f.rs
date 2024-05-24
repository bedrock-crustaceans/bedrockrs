use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign};

#[derive(Debug, Copy, Clone)]
pub struct Vec2f {
    pub x: f32,
    pub z: f32,
}

impl Add for Vec2f {
    type Output = Self;

    
    #[inline]
    #[track_caller]
fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vec2f {
    
    #[inline]
    #[track_caller]
fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.z += rhs.z;
    }
}

impl Sub for Vec2f {
    type Output = Self;

    
    #[inline]
    #[track_caller]
fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign for Vec2f {
    
    #[inline]
    #[track_caller]
fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.z -= rhs.z;
    }
}

impl Mul for Vec2f {
    type Output = Self;

    
    #[inline]
    #[track_caller]
fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            z: self.z * rhs.z,
        }
    }
}

impl MulAssign for Vec2f {
    
    #[inline]
    #[track_caller]
fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.z *= rhs.z;
    }
}

impl Div for Vec2f {
    type Output = Self;

    
    #[inline]
    #[track_caller]
fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            z: self.z / rhs.z,
        }
    }
}

impl DivAssign for Vec2f {
    
    #[inline]
    #[track_caller]
fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.z /= rhs.z;
    }
}

impl Rem for Vec2f {
    type Output = Self;

    
    #[inline]
    #[track_caller]
fn rem(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x % rhs.x,
            z: self.z % rhs.z,
        }
    }
}

impl RemAssign for Vec2f {
    
    #[inline]
    #[track_caller]
fn rem_assign(&mut self, rhs: Self) {
        self.x %= rhs.x;
        self.z %= rhs.z;
    }
}

impl Neg for Vec2f {
    type Output = Self;

    
    #[inline]
    #[track_caller]
fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            z: -self.z,
        }
    }
}