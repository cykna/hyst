use std::{
    any::Any,
    ops::{Deref, DerefMut},
};

use hyst_math::{Rect, vectors::Vec4f32};
use slotmap::SlotMap;
use smol_str::SmolStr;
use taffy::NodeId;

use crate::{
    HystLayout,
    background::Background,
    core::RenderingCore,
    elements::{
        HystBox, HystBoxCreationOption, HystElement, HystImage, HystImageCreationOption, HystText,
        TextCreationOption,
    },
    error::LayoutError,
    meshes::container::ContainerInstance,
};

use super::{HystElementKey, HystTextOptions, renderer::Renderer};

///Entry point for the managing how the ui is shown on the screen.
///Things related to pulses, and events, even if they do modify the ui, they're handled on the HystUi which is used to request some management
pub struct ElementManager {
    ///Rendered uses instanced drawing for making draw calls, so it does manipulate the data of the buffers on the gpu
    elements_renderer: Renderer,
    layout: HystLayout,
    elements: SlotMap<HystElementKey, Box<dyn HystElement>>,
    texts: Vec<HystElementKey>, // used for getting track of texts and using them for drawing.
    roots: Vec<HystElementKey>,
}

impl ElementManager {
    pub fn new(core: &mut RenderingCore) -> Self {
        Self {
            elements_renderer: Renderer::new(core),
            layout: HystLayout::new(),
            texts: Vec::new(),
            elements: SlotMap::with_key(),
            roots: Vec::new(),
        }
    }

    #[inline]
    ///Generates a new layout supposing the parent if the given `parent. If it's None, then the element is understood as a Root element
    pub fn generate_layout(
        &mut self,
        parent: Option<NodeId>,
        name: SmolStr,
    ) -> Result<NodeId, LayoutError> {
        self.layout.create_element_style(parent, name)
    }

    #[inline]
    ///Creates a new layout and maps the given name to it.
    pub fn create_layout(&mut self, name: &str, style: taffy::Style) {
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

    ///Inserts a new HystBox on the ui.
    pub fn insert_box(
        &mut self,
        layout_id: NodeId,
        background: Vec4f32,
        rect: hyst_math::Rect,
    ) -> HystElementKey {
        let index = self
            .elements_renderer
            .insert_box(ContainerInstance::new(background, rect));
        self.elements.insert_with_key(|key| {
            self.roots.push(key);
            Box::new(HystBox::new(HystBoxCreationOption {
                index,
                parent: None,
                style: layout_id,
                key,
            }))
        })
    }
    ///Inserts a new HystText on the ui
    pub fn insert_text(
        &mut self,
        core: &mut RenderingCore,
        opts: HystTextOptions,
    ) -> Result<HystElementKey, LayoutError> {
        let style = self.generate_layout(None, opts.style)?;
        Ok(self.elements.insert_with_key(|key| {
            self.texts.push(key);
            Box::new(HystText::new(
                core,
                TextCreationOption {
                    key,
                    layout: style,
                    font_size: opts.font_size,
                    line_height: opts.font_size * 0.5,
                    position: opts.position,
                    content: opts.content,
                    color: opts.color,
                },
            ))
        }))
    }

    ///Inserts a new HystImage on the ui .
    pub fn insert_image(
        &mut self,
        core: &RenderingCore,
        rect: Rect,
        source: String,
        layout_id: NodeId,
    ) -> HystElementKey {
        self.elements.insert_with_key(|key| {
            self.roots.push(key);
            Box::new(HystImage::new(
                core,
                HystImageCreationOption {
                    source,
                    rect,
                    style: layout_id,
                    key,
                },
            ))
        })
    }

    #[inline]
    ///Gets the list of all Texts id's on the Ui
    pub fn texts(&self) -> &Vec<HystElementKey> {
        &self.texts
    }

    #[inline]
    ///Gets the list of all Texts in the ui
    pub fn text_elements(&self) -> Vec<&HystText> {
        self.texts
            .iter()
            .filter_map(|key| self.get_element_with_type(*key).unwrap())
            .collect()
    }

    #[inline]
    ///Gets the list of all elements in the Ui
    pub fn elements(&self) -> &SlotMap<HystElementKey, Box<dyn HystElement>> {
        &self.elements
    }

    #[inline]
    pub fn elements_mut(&mut self) -> &mut SlotMap<HystElementKey, Box<dyn HystElement>> {
        &mut self.elements
    }

    #[inline]
    ///Gets the keys of the root elements
    pub fn roots_keys(&self) -> &Vec<HystElementKey> {
        &self.roots
    }

    #[inline]
    ///Gets the element which has the given `key`
    pub fn get_element(&self, key: HystElementKey) -> Option<&Box<dyn HystElement>> {
        self.elements.get(key)
    }

    #[inline]
    ///Gets the element which has the given `key`
    pub fn get_element_mut(&mut self, key: HystElementKey) -> Option<&mut Box<dyn HystElement>> {
        self.elements.get_mut(key)
    }

    #[inline]
    ///Gets every root element
    pub fn roots(&self) -> Vec<&Box<dyn HystElement>> {
        self.roots
            .iter()
            .filter_map(|root| self.elements.get(*root))
            .collect()
    }

    ///Tries to get the element which has the given `key` casting it to <T>
    ///If the element does not exist, None will be returned.
    ///If the element does exist but it's type is incorrect, return Some(None), otherwhise, Some(Some(&element_ref))
    pub fn get_element_with_type<T: HystElement>(&self, key: HystElementKey) -> Option<Option<&T>> {
        let el = &**self.get_element(key)? as &dyn Any;
        Some(el.downcast_ref::<T>())
    }

    ///Gets a vector containing all the children of the element which has the given `key`
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

    #[inline]
    ///Recalculates the sizes of the layouts of the elements. Still only updates positioning
    fn recalc_layouts(&mut self, width: f32, height: f32) {
        self.layout.recalculate(width, height).unwrap();
    }

    ///Resizes the root and its children recursively
    pub fn resize_root(&mut self, core: &RenderingCore, root: HystElementKey) {
        let children: Vec<_> = {
            let Some(parent) = self.elements.get_mut(root) else {
                return;
            };
            let layout = self.layout.layout_of(parent.layout()).unwrap();
            {
                let parent = &mut **parent as &mut dyn Any;
                if let Some(bx) = parent.downcast_mut::<HystBox>() {
                    self.elements_renderer
                        .box_renderer_mut()
                        .resize(bx.instance_index(), layout)
                } else if let Some(_) = parent.downcast_mut::<HystImage>() {
                    println!("Must implement instanced drawing for images");
                    todo!()
                    //img.resize(core, self.elements_renderer.image_renderer_mut(), layout);
                } else {
                    panic!("What in the fuck is this element?");
                }
            }

            parent.children().iter().cloned().collect()
        };
        for child in children {
            self.resize_root(core, child);
        }
    }

    ///Resizes all the elements starts by their roots
    /// # Arguments
    /// `width` The current width of the window
    /// `height` The current height of the window
    pub fn resize_roots(&mut self, core: &RenderingCore, (width, height): (f32, f32)) {
        self.recalc_layouts(width, height);
        self.elements_renderer.set_size(core, width, height);
        let mut idx = 0;
        while let Some(root) = self.roots.get(idx) {
            idx += 1;
            self.resize_root(core, *root);
        }

        self.flush_modifications(core);
    }
}

impl Deref for ElementManager {
    type Target = Renderer;
    fn deref(&self) -> &Self::Target {
        &self.elements_renderer
    }
}

impl DerefMut for ElementManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.elements_renderer
    }
}
