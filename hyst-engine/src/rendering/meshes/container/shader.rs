use std::sync::Arc;

use super::ContainerInput;
use crate::shaders::{HystConstructor, HystShader, ShaderInput};

#[derive(Debug)]
pub struct ContainerShader {
    module: Arc<wgpu::ShaderModule>,
    bindgroups: Vec<wgpu::BindGroup>,
    layouts: Vec<wgpu::BindGroupLayout>,
    pipeline: std::sync::Arc<wgpu::RenderPipeline>,
}

impl HystConstructor for ContainerShader {
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
            bindgroups,
            layouts,
            pipeline,
        }
    }

    fn name() -> &'static str {
        "container"
    }
    fn shader_inputs() -> Vec<wgpu::VertexBufferLayout<'static>> {
        ContainerInput::LAYOUT.to_vec()
    }
}

impl HystShader for ContainerShader {
    fn module(&self) -> &Arc<wgpu::ShaderModule> {
        &self.module
    }
    fn pipeline(&self) -> &wgpu::RenderPipeline {
        &self.pipeline
    }
    fn bind_group_layouts(&self) -> Option<&[wgpu::BindGroupLayout]> {
        None
    }
    fn bind_groups(&self) -> &[wgpu::BindGroup] {
        &self.bindgroups
    }
}
