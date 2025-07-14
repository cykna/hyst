use bytemuck::{Pod, Zeroable};
use taffy::Layout;
use wgpu::RenderPass;

use crate::{
    batch::{BatchRenderer, BatchSubmitter},
    core::RenderingCore,
    shaders::{HystShader, ShaderInput},
};
#[derive(Debug, Clone, Copy)]
pub enum SizeMethod {
    Physical,
    Percentage(f32, f32),
}

pub trait Mesh: Sized {
    type Shader: HystShader;
    type Vertices: ShaderInput;
    type Instance: Pod + Zeroable;

    fn resize(&mut self, core: &RenderingCore, renderer: &mut dyn BatchSubmitter, layout: &Layout);
}
