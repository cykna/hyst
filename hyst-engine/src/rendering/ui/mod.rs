mod element_manager;
mod options;
mod pulse;
use std::ops::{Deref, DerefMut};

use element_manager::ElementManager;
pub use options::*;
use slotmap::SlotMap;
pub use smol_str;
pub use taffy;
use taffy::NodeId;

use crate::{core::RenderingCore, error::LayoutError};
use hyst_math::{Rect, vectors::Vec4f32};

slotmap::new_key_type! {pub struct HystElementKey;}

pub struct HystUi {
    core: RenderingCore,
    element_manager: ElementManager,
    bg: Vec4f32,
}

///Struct that manages the creation and modification of elements. Until now the modification can only be done here
impl HystUi {
    pub fn new(core: RenderingCore, bg: Vec4f32) -> Self {
        Self {
            element_manager: ElementManager::new(),
            core,
            bg,
        }
    }
    pub fn create_box(&mut self, options: HystBoxOptions) -> Result<HystElementKey, LayoutError> {
        let style = self.generate_layout(None, options.style)?;
        let rect = self.get_rect(style)?;
        Ok(self
            .element_manager
            .insert_box(style, options.bg, rect, &mut self.core))
    }
    pub fn create_image(
        &mut self,
        options: HystImageOptions,
    ) -> Result<HystElementKey, LayoutError> {
        let style = self.generate_layout(None, options.style)?;
        let rect = self.get_rect(style)?;
        Ok(self
            .element_manager
            .insert_image(&mut self.core, rect, options.source, style))
    }

    pub fn core(&self) -> &RenderingCore {
        &self.core
    }

    pub fn core_mut(&mut self) -> &mut RenderingCore {
        &mut self.core
    }
    #[inline]
    pub fn resize_roots(&mut self, width: f32, height: f32) {
        self.element_manager.resize_roots(&self.core, width, height);
    }

    pub fn draw(&self) {
        let mut children = Vec::new();
        for root in self.roots_keys().iter() {
            let Some(parent) = self.elements().get(*root) else {
                continue;
            };
            children.push(parent);
            children.append(&mut self.get_children_of(*root));
        }
        self.core.draw(&children, self.bg);
    }
}

impl Deref for HystUi {
    type Target = ElementManager;
    fn deref(&self) -> &Self::Target {
        &self.element_manager
    }
}
impl DerefMut for HystUi {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.element_manager
    }
}
