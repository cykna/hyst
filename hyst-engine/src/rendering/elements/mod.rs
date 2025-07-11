//hbox due to errors with 'box' key
mod hbox;
pub use hbox::*;

mod image;
pub use image::*;
use taffy::{Layout, NodeId};

use super::{core::RenderingCore, ui::HystElementKey};

pub trait HystElement {
    ///Retrieves the Id for the layout of this element. Used for positioning and stuff like that
    fn layout(&self) -> NodeId;

    fn resize(&mut self, core: &RenderingCore, screen_size: (f32, f32), layout: &Layout);
    fn children(&self) -> &Vec<HystElementKey>;

    ///Used for when the ui requests this Element to update. Normally due to a Pulse dependency update.
    fn update(&mut self, core: &RenderingCore);
}
