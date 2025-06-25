mod options;
pub use options::*;
use slotmap::SlotMap;
pub use smol_str;
pub use taffy;
use taffy::{Layout, NodeId};
use wgpu::RenderPass;

use crate::{
    HystLayout,
    core::RenderingCore,
    elements::{HystBox, HystBoxCreationOption, HystElementImageCreationOption, HystImage},
    error::LayoutError,
    meshes::Mesh,
};
use hyst_math::{Rect, vectors::Vec4f32};

#[derive(Debug)]
pub enum HystElement {
    Box(HystBox),
    Image(HystImage),
}

impl HystElement {
    pub fn draw(&self, pass: &mut RenderPass) {
        match self {
            Self::Box(bx) => bx.container().draw(pass),
            Self::Image(img) => img.draw(pass),
        }
    }
    pub fn resize(&mut self, core: &RenderingCore, screen_size: (f32, f32), layout: &Layout) {
        match self {
            Self::Box(bx) => bx.container_mut().resize(core, screen_size, layout),
            Self::Image(img) => img.resize(core, screen_size, layout),
        }
    }

    pub fn style(&self) -> taffy::NodeId {
        match self {
            Self::Box(bx) => bx.style(),
            Self::Image(img) => img.style(),
        }
    }

    pub fn parent(&self) -> Option<&HystElementKey> {
        match self {
            Self::Box(bx) => bx.parent(),
            Self::Image(img) => img.parent(),
        }
    }

    pub fn children(&self) -> &[HystElementKey] {
        match self {
            Self::Box(bx) => bx.children(),
            Self::Image(img) => img.children(),
        }
    }
}

slotmap::new_key_type! {pub struct HystElementKey;}

pub struct HystUi {
    core: RenderingCore,
    elements: SlotMap<HystElementKey, HystElement>,
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
            HystElement::Box(HystBox::new(
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
            HystElement::Image(HystImage::new(
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

    pub fn elements(&self) -> &SlotMap<HystElementKey, HystElement> {
        &self.elements
    }

    pub fn elements_mut(&mut self) -> &mut SlotMap<HystElementKey, HystElement> {
        &mut self.elements
    }

    pub fn roots(&self) -> Vec<&HystElement> {
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

    pub fn get_children_of(&self, key: HystElementKey) -> Vec<&HystElement> {
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
            let layout = self.layout.layout_of(parent.style()).unwrap();
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
