use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

use crate::core::utils::endian::BytesNum;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec2<'de, T>
where 
    T: BytesNum + Serialize + Deserialize<'de>
{
    pub x: T,
    pub y: T,
    _marker: PhantomData<&'de ()>
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec3<'de, T>
where 
    T: BytesNum + Serialize + Deserialize<'de>
{
    pub x: T,
    pub y: T,
    pub z: T,
    _marker: PhantomData<&'de ()>
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec4<'de, T> 
where 
    T: BytesNum + Serialize + Deserialize<'de>
{
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
    _marker: PhantomData<&'de ()>
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Quat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

pub trait VectorNumber: BytesNum + Serialize + for<'de> Deserialize<'de> + Copy {}

impl<T> VectorNumber for T
where
    T: BytesNum + Serialize + for<'de> Deserialize<'de> + Copy,
{}

impl<'de, T> Vec2<'de, T>
where 
    T: VectorNumber
{
    pub fn new(x: T, y: T) -> Self {
        return Self { x, y, _marker: PhantomData };
    }

    pub fn as_arr(&self) -> [T; 2] {
        return [self.x, self.y];
    }
}

impl<'de, T> Vec3<'de, T>
where
    T: VectorNumber
{

    pub fn new(x: T, y: T, z: T ) -> Self {
        return Self { x, y, z,_marker: PhantomData };
    }

    pub fn as_arr(&self) -> [T; 3] {
        return [self.x, self.y, self.z];
    }
}

impl<'de, T> Vec4<'de, T>
where 
    T: VectorNumber
{
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        return Self { x, y, z, w, _marker: PhantomData };
    }

    pub fn as_arr(&self) -> [T; 4] {
        unsafe { *(self as *const Vec4<T> as *const [T; 4]) }
    }
    
}

impl Quat {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        return Self { x, y, z, w };
    }
    
    pub fn as_arr(&self) -> [f32; 4] {
        unsafe { *(self as *const Quat as *const [f32; 4]) }
    }
}