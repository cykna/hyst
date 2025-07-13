use hyst_math::Rect;
use taffy::NodeId;

use crate::{
    core::RenderingCore,
    meshes::{
        Mesh,
        image::{Image, ImageCreationOption},
    },
    ui::HystElementKey,
};

use super::HystElement;

pub struct HystImageCreationOption {
    pub source: String,
    pub rect: Rect,
    pub style: NodeId,
    pub key: HystElementKey,
}
#[derive(Debug)]
pub struct HystImage {
    img: Image,
    key: HystElementKey,
    parent: Option<HystElementKey>,
    children: Vec<HystElementKey>,
    style: NodeId,
}

impl HystImage {
    pub fn new(core: &mut RenderingCore, options: HystImageCreationOption) -> Self {
        Self {
            img: Image::from_configs(
                core,
                ImageCreationOption {
                    rect: options.rect,
                    source: options.source,
                },
            )
            .unwrap(),
            key: options.key,
            parent: None,
            children: Vec::new(),
            style: options.style,
        }
    }

    pub fn style(&self) -> NodeId {
        self.style
    }

    pub fn parent(&self) -> Option<&HystElementKey> {
        self.parent.as_ref()
    }
}

impl std::ops::Deref for HystImage {
    type Target = Image;
    fn deref(&self) -> &Self::Target {
        &self.img
    }
}
impl std::ops::DerefMut for HystImage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.img
    }
}

impl HystElement for HystImage {
    fn id(&self) -> HystElementKey {
        self.key
    }
    fn layout(&self) -> NodeId {
        self.style
    }
    fn resize(
        &mut self,
        core: &mut RenderingCore,
        screen_size: (f32, f32),
        layout: &taffy::Layout,
    ) {
        self.img.resize(core, screen_size, layout);
    }
    fn children(&self) -> &Vec<HystElementKey> {
        &self.children
    }
    fn update(&mut self, core: &mut RenderingCore) {}
    fn render(&self, pass: &mut wgpu::RenderPass) {
        self.img.draw(pass);
    }
}
