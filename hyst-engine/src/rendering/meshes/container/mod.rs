mod input;
pub use input::*;
mod batch;
pub use batch::*;
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

// Estrutura para um elemento de container
#[derive(Debug)]
pub struct Container {
    vertices: AbstractBuffer<[ContainerInput; 4]>,
    rect_buf: AbstractBuffer<Rect>,
    screen_size: AbstractBuffer<[f32; 2]>,
    depth: i32, // Profundidade para ordenação
}

impl Container {
    pub fn new(core: &mut RenderingCore, bg: Background, rect: Rect, depth: i32) -> Self {
        let size = core.size();
        let rect_buf = AbstractBuffer::new(core, rect, BufferType::Uniform);
        let screen_size = AbstractBuffer::new(core, [size.0 as f32, size.1 as f32], BufferType::Uniform);
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
            vertices,
            rect_buf,
            screen_size,
            depth,
        }
    }

    pub fn depth(&self) -> i32 {
        self.depth
    }

    pub fn vertices_buffer(&self) -> &AbstractBuffer<[ContainerInput; 4]> {
        &self.vertices
    }

    pub fn rect_buffer(&self) -> &AbstractBuffer<Rect> {
        &self.rect_buf
    }

    pub fn screen_size_buffer(&self) -> &AbstractBuffer<[f32; 2]> {
        &self.screen_size
    }
}

impl Mesh for Container {
    fn area_buffer(&mut self) -> &mut AbstractBuffer<Rect> {
        &mut self.rect_buf
    }

    fn screen_size(&mut self) -> &mut AbstractBuffer<[f32; 2]> {
        &mut self.screen_size
    }

    fn resize(&mut self, core: &RenderingCore, screen_size: (f32, f32), layout: &taffy::Layout) {
        self.screen_size.write_with(core, [screen_size.0, screen_size.1]);

        let rect_buf = self.area_buffer();
        let rect_mut = rect_buf.inner_mut();
        let Size { width, height } = layout.size;
        rect_mut.size_mut().set_coords(width, height);

        let Point { x, y } = layout.location;
        rect_mut.position_mut().set_coords(x, y);
        rect_buf.write(core);
    }
}
