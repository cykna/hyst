use wgpu::RenderPass;

use crate::{meshes::container::AbstractBuffer, rectangle::Rect};

#[derive(Clone, Copy)]
pub enum SizeMethod {
    Physical,
    Percentage(f32, f32),
}

pub trait Mesh {
    fn draw(&self, pass: &mut RenderPass);
    ///Gets the screen_size uniform buffer. It's used for modifying the values of it when the window is resized
    fn screen_size(&mut self) -> &mut AbstractBuffer<[f32; 2]>;
    fn area_buffer(&mut self) -> &mut AbstractBuffer<Rect>;
}
