use std::sync::Arc;
use super::ContainerInput;
use crate::shaders::{HystConstructor, HystShader, ShaderInput};
use bytemuck::{Pod, Zeroable};

// Dados de instância para índices
#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct InstanceData {
    index: u32, // Índice do elemento no buffer de vértices
}

#[derive(Debug)]
pub struct ContainerShader {
    module: Arc<wgpu::ShaderModule>,
    bindgroups: Vec<wgpu::BindGroup>,
    layouts: Vec<wgpu::BindGroupLayout>,
    pipeline: Arc<wgpu::RenderPipeline>,
}

impl HystConstructor for ContainerShader {
    fn new(
        module: Arc<wgpu::ShaderModule>,
        bindgroups: Vec<wgpu::BindGroup>,
        layouts: Vec<wgpu::BindGroupLayout>,
        pipeline: Arc<wgpu::RenderPipeline>,
    ) -> Self {
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
        vec![
            ContainerInput::LAYOUT, // Buffer de vértices
            wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<InstanceData>() as u64,
                step_mode: wgpu::VertexStepMode::Instance,
                attributes: &[wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Uint32,
                    offset: 0,
                    shader_location: 2,
                }],
            },
        ]
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
        Some(&self.layouts)
    }

    fn bind_groups(&self) -> &[wgpu::BindGroup] {
        &self.bindgroups
    }
}
