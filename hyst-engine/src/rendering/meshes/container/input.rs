use crate::{meshes::Updatable, shaders::ShaderInput};
use bytemuck::{Pod, Zeroable};
use hyst_math::{
    Rect,
    vectors::{Vec2f32, Vec4f32},
};
use taffy::Layout;
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
    rect: Rect,
    color: Vec4f32,
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

impl Updatable<Layout> for ContainerInstance {
    fn update(&mut self, data: &Layout) {
        self.rect
            .position_mut()
            .set_coords(data.content_box_x(), data.content_box_y());
        self.rect
            .size_mut()
            .set_coords(data.content_box_width(), data.content_box_height());
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
                1 => Float32x2,
                2 => Float32x2,
                3 => Float32x4
            ],
        },
    ];
}
