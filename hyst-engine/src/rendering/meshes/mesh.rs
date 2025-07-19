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
pub trait Updatable<T>: std::fmt::Debug {
    fn update(&mut self, data: &T);
}

impl Updatable<taffy::Layout> for hyst_math::Rect {
    fn update(&mut self, data: &taffy::Layout) {
        self.position_mut()
            .set_coords(data.content_box_x(), data.content_box_y());
        self.position_mut()
            .set_coords(data.content_box_width(), data.content_box_height());
    }
}

pub trait Mesh: Sized {
    type Shader: HystShader;
    type Vertices: ShaderInput;
    type Instance: Pod + Zeroable + Updatable<Layout>;
}
