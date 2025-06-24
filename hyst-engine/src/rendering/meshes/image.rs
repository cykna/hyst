use std::{path::Path, sync::Arc};

use bytemuck::{Pod, Zeroable};
use image::GenericImageView;
use wgpu::{BindGroup, BindGroupLayout, RenderPipeline, ShaderStages};

use crate::{
    core::{BindGroupAndLayoutConfig, GpuImage, RenderingCore},
    mesh::Mesh,
    meshes::container::{AbstractBuffer, BufferType},
    rectangle::Rect,
    shaders::{HystConstructor, HystShader, ShaderRenderMethod},
    ui::HystImageCreationOption,
    vec2::Vec2f32,
};

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

pub struct ImageShader {
    module: wgpu::ShaderModule,
    layouts: Vec<BindGroupLayout>,
    bind_groups: Vec<BindGroup>,
    pipeline: Arc<RenderPipeline>,
}

impl HystShader for ImageShader {
    fn module(&self) -> &wgpu::ShaderModule {
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
        module: wgpu::ShaderModule,
        bindgroups: Vec<BindGroup>,
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
        vec![ImageInput::LAYOUT]
    }
    fn name() -> &'static str {
        "image_shader"
    }
}

pub struct Image {
    vertices: AbstractBuffer<[ImageInput; 4]>,
    screen_size: AbstractBuffer<[f32; 2]>,
    indices: wgpu::Buffer,
    area: AbstractBuffer<Rect>,
    shader: ImageShader,
    indices_len: u32,
    image: GpuImage,
}

impl Image {
    pub fn from_configs(
        core: &mut RenderingCore,
        configs: HystImageCreationOption,
    ) -> std::io::Result<Self> {
        let img_bytes = std::fs::read(&configs.source)?;
        let img = image::load_from_memory(&img_bytes).unwrap();
        let dimensions = img.dimensions();
        let rgba = img.into_rgba8();
        Ok(Self::new(
            core,
            core.size(),
            configs.rect,
            dimensions,
            &rgba,
        ))
    }
    pub fn new(
        core: &mut RenderingCore,
        core_size: (u32, u32),
        rect: Rect,
        img_size: (u32, u32),
        data: &[u8],
    ) -> Self {
        let vertices = AbstractBuffer::new(
            core,
            [
                ImageInput::new(-1.0, 1.0, 0.0, 0.0),
                ImageInput::new(1.0, 1.0, 1.0, 0.0),
                ImageInput::new(-1.0, -1.0, 0.0, 1.0),
                ImageInput::new(1.0, -1.0, 1.0, 1.0),
            ],
            BufferType::Vertex,
        );
        let indices = core.create_index_buffer(&[0, 1, 2, 2, 1, 3], None);
        let screen_size = AbstractBuffer::new(
            core,
            [core_size.0 as f32, core_size.1 as f32],
            BufferType::Uniform,
        );
        let area = AbstractBuffer::new(core, rect, BufferType::Uniform);
        let image = core.create_image(img_size, data);
        let shader = core.create_shader(crate::shaders::ShaderCreationOptions {
            source: &std::fs::read_to_string("./shaders/image.wgsl").unwrap(),
            bind_group_configs: vec![
                vec![
                    BindGroupAndLayoutConfig::Uniform(
                        ShaderStages::VERTEX,
                        screen_size.inner_buffer(),
                    ),
                    BindGroupAndLayoutConfig::Uniform(ShaderStages::VERTEX, area.inner_buffer()),
                ],
                vec![
                    BindGroupAndLayoutConfig::Texutre(
                        wgpu::TextureViewDimension::D2,
                        wgpu::TextureSampleType::Float { filterable: true },
                        image.view(),
                    ),
                    BindGroupAndLayoutConfig::Sampler(
                        wgpu::SamplerBindingType::Filtering,
                        image.sampler(),
                    ),
                ],
            ],
            rendering_style: ShaderRenderMethod::TriangleCcwBack,
            name: "img".into(),
        });
        Self {
            image,
            indices_len: 6,
            indices,
            vertices,
            screen_size,
            area,
            shader,
        }
    }
}

impl Mesh for Image {
    fn screen_size(&mut self) -> &mut super::container::AbstractBuffer<[f32; 2]> {
        &mut self.screen_size
    }
    fn area_buffer(&mut self) -> &mut AbstractBuffer<crate::rectangle::Rect> {
        &mut self.area
    }
    fn draw(&self, pass: &mut wgpu::RenderPass) {
        pass.set_pipeline(&self.shader.pipeline);
        for (idx, bindgroups) in self.shader.bind_groups.iter().enumerate() {
            pass.set_bind_group(idx as u32, bindgroups, &[]);
        }
        pass.set_index_buffer(self.indices.slice(..), wgpu::IndexFormat::Uint16);
        pass.set_vertex_buffer(0, self.vertices.inner_buffer().slice(..));
        pass.draw_indexed(0..self.indices_len, 0, 0..1);
    }
}
