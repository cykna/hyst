mod element_manager;
mod options;
pub mod pulse;
use std::{
    ops::{Deref, DerefMut},
    sync::mpsc::{Receiver, Sender, channel},
};

use element_manager::ElementManager;
pub use options::*;
use pulse::Pulse;

use crate::{core::RenderingCore, error::LayoutError};
use hyst_math::vectors::{Vec2f32, Vec4f32};
pub use smol_str;
pub use taffy;

use super::elements::HystText;

slotmap::new_key_type! {pub struct HystElementKey;}

pub struct HystUi {
    core: RenderingCore,
    element_manager: ElementManager,
    bg: Vec4f32,
    rx: Receiver<HystElementKey>,
    tx: Sender<HystElementKey>,
}

///Struct that manages the creation and modification of elements. Until now the modification can only be done here
impl HystUi {
    pub fn new(core: RenderingCore, bg: Vec4f32) -> Self {
        let (tx, rx) = channel();
        Self {
            element_manager: ElementManager::new(),
            core,
            bg,
            rx,
            tx,
        }
    }

    pub fn create_pulse<T>(&self, value: T) -> Pulse<T> {
        Pulse::new(value, self.tx.clone())
    }

    pub fn create_text(&mut self, options: HystTextOptions) -> Result<HystElementKey, LayoutError> {
        self.element_manager.insert_text(&mut self.core, options)
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
        self.element_manager
            .resize_roots(&mut self.core, width, height);
    }

    fn prepare_texts(&mut self) {
        let texts = self
            .text_elements()
            .iter()
            .map(|text| {
                let buffer = text.inner().buffer();
                let x = text.inner().x();
                let y = text.inner().y();
                (buffer.clone(), Vec2f32::new(x, y), text.color())
            })
            .collect::<Vec<_>>();
        self.core.prepare_texts(texts);
    }

    pub fn draw(&mut self) {
        let mut children = Vec::new();
        self.prepare_texts();
        for root in self.roots_keys().iter() {
            let Some(parent) = self.elements().get(*root) else {
                continue;
            };
            children.push(parent);
            children.append(&mut self.get_children_of(*root));
        }
        self.core.draw(&children, self.bg);
    }

    ///Checks if there are some pending element keys that require updating, if so, updates the elements that require.
    /// # Returns
    /// * Wheather some element was updated and a draw request is required
    pub fn check_for_updates(&mut self) -> bool {
        let mut flag = false;
        while let Ok(key) = self.rx.try_recv() {
            if let Some(element) = self.element_manager.get_element_mut(key) {
                flag = true;
                element.update(&mut self.core);
            }
        }
        flag
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
