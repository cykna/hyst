use bytemuck::{Pod, Zeroable};

use crate::vectors::Vec2f32;

#[repr(C)]
#[derive(Clone, Debug, Copy, Pod, Zeroable)]
pub struct Rect {
    position: Vec2f32,
    size: Vec2f32,
}

impl Rect {
    pub fn from_xywh(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self {
            position: Vec2f32::new(x, y),
            size: Vec2f32::new(w, h),
        }
    }
    pub fn new(position: Vec2f32, size: Vec2f32) -> Self {
        Self { position, size }
    }

    ///Gets a mutable reference to the size of this rectangle
    pub fn size_mut(&mut self) -> &mut Vec2f32 {
        &mut self.size
    }
    pub fn size(&self) -> &Vec2f32 {
        &self.size
    }
}
