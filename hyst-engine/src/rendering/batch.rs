use taffy::Layout;
use wgpu::{
    RenderPass, ShaderStages,
    wgt::{DrawIndexedIndirectArgs, DrawIndirectArgs},
};

use super::{
    AbstractBuffer, AbstractVecBuffer, BindGroupAndLayoutConfig, GpuImage,
    core::RenderingCore,
    meshes::{Mesh, Updatable, image::Image},
    shaders::{HystConstructor, HystShader, ShaderCreationOptions, ShaderRenderMethod},
};

///A Renderer which renderes the given Mesh `M` using the least amount needed of draw calls
pub struct BatchRenderer<M: Mesh>
where
    M::Shader: HystConstructor,
{
    vertices: wgpu::Buffer,
    indices: wgpu::Buffer,
    shader: M::Shader,
    indirect_buffer: wgpu::Buffer,
    instances: AbstractVecBuffer<M::Instance>,
    window_size: AbstractBuffer<[f32; 2]>,
    len: u64,
    cap: u64,
    indices_count: u16,
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
        let cap = if instances.is_empty() {
            128
        } else {
            instances.len() as u64
        };
        let window_size = AbstractBuffer::new(core, window_size, super::BufferType::Uniform);
        Self {
            indices_count: indices.len() as u16,
            indirect_buffer: core
                .create_indexed_indirect_buffer(cap as usize, Some("batch renderer")),
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

            len: instances.len() as u64,
            cap,
            instances: if instances.is_empty() {
                AbstractVecBuffer::empty(core, 128)
            } else {
                AbstractVecBuffer::new_vec(core, instances, super::BufferType::Vertex)
            },
            window_size,
        }
    }

    #[inline]
    ///Flushes all the modifications made on the instances buffer
    pub fn flush(&self, core: &RenderingCore) {
        self.instances.write_vec(core);
    }

    ///Updates the instance on the given `index` with the given `layout`.
    pub fn resize(&mut self, index: u64, layout: &Layout) {
        (&mut self.instances.inner_mut()[index as usize]).update(layout);
    }

    ///Appends the given instance at the end ot the instance buffer, and returns the old length of the buffer.
    pub fn push(&mut self, instance: M::Instance) -> u64 {
        let len = self.len;
        self.instances.inner_mut().push(instance);
        self.len += 1;
        len
    }
    ///Removes the last element on the instances and returns the new length.
    ///If the length is 0 and this is executed, nothing happens
    #[inline]
    pub fn pop(&mut self) -> u64 {
        if self.len == 0 {
            return 0;
        }
        self.len -= 1;
        self.len
    }

    ///Removes the element on the given index and moves the last element to its position, thus, it is required to update the element id after this operation.
    pub fn remove(&mut self, idx: u64) -> Option<M::Instance> {
        if self.len == 0 {
            return None;
        }
        let out = self.instances.inner_mut().swap_remove(idx as usize);
        self.len -= 1;
        Some(out)
    }

    pub fn prepare_for(&self, indices: &Vec<u64>, core: &RenderingCore) {
        let mut out = Vec::new();
        for index in indices.iter().cloned() {
            out.push(DrawIndexedIndirectArgs {
                index_count: self.indices_count as u32,
                instance_count: 1,
                first_index: 0,
                first_instance: index as u32,
                base_vertex: 0,
            });
        }
        core.write_buffer(&out, &self.indirect_buffer);
    }

    pub fn render(&self, instance_indices: Vec<u64>, rpass: &mut RenderPass) {
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
        rpass.multi_draw_indexed_indirect(&self.indirect_buffer, 0, instance_indices.len() as u32);
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
        let cap = if instances.is_empty() {
            128
        } else {
            instances.len() as u64
        };
        Self {
            indices_count: indices.len() as u16,
            indirect_buffer: core
                .create_indexed_indirect_buffer(cap as usize, Some("batch renderer")),
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
            len: instances.len() as u64,
            cap,
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
