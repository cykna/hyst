use crate::shaders::ShaderInput;
use bytemuck::{Pod, Zeroable};
use hyst_math::{
    Rect,
    vectors::{Vec2f32, Vec4f32},
};
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct ContainerInput {
    position: Vec2f32,
}

impl ContainerInput {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            position: Vec2f32::new(x, y),
        }
    }
}

#[repr(C)]
#[derive(Debug, Pod, Zeroable, Clone, Copy)]
pub struct ContainerInstance {
    color: Vec4f32,
    rect: Rect,
}

impl ContainerInstance {
    pub fn new(color: Vec4f32, rect: Rect) -> Self {
        Self { color, rect }
    }
    pub fn color_mut(&mut self) -> &mut Vec4f32 {
        &mut self.color
    }

    pub fn rect(&self) -> &Rect {
        &self.rect
    }

    pub fn rect_mut(&mut self) -> &mut Rect {
        &mut self.rect
    }
}

impl ShaderInput for ContainerInput {
    const LAYOUT: &[wgpu::VertexBufferLayout<'static>] = &[
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &wgpu::vertex_attr_array![
                0 => Float32x2,
            ],
        },
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<ContainerInstance>() as u64,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &wgpu::vertex_attr_array![
                1 => Float32x4,
                2 => Float32x2,
                3 => Float32x2
            ],
        },
    ];
}
