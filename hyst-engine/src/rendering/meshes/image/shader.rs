use std::sync::Arc;

use super::ImageInput;
use crate::shaders::{HystConstructor, HystShader, ShaderInput};

#[derive(Debug)]
pub struct ImageShader {
    module: Arc<wgpu::ShaderModule>,
    layouts: Vec<wgpu::BindGroupLayout>,
    bind_groups: Vec<wgpu::BindGroup>,
    pipeline: Arc<wgpu::RenderPipeline>,
}

impl HystShader for ImageShader {
    fn module(&self) -> &Arc<wgpu::ShaderModule> {
        &self.module
    }
    fn pipeline(&self) -> &wgpu::RenderPipeline {
        &self.pipeline
    }
    fn bind_group_layouts(&self) -> Option<&[wgpu::BindGroupLayout]> {
        Some(&self.layouts)
    }
    fn bind_groups(&self) -> &[wgpu::BindGroup] {
        &self.bind_groups
    }
}

impl HystConstructor for ImageShader {
    fn new(
        module: Arc<wgpu::ShaderModule>,
        bindgroups: Vec<wgpu::BindGroup>,
        layouts: Vec<wgpu::BindGroupLayout>,
        pipeline: std::sync::Arc<wgpu::RenderPipeline>,
    ) -> Self
    where
        Self: Sized,
    {
        Self {
            module,
            bind_groups: bindgroups,
            layouts,
            pipeline,
        }
    }
    fn shader_inputs() -> Vec<wgpu::VertexBufferLayout<'static>> {
        ImageInput::LAYOUT.to_vec()
    }
    fn name() -> &'static str {
        "image_shader"
    }
}
