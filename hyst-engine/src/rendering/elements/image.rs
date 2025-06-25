use hyst_math::Rect;
use taffy::NodeId;

use crate::{
    core::RenderingCore,
    meshes::image::{HystImageCreationOption, Image},
    ui::HystElementKey,
};

pub struct HystElementImageCreationOption {
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
    pub fn new(core: &mut RenderingCore, options: HystElementImageCreationOption) -> Self {
        Self {
            img: Image::from_configs(
                core,
                HystImageCreationOption {
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

    pub fn children(&self) -> &[HystElementKey] {
        &self.children
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
