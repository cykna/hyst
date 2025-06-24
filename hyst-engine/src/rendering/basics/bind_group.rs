use wgpu::{ShaderStages, Buffer, TextureView, TextureViewDimension, Sampler, TextureSampleType, SamplerBindingType};

///Used to define bind group and layout configs
pub enum BindGroupAndLayoutConfig<'a> {
    Uniform(ShaderStages, &'a Buffer),
    Texutre(TextureViewDimension, TextureSampleType, &'a TextureView),
    Sampler(SamplerBindingType, &'a Sampler),
}