use wgpu::{RenderPass, ShaderStages, naga::ShaderStage};

use super::{
    AbstractBuffer, AbstractVecBuffer, BindGroupAndLayoutConfig, GpuImage,
    core::RenderingCore,
    meshes::{Mesh, image::Image},
    shaders::{HystConstructor, HystShader, ShaderCreationOptions, ShaderRenderMethod},
};

pub struct BatchRenderer<M: Mesh>
where
    M::Shader: HystConstructor,
{
    vertices: wgpu::Buffer,
    indices: wgpu::Buffer,
    shader: M::Shader,
    instances: AbstractVecBuffer<M::Instance>,
    window_size: AbstractBuffer<[f32; 2]>,
}

impl<M> BatchRenderer<M>
where
    M: Mesh,
    <M as Mesh>::Shader: HystConstructor,
{
    ///Creates a new Batch renderer. By default, if no instance is given, it allocates size enough for 128 of them.
    pub fn new(
        core: &mut RenderingCore,
        vertices: &[M::Vertices],
        source: &str,
        name: &str,
        indices: &[u16],
        instances: Vec<M::Instance>,
        window_size: [f32; 2],
    ) -> Self {
        let window_size = AbstractBuffer::new(core, window_size, super::BufferType::Uniform);
        Self {
            vertices: core.create_vertex_buffer(vertices, Some("batch renderer")),
            indices: core.create_index_buffer(indices, Some("batch renderer")),
            shader: core.create_shader(super::shaders::ShaderCreationOptions {
                source,
                bind_group_configs: vec![vec![BindGroupAndLayoutConfig::Uniform(
                    ShaderStages::VERTEX,
                    window_size.inner_buffer(),
                )]],
                rendering_style: ShaderRenderMethod::TriangleCcwBack,
                name: name.to_string(),
            }),
            instances: if instances.is_empty() {
                AbstractVecBuffer::empty(core, 128)
            } else {
                AbstractVecBuffer::new_vec(core, instances, super::BufferType::Vertex)
            },
            window_size,
        }
    }

    pub fn render(&self, rpass: &mut RenderPass) {
        rpass.set_pipeline(self.shader.pipeline());
        {
            let mut idx = 0;
            for bg in self.shader.bind_groups() {
                rpass.set_bind_group(idx, bg, &[]);
                idx += 1;
            }
        }
        rpass.set_index_buffer(self.indices.slice(..), wgpu::IndexFormat::Uint16);
        rpass.set_vertex_buffer(0, self.vertices.slice(..));
        rpass.set_vertex_buffer(1, self.instances.inner_buffer().slice(..));
        rpass.draw_indexed(
            0..(self.indices.size() as u32 >> 1),
            0,
            0..self.instances.inner().len() as u32,
        );
    }

    #[inline]
    ///Sets the window size of every instance to be the given one
    pub fn set_window_size(&mut self, core: &RenderingCore, size: [f32; 2]) {
        self.window_size.write_with(core, size);
    }
}

impl BatchRenderer<Image> {
    pub fn new_image(
        core: &mut RenderingCore,
        vertices: &[<Image as Mesh>::Vertices],
        source: &str,
        name: &str,
        indices: &[u16],
        instances: Vec<<Image as Mesh>::Instance>,
        window_size: [f32; 2],
        img: &GpuImage,
    ) -> Self {
        let window_size = AbstractBuffer::new(core, window_size, super::BufferType::Uniform);
        Self {
            vertices: core.create_vertex_buffer(vertices, Some("batch renderer")),
            indices: core.create_index_buffer(indices, Some("batch renderer")),
            shader: core.create_shader(ShaderCreationOptions {
                source,
                bind_group_configs: vec![
                    vec![BindGroupAndLayoutConfig::Uniform(
                        ShaderStages::VERTEX,
                        window_size.inner_buffer(),
                    )],
                    vec![
                        BindGroupAndLayoutConfig::Texutre(
                            wgpu::TextureViewDimension::D2,
                            wgpu::TextureSampleType::Depth,
                            img.view(),
                        ),
                        BindGroupAndLayoutConfig::Sampler(
                            wgpu::SamplerBindingType::Filtering,
                            img.sampler(),
                        ),
                    ],
                ],
                rendering_style: ShaderRenderMethod::TriangleCcwBack,
                name: name.to_string(),
            }),
            window_size,
            instances: if instances.is_empty() {
                AbstractVecBuffer::empty(core, 128)
            } else {
                AbstractVecBuffer::new_vec(core, instances, super::BufferType::Vertex)
            },
        }
    }
}

pub trait BatchSubmitter {
    fn submit(&mut self, core: &RenderingCore, instance: &[u8], index: u64);
}

impl<M> BatchSubmitter for BatchRenderer<M>
where
    M: Mesh,
    <M as Mesh>::Shader: HystConstructor,
{
    fn submit(&mut self, core: &RenderingCore, instance: &[u8], index: u64) {
        core.write_buffer_raw(instance, self.instances.inner_buffer(), Some(index));
    }
}
