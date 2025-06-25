use taffy::Layout;
use wgpu::RenderPass;

use crate::{core::RenderingCore, rendering::helpers::AbstractBuffer};
use hyst_math::Rect;

#[derive(Debug, Clone, Copy)]
pub enum SizeMethod {
    Physical,
    Percentage(f32, f32),
}

pub trait Mesh {
    fn draw(&self, pass: &mut RenderPass);
    ///Gets the screen_size uniform buffer. It's used for modifying the values of it when the window is resized
    fn screen_size(&mut self) -> &mut AbstractBuffer<[f32; 2]>;
    fn area_buffer(&mut self) -> &mut AbstractBuffer<Rect>;
    fn resize(&mut self, core: &RenderingCore, screen_size: (f32, f32), layout: &Layout);
}
