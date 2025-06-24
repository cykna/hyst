use wgpu::{Sampler, Texture, TextureView};

pub struct GpuImage {
    view: TextureView,
    sampler: Sampler,
    texture: Texture,
}

impl GpuImage {
    pub fn new(texture: Texture, sampler: Sampler, view: TextureView) -> Self {
        Self {
            view,
            texture,
            sampler,
        }
    }
    pub fn view(&self) -> &TextureView {
        &self.view
    }
    pub fn sampler(&self) -> &Sampler {
        &self.sampler
    }
}
