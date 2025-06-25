use super::Mesh;
use crate::core::RenderingCore;
use crate::shaders::ShaderRenderMethod;
use crate::{AbstractBuffer, BindGroupAndLayoutConfig, BufferType, GpuImage, shaders::HystShader};
use hyst_math::Rect;
use image::GenericImageView;

mod input;
pub use input::*;
mod shader;
pub use shader::*;
use taffy::{Point, Size};

pub struct HystImageCreationOption {
    pub rect: Rect,
    pub source: String,
}

#[derive(Debug)]
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
                        wgpu::ShaderStages::VERTEX,
                        screen_size.inner_buffer(),
                    ),
                    BindGroupAndLayoutConfig::Uniform(
                        wgpu::ShaderStages::VERTEX,
                        area.inner_buffer(),
                    ),
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
    fn screen_size(&mut self) -> &mut AbstractBuffer<[f32; 2]> {
        &mut self.screen_size
    }
    fn area_buffer(&mut self) -> &mut AbstractBuffer<Rect> {
        &mut self.area
    }
    fn draw(&self, pass: &mut wgpu::RenderPass) {
        pass.set_pipeline(&self.shader.pipeline());
        for (idx, bindgroups) in self.shader.bind_groups().iter().enumerate() {
            pass.set_bind_group(idx as u32, bindgroups, &[]);
        }
        pass.set_index_buffer(self.indices.slice(..), wgpu::IndexFormat::Uint16);
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
