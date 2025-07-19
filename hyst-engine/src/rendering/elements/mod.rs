//hbox due to errors with 'box' key
mod hbox;
use std::{any::Any, fmt::Debug};

pub use hbox::*;

mod image;
pub use image::*;

mod text;
pub use text::*;

use taffy::NodeId;
use wgpu::RenderPass;

use super::{core::RenderingCore, ui::HystElementKey};

pub trait HystElement: Any + Debug {
    fn instance_index(&self) -> u64;
    fn id(&self) -> HystElementKey;

    ///Retrieves the Id for the layout of this element. Used for positioning and how the element will be displayed on the window
    fn layout(&self) -> NodeId;

    fn children(&self) -> &Vec<HystElementKey>;

    ///Used for when the ui requests this Element to update. Normally due to a Pulse dependency update.
    fn update(&mut self, core: &mut RenderingCore);

    fn render(&self, pass: &mut RenderPass);
}
