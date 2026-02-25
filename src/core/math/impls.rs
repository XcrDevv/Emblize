use crate::core::math::*;


impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        return Self { x, y };
    }

    pub fn as_arr(&self) -> [f32; 2] {
        return [self.x, self.y];
    }
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32 ) -> Self {
        return Self { x, y, z };
    }

    pub fn as_arr(&self) -> [f32; 3] {
        return [self.x, self.y, self.z];
    }
}

impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        return Self { x, y, z, w };
    }

    pub fn as_arr(&self) -> [f32; 4] {
        unsafe { *(self as *const Vec4 as *const [f32; 4]) }
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