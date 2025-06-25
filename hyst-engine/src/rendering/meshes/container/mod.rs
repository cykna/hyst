mod input;
pub use input::*;
use taffy::{Point, Size};
mod shader;
use crate::{
    AbstractBuffer, BindGroupAndLayoutConfig, BufferType,
    background::Background,
    core::RenderingCore,
    meshes::Mesh,
    shaders::{HystShader, ShaderRenderMethod},
};
use hyst_math::Rect;
pub use shader::*;

#[derive(Debug)]
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
                        wgpu::ShaderStages::VERTEX,
                        screen_size.inner_buffer(),
                    ),
                    BindGroupAndLayoutConfig::Uniform(
                        wgpu::ShaderStages::VERTEX,
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
    fn resize(&mut self, core: &RenderingCore, screen_size: (f32, f32), layout: &taffy::Layout) {
        self.screen_size
            .write_with(core, [screen_size.0, screen_size.1]);

        let rect_buf = self.area_buffer();
        let rect_mut = rect_buf.inner_mut();
        let Size { width, height } = layout.size;
        rect_mut.size_mut().set_coords(width, height);

        let Point { x, y } = layout.location;
        rect_mut.position_mut().set_coords(x, y);
        rect_buf.write(core);
    }
}
