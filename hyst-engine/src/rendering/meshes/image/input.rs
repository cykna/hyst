use bytemuck::{Pod,Zeroable};
use hyst_math::vectors::{Vec2f32, Vec4f32};
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct ImageInput {
    position: Vec2f32,
    uv: Vec2f32,
}

impl ImageInput {
    pub const LAYOUT: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<Self>() as u64,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &wgpu::vertex_attr_array![
            0=>Float32x2,
            1 => Float32x2
        ],
    };
    pub fn new(x: f32, y: f32, u: f32, v: f32) -> Self {
        Self {
            position: Vec2f32::new(x, y),
            uv: Vec2f32::new(u, v),
        }
    }
}