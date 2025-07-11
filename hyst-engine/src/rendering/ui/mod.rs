mod options;
mod pulse;
pub use options::*;
use slotmap::SlotMap;
pub use smol_str;
pub use taffy;
use taffy::NodeId;

use crate::{
    HystLayout,
    core::RenderingCore,
    elements::{HystBox, HystBoxCreationOption, HystElementImageCreationOption, HystImage},
    error::LayoutError,
};
use hyst_math::{Rect, vectors::Vec4f32};

use super::elements::HystElement;

slotmap::new_key_type! {pub struct HystElementKey;}

pub struct HystUi {
    core: RenderingCore,
    elements: SlotMap<HystElementKey, Box<dyn HystElement>>,
    roots: Vec<HystElementKey>,
    layout: HystLayout,
    bg: Vec4f32,
}

///Struct that manages the creation and modification of elements. Until now the modification can only be done here
impl HystUi {
    pub fn new(core: RenderingCore, bg: Vec4f32) -> Self {
        Self {
            layout: HystLayout::new(),
            core,
            elements: SlotMap::with_key(),
            roots: Vec::new(),
            bg,
        }
    }
    pub fn create_style(&mut self, name: &str, style: taffy::Style) {
        self.layout.create_style(name.into(), style);
    }

    ///Recalculates the styles and gets the rect of the given element
    pub fn get_rect(&self, id: NodeId) -> Result<Rect, LayoutError> {
        let layout = self.layout.layout_of(id)?;
        Ok(Rect::from_xywh(
            layout.location.x,
            layout.location.y,
            layout.size.width,
            layout.size.height,
        ))
    }

    pub fn create_box(&mut self, options: HystBoxOptions) -> Result<(), LayoutError> {
        let style = self.layout.create_element_style(None, options.style)?;
        let rect = self.get_rect(style)?;
        self.elements.insert_with_key(|key| {
            self.roots.push(key);
            Box::new(HystBox::new(
                &mut self.core,
                HystBoxCreationOption {
                    background: options.bg,
                    rect,
                    parent: None,
                    style,
                    key,
                },
            ))
        });
        Ok(())
    }
    pub fn create_image(&mut self, options: HystImageOptions) -> Result<(), LayoutError> {
        let style = self.layout.create_element_style(None, options.style)?;
        let rect = self.get_rect(style)?;
        self.elements.insert_with_key(|key| {
            self.roots.push(key);
            Box::new(HystImage::new(
                &mut self.core,
                HystElementImageCreationOption {
                    rect,
                    source: options.source,
                    style,
                    key,
                },
            ))
        });
        Ok(())
    }

    pub fn elements(&self) -> &SlotMap<HystElementKey, Box<dyn HystElement>> {
        &self.elements
    }

    pub fn elements_mut(&mut self) -> &mut SlotMap<HystElementKey, Box<dyn HystElement>> {
        &mut self.elements
    }

    pub fn roots(&self) -> Vec<&Box<dyn HystElement>> {
        self.roots
            .iter()
            .filter_map(|root| self.elements.get(*root))
            .collect()
    }
    pub fn core(&self) -> &RenderingCore {
        &self.core
    }

    pub fn core_mut(&mut self) -> &mut RenderingCore {
        &mut self.core
    }

    pub fn get_children_of(&self, key: HystElementKey) -> Vec<&Box<dyn HystElement>> {
        let mut out = Vec::new();
        if let Some(element) = self.elements.get(key) {
            for child_key in element.children() {
                if let Some(child) = self.elements.get(*child_key) {
                    out.push(child);
                    out.append(&mut self.get_children_of(*child_key));
                }
            }
        }
        out
    }

    pub fn draw(&self) {
        let mut children = Vec::new();
        for root in self.roots.iter() {
            let Some(parent) = self.elements.get(*root) else {
                continue;
            };
            children.push(parent);
            children.append(&mut self.get_children_of(*root));
        }
        self.core.draw(&children, self.bg);
    }
    #[inline]
    ///Recalculates the sizes of the taffy styles. Later i will provide customization for stuff other than
    ///only positioning
    fn recalc_styles(&mut self, width: f32, height: f32) {
        self.layout.recalculate(width, height).unwrap();
    }

    ///Resizes the root and its children recursively
    pub fn resize_root(&mut self, root: HystElementKey, width: f32, height: f32) {
        let children: Vec<_> = {
            let Some(parent) = self.elements.get_mut(root) else {
                return;
            };
            let layout = self.layout.layout_of(parent.layout()).unwrap();
            parent.resize(&self.core, (width, height), layout);
            parent.children().iter().cloned().collect()
        };
        for child in children {
            self.resize_root(child, width, height);
        }
    }
    ///Resizes all the elements starts by their roots
    /// # Arguments
    /// `width` The current width of the window
    /// `height` The current height of the window
    pub fn resize_roots(&mut self, width: f32, height: f32) {
        self.recalc_styles(width, height);
        let mut idx = 0;
        while let Some(root) = self.roots.get(idx) {
            idx += 1;
            self.resize_root(*root, width, height);
        }
    }
}
