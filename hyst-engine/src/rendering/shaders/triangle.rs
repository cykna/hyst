use std::sync::Arc;

use bytemuck::{Pod, Zeroable};
use wgpu::{BindGroup, BindGroupLayout, RenderPipeline};

use crate::{
    shaders::{HystConstructor, ShaderInput},
    vec2::Vec2f32,
    vec4::Vec4f32,
};

pub use super::HystShader;

#[derive(Debug, Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct TriangleInput {
    pub position: Vec2f32,
    pub color: Vec4f32,
}

impl ShaderInput for TriangleInput {
    const LAYOUT: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<Self>() as u64,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &wgpu::vertex_attr_array![
            0 => Float32x2,
            1 => Float32x4,
        ],
    };
}
pub struct TriangleShader {
    bind_groups: Vec<BindGroup>,
    bind_group_layouts: Vec<BindGroupLayout>,
    module: wgpu::ShaderModule,
    pipeline: Arc<wgpu::RenderPipeline>,
}

impl HystConstructor for TriangleShader {
    fn new(
        module: wgpu::ShaderModule,
        bindgroups: Vec<BindGroup>,
        layouts: Vec<BindGroupLayout>,
        pipeline: Arc<RenderPipeline>,
    ) -> Self
    where
        Self: Sized,
    {
        Self {
            module,
            bind_groups: bindgroups,
            bind_group_layouts: layouts,
            pipeline,
        }
    }

    fn shader_inputs() -> Vec<wgpu::VertexBufferLayout<'static>> {
        vec![TriangleInput::LAYOUT]
    }

    fn name() -> &'static str {
        "triangle"
    }
}
impl HystShader for TriangleShader {
    fn pipeline(&self) -> &wgpu::RenderPipeline {
        &self.pipeline
    }
    fn module(&self) -> &wgpu::ShaderModule {
        &self.module
    }
    fn bind_group_layouts(&self) -> Option<&[wgpu::BindGroupLayout]> {
        Some(&self.bind_group_layouts)
    }
    fn bind_groups(&self) -> &[wgpu::BindGroup] {
        &self.bind_groups
    }
}
