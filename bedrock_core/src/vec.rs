use crate::types::i32le;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Vec2 {
    pub x: i32le,
    pub z: i32le,
}

#[derive(Debug, Copy, Clone)]
pub struct Vec2f {
    pub x: f32,
    pub z: f32,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Vec3 {
    pub x: i32le,
    pub y: i32le,
    pub z: i32le,
}

#[derive(Debug, Copy, Clone)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
