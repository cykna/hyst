use bytemuck::{Pod, Zeroable};

#[derive(Debug, Clone, Copy, Zeroable, Pod)]
#[repr(C)]
pub struct Vec4f32(f32, f32, f32, f32);

impl Vec4f32 {
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self(x, y, z, w)
    }
    pub const fn x(&self) -> f32 {
        self.0
    }
    pub const fn y(&self) -> f32 {
        self.1
    }
    pub const fn z(&self) -> f32 {
        self.2
    }
    pub const fn w(&self) -> f32 {
        self.3
    }
}
