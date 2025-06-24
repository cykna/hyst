use bytemuck::{Pod, Zeroable};
use hyst_math::vectors::{Vec2f32, Vec4f32};
use crate::shaders::{ShaderInput};
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct ContainerInput {
    position: Vec2f32,
    color: Vec4f32,
}

impl ContainerInput {
    pub fn transparent(x: f32, y: f32) -> Self {
        Self {
            position: Vec2f32::new(x, y),
            color: Vec4f32::new(0.0, 0.0, 0.0, 0.0),
        }
    }
    pub fn solid(x: f32, y: f32, rgba: Vec4f32) -> Self {
        Self {
            position: Vec2f32::new(x, y),
            color: rgba,
        }
    }
}

impl ShaderInput for ContainerInput {
    const LAYOUT: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<Self>() as u64,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &wgpu::vertex_attr_array![
            0 => Float32x2,
            1 => Float32x4
        ],
    };
}