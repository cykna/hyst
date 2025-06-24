use hyst_math::Rect;
use smol_str::SmolStr;

use crate::{
    core::RenderingCore,
    meshes::{
        SizeMethod,
        image::{HystImageCreationOption, Image},
    },
    ui::HystElementKey,
};

pub struct HystElementImageCreationOption {
    pub source: String,
    pub rect: Rect,
    pub size_method: SizeMethod,
    pub styles: Vec<SmolStr>,
    pub key: HystElementKey,
}

pub struct HystImage {
    img: Image,
    size_method: SizeMethod,
    key: HystElementKey,
    parent: Option<HystElementKey>,
    children: Vec<HystElementKey>,
    styles: Vec<SmolStr>,
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
            size_method: options.size_method,
            key: options.key,
            parent: None,
            children: Vec::new(),
            styles: options.styles,
        }
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
