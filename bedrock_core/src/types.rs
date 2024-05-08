#![allow(non_camel_case_types)]

// i8
// use normal i8

// u8
// use normal u8

use std::fmt::Debug;

// i16
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct i16le(pub i16);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct i16be(pub i16);

// u16
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct u16le(pub u16);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct u16be(pub u16);

// i32
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct i32le(pub i32);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct i32be(pub i32);

// u32
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct u32le(pub u32);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct u32be(pub u32);

// i64
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct i64le(pub i64);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct i64be(pub i64);

// u64
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct u64le(pub u64);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct u64be(pub u64);

// i128
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct i128le(pub i128);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct i128be(pub i128);

// u128
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct u128le(pub u128);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct u128be(pub u128);

// f32
// use normal f32

// f64
// use normal f64

// uvarint 32
#[derive(Debug, Copy, Clone)]
pub struct uvar32(pub u32);

// ivarint 32
#[derive(Debug, Copy, Clone)]
pub struct ivar32(pub i32);

// uvarint 64
#[derive(Debug, Copy, Clone)]
pub struct uvar64(pub u64);

// ivarint 64
#[derive(Debug, Copy, Clone)]
pub struct ivar64(pub i64);

// bool
// use normal bool

// string
// use normal string
