mod element_manager;
mod options;
pub mod pulse;
pub mod renderer;
use std::{
    any::Any,
    collections::VecDeque,
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

use super::elements::{HystBox, HystElement, HystImage};

slotmap::new_key_type! {pub struct HystElementKey;}

///Used for passing elements for the renderer. The idea is to transform a Tree like the following:
/// Img {
///   B1 = Box {}
///   B2 = Box {}
/// }
/// R1 = Box {}
/// R2 = Box {
///   B3 = Box {}
/// }
/// into the following Vec<DrawRequestedElements<'a>>:
/// [
///   [Img],
///   [R1, R2],
///   [B1, B2, B3]
/// ]
/// So only 3 draw calls are made
#[derive(Debug)]
pub enum DrawRequestedElements {
    Img(Vec<u64>), //u64 is the inner indice of the instance on the gpu
    Box(Vec<u64>),
}

pub struct HystUi {
    core: RenderingCore,
    element_manager: ElementManager,
    bg: Vec4f32,
    rx: Receiver<HystElementKey>,
    tx: Sender<HystElementKey>,
}

///Struct that manages a high level creation and selection of elements as well as their needs such as Pulses.
impl HystUi {
    pub fn new(mut core: RenderingCore, bg: Vec4f32) -> Self {
        let (tx, rx) = channel();
        Self {
            element_manager: ElementManager::new(&mut core),
            core,
            bg,
            rx,
            tx,
        }
    }

    #[inline]
    pub fn core_mut(&mut self) -> &mut RenderingCore {
        &mut self.core
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
        Ok(self.element_manager.insert_box(style, options.bg, rect))
    }
    pub fn create_image(
        &mut self,
        options: HystImageOptions,
    ) -> Result<HystElementKey, LayoutError> {
        let style = self.generate_layout(None, options.style)?;
        let rect = self.get_rect(style)?;
        Ok(self
            .element_manager
            .insert_image(&self.core, rect, options.source, style))
    }

    #[inline]
    pub fn resize_roots(&mut self, width: f32, height: f32) {
        self.element_manager
            .resize_roots(&self.core, (width, height));
    }

    fn prepare_texts(&mut self) {
        let texts = self
            .text_elements()
            .iter()
            .map(|text| {
                let buffer = text.inner().buffer();
                let x = text.inner().x();
                let y = text.inner().y();
                (buffer.clone(), Vec2f32::new(x, y), text.color().cloned())
            })
            .collect::<Vec<_>>();
        self.core.prepare_texts(texts);
    }

    ///Transforms the tree into an array of vectors to pass to the batch renderer which elements to draw and the order to draw them.
    ///
    ///Observations: The actual implementation is a Breadth first search, algorithm suggested by @ry-diffusion but the implementation was written by @FelipePn10
    pub fn traverse_elements(&self) -> Vec<DrawRequestedElements> {
        let mut out = Vec::new();
        let mut stack = self
            .roots_keys()
            .into_iter()
            .map(|key| (0u16, *key)) //(u16, HystElementKey)
            .collect::<Vec<_>>();
        let mut current_depth = 0;
        let mut next_level = Vec::new();

        while !stack.is_empty() {
            let mut img = Vec::new();
            let mut vbox = Vec::new(); //vec box

            for (depth, key) in stack.drain(..) {
                if depth > current_depth {
                    next_level.push((depth, key));
                    continue;
                }

                if let Some(Some(image)) = self.get_element_with_type::<HystImage>(key) {
                    img.push(image.instance_index());
                } else if let Some(Some(bx)) = self.get_element_with_type::<HystBox>(key) {
                    vbox.push(bx.instance_index());
                }

                for child in self.get_children_of(key) {
                    next_level.push((depth + 1, child.id()));
                }
            }

            if !img.is_empty() {
                out.push(DrawRequestedElements::Img(img));
            }
            if !vbox.is_empty() {
                out.push(DrawRequestedElements::Box(vbox));
            }
            stack.extend(next_level.drain(..));
            current_depth += 1;
            // chico viadinho
        }

        out
    }

    pub fn draw(&mut self) {
        self.prepare_texts();
        self.core.draw(self.traverse_elements(), self, self.bg);
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
        self.flush();
        flag
    }

    ///Flushes all the modifications made on the library.
    ///
    ///Obs: Under the hood there is no queue or something like that, actually, it only writes the data on the AbstractBuffers of the renderers to the buffers, so this is expected to use
    ///when something changes. Even though if nothing changes, calling this will trigger the rewrite, so it is not meant to be executed everytime, only when it's sure that a modification was made.
    pub fn flush(&mut self) {
        self.flush_modifications(&self.core);
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
