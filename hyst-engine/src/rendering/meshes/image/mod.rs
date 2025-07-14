use super::Mesh;
use crate::batch::{BatchRenderer, BatchSubmitter};
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

pub struct ImageCreationOption {
    pub rect: Rect,
    pub source: String,
}

#[derive(Debug)]
pub struct Image {
    image_instance: Rect,
    image: GpuImage,
}

impl Image {
    pub fn from_configs(
        core: &RenderingCore,
        configs: ImageCreationOption,
    ) -> std::io::Result<Self> {
        let img_bytes = std::fs::read(&configs.source)?;
        let img = image::load_from_memory(&img_bytes).unwrap();
        let dimensions = img.dimensions();
        let rgba = img.into_rgba8();
        Ok(Self::new(core, dimensions, &rgba, configs.rect))
    }
    pub fn new(core: &RenderingCore, img_size: (u32, u32), data: &[u8], rect: Rect) -> Self {
        let image = core.create_image(img_size, data);
        Self {
            image,
            image_instance: rect,
        }
    }
}

impl Mesh for Image {
    type Shader = ImageShader;
    type Vertices = ImageInput;
    type Instance = Rect;
    fn resize(
        &mut self,
        core: &RenderingCore,
        renderer: &mut dyn BatchSubmitter,
        layout: &taffy::Layout,
    ) {
        let Size { width, height } = layout.size;
        self.image_instance.size_mut().set_coords(width, height);

        let Point { x, y } = layout.location;
        self.image_instance.position_mut().set_coords(x, y);

        renderer.submit(core, bytemuck::bytes_of(&self.image_instance), 0);
    }
}
