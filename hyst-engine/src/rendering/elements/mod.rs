//hbox due to errors with 'box' key
mod hbox;
use std::any::Any;

pub use hbox::*;

mod image;
pub use image::*;

mod text;
pub use text::*;

use taffy::{Layout, NodeId};
use wgpu::RenderPass;

use super::{core::RenderingCore, ui::HystElementKey};

pub trait HystElement: Any {
    fn id(&self) -> HystElementKey;

    ///Retrieves the Id for the layout of this element. Used for positioning and how the element will be displayed on the window
    fn layout(&self) -> NodeId;

    ///Event called when the window is resized.
    /// # Arguments
    /// * `screen_size` - The new size of the screen.
    /// * `layout` - The new layout computed for this element.
    fn resize(&mut self, core: &mut RenderingCore, screen_size: (f32, f32), layout: &Layout);
    fn children(&self) -> &Vec<HystElementKey>;

    ///Used for when the ui requests this Element to update. Normally due to a Pulse dependency update.
    fn update(&mut self, core: &mut RenderingCore);

    fn render(&self, pass: &mut RenderPass);
}
