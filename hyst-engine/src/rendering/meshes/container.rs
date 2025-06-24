use bytemuck::{Pod, Zeroable};
use wgpu::ShaderStages;

use crate::{
    background::Background,
    core::{BindGroupAndLayoutConfig, RenderingCore},
    mesh::Mesh,
    rectangle::Rect,
    shaders::{HystConstructor, HystShader, ShaderInput, ShaderRenderMethod},
    vec2::Vec2f32,
    vec4::Vec4f32,
};

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

pub struct ContainerShader {
    module: wgpu::ShaderModule,
    bindgroups: Vec<wgpu::BindGroup>,
    layouts: Vec<wgpu::BindGroupLayout>,
    pipeline: std::sync::Arc<wgpu::RenderPipeline>,
}

impl HystConstructor for ContainerShader {
    fn new(
        module: wgpu::ShaderModule,
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
        vec![ContainerInput::LAYOUT]
    }
}

impl HystShader for ContainerShader {
    fn module(&self) -> &wgpu::ShaderModule {
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

pub enum BufferType {
    Vertex,
    Uniform,
}

pub struct AbstractBuffer<T> {
    inner: T,
    buffer: wgpu::Buffer,
}

impl<T> AbstractBuffer<T>
where
    T: Pod + Zeroable,
{
    pub fn new(core: &RenderingCore, data: T, buffer_type: BufferType) -> Self {
        Self {
            buffer: match buffer_type {
                BufferType::Vertex => core.create_vertex_buffer(&[data], None),
                BufferType::Uniform => core.create_uniform_buffer(&[data], None),
            },
            inner: data,
        }
    }
    ///Writes the modifications made in the inner value into the buffer
    pub fn write(&mut self, core: &RenderingCore) {
        core.write_buffer_single(&self.inner, &self.buffer);
    }
    ///Writes the given data into this buffer and modifies the inner value, returning the old value
    pub fn write_with(&mut self, core: &RenderingCore, data: T) -> T {
        let out = std::mem::replace(&mut self.inner, data);
        self.write(core);
        out
    }
    pub fn inner_buffer(&self) -> &wgpu::Buffer {
        &self.buffer
    }
    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.inner
    }
    pub fn inner(&self) -> &T {
        &self.inner
    }
}

impl<T, const AMOUNT: usize> std::ops::IndexMut<usize> for AbstractBuffer<[T; AMOUNT]>
where
    T: Pod + Zeroable,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.inner[index]
    }
}

impl<T, const AMOUNT: usize> std::ops::Index<usize> for AbstractBuffer<[T; AMOUNT]>
where
    T: Pod + Zeroable,
{
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

pub struct Container {
    shader: ContainerShader,
    vertices: AbstractBuffer<[ContainerInput; 4]>,
    index: wgpu::Buffer,
    indices_len: u32,
    rect_buf: AbstractBuffer<Rect>,
    screen_size: AbstractBuffer<[f32; 2]>,
}

impl Container {
    pub fn new(core: &mut RenderingCore, bg: Background, rect: Rect) -> Self {
        let size = core.size();
        let rect_buf = AbstractBuffer::new(core, rect, BufferType::Uniform);
        let screen_size =
            AbstractBuffer::new(core, [size.0 as f32, size.1 as f32], BufferType::Uniform);
        let vertices = AbstractBuffer::new(
            core,
            match bg {
                Background::Transparent => [
                    ContainerInput::transparent(-1.0, 1.0),
                    ContainerInput::transparent(1.0, 1.0),
                    ContainerInput::transparent(-1.0, -1.0),
                    ContainerInput::transparent(1.0, -1.0),
                ],
                Background::Solid(rgba) => [
                    ContainerInput::solid(-1.0, 1.0, rgba),
                    ContainerInput::solid(1.0, 1.0, rgba),
                    ContainerInput::solid(-1.0, -1.0, rgba),
                    ContainerInput::solid(1.0, -1.0, rgba),
                ],
                Background::Gradient {
                    top_left,
                    top_right,
                    bottom_left,
                    bottom_right,
                } => [
                    ContainerInput::solid(-1.0, 1.0, top_left),
                    ContainerInput::solid(1.0, 1.0, top_right),
                    ContainerInput::solid(-1.0, -1.0, bottom_left),
                    ContainerInput::solid(1.0, -1.0, bottom_right),
                ],
            },
            BufferType::Vertex,
        );
        Self {
            indices_len: 6,
            vertices,
            index: core.create_index_buffer(&[0, 1, 2, 2, 1, 3], None),
            shader: core.create_shader(crate::shaders::ShaderCreationOptions {
                source: &std::fs::read_to_string("./shaders/container.wgsl").unwrap(),
                bind_group_configs: vec![vec![
                    BindGroupAndLayoutConfig::Uniform(
                        ShaderStages::VERTEX,
                        screen_size.inner_buffer(),
                    ),
                    BindGroupAndLayoutConfig::Uniform(
                        ShaderStages::VERTEX,
                        rect_buf.inner_buffer(),
                    ),
                ]],
                rendering_style: ShaderRenderMethod::TriangleCcwBack,
                name: "container".to_string(),
            }),
            screen_size,
            rect_buf,
        }
    }
}

impl Mesh for Container {
    fn area_buffer(&mut self) -> &mut AbstractBuffer<Rect> {
        &mut self.rect_buf
    }

    fn screen_size(&mut self) -> &mut AbstractBuffer<[f32; 2]> {
        &mut self.screen_size
    }
    fn draw(&self, pass: &mut wgpu::RenderPass) {
        pass.set_pipeline(self.shader.pipeline());
        {
            let mut idx = 0;
            for bind_group in self.shader.bind_groups() {
                pass.set_bind_group(idx, bind_group, &[]);
                idx += 1;
            }
        }
        pass.set_index_buffer(self.index.slice(..), wgpu::IndexFormat::Uint16);
        pass.set_vertex_buffer(0, self.vertices.inner_buffer().slice(..));
        pass.draw_indexed(0..self.indices_len, 0, 0..1);
    }
}
