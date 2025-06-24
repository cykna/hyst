mod options;
use ahash::RandomState;
pub use options::*;

use slotmap::SlotMap;
use smol_str::SmolStr;
use taffy::Style;
use wgpu::RenderPass;

use crate::{
    background::Background,
    core::RenderingCore,
    elements::{HystBox, HystBoxCreationOption, HystElementImageCreationOption, HystImage},
    meshes::{
        Mesh, SizeMethod,
        image::{HystImageCreationOption, Image},
    },
};
use hyst_math::{Rect, vectors::Vec4f32};

pub trait HystUiElement: Sized {
    fn new(core: &mut RenderingCore, options: HystElementOptions) -> Self;
}

pub struct HystElementOptions {
    pub parent: Option<HystElementKey>,
    pub size_method: SizeMethod,
    pub background: Background,
    pub initial_rect: Rect,
    pub key: HystElementKey,
}

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
}

slotmap::new_key_type! {pub struct HystElementKey;}

type StyleMap = hashbrown::HashMap<SmolStr, Style, RandomState>;

pub struct HystUi {
    core: RenderingCore,
    elements: SlotMap<HystElementKey, HystElement>,
    roots: Vec<HystElementKey>,
    styles: StyleMap,
    bg: Vec4f32,
}

impl HystUi {
    pub fn new(core: RenderingCore, bg: Vec4f32) -> Self {
        Self {
            core,
            elements: SlotMap::with_key(),
            roots: Vec::new(),
            styles: StyleMap::with_hasher(RandomState::new()),
            bg,
        }
    }
    pub fn create_box(&mut self, options: HystBoxOptions) {
        self.elements.insert_with_key(|key| {
            self.roots.push(key);
            HystElement::Box(HystBox::new(
                &mut self.core,
                HystBoxCreationOption {
                    background: options.bg,
                    rect: options.rect,
                    size_method: options.size_method,
                    parent: None,
                    key,
                },
            ))
        });
    }
    pub fn create_image(&mut self, options: HystImageOptions) {
        self.elements.insert_with_key(|key| {
            self.roots.push(key);
            HystElement::Image(HystImage::new(
                &mut self.core,
                HystElementImageCreationOption {
                    rect: options.rect,
                    source: options.source,
                    size_method: options.size_method,
                    styles: options.styles,
                    key,
                },
            ))
        });
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
            out.push(element);
            let children = match element {
                HystElement::Box(element) => element.children(),
                HystElement::Image(_) => return out,
            };
            for child_key in children {
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
            children.append(&mut self.get_children_of(*root));
        }
        self.core.draw(&children, self.bg);
    }

    pub fn resize_roots(&mut self, width: f32, height: f32) {
        for root_id in self.roots.iter() {
            let Some(root) = self.elements.get_mut(*root_id) else {
                continue;
            };
            match root {
                HystElement::Box(root) => {
                    {
                        let container = root.container_mut();
                        container
                            .screen_size()
                            .write_with(&self.core, [width, height]);
                    }
                    let SizeMethod::Percentage(w, h) = root.size_method() else {
                        continue;
                    };
                    let container = root.container_mut();
                    let area = container.area_buffer();
                    area.inner_mut()
                        .size_mut()
                        .set_coords(w * width, h * height);
                    area.write(&self.core);
                }
                HystElement::Image(img) => {
                    img.screen_size().write_with(&self.core, [width, height]);
                }
            }
        }
    }
}
