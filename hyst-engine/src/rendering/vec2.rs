use bytemuck::{Pod, Zeroable};
#[derive(Debug, Clone, Copy, Zeroable, Pod)]
#[repr(C)]
pub struct Vec2f32(f32, f32);

impl Vec2f32 {
    pub fn new(x: f32, y: f32) -> Self {
        Self(x, y)
    }
    pub fn x(&self) -> f32 {
        self.0
    }
    pub fn y(&self) -> f32 {
        self.1
    }
    pub fn set_coords(&mut self, x: f32, y: f32) {
        self.0 = x;
        self.1 = y;
    }
    pub fn min(&self, rhs: &Self) -> Self {
        Self(self.0.min(rhs.0), self.1.min(rhs.1))
    }
}
impl<T> From<Vec2f32> for (T, T)
where
    f32: Into<T>,
{
    fn from(value: Vec2f32) -> Self {
        (value.0.into(), value.1.into())
    }
}
